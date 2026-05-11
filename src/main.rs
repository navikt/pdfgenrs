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

use anyhow::{Context, Result};
use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use opentelemetry::{global, propagation::Extractor};
use axum::http::{HeaderMap, Request};
use serde_json::Value;
use state::AppAliveness;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_opentelemetry::OpenTelemetrySpanExt;
use typst_world::Fonts;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static std::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
}

/// Implements [`opentelemetry::propagation::Extractor`] for an Axum [`HeaderMap`] so that
/// the global W3C TraceContext + Baggage propagators can extract an incoming parent trace context
/// from request headers.
struct HeaderExtractor<'a>(&'a HeaderMap);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|k| k.as_str()).collect()
    }
}

/// Creates a tracing span for an HTTP request and sets the remote span as OTel parent.
///
/// This is passed to [`TraceLayer::make_span_with`].  Extracting the trace context and calling
/// [`OpenTelemetrySpanExt::set_parent`] here (synchronously, before any `.await`) avoids the
/// `!Send` constraint of [`opentelemetry::ContextGuard`] and correctly parents the new span to
/// the caller's distributed trace when a `traceparent` header is present.
fn make_otel_span(request: &Request<Body>) -> tracing::Span {
    let span = tracing::info_span!(
        "HTTP request",
        http.method = %request.method(),
        http.uri = %request.uri(),
        http.version = ?request.version(),
        otel.kind = "server",
        otel.status_code = tracing::field::Empty,
        http.status_code = tracing::field::Empty,
    );
    let parent_cx = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    let _ = span.set_parent(parent_cx);
    span
}

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
async fn main() -> Result<()> {
    let tracer_provider = tracing_setup::setup_tracing().map_err(|e| {
        eprintln!("Failed to initialise tracing: {e}");
        e
    })?;

    let cfg = config::Config::default();

    info!(path = %cfg.templates_dir.display(), "Loading templates");
    let templates = Arc::new(
        template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_else(|e| {
            tracing::warn!(error = %e, "Failed to load templates");
            HashMap::new()
        }),
    );
    info!(count = templates.len(), "Loaded templates");

    let data = if cfg.dev_mode {
        info!(path = %cfg.data_dir.display(), "Loading test data");
        let result = template::load_test_data(&cfg.data_dir);
        for diagnostic in &result.diagnostics {
            warn!(
                path = %diagnostic.path.display(),
                kind = ?diagnostic.kind,
                error = %diagnostic.message,
                "Failed to load test data file"
            );
        }
        let summary = result.error_summary();
        if !summary.is_empty() {
            warn!(?summary, "Test data loading completed with errors");
        }
        info!(
            count = result.data.len(),
            errors = result.diagnostics.len(),
            "Loaded test data entries"
        );
        result.data
    } else {
        info!("Dev mode disabled, skipping test data loading");
        HashMap::new()
    };

    info!(path = %cfg.fonts_dir.display(), "Loading fonts");
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir).map_err(|e| {
        tracing::error!(
            error = %e,
            path = %cfg.fonts_dir.display(),
            "Failed to load fonts"
        );
        e
    })?);
    info!(count = fonts.fonts.len(), "Loaded fonts");

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
    info!(address = %addr, "Starting pdfgenrs server");

    aliveness_clone.set_alive(true);
    aliveness_clone.set_ready(true);

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
        tracing::error!(error = %e, address = %addr, "Failed to bind TCP listener");
        e
    })?;

    let aliveness_for_shutdown = aliveness_clone.clone();
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            if let Err(e) = shutdown_signal(aliveness_for_shutdown.clone()).await {
                tracing::error!(error = %e, "Shutdown signal handler failed");
                aliveness_for_shutdown.set_ready(false);
                aliveness_for_shutdown.set_alive(false);
            }
        })
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Server error");
            e
        })?;

    // Flush and export any remaining spans buffered by the batch processor.
    if let Err(e) = tracer_provider.shutdown() {
        tracing::warn!(error = %e, "OpenTelemetry tracer provider shutdown error");
    }

    Ok(())
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
        // Creates a tracing::Span for every HTTP request.  The custom make_span_with function
        // also extracts W3C traceparent/baggage headers and sets the remote span as the OTel
        // parent so that pdfgenrs spans are correctly nested inside the caller's distributed
        // trace.  The OpenTelemetryLayer (registered in setup_tracing) converts those tracing
        // spans into OpenTelemetry spans forwarded to the OTLP exporter.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(make_otel_span)
        )
}

async fn shutdown_signal(aliveness: AppAliveness) -> Result<()> {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .context("Failed to install Ctrl+C handler")?;
        Ok::<(), anyhow::Error>(())
    };

    #[cfg(unix)]
    let terminate = async {
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .context("Failed to install SIGTERM handler")?;
        sigterm.recv().await;
        Ok::<(), anyhow::Error>(())
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<Result<()>>();

    tokio::select! {
        result = ctrl_c => result?,
        result = terminate => result?,
    }

    info!("Shutdown signal received, stopping server...");
    aliveness.set_ready(false);
    aliveness.set_alive(false);
    Ok(())
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
