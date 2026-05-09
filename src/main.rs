mod config;
mod html;
mod pdf;
mod routes;
mod state;
mod template;
mod tracing_setup;
mod typst_world;

#[cfg(test)]
mod performance_test;

use axum::{
    routing::{get, post},
    Router,
};
use serde_json::Value;
use state::AppAliveness;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;
use typst_world::Fonts;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

/// Shared application state injected into every Axum handler.
#[derive(Clone)]
pub struct AppState {
    /// Pre-loaded Typst templates keyed by `"<app_name>/<template_name>"`.
    pub templates: Arc<HashMap<String, String>>,
    /// Test JSON data keyed by `(app_name, template_name)`, used in dev mode.
    pub data: Arc<RwLock<HashMap<(String, String), Value>>>,
    /// Liveness / readiness flags exposed via the NAIS health endpoints.
    pub aliveness: AppAliveness,
    /// Server configuration derived from environment variables.
    pub config: config::Config,
    /// Shared font data used by the Typst compiler.
    pub fonts: Arc<Fonts>,
}

#[tokio::main]
async fn main() {
    tracing_setup::setup_tracing().expect("Failed to initialise tracing");

    let cfg = config::Config::default();

    info!("Loading templates from '{}'", cfg.templates_dir.display());
    let templates = Arc::new(
        template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_else(|e| {
            tracing::warn!("Failed to load templates: {e}");
            HashMap::new()
        }),
    );
    info!("Loaded {} templates", templates.len());

    let data = if cfg.dev_mode {
        info!("Loading test data from '{}'", cfg.data_dir.display());
        let data = template::load_test_data(&cfg.data_dir);
        info!("Loaded {} test data entries", data.len());
        data
    } else {
        info!("Dev mode disabled, skipping test data loading");
        HashMap::new()
    };

    info!("Loading fonts from '{}'", cfg.fonts_dir.display());
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir).unwrap_or_else(|e| {
        panic!(
            "Failed to load fonts from '{}': {e}",
            cfg.fonts_dir.display()
        )
    }));
    info!("Loaded {} fonts", fonts.fonts.len());

    let aliveness = AppAliveness::new();
    let aliveness_clone = aliveness.clone();

    let state = AppState {
        templates,
        data: Arc::new(RwLock::new(data)),
        aliveness: aliveness.clone(),
        config: cfg.clone(),
        fonts,
    };

    let app = build_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.port));
    info!("Starting pdfgenrs server on {addr}");

    aliveness_clone.set_alive(true);
    aliveness_clone.set_ready(true);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(aliveness_clone))
        .await
        .expect("Server error");
}

fn build_router(state: AppState) -> Router {
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

    Router::new()
        .nest("/api/v1/genpdf", pdf_router)
        .nest("/api/v1/genhtml", html_router)
        .merge(routes::nais::nais_router())
        .with_state(state)
}

async fn shutdown_signal(aliveness: AppAliveness) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    info!("Shutdown signal received, stopping server...");
    aliveness.set_ready(false);
    aliveness.set_alive(false);
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use crate::{build_router, config, state, typst_world, AppState};

    fn make_state(dev_mode: bool) -> AppState {
        AppState {
            templates: Arc::new(HashMap::new()),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness: state::AppAliveness::new(),
            config: config::Config {
                port: 8080,
                root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
                templates_dir: PathBuf::from("templates"),
                resources_dir: PathBuf::from("resources"),
                data_dir: PathBuf::from("data"),
                fonts_dir: PathBuf::from("fonts"),
                dev_mode,
            },
            fonts: Arc::new(
                typst_world::load_fonts(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"))
                    .expect("test fonts should load"),
            ),
        }
    }

    #[tokio::test]
    async fn build_router_is_alive_route_exists() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn build_router_is_ready_route_exists() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn build_router_post_pdf_returns_404_for_missing_template() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn build_router_post_pdf_from_html_returns_pdf() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server
            .post("/api/v1/genpdf/html/myapp")
            .text("<!DOCTYPE html><html><body><h1>Hello</h1></body></html>")
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
    }

    #[tokio::test]
    async fn build_router_post_pdf_from_image_returns_pdf() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server
            .post("/api/v1/genpdf/image/myapp")
            .content_type("image/png")
            .bytes(axum::body::Bytes::from(
                std::fs::read(
                    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("resources")
                        .join("NAVLogoRed.png"),
                )
                .unwrap(),
            ))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
    }

    #[tokio::test]
    async fn build_router_get_pdf_returns_405_when_dev_mode_disabled() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server.get("/api/v1/genpdf/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn build_router_get_pdf_returns_404_when_dev_mode_enabled() {
        let server = TestServer::new(build_router(make_state(true)));
        let response = server.get("/api/v1/genpdf/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn build_router_post_html_returns_404_for_missing_template() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server
            .post("/api/v1/genhtml/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn build_router_get_html_returns_405_when_dev_mode_disabled() {
        let server = TestServer::new(build_router(make_state(false)));
        let response = server.get("/api/v1/genhtml/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    async fn build_router_get_html_returns_404_when_dev_mode_enabled() {
        let server = TestServer::new(build_router(make_state(true)));
        let response = server.get("/api/v1/genhtml/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
