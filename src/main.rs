mod config;
mod logging;
mod pdf;
mod routes;
mod state;
mod template;
mod typst_world;

#[cfg(test)]
mod performance_test;

use axum::{
    routing::{get, post},
    Router,
};
use serde_json::Value;
use state::AppAliveness;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::RwLock;
use typst_world::Fonts;
use log::info;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<HashMap<String, String>>,
    pub data: Arc<RwLock<HashMap<(String, String), Value>>>,
    pub aliveness: AppAliveness,
    pub config: config::Config,
    pub fonts: Arc<Fonts>,
}

#[tokio::main]
async fn main() {
    logging::init_log4rs();

    let cfg = config::Config::default();

    info!("Loading templates from '{}'", cfg.templates_dir.display());
    let templates = Arc::new(template::load_templates_from_dir(&cfg.templates_dir)
        .unwrap_or_else(|e| {
            log::warn!("Failed to load templates: {e}");
            HashMap::new()
        }));
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

    info!("Loading fonts");
    let fonts = Arc::new(typst_world::load_fonts());
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
        .route("/{app_name}/{template}", post(routes::pdf::post_pdf));

    if state.config.dev_mode {
        pdf_router = pdf_router
            .route("/{app_name}/{template}", get(routes::pdf::get_pdf));
    }

    Router::new()
        .nest("/api/v1/genpdf", pdf_router)
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
        _ = ctrl_c => {},
        _ = terminate => {},
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
                templates_dir: PathBuf::from("templates"),
                resources_dir: PathBuf::from("resources"),
                data_dir: PathBuf::from("data"),
                dev_mode,
            },
            fonts: Arc::new(typst_world::load_fonts()),
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

}
