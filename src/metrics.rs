use std::time::Instant;

use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use metrics::{counter, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

/// Installs the global Prometheus metrics recorder and returns a handle for rendering.
///
/// Must be called once at application startup before any metrics are recorded.
pub fn setup_metrics_recorder() -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    builder
        .install_recorder()
        .unwrap_or_else(|e| panic!("Failed to install Prometheus recorder: {e}"))
}

/// Creates a [`PrometheusHandle`] without installing a global recorder.
///
/// Useful in tests where multiple recorders cannot coexist.
pub fn test_metrics_handle() -> PrometheusHandle {
    PrometheusBuilder::new().build_recorder().handle()
}

/// Axum middleware that records HTTP request metrics.
///
/// Records:
/// - `http_requests_total` counter with labels: method, path, status
/// - `http_request_duration_seconds` histogram with labels: method, path, status
pub async fn track_metrics(request: Request<Body>, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_owned())
        .unwrap_or_else(|| "unknown".to_owned());

    let start = Instant::now();
    let response = next.run(request).await;
    let duration = start.elapsed().as_secs_f64();

    let status = response.status().as_u16().to_string();

    let labels = [("method", method), ("path", path), ("status", status)];

    counter!("http_requests_total", &labels).increment(1);
    histogram!("http_request_duration_seconds", &labels).record(duration);

    response
}
