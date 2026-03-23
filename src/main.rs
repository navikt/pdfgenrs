mod config;
mod log;
mod metrics;
mod pdf;
mod routes;
mod state;
mod template;
mod typst_world;

use axum::{
    middleware,
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
use ::log::info;

#[derive(Clone)]
pub struct AppState {
    pub templates: Arc<HashMap<String, String>>,
    pub data: Arc<RwLock<HashMap<(String, String), Value>>>,
    pub aliveness: AppAliveness,
    pub config: config::Config,
}

#[tokio::main]
async fn main() {
    log::init_log4rs();

    let cfg = config::Config::default();

    info!("Loading templates from '{}'", cfg.templates_dir);
    let templates = Arc::new(template::load_templates_from_dir(&cfg.templates_dir)
        .unwrap_or_else(|e| {
            ::log::warn!("Failed to load templates: {e}");
            HashMap::new()
        }));
    info!("Loaded {} templates", templates.len());

    info!("Loading test data from '{}'", cfg.data_dir);
    let data = template::load_test_data(&cfg.data_dir);
    info!("Loaded {} test data entries", data.len());

    let aliveness = AppAliveness::new();
    let aliveness_clone = aliveness.clone();

    metrics::register_metrics(prometheus::default_registry());

    let state = AppState {
        templates,
        data: Arc::new(RwLock::new(data)),
        aliveness: aliveness.clone(),
        config: cfg.clone(),
    };

    let app = build_router(state, &cfg);

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

fn build_router(state: AppState, cfg: &config::Config) -> Router {
    let pdf_template_route = if !cfg.disable_pdf_get {
        get(routes::pdf::get_pdf).post(routes::pdf::post_pdf)
    } else {
        post(routes::pdf::post_pdf)
    };

    let pdf_router = Router::new()
        .route("/html/{app_name}", post(routes::pdf::post_html_to_pdf))
        .route("/image/{app_name}", post(routes::pdf::post_image_to_pdf))
        .route("/{app_name}/{template}", pdf_template_route);

    Router::new()
        .nest("/api/v1/genpdf", pdf_router)
        .route("/internal/is_alive", get(routes::nais::is_alive))
        .route("/internal/is_ready", get(routes::nais::is_ready))
        .route("/internal/prometheus", get(routes::nais::prometheus_metrics))
        .layer(middleware::from_fn(http_metrics_middleware))
        .with_state(state)
}

async fn http_metrics_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let path = req.uri().path().to_string();
    let timer = metrics::HTTP_HISTOGRAM.with_label_values(&[&path]).start_timer();
    let resp = next.run(req).await;
    timer.observe_duration();
    resp
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
