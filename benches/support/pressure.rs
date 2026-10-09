use std::future::IntoFuture;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use anyhow::Context;
use reqwest::{Client, StatusCode, header};
use tokio::sync::{Semaphore, oneshot};
use tokio::task::JoinSet;
use tracing::info;

use super::{BENCH_HTML_BODY, append_summary, create_bench_state, support};
use pdfgenrs::{build_router, metrics};

const SAMPLE_INTERVAL: Duration = Duration::from_millis(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(8);
const IMAGE_SIDE: u32 = 2048;
const ENDPOINTS: [&str; 4] = ["typst-pdf", "image-pdf", "html-pdf", "generated-html"];

#[derive(Debug)]
pub(super) struct PressureConfig {
    duration: Duration,
    concurrency: Vec<usize>,
    permits: usize,
}

impl PressureConfig {
    pub(super) fn from_env() -> anyhow::Result<Self> {
        Self::parse(|name| {
            std::env::var(name).map(Some).or_else(|error| match error {
                std::env::VarError::NotPresent => Ok(None),
                other => Err(other.into()),
            })
        })
    }

    fn parse(get: impl Fn(&str) -> anyhow::Result<Option<String>>) -> anyhow::Result<Self> {
        let integer = |name: &str, default: usize, max: usize| -> anyhow::Result<usize> {
            let raw = get(name)?.unwrap_or_else(|| default.to_string());
            let value = raw
                .parse::<usize>()
                .with_context(|| format!("invalid {name}: {raw}"))?;
            anyhow::ensure!((1..=max).contains(&value), "{name} must be 1..={max}");
            Ok(value)
        };
        let seconds = integer("PDFGEN_BENCH_DURATION_SECONDS", 1, 60)?;
        let permits = integer("PDFGEN_BENCH_COMPILE_PERMITS", 4, 32)?;
        let raw = get("PDFGEN_BENCH_CONCURRENCY")?.unwrap_or_else(|| "1,4,8".into());
        let concurrency = raw
            .split(',')
            .map(|entry| {
                let value = entry
                    .trim()
                    .parse::<usize>()
                    .context("invalid PDFGEN_BENCH_CONCURRENCY")?;
                anyhow::ensure!((1..=32).contains(&value), "concurrency must be 1..=32");
                Ok(value)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        anyhow::ensure!(
            concurrency.len() <= 8,
            "at most 8 concurrency levels are allowed"
        );
        let mut distinct = concurrency.clone();
        distinct.sort_unstable();
        distinct.dedup();
        anyhow::ensure!(
            distinct.len() == concurrency.len(),
            "concurrency levels must be distinct"
        );
        Ok(Self {
            duration: Duration::from_secs(seconds as u64),
            concurrency,
            permits,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Success,
    Overload,
    ServerTimeout,
    ClientTimeout,
    Unexpected,
}

#[derive(Debug)]
struct Observation {
    endpoint: usize,
    latency_ms: f64,
    outcome: Outcome,
    error: Option<String>,
}

#[derive(Default, Debug)]
struct Totals {
    success: usize,
    overload: usize,
    server_timeout: usize,
    client_timeout: usize,
    unexpected: usize,
    latencies: Vec<f64>,
}

impl Totals {
    fn add(&mut self, observation: &Observation) {
        self.latencies.push(observation.latency_ms);
        match observation.outcome {
            Outcome::Success => self.success += 1,
            Outcome::Overload => self.overload += 1,
            Outcome::ServerTimeout => self.server_timeout += 1,
            Outcome::ClientTimeout => self.client_timeout += 1,
            Outcome::Unexpected => self.unexpected += 1,
        }
    }

    fn report(
        &mut self,
        workload: &str,
        concurrency: usize,
        endpoint: &str,
        elapsed: f64,
    ) -> String {
        self.latencies.sort_unstable_by(f64::total_cmp);
        let total = self.latencies.len();
        let p50 = percentile(&self.latencies, 50);
        let p95 = percentile(&self.latencies, 95);
        let p99 = percentile(&self.latencies, 99);
        let throughput = total as f64 / elapsed;
        let success_throughput = self.success as f64 / elapsed;
        info!(
            workload,
            concurrency,
            endpoint,
            total,
            success = self.success,
            overload_503 = self.overload,
            server_timeout_408 = self.server_timeout,
            client_timeout = self.client_timeout,
            unexpected = self.unexpected,
            elapsed_seconds = elapsed,
            completion_req_per_second = throughput,
            success_req_per_second = success_throughput,
            p50_ms = p50,
            p95_ms = p95,
            p99_ms = p99,
            "Pressure request results (latency includes body receipt, all outcomes)"
        );
        format!(
            "| {workload} | {concurrency} | {endpoint} | {total} | {} | {} | {} | {} | {} | {elapsed:.3} | {throughput:.2} | {success_throughput:.2} | {p50:.2} | {p95:.2} | {p99:.2} |\n",
            self.success, self.overload, self.server_timeout, self.client_timeout, self.unexpected
        )
    }
}

fn percentile(sorted: &[f64], percent: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    sorted[(sorted.len() * percent).div_ceil(100).saturating_sub(1)]
}

fn parse_rss(status: &str) -> Option<u64> {
    let line = status.lines().find(|line| line.starts_with("VmRSS:"))?;
    let mut fields = line.split_whitespace();
    fields.next()?;
    let kib = fields.next()?.parse::<u64>().ok()?;
    (fields.next()? == "kB").then(|| kib * 1024)
}

fn current_rss() -> anyhow::Result<u64> {
    anyhow::ensure!(
        cfg!(target_os = "linux"),
        "current-process RSS sampling unsupported on this OS"
    );
    parse_rss(&std::fs::read_to_string("/proc/self/status")?)
        .context("current-process VmRSS unavailable in /proc/self/status")
}

#[derive(Debug)]
struct Samples {
    count: u64,
    zero_free: u64,
    min_free: usize,
    baseline: Option<u64>,
    peak: Option<u64>,
    end: Option<u64>,
    rss_error: Option<String>,
}

impl Samples {
    fn new(permits: usize) -> Self {
        let baseline = current_rss();
        let rss_error = baseline.as_ref().err().map(ToString::to_string);
        let baseline = baseline.ok();
        Self {
            count: 0,
            zero_free: 0,
            min_free: permits,
            baseline,
            peak: baseline,
            end: baseline,
            rss_error,
        }
    }

    fn sample(&mut self, semaphore: Option<&Semaphore>) {
        if let Some(semaphore) = semaphore {
            self.count += 1;
            let free = semaphore.available_permits();
            self.min_free = self.min_free.min(free);
            self.zero_free += u64::from(free == 0);
        }
        match current_rss() {
            Ok(rss) => {
                self.peak = Some(self.peak.unwrap_or(rss).max(rss));
                self.end = Some(rss);
            }
            Err(error) => self.rss_error = Some(error.to_string()),
        }
    }

    fn report(&self, workload: &str, concurrency: usize, permits: usize, elapsed: f64) -> String {
        let saturated_percent = 100.0 * self.zero_free as f64 / self.count as f64;
        let memory = if let (Some(baseline), Some(peak), Some(end), None) =
            (self.baseline, self.peak, self.end, &self.rss_error)
        {
            let mib = |bytes: u64| bytes as f64 / 1_048_576.0;
            let delta = mib(end) - mib(baseline);
            info!(
                workload,
                concurrency,
                rss_baseline_mib = mib(baseline),
                rss_sampled_peak_mib = mib(peak),
                rss_end_mib = mib(end),
                rss_end_minus_baseline_mib = delta,
                "Current-process RSS samples (not lifetime high-water mark)"
            );
            format!(
                "{:.2} | {:.2} | {:.2} | {delta:+.2}",
                mib(baseline),
                mib(peak),
                mib(end)
            )
        } else {
            info!(workload, concurrency, reason = ?self.rss_error, "RSS sampling unsupported/unavailable");
            "unsupported/unavailable | unsupported/unavailable | unsupported/unavailable | unsupported/unavailable".into()
        };
        info!(
            workload,
            concurrency,
            compile_permits = permits,
            samples = self.count,
            min_free_permits = self.min_free,
            zero_free_samples = self.zero_free,
            saturated_sample_percent = saturated_percent,
            elapsed_seconds = elapsed,
            "Measured compile semaphore pressure (10ms samples)"
        );
        format!(
            "| {workload} | {concurrency} | {permits} | {} | {} | {} | {saturated_percent:.2} | {memory} |\n",
            self.count, self.min_free, self.zero_free
        )
    }
}

async fn request(
    client: &Client,
    base: &str,
    endpoint: usize,
    sequence: u64,
    fixtures: &[Vec<u8>],
) -> anyhow::Result<Outcome> {
    let request = match endpoint {
        0 | 3 => {
            let kind = if endpoint == 0 { "genpdf" } else { "genhtml" };
            client.post(format!("{base}/api/v1/{kind}/pressure/document"))
                .json(&serde_json::json!({"sequence": sequence, "body": format!("Unique request {sequence}")}))
        }
        1 => client
            .post(format!("{base}/api/v1/genpdf/image/pressure"))
            .header(header::CONTENT_TYPE, "image/png")
            .body(support::unique_png(
                &fixtures[((sequence / 4) % fixtures.len() as u64) as usize],
                sequence,
            )),
        2 => client
            .post(format!("{base}/api/v1/genpdf/html/pressure"))
            .header(header::CONTENT_TYPE, "text/html")
            .body(BENCH_HTML_BODY.replace(
                "performance test document.",
                &format!("unique request {sequence}."),
            )),
        _ => anyhow::bail!("invalid endpoint"),
    };
    let response = request.send().await?;
    let status = response.status();
    let content_type = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_owned();
    let retry_after = response
        .headers()
        .get(header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());
    let bytes = response.bytes().await?;
    validate_response(
        status,
        &content_type,
        retry_after,
        &bytes,
        endpoint,
        sequence,
    )
}

fn validate_response(
    status: StatusCode,
    content_type: &str,
    retry_after: Option<u64>,
    bytes: &[u8],
    endpoint: usize,
    sequence: u64,
) -> anyhow::Result<Outcome> {
    match status {
        StatusCode::OK => {
            if endpoint == 3 {
                anyhow::ensure!(
                    content_type.starts_with("text/html"),
                    "unexpected HTML content type: {content_type}"
                );
                let html = std::str::from_utf8(bytes)?;
                anyhow::ensure!(
                    html.contains("<html")
                        && html.contains("</html>")
                        && html.contains(&format!("Unique request {sequence}")),
                    "invalid or stale generated HTML"
                );
            } else {
                anyhow::ensure!(
                    content_type.starts_with("application/pdf"),
                    "unexpected PDF content type: {content_type}"
                );
                support::validate_pdf(bytes)?;
            }
            Ok(Outcome::Success)
        }
        StatusCode::SERVICE_UNAVAILABLE | StatusCode::REQUEST_TIMEOUT => {
            anyhow::ensure!(
                content_type.starts_with("application/problem+json"),
                "invalid overload/timeout content type"
            );
            let problem: serde_json::Value = serde_json::from_slice(bytes)?;
            let expected_type = if status == StatusCode::SERVICE_UNAVAILABLE {
                anyhow::ensure!(retry_after.is_some(), "overload missing valid Retry-After");
                "urn:pdfgenrs:error:overloaded"
            } else {
                "urn:pdfgenrs:error:timeout"
            };
            anyhow::ensure!(
                problem["status"].as_u64() == Some(u64::from(status.as_u16()))
                    && problem["type"].as_str() == Some(expected_type),
                "unexpected overload/timeout problem: {problem}"
            );
            Ok(if status == StatusCode::SERVICE_UNAVAILABLE {
                Outcome::Overload
            } else {
                Outcome::ServerTimeout
            })
        }
        _ => anyhow::bail!(
            "unexpected HTTP {status}: {}",
            String::from_utf8_lossy(&bytes[..bytes.len().min(512)])
        ),
    }
}

pub(super) async fn run(config: &PressureConfig) -> anyhow::Result<()> {
    check_drain().await?;
    let fixtures = Arc::new(
        (0..8)
            .map(|variant| support::raster_png(IMAGE_SIDE, variant))
            .collect::<Vec<_>>(),
    );
    let mut state = create_bench_state()?;
    state.config.max_concurrent_compilations = config.permits;
    state.config.request_body_limit_bytes = 8 * 1024 * 1024;
    state.config.semaphore_acquire_timeout_seconds = 1;
    state.config.compile_timeout_seconds = 5;
    let semaphore = Arc::new(Semaphore::new(config.permits));
    state.compile_semaphore = Some(Arc::clone(&semaphore));
    let mut templates = state.templates.as_ref().clone();
    templates.insert(
        ("pressure".into(), "document".into()),
        Arc::from("#set document(title: \"Pressure benchmark\", date: auto)\n#let data = json(\"/data/pressure/document.json\")\n= Pressure benchmark\n#data.body\n#data.sequence"),
    );
    state.templates = Arc::new(templates);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let base = Arc::new(format!("http://{}", listener.local_addr()?));
    let server = tokio::spawn(
        axum::serve(
            listener,
            build_router(state, metrics::test_metrics_handle()),
        )
        .into_future(),
    );
    let result = sweeps(config, &fixtures, &base, &semaphore).await;
    let drained = drain_compilations(&semaphore, config.permits).await;
    server.abort();
    let _ = server.await;
    result?;
    drained
}

async fn drain_compilations(semaphore: &Semaphore, permits: usize) -> anyhow::Result<()> {
    let guard = tokio::time::timeout(CLIENT_TIMEOUT, semaphore.acquire_many(permits as u32))
        .await
        .context("compilations did not drain within 8 seconds")??;
    drop(guard);
    Ok(())
}

async fn check_drain() -> anyhow::Result<()> {
    let semaphore = Arc::new(Semaphore::new(2));
    let permit = Arc::clone(&semaphore).try_acquire_owned()?;
    let release = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(20)).await;
        drop(permit);
    });
    drain_compilations(&semaphore, 2).await?;
    anyhow::ensure!(
        semaphore.available_permits() == 2,
        "drain did not wait for retained compile permit"
    );
    release.await?;
    Ok(())
}

async fn sweeps(
    config: &PressureConfig,
    fixtures: &Arc<Vec<Vec<u8>>>,
    base: &Arc<String>,
    semaphore: &Arc<Semaphore>,
) -> anyhow::Result<()> {
    let client = Client::builder().timeout(CLIENT_TIMEOUT).build()?;
    for endpoint in 0..4 {
        anyhow::ensure!(
            matches!(
                request(&client, base, endpoint, endpoint as u64, fixtures).await?,
                Outcome::Success
            ),
            "pressure preflight must succeed"
        );
    }
    let sequence = Arc::new(AtomicU64::new(4));
    let mut request_rows = String::new();
    let mut resource_rows = String::new();
    let mut errors = Vec::new();
    for workload in ["large-image", "mixed"] {
        for &concurrency in &config.concurrency {
            let mut samples = Samples::new(config.permits);
            samples.sample(Some(semaphore));
            let (stop_tx, mut stop_rx) = oneshot::channel();
            let sampled_semaphore = Arc::clone(semaphore);
            let pressure_sampling = Arc::new(AtomicBool::new(true));
            let sampled_pressure = Arc::clone(&pressure_sampling);
            let sampler = tokio::spawn(async move {
                let mut samples = samples;
                let mut interval = tokio::time::interval(SAMPLE_INTERVAL);
                interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                loop {
                    tokio::select! {
                        _ = interval.tick() => samples.sample(
                            sampled_pressure.load(Ordering::Acquire).then_some(sampled_semaphore.as_ref())
                        ),
                        _ = &mut stop_rx => {
                            samples.sample(None);
                            return samples;
                        }
                    }
                }
            });
            let start = Instant::now();
            let deadline = start + config.duration;
            let mut workers = JoinSet::new();
            for _ in 0..concurrency {
                let client = client.clone();
                let base = Arc::clone(base);
                let fixtures = Arc::clone(fixtures);
                let sequence = Arc::clone(&sequence);
                workers.spawn(async move {
                    let mut observations = Vec::new();
                    while Instant::now() < deadline {
                        let sequence = sequence.fetch_add(1, Ordering::Relaxed);
                        let endpoint = if workload == "large-image" {
                            1
                        } else {
                            (sequence % 4) as usize
                        };
                        let start = Instant::now();
                        let result = request(&client, &base, endpoint, sequence, &fixtures).await;
                        let (outcome, error) = match result {
                            Ok(outcome) => (outcome, None),
                            Err(error)
                                if error
                                    .downcast_ref::<reqwest::Error>()
                                    .is_some_and(reqwest::Error::is_timeout) =>
                            {
                                (Outcome::ClientTimeout, None)
                            }
                            Err(error) => {
                                (Outcome::Unexpected, Some(format!("{endpoint}: {error:#}")))
                            }
                        };
                        observations.push(Observation {
                            endpoint,
                            latency_ms: start.elapsed().as_secs_f64() * 1000.0,
                            outcome,
                            error,
                        });
                    }
                    observations
                });
            }
            let mut totals = Totals::default();
            let mut endpoints: [Totals; 4] = std::array::from_fn(|_| Totals::default());
            while let Some(worker) = workers.join_next().await {
                for observation in worker? {
                    totals.add(&observation);
                    endpoints[observation.endpoint].add(&observation);
                    if let Some(error) = observation.error
                        && errors.len() < 10
                    {
                        errors.push(error);
                    }
                }
            }
            pressure_sampling.store(false, Ordering::Release);
            drain_compilations(semaphore, config.permits).await?;
            let elapsed = start.elapsed().as_secs_f64();
            let _ = stop_tx.send(());
            let samples = sampler.await?;
            anyhow::ensure!(
                !totals.latencies.is_empty(),
                "no pressure requests completed"
            );
            request_rows.push_str(&totals.report(workload, concurrency, "TOTAL", elapsed));
            for (index, endpoint) in endpoints.iter_mut().enumerate() {
                if !endpoint.latencies.is_empty() {
                    request_rows.push_str(&endpoint.report(
                        workload,
                        concurrency,
                        ENDPOINTS[index],
                        elapsed,
                    ));
                }
            }
            resource_rows.push_str(&samples.report(workload, concurrency, config.permits, elapsed));
        }
    }
    append_summary(&format!(
        "\n## Duration-driven request pressure\n\nClosed-loop load: {}s dispatch window per sweep; concurrency {:?}; {} compile permits; {}px grayscale PNG (8 pixel variants plus unique request metadata). Fixtures and successful endpoint preflight are outside measurement. Server acquire/compile timeouts: 1s/5s; client timeout: 8s. Elapsed includes final in-flight requests and retained-compilation drain; throughput uses that elapsed time. Latencies include preparation, HTTP and full body receipt, across **all outcomes** (nearest-rank percentiles). TOTAL is the sum of endpoint counts, not an average of their percentiles. No new timing gates.\n\n\
         | Workload | Concurrency | Endpoint | Attempts | 200 | 503 | 408 | Client timeout | Unexpected | Elapsed (s) | Completed (req/s) | Successful (req/s) | p50 (ms) | p95 (ms) | p99 (ms) |\n\
         |---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n{request_rows}\n\
         ### Sampled resources\n\n10ms RSS samples continue until all compile permits have been acquired and released (bounded 8s drain), including compilations retained after 408 responses. Semaphore samples cover request pressure only, excluding the drain barrier itself. Zero-free samples measure actual semaphore saturation, not configured concurrency; sampling can miss short peaks. RSS is current-process `/proc/self/status` VmRSS, including the in-process HTTP server/client and caches; baseline is after fixtures/preflight, peak is the maximum **within this sweep**, end is after compilation drain, delta is end minus baseline (may be negative). Unsupported/unavailable RSS is explicit.\n\n\
         | Workload | Concurrency | Compile permits | Samples | Min free permits | Zero-free samples | Saturated samples (%) | Baseline RSS (MiB) | Sampled peak RSS (MiB) | End RSS (MiB) | Delta RSS (MiB) |\n\
         |---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n{resource_rows}\n",
        config.duration.as_secs(),
        config.concurrency,
        config.permits,
        IMAGE_SIDE
    ))?;
    anyhow::ensure!(
        errors.is_empty(),
        "unexpected pressure failures: {}",
        errors.join("; ")
    );
    Ok(())
}

pub(super) fn check_helpers() -> anyhow::Result<()> {
    let defaults = PressureConfig::parse(|_| Ok(None))?;
    anyhow::ensure!(
        defaults.concurrency == [1, 4, 8]
            && defaults.permits == 4
            && defaults.duration.as_secs() == 1,
        "invalid defaults"
    );
    for (key, value) in [
        ("PDFGEN_BENCH_DURATION_SECONDS", "0"),
        ("PDFGEN_BENCH_DURATION_SECONDS", "61"),
        ("PDFGEN_BENCH_DURATION_SECONDS", "bad"),
        ("PDFGEN_BENCH_COMPILE_PERMITS", "0"),
        ("PDFGEN_BENCH_CONCURRENCY", ""),
        ("PDFGEN_BENCH_CONCURRENCY", "1,1"),
        ("PDFGEN_BENCH_CONCURRENCY", "33"),
        ("PDFGEN_BENCH_CONCURRENCY", "1,"),
    ] {
        anyhow::ensure!(
            PressureConfig::parse(|name| Ok((name == key).then(|| value.into()))).is_err(),
            "invalid {key} accepted"
        );
    }
    anyhow::ensure!(
        percentile(&[1.0, 2.0, 3.0, 4.0], 50) == 2.0
            && percentile(&[1.0, 2.0, 3.0, 4.0], 99) == 4.0,
        "invalid percentiles"
    );
    anyhow::ensure!(
        parse_rss("VmRSS:\t2048 kB\n") == Some(2 * 1024 * 1024)
            && parse_rss("VmHWM: 4096 kB").is_none(),
        "invalid current RSS parser"
    );
    for (status, kind, outcome) in [
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "overloaded",
            Outcome::Overload,
        ),
        (
            StatusCode::REQUEST_TIMEOUT,
            "timeout",
            Outcome::ServerTimeout,
        ),
    ] {
        let body = serde_json::to_vec(&serde_json::json!({
            "status": status.as_u16(), "type": format!("urn:pdfgenrs:error:{kind}")
        }))?;
        let result = validate_response(status, "application/problem+json", Some(1), &body, 0, 0)?;
        anyhow::ensure!(
            std::mem::discriminant(&result) == std::mem::discriminant(&outcome),
            "incorrect overload/timeout classification"
        );
        anyhow::ensure!(
            validate_response(status, "application/problem+json", Some(1), b"{}", 0, 0).is_err(),
            "malformed error response accepted"
        );
    }
    anyhow::ensure!(
        validate_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "application/problem+json",
            None,
            b"{}",
            0,
            0
        )
        .is_err()
            && validate_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "text/plain",
                None,
                b"failure",
                0,
                0
            )
            .is_err()
            && validate_response(StatusCode::OK, "application/pdf", None, b"broken", 0, 0).is_err()
            && validate_response(
                StatusCode::OK,
                "text/html",
                None,
                b"<html>Unique request 1</html>",
                3,
                2
            )
            .is_err(),
        "invalid or unexpected response accepted"
    );
    Ok(())
}
