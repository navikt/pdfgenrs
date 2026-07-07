//! Public library API for configuring and running `pdfgenrs`.
//!
//! This crate exposes configuration and rendering building blocks and a
//! ready-to-use Axum router for the HTTP API.

/// Runtime server configuration sourced from environment variables.
pub mod config;
/// Prometheus metrics middleware and recorder setup.
pub mod metrics;
/// Shared application state and liveness/readiness primitives.
pub mod state;
/// Template and development data loading helpers.
pub mod template;
/// Typst world, font loading, and compilation utilities.
pub mod typst_world;

pub(crate) mod html;
pub(crate) mod http_tracing;
/// PDF generation functions: Typst-to-PDF, HTML-to-PDF, and image-to-PDF.
pub mod pdf;
pub(crate) mod request_id;
pub(crate) mod routes;
#[doc(hidden)]
pub mod testutil;

use axum::extract::DefaultBodyLimit;
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
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
        .fallback(fallback_handler);

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
async fn fallback_handler(State(state): State<AppState>) -> impl IntoResponse {
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

    (StatusCode::NOT_FOUND, body)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::http::StatusCode;
    use axum_test::TestServer;

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
        let body = response.text();
        assert!(body.contains("Unknown path. Known templates:"));
        assert!(body.contains("appa/doc"));
        assert!(body.contains("appb/letter"));
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
        let body = response.text();
        assert!(body.contains("Unknown path. Known templates:"));
        Ok(())
    }
}
