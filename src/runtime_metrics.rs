//! Runtime, process, and saturation metrics exposed on the Prometheus endpoint.
//!
//! NAIS auto-instrumentation only injects an OpenTelemetry agent for Java, Node.js,
//! Python, and .NET. Rust applications run in SDK-only mode, which means the runtime
//! metrics an agent would otherwise provide have to be produced by the application
//! itself. This module does that.
//!
//! Three groups of metrics are collected:
//!
//! - OS process metrics (`process_*`) via [`metrics_process`], using the same names
//!   the official Prometheus client libraries use.
//! - Tokio runtime metrics (`tokio_runtime_*`), limited to the counters that are
//!   stable without `--cfg tokio_unstable`.
//! - Compilation saturation (`pdfgenrs_compile_*`), derived from the semaphore that
//!   limits concurrent Typst compilations.

use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use metrics::{Unit, describe_gauge, gauge};
use metrics_process::Collector;
use tokio::sync::{Semaphore, oneshot};
use tokio::task::JoinHandle;

/// Default interval between metric collections.
pub const DEFAULT_COLLECTION_INTERVAL_SECONDS: u64 = 15;

const TOKIO_WORKERS: &str = "tokio_runtime_workers";
const TOKIO_ALIVE_TASKS: &str = "tokio_runtime_alive_tasks";
const TOKIO_GLOBAL_QUEUE_DEPTH: &str = "tokio_runtime_global_queue_depth";
const COMPILE_PERMITS_AVAILABLE: &str = "pdfgenrs_compile_permits_available";
const COMPILE_PERMITS_TOTAL: &str = "pdfgenrs_compile_permits_total";
const COMPILE_IN_FLIGHT: &str = "pdfgenrs_compile_in_flight";

/// Collects process, Tokio runtime, and compilation saturation metrics.
pub struct RuntimeMetricsCollector {
    process: Collector,
    compile_semaphore: Option<Arc<Semaphore>>,
    compile_permits_total: usize,
}

impl fmt::Debug for RuntimeMetricsCollector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RuntimeMetricsCollector")
            .field("compile_semaphore", &self.compile_semaphore)
            .field("compile_permits_total", &self.compile_permits_total)
            .finish_non_exhaustive()
    }
}

impl RuntimeMetricsCollector {
    /// Creates a collector.
    ///
    /// `compile_semaphore` and `compile_permits_total` come from [`crate::state::AppState`].
    /// When the semaphore is `None` the concurrency limit is disabled and the
    /// `pdfgenrs_compile_*` metrics are not emitted.
    pub fn new(compile_semaphore: Option<Arc<Semaphore>>, compile_permits_total: usize) -> Self {
        Self {
            process: Collector::default(),
            compile_semaphore,
            compile_permits_total,
        }
    }

    /// Registers metric descriptions so the Prometheus output carries `HELP` and `TYPE`.
    ///
    /// Call once at startup, before the first collection.
    pub fn describe(&self) {
        self.process.describe();

        describe_gauge!(
            TOKIO_WORKERS,
            Unit::Count,
            "Number of worker threads in the Tokio runtime."
        );
        describe_gauge!(
            TOKIO_ALIVE_TASKS,
            Unit::Count,
            "Number of alive tasks in the Tokio runtime."
        );
        describe_gauge!(
            TOKIO_GLOBAL_QUEUE_DEPTH,
            Unit::Count,
            "Number of tasks waiting in the Tokio global queue."
        );
        describe_gauge!(
            COMPILE_PERMITS_AVAILABLE,
            Unit::Count,
            "Available permits on the compilation semaphore. Zero means requests are queueing."
        );
        describe_gauge!(
            COMPILE_PERMITS_TOTAL,
            Unit::Count,
            "Configured maximum number of concurrent compilations."
        );
        describe_gauge!(
            COMPILE_IN_FLIGHT,
            Unit::Count,
            "Compilations currently holding a semaphore permit."
        );
    }

    /// Takes one sample of every metric and records it.
    pub fn collect(&self) {
        self.process.collect();
        self.collect_tokio();
        self.collect_compile_saturation();
    }

    /// Records Tokio runtime metrics.
    ///
    /// Only the metrics that are stable without `--cfg tokio_unstable` are read, since
    /// enabling that flag would propagate to every application built on this base image.
    /// Does nothing when called outside a Tokio runtime.
    fn collect_tokio(&self) {
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let runtime = handle.metrics();

        gauge!(TOKIO_WORKERS).set(runtime.num_workers() as f64);
        gauge!(TOKIO_ALIVE_TASKS).set(runtime.num_alive_tasks() as f64);
        gauge!(TOKIO_GLOBAL_QUEUE_DEPTH).set(runtime.global_queue_depth() as f64);
    }

    /// Records how saturated the compilation semaphore is.
    ///
    /// This is the most direct answer to whether the application is overloaded: when no
    /// permits are available, requests queue and eventually time out with 503.
    fn collect_compile_saturation(&self) {
        let Some(ref semaphore) = self.compile_semaphore else {
            return;
        };

        let available = semaphore.available_permits();
        let in_flight = self.compile_permits_total.saturating_sub(available);

        gauge!(COMPILE_PERMITS_AVAILABLE).set(available as f64);
        gauge!(COMPILE_PERMITS_TOTAL).set(self.compile_permits_total as f64);
        gauge!(COMPILE_IN_FLIGHT).set(in_flight as f64);
    }

    /// Describes the metrics, takes an initial sample, and spawns a background task
    /// that samples every `interval` until the returned handle is shut down.
    pub fn spawn(self, interval: Duration) -> RuntimeMetricsHandle {
        self.describe();
        self.collect();

        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let join = tokio::spawn(async move {
            let mut ticker = tokio::time::interval(interval);
            // The first tick completes immediately; the initial sample is already recorded.
            ticker.tick().await;
            loop {
                tokio::select! {
                    _ = ticker.tick() => self.collect(),
                    _ = &mut shutdown_rx => break,
                }
            }
        });

        RuntimeMetricsHandle { shutdown_tx, join }
    }
}

/// Handle to the background collection task.
#[derive(Debug)]
pub struct RuntimeMetricsHandle {
    shutdown_tx: oneshot::Sender<()>,
    join: JoinHandle<()>,
}

impl RuntimeMetricsHandle {
    /// Stops the background task and waits for it to finish.
    pub async fn shutdown(self) {
        // An error means the task already stopped, which is the desired end state anyway.
        let _ = self.shutdown_tx.send(());
        if let Err(e) = self.join.await {
            tracing::warn!(error = %e, "Runtime metrics task did not shut down cleanly");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics_exporter_prometheus::PrometheusBuilder;

    fn rendered_with<F: FnOnce()>(f: F) -> String {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, f);
        handle.render()
    }

    #[test]
    fn collect_emits_process_metrics() {
        let output = rendered_with(|| {
            RuntimeMetricsCollector::new(None, 0).collect();
        });

        assert!(
            output.contains("process_cpu_seconds_total"),
            "expected process CPU metric in output: {output}"
        );
        assert!(
            output.contains("process_resident_memory_bytes"),
            "expected resident memory metric in output: {output}"
        );
    }

    #[test]
    fn collect_omits_compile_metrics_without_semaphore() {
        let output = rendered_with(|| {
            RuntimeMetricsCollector::new(None, 0).collect();
        });

        assert!(
            !output.contains(COMPILE_PERMITS_AVAILABLE),
            "expected no compile metrics when the semaphore is disabled: {output}"
        );
    }

    #[tokio::test]
    async fn collect_reports_compile_saturation() -> anyhow::Result<()> {
        let semaphore = Arc::new(Semaphore::new(4));
        let permit = semaphore.clone().acquire_owned().await?;

        let output = rendered_with(|| {
            RuntimeMetricsCollector::new(Some(semaphore.clone()), 4).collect();
        });

        assert!(
            output.contains(&format!("{COMPILE_PERMITS_AVAILABLE} 3")),
            "expected 3 available permits: {output}"
        );
        assert!(
            output.contains(&format!("{COMPILE_IN_FLIGHT} 1")),
            "expected 1 compilation in flight: {output}"
        );
        assert!(
            output.contains(&format!("{COMPILE_PERMITS_TOTAL} 4")),
            "expected 4 total permits: {output}"
        );

        drop(permit);
        Ok(())
    }

    #[tokio::test]
    async fn collect_reports_tokio_runtime_metrics() {
        let output = rendered_with(|| {
            RuntimeMetricsCollector::new(None, 0).collect();
        });

        assert!(
            output.contains(TOKIO_WORKERS),
            "expected tokio worker metric in output: {output}"
        );
        assert!(
            output.contains(TOKIO_ALIVE_TASKS),
            "expected tokio alive tasks metric in output: {output}"
        );
        assert!(
            output.contains(TOKIO_GLOBAL_QUEUE_DEPTH),
            "expected tokio queue depth metric in output: {output}"
        );
    }

    #[test]
    fn collect_skips_tokio_metrics_outside_runtime() {
        let output = rendered_with(|| {
            RuntimeMetricsCollector::new(None, 0).collect();
        });

        assert!(
            !output.contains(TOKIO_WORKERS),
            "expected no tokio metrics outside a runtime: {output}"
        );
    }

    #[test]
    fn describe_emits_help_and_type() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let collector = RuntimeMetricsCollector::new(Some(Arc::new(Semaphore::new(2))), 2);
            collector.describe();
            collector.collect();
        });

        let output = handle.render();
        assert!(
            output.contains(&format!("# HELP {COMPILE_PERMITS_AVAILABLE}")),
            "expected HELP line for compile permits: {output}"
        );
        assert!(
            output.contains(&format!("# TYPE {COMPILE_PERMITS_AVAILABLE} gauge")),
            "expected TYPE line for compile permits: {output}"
        );
    }

    #[tokio::test]
    async fn spawned_task_collects_and_shuts_down() -> anyhow::Result<()> {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();

        let metrics_handle = metrics::with_local_recorder(&recorder, || {
            RuntimeMetricsCollector::new(Some(Arc::new(Semaphore::new(2))), 2)
                .spawn(Duration::from_millis(10))
        });

        let output = handle.render();
        assert!(
            output.contains(COMPILE_PERMITS_AVAILABLE),
            "expected an initial sample before the first tick: {output}"
        );

        metrics_handle.shutdown().await;
        Ok(())
    }
}
