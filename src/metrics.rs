use std::time::Instant;

use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use metrics::{counter, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

/// Installs the global Prometheus metrics recorder and returns a handle for rendering.
///
/// Must be called once at application startup before any metrics are recorded.
#[allow(clippy::expect_used)]
pub fn setup_metrics_recorder() -> PrometheusHandle {
    let builder = PrometheusBuilder::new();
    builder
        .install_recorder()
        .expect("Failed to install Prometheus recorder")
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{Router, middleware, routing::get};
    use axum_test::TestServer;

    async fn handler() -> StatusCode {
        StatusCode::OK
    }

    async fn not_found_handler() -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn test_app() -> Router {
        Router::new()
            .route("/hello", get(handler))
            .route("/missing", get(not_found_handler))
            .layer(middleware::from_fn(track_metrics))
    }

    #[test]
    fn records_request_counter_with_correct_labels() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let server = TestServer::new(test_app());
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
            });
        });
    }

    #[test]
    fn records_histogram_with_correct_labels() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let server = TestServer::new(test_app());
                server.get("/hello").await;

                let output = handle.render();
                assert!(
                    output.contains("http_request_duration_seconds"),
                    "expected http_request_duration_seconds in output: {output}"
                );
            });
        });
    }

    #[test]
    fn records_non_200_status_label() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let server = TestServer::new(test_app());
                server.get("/missing").await;

                let output = handle.render();
                assert!(
                    output.contains(r#"status="404""#),
                    "expected status=404 label: {output}"
                );
            });
        });
    }

    #[test]
    fn unknown_path_when_no_matched_path() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let server = TestServer::new(test_app());
                server.get("/nonexistent").await;

                let output = handle.render();
                assert!(
                    output.contains(r#"path="unknown""#),
                    "expected path=unknown for unmatched route: {output}"
                );
            });
        });
    }

    #[test]
    fn counter_increments_on_multiple_requests() {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("failed to build runtime");
            rt.block_on(async {
                let server = TestServer::new(test_app());
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
                let line =
                    matching_line.expect("matching metric line should exist after assertion");
                assert!(line.ends_with(" 3"), "expected counter value 3: {line}");
            });
        });
    }
}
