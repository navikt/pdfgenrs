//! Public library API for configuring and running `pdfgenrs`.
//!
//! This crate exposes configuration and rendering building blocks and a
//! ready-to-use Axum router for the HTTP API.

/// Runtime server configuration sourced from environment variables.
pub mod config;
mod http;
/// Prometheus metrics middleware and recorder setup.
pub mod metrics;
mod rendering;
/// Process, Tokio runtime, and compilation saturation metrics.
pub mod runtime_metrics;
/// Shared application state and liveness/readiness primitives.
pub mod state;
#[cfg(test)]
mod testutil;

pub(crate) use http::{http_tracing, request_id, routes};
/// Typst-to-HTML rendering functions.
pub use rendering::html;
/// PDF generation functions: Typst-to-PDF, HTML-to-PDF, and image-to-PDF.
pub use rendering::pdf;
/// Template and development data loading helpers.
pub use rendering::template;
/// Typst world, font loading, and compilation utilities.
pub use rendering::typst_world;

use crate::http::routes::error::{ApiError, framework_error_response};
use axum::extract::DefaultBodyLimit;
use axum::{
    Router,
    extract::State,
    middleware,
    routing::{get, post},
};
use metrics_exporter_prometheus::PrometheusHandle;
use state::AppState;
use tower_http::limit::RequestBodyLimitLayer;

/// Builds a pre-configured HTML-to-PDF converter with font aliases.
///
/// The converter is built once at startup and shared across requests.
pub use pdf::build_html_converter;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

/// Builds the full HTTP router for the PDF/HTML generation API.
pub fn build_router(state: AppState, metrics_handle: PrometheusHandle) -> Router {
    let request_body_limit_bytes = state.config.request_body_limit_bytes;
    let mut pdf_router = Router::new()
        .route("/html/{app_name}", post(routes::pdf::post_pdf_from_html))
        .route("/image/{app_name}", post(routes::pdf::post_pdf_from_image))
        .route("/{app_name}/{template}", post(routes::pdf::post_pdf));

    let mut html_router =
        Router::new().route("/{app_name}/{template}", post(routes::html::post_html));

    if state.config.dev_mode {
        pdf_router = pdf_router.route("/{app_name}/{template}", get(routes::pdf::get_pdf));
        html_router = html_router.route("/{app_name}/{template}", get(routes::html::get_html));
    }

    let api_routes = Router::new()
        .nest("/api/v1/genpdf", pdf_router)
        .nest("/api/v1/genhtml", html_router)
        .fallback(fallback_handler)
        .layer(middleware::map_response(framework_error_response));

    let api_routes = http_tracing::apply_http_tracing_layer(api_routes);

    api_routes
        .merge(routes::nais::nais_router(metrics_handle))
        .layer(middleware::from_fn(request_id::request_id_middleware))
        .layer(middleware::from_fn(metrics::track_metrics))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(request_body_limit_bytes))
        .with_state(state)
}

/// Fallback handler that returns 404 with a list of all known templates.
async fn fallback_handler(State(state): State<AppState>) -> ApiError {
    let mut template_names: Vec<String> = state
        .templates
        .keys()
        .map(|(app, tmpl)| format!("{app}/{tmpl}"))
        .collect();
    template_names.sort();

    let body = format!(
        "Unknown path. Known templates:\n{}",
        template_names
            .iter()
            .map(|name| format!("  - {name}"))
            .collect::<Vec<_>>()
            .join("\n")
    );

    ApiError::UnknownPath { detail: body }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;

    use axum::http::{HeaderValue, StatusCode, header};
    use axum_test::TestServer;
    use metrics_exporter_prometheus::PrometheusBuilder;
    use tokio::sync::Semaphore;
    use tokio::time::timeout;

    use super::*;
    use crate::testutil::make_state;

    #[tokio::test]
    async fn fallback_returns_404_with_template_list() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("appa".to_string(), "doc".to_string()),
            "Hello\n".to_string(),
        );
        templates.insert(
            ("appb".to_string(), "letter".to_string()),
            "World\n".to_string(),
        );
        let state = make_state(templates, HashMap::new(), false)?;
        let metrics_handle = metrics::test_metrics_handle();
        let router = build_router(state, metrics_handle);
        let server = TestServer::new(router);

        let response = server.get("/nonexistent/path").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_problem_response(
            &response,
            StatusCode::NOT_FOUND,
            "urn:pdfgenrs:error:not-found",
        )?;
        let body: serde_json::Value = serde_json::from_str(&response.text())?;
        assert!(
            body["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("Unknown path. Known templates:"))
        );
        assert!(
            body["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("appa/doc"))
        );
        assert!(
            body["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("appb/letter"))
        );
        Ok(())
    }

    #[tokio::test]
    async fn fallback_returns_404_with_empty_template_list() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let metrics_handle = metrics::test_metrics_handle();
        let router = build_router(state, metrics_handle);
        let server = TestServer::new(router);

        let response = server.get("/does-not-exist").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_problem_response(
            &response,
            StatusCode::NOT_FOUND,
            "urn:pdfgenrs:error:not-found",
        )?;
        let body: serde_json::Value = serde_json::from_str(&response.text())?;
        assert!(
            body["detail"]
                .as_str()
                .is_some_and(|detail| detail.contains("Unknown path. Known templates:"))
        );
        Ok(())
    }

    #[tokio::test]
    async fn body_limit_rejects_oversized_request() -> anyhow::Result<()> {
        use crate::testutil::make_state_with_body_limit;

        let limit: usize = 1024;
        let state = make_state_with_body_limit(HashMap::new(), HashMap::new(), false, limit)?;
        let metrics_handle = metrics::test_metrics_handle();
        let router = build_router(state, metrics_handle);
        let server = TestServer::new(router);

        let oversized = vec![b'a'; limit + 1];
        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .content_type("application/json")
            .bytes(axum::body::Bytes::from(oversized))
            .await;

        assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_problem_response(
            &response,
            StatusCode::PAYLOAD_TOO_LARGE,
            "urn:pdfgenrs:error:payload-too-large",
        )?;
        Ok(())
    }

    #[tokio::test]
    async fn invalid_json_returns_problem_details() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let server = TestServer::new(build_router(state, metrics::test_metrics_handle()));

        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .content_type("application/json")
            .bytes(axum::body::Bytes::from_static(b"{"))
            .await;

        assert_problem_response(
            &response,
            StatusCode::BAD_REQUEST,
            "urn:pdfgenrs:error:invalid-request",
        )
    }

    #[tokio::test]
    async fn generation_errors_hide_details_in_production_and_expose_them_in_development()
    -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "broken".to_string()),
            "#this-is-not-valid-typst-syntax(((".to_string(),
        );
        for path in [
            "/api/v1/genpdf/myapp/broken",
            "/api/v1/genhtml/myapp/broken",
        ] {
            for dev_mode in [false, true] {
                let state = make_state(templates.clone(), HashMap::new(), dev_mode)?;
                let server = TestServer::new(build_router(state, metrics::test_metrics_handle()));
                let response = server.post(path).json(&serde_json::json!({})).await;

                assert_problem_response(
                    &response,
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "urn:pdfgenrs:error:generation-failed",
                )?;
                let body: serde_json::Value = serde_json::from_slice(response.as_bytes())?;
                let detail = body["detail"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("missing error detail"))?;
                if dev_mode {
                    assert!(!detail.is_empty());
                    assert_ne!(detail, "Internal server error");
                } else {
                    assert_eq!(detail, "Internal server error");
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn malformed_content_types_return_problem_details() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let server = TestServer::new(build_router(state, metrics::test_metrics_handle()));
        let malformed = HeaderValue::from_bytes(b"image/png\xff")?;

        let image = server
            .post("/api/v1/genpdf/image/myapp")
            .add_header(header::CONTENT_TYPE, malformed)
            .bytes(axum::body::Bytes::from_static(b"not an image"))
            .await;
        assert_problem_response(
            &image,
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "urn:pdfgenrs:error:unsupported-media-type",
        )?;

        for path in [
            "/api/v1/genpdf/myapp/mytemplate",
            "/api/v1/genhtml/myapp/mytemplate",
        ] {
            let response = server
                .post(path)
                .content_type("application/json, text/plain")
                .bytes(axum::body::Bytes::from_static(b"{}"))
                .await;
            assert_problem_response(
                &response,
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "urn:pdfgenrs:error:unsupported-media-type",
            )?;
        }
        Ok(())
    }

    #[tokio::test]
    async fn client_timeout_cancels_queued_compilation_without_stealing_permit()
    -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "document".to_string()),
            "#set document(title: \"Test\", date: auto)\n#set page(margin: 1cm)\nHello!"
                .to_string(),
        );
        let mut state = make_state(templates, HashMap::new(), false)?;
        let semaphore = Arc::new(Semaphore::new(1));
        state.compile_semaphore = Some(Arc::clone(&semaphore));
        let server = TestServer::new(build_router(state, metrics::test_metrics_handle()));
        let held = Arc::clone(&semaphore).acquire_owned().await?;

        let canceled = timeout(
            Duration::from_millis(100),
            server
                .post("/api/v1/genpdf/myapp/document")
                .json(&serde_json::json!({})),
        )
        .await;
        assert!(canceled.is_err(), "request should wait for the held permit");

        drop(held);
        let permit = timeout(
            Duration::from_secs(5),
            Arc::clone(&semaphore).acquire_owned(),
        )
        .await
        .map_err(|_| anyhow::anyhow!("canceled request retained the compilation permit"))??;
        drop(permit);

        let response = server
            .post("/api/v1/genpdf/myapp/document")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response.as_bytes().starts_with(b"%PDF"));
        Ok(())
    }

    #[tokio::test]
    async fn method_not_allowed_returns_problem_details() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let server = TestServer::new(build_router(state, metrics::test_metrics_handle()));

        let response = server.get("/api/v1/genpdf/myapp/mytemplate").await;

        assert_problem_response(
            &response,
            StatusCode::METHOD_NOT_ALLOWED,
            "urn:pdfgenrs:error:method-not-allowed",
        )
    }

    fn assert_problem_response(
        response: &axum_test::TestResponse,
        expected_status: StatusCode,
        expected_type: &str,
    ) -> anyhow::Result<()> {
        assert_eq!(response.status_code(), expected_status);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?
                .to_str()?,
            "application/problem+json; charset=utf-8"
        );
        let body: serde_json::Value = serde_json::from_str(&response.text())?;
        assert_eq!(body["type"], expected_type);
        assert_eq!(body["status"], expected_status.as_u16());
        Ok(())
    }

    #[tokio::test]
    async fn body_limit_allows_request_within_limit() -> anyhow::Result<()> {
        use crate::testutil::make_state_with_body_limit;

        let limit: usize = 1024;
        let state = make_state_with_body_limit(HashMap::new(), HashMap::new(), false, limit)?;
        let metrics_handle = metrics::test_metrics_handle();
        let router = build_router(state, metrics_handle);
        let server = TestServer::new(router);

        let within_limit = vec![b'a'; limit - 1];
        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .content_type("application/json")
            .bytes(axum::body::Bytes::from(within_limit))
            .await;

        assert_ne!(
            response.status_code(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "Request within body limit should not be rejected as 413"
        );
        Ok(())
    }

    #[tokio::test]
    async fn body_limit_allows_request_exactly_at_limit() -> anyhow::Result<()> {
        use crate::testutil::make_state_with_body_limit;

        let limit: usize = 1024;
        let state = make_state_with_body_limit(HashMap::new(), HashMap::new(), false, limit)?;
        let metrics_handle = metrics::test_metrics_handle();
        let router = build_router(state, metrics_handle);
        let server = TestServer::new(router);

        let exactly_at_limit = vec![b'a'; limit];
        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .content_type("application/json")
            .bytes(axum::body::Bytes::from(exactly_at_limit))
            .await;

        assert_ne!(
            response.status_code(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "Request exactly at body limit should not be rejected as 413"
        );
        Ok(())
    }

    #[test]
    fn body_limit_rejection_emits_http_metrics() -> anyhow::Result<()> {
        use crate::testutil::make_state_with_body_limit;

        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        ::metrics::with_local_recorder(&recorder, || -> anyhow::Result<()> {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(async {
                let limit: usize = 1024;
                let state =
                    make_state_with_body_limit(HashMap::new(), HashMap::new(), false, limit)?;
                let metrics_handle = metrics::test_metrics_handle();
                let router = build_router(state, metrics_handle);
                let server = TestServer::new(router);
                let oversized = vec![b'a'; limit + 1];
                let response = server
                    .post("/api/v1/genpdf/myapp/mytemplate")
                    .content_type("application/json")
                    .bytes(axum::body::Bytes::from(oversized))
                    .await;

                assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
                Ok::<(), anyhow::Error>(())
            })?;
            Ok(())
        })?;

        let output = handle.render();
        assert!(output.contains("http_requests_total"));
        assert!(output.contains(r#"status="413""#));
        Ok(())
    }

    #[test]
    fn non_rejected_requests_still_emit_http_metrics() -> anyhow::Result<()> {
        use crate::testutil::make_state_with_body_limit;

        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        ::metrics::with_local_recorder(&recorder, || -> anyhow::Result<()> {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()?;
            rt.block_on(async {
                let limit: usize = 1024;
                let state =
                    make_state_with_body_limit(HashMap::new(), HashMap::new(), false, limit)?;
                let metrics_handle = metrics::test_metrics_handle();
                let router = build_router(state, metrics_handle);
                let server = TestServer::new(router);
                let within_limit = vec![b'a'; limit - 1];
                let response = server
                    .post("/api/v1/genpdf/myapp/mytemplate")
                    .content_type("application/json")
                    .bytes(axum::body::Bytes::from(within_limit))
                    .await;

                assert_eq!(response.status_code(), StatusCode::BAD_REQUEST);
                Ok::<(), anyhow::Error>(())
            })?;
            Ok(())
        })?;

        let output = handle.render();
        assert!(output.contains("http_requests_total"));
        assert!(output.contains(r#"status="400""#));
        Ok(())
    }
}
