pub mod config;
pub mod state;
pub mod template;
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

pub use pdf::load_html_font_aliases;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

pub fn build_router(state: AppState) -> Router {
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
        .with_state(state);

    http_tracing::apply_http_tracing_layer(app)
}
