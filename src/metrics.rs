use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Instant;

use axum::{
    body::Body, extract::MatchedPath, extract::State, http::Request, middleware::Next,
    response::Response,
};

const DURATION_BUCKETS: [f64; 11] = [
    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
];
const SIZE_BUCKETS: [f64; 7] = [
    100.0,
    500.0,
    1_000.0,
    5_000.0,
    10_000.0,
    100_000.0,
    1_000_000.0,
];

#[derive(Clone, Debug, Default)]
pub struct MetricsHandle {
    inner: Arc<Mutex<MetricsState>>,
}

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Labels {
    method: String,
    path: String,
    status: String,
}

#[derive(Clone, Debug)]
struct HistogramState {
    bucket_counts: Vec<u64>,
    count: u64,
    sum: f64,
}

impl HistogramState {
    fn new(bucket_count: usize) -> Self {
        Self {
            bucket_counts: vec![0; bucket_count],
            count: 0,
            sum: 0.0,
        }
    }

    fn record(&mut self, value: f64, buckets: &[f64]) {
        for (bucket_count, bucket) in self.bucket_counts.iter_mut().zip(buckets) {
            if value <= *bucket {
                *bucket_count += 1;
            }
        }
        self.count += 1;
        self.sum += value;
    }
}

#[derive(Debug, Default)]
struct MetricsState {
    request_counts: BTreeMap<Labels, u64>,
    request_durations: BTreeMap<Labels, HistogramState>,
    request_body_sizes: BTreeMap<Labels, HistogramState>,
    response_body_sizes: BTreeMap<Labels, HistogramState>,
}

impl MetricsHandle {
    pub fn render(&self) -> String {
        let state = self.lock_state();
        let mut output = String::new();

        render_counter(
            &mut output,
            "http_requests_total",
            "Total number of HTTP requests",
            &state.request_counts,
        );
        render_histogram(
            &mut output,
            "http_request_duration_seconds",
            "Request latency distribution",
            &state.request_durations,
            &DURATION_BUCKETS,
        );
        render_histogram(
            &mut output,
            "http_request_body_size_bytes",
            "Request body size distribution",
            &state.request_body_sizes,
            &SIZE_BUCKETS,
        );
        render_histogram(
            &mut output,
            "http_response_body_size_bytes",
            "Response body size distribution",
            &state.response_body_sizes,
            &SIZE_BUCKETS,
        );

        output
    }

    fn record_request(
        &self,
        method: String,
        path: String,
        status: String,
        duration: f64,
        request_body_size: Option<u64>,
        response_body_size: Option<u64>,
    ) {
        let labels = Labels {
            method,
            path,
            status,
        };
        let state = &mut *self.lock_state();

        *state.request_counts.entry(labels.clone()).or_default() += 1;
        state
            .request_durations
            .entry(labels.clone())
            .or_insert_with(|| HistogramState::new(DURATION_BUCKETS.len()))
            .record(duration, &DURATION_BUCKETS);

        if let Some(size) = request_body_size {
            state
                .request_body_sizes
                .entry(labels.clone())
                .or_insert_with(|| HistogramState::new(SIZE_BUCKETS.len()))
                .record(size as f64, &SIZE_BUCKETS);
        }

        if let Some(size) = response_body_size {
            state
                .response_body_sizes
                .entry(labels)
                .or_insert_with(|| HistogramState::new(SIZE_BUCKETS.len()))
                .record(size as f64, &SIZE_BUCKETS);
        }
    }

    fn lock_state(&self) -> MutexGuard<'_, MetricsState> {
        self.inner
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }
}

/// Creates the shared metrics handle used by the application.
pub fn setup_metrics_recorder() -> anyhow::Result<MetricsHandle> {
    Ok(MetricsHandle::default())
}

/// Creates an isolated metrics handle for tests.
pub fn test_metrics_handle() -> MetricsHandle {
    MetricsHandle::default()
}

/// Axum middleware that records HTTP request metrics.
///
/// Records:
/// - `http_requests_total` counter with labels: method, path, status
/// - `http_request_duration_seconds` histogram with labels: method, path, status
/// - `http_request_body_size_bytes` histogram with labels: method, path, status
/// - `http_response_body_size_bytes` histogram with labels: method, path, status
pub async fn track_metrics(
    State(metrics_handle): State<MetricsHandle>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|matched_path| matched_path.as_str().to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    let request_body_size = request
        .headers()
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());

    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed().as_secs_f64();

    let status = response.status().as_u16().to_string();
    let response_body_size = response
        .headers()
        .get(axum::http::header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok());

    metrics_handle.record_request(
        method,
        path,
        status,
        duration,
        request_body_size,
        response_body_size,
    );

    response
}

fn render_counter(output: &mut String, name: &str, help: &str, metrics: &BTreeMap<Labels, u64>) {
    if metrics.is_empty() {
        return;
    }

    let _ = writeln!(output, "# HELP {name} {help}");
    let _ = writeln!(output, "# TYPE {name} counter");
    for (labels, value) in metrics {
        let _ = writeln!(output, r#"{name}{{{}}} {value}"#, labels.render());
    }
}

fn render_histogram(
    output: &mut String,
    name: &str,
    help: &str,
    metrics: &BTreeMap<Labels, HistogramState>,
    buckets: &[f64],
) {
    if metrics.is_empty() {
        return;
    }

    let _ = writeln!(output, "# HELP {name} {help}");
    let _ = writeln!(output, "# TYPE {name} histogram");
    for (labels, histogram) in metrics {
        for (index, bucket) in buckets.iter().enumerate() {
            let _ = writeln!(
                output,
                r#"{name}_bucket{{{},le="{}"}} {}"#,
                labels.render(),
                bucket,
                histogram.bucket_counts[index]
            );
        }
        let _ = writeln!(
            output,
            r#"{name}_bucket{{{},le="+Inf"}} {}"#,
            labels.render(),
            histogram.count
        );
        let _ = writeln!(
            output,
            r#"{name}_sum{{{}}} {}"#,
            labels.render(),
            histogram.sum
        );
        let _ = writeln!(
            output,
            r#"{name}_count{{{}}} {}"#,
            labels.render(),
            histogram.count
        );
    }
}

impl Labels {
    fn render(&self) -> String {
        format!(
            r#"method="{}",path="{}",status="{}""#,
            escape_label_value(&self.method),
            escape_label_value(&self.path),
            escape_label_value(&self.status),
        )
    }
}

fn escape_label_value(value: &str) -> String {
    value
        .replace('\\', r"\\")
        .replace('\n', r"\n")
        .replace('"', r#"\""#)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use axum::http::StatusCode;
    use axum::{Router, middleware, routing::get, routing::post};
    use axum_test::TestServer;

    fn build_runtime() -> anyhow::Result<tokio::runtime::Runtime> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .context("failed to build runtime")
    }

    async fn handler() -> StatusCode {
        StatusCode::OK
    }

    async fn not_found_handler() -> StatusCode {
        StatusCode::NOT_FOUND
    }

    async fn echo_handler(body: axum::body::Bytes) -> Response {
        match Response::builder()
            .status(StatusCode::OK)
            .header(axum::http::header::CONTENT_LENGTH, body.len().to_string())
            .body(Body::from(body))
        {
            Ok(response) => response,
            Err(_) => Response::new(Body::empty()),
        }
    }

    fn test_app(metrics_handle: MetricsHandle) -> Router {
        Router::new()
            .route("/hello", get(handler))
            .route("/missing", get(not_found_handler))
            .route("/echo", post(echo_handler))
            .layer(middleware::from_fn_with_state(
                metrics_handle,
                track_metrics,
            ))
    }

    #[test]
    fn records_request_counter_with_correct_labels() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/hello").await;

            let output = handle.render();
            assert!(
                output.contains("http_requests_total"),
                "expected http_requests_total in output: {output}"
            );
            assert!(
                output.contains(r#"method="GET""#),
                "expected method=GET label: {output}"
            );
            assert!(
                output.contains(r#"path="/hello""#),
                "expected path=/hello label: {output}"
            );
            assert!(
                output.contains(r#"status="200""#),
                "expected status=200 label: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn records_histogram_with_correct_labels() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/hello").await;

            let output = handle.render();
            assert!(
                output.contains("http_request_duration_seconds"),
                "expected http_request_duration_seconds in output: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn records_non_200_status_label() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/missing").await;

            let output = handle.render();
            assert!(
                output.contains(r#"status="404""#),
                "expected status=404 label: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn unknown_path_when_no_matched_path() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/nonexistent").await;

            let output = handle.render();
            assert!(
                output.contains(r#"path="unknown""#),
                "expected path=unknown for unmatched route: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn counter_increments_on_multiple_requests() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/hello").await;
            server.get("/hello").await;
            server.get("/hello").await;

            let output = handle.render();
            let matching_line = output.lines().find(|line| {
                line.starts_with("http_requests_total{")
                    && line.contains(r#"path="/hello""#)
                    && line.contains(r#"status="200""#)
            });
            assert!(
                matching_line.is_some(),
                "http_requests_total metric line not found in output: {output}"
            );
            let Some(line) = matching_line else {
                anyhow::bail!("matching metric line should exist after assertion");
            };
            assert!(line.ends_with(" 3"), "expected counter value 3: {line}");
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn records_request_body_size_histogram() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            let body = "hello";
            server
                .post("/echo")
                .add_header(
                    axum::http::header::CONTENT_LENGTH,
                    axum::http::HeaderValue::from_static("5"),
                )
                .text(body)
                .await;

            let output = handle.render();
            assert!(
                output.contains("http_request_body_size_bytes"),
                "expected http_request_body_size_bytes in output: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn records_response_body_size_histogram() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.post("/echo").text("hello").await;

            let output = handle.render();
            assert!(
                output.contains("http_response_body_size_bytes"),
                "expected http_response_body_size_bytes in output: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn no_body_size_metrics_when_content_length_absent() -> anyhow::Result<()> {
        let handle = test_metrics_handle();
        let rt = build_runtime()?;
        rt.block_on(async {
            let server = TestServer::new(test_app(handle.clone()));
            server.get("/hello").await;

            let output = handle.render();
            assert!(
                !output.contains("http_request_body_size_bytes"),
                "unexpected http_request_body_size_bytes for GET without body: {output}"
            );
            Ok::<_, anyhow::Error>(())
        })?;
        Ok(())
    }
}
