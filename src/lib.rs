//! Public library API for configuring and running `pdfgenrs`.
//!
//! This crate exposes configuration and rendering building blocks and a
//! ready-to-use Axum router for the HTTP API.

/// Runtime server configuration sourced from environment variables.
pub mod config;
/// Shared application state and liveness/readiness primitives.
pub mod state;
/// Template and development data loading helpers.
pub mod template;
/// Typst world, font loading, and compilation utilities.
pub mod typst_world;

pub(crate) mod html;
pub(crate) mod http_tracing;
pub(crate) mod pdf;
pub(crate) mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use state::AppState;
use tower_http::limit::RequestBodyLimitLayer;

/// Loads optional HTML font aliases from `fonts_dir`.
///
/// Missing font files are skipped and logged as warnings.
pub use pdf::load_html_font_aliases;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

/// Builds the full HTTP router for the PDF/HTML generation API.
pub fn build_router(state: AppState) -> Router {
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

    let app = Router::new()
        .nest("/api/v1/genpdf", pdf_router)
        .nest("/api/v1/genhtml", html_router)
        .merge(routes::nais::nais_router())
        .layer(RequestBodyLimitLayer::new(request_body_limit_bytes))
        .with_state(state);

    http_tracing::apply_http_tracing_layer(app)
}
