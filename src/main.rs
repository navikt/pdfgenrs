mod tracing_setup;

use anyhow::{Context, Result};
use pdfgenrs::metrics;
use pdfgenrs::state::{AppAliveness, AppState};
use pdfgenrs::{build_html_converter, build_router, config, template, typst_world};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    let tracer_provider = tracing_setup::setup_tracing().map_err(|e| {
        eprintln!("Failed to initialise tracing: {e}");
        e
    })?;

    let cfg = config::Config::default();

    info!(path = %cfg.templates_dir.display(), "Loading templates");
    let templates = Arc::new(
        template::load_templates_from_dir(&cfg.templates_dir).map_err(|e| {
            tracing::error!(
                error = %e,
                path = %cfg.templates_dir.display(),
                "Failed to load templates"
            );
            e
        })?,
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

    let (html_converter, html_font_count) =
        build_html_converter(&cfg.root_dir.join(&cfg.fonts_dir), &cfg.root_dir);
    let html_converter = Arc::new(html_converter);
    info!(
        count = html_font_count,
        "Built HTML converter with font aliases"
    );

    let aliveness = AppAliveness::new();
    let aliveness_clone = aliveness.clone();

    let state = AppState {
        templates,
        data: Arc::new(RwLock::new(data)),
        aliveness: aliveness.clone(),
        config: cfg.clone(),
        fonts,
        html_converter,
    };

    let metrics_handle = metrics::setup_metrics_recorder();

    let app = build_router(state, metrics_handle);

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
                tokio::time::sleep(Duration::from_secs(5));
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
        warn!(error = %e, "OpenTelemetry tracer provider shutdown error");
    }

    Ok(())
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
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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

    use axum::http::StatusCode;
    use axum_test::TestServer;

    use pdfgenrs::testutil::make_state;
    use pdfgenrs::{build_router, metrics};

    fn make_empty_state(dev_mode: bool) -> anyhow::Result<pdfgenrs::state::AppState> {
        make_state(HashMap::new(), HashMap::new(), dev_mode)
    }

    #[tokio::test]
    async fn build_router_is_alive_route_exists() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_is_ready_route_exists() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_post_pdf_returns_404_for_missing_template() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server
            .post("/api/v1/genpdf/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_post_pdf_from_html_returns_pdf() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server
            .post("/api/v1/genpdf/html/myapp")
            .text("<!DOCTYPE html><html><body><h1>Hello</h1></body></html>")
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/pdf"
        );
        Ok(())
    }

    #[tokio::test]
    async fn build_router_post_pdf_from_image_returns_pdf() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server
            .post("/api/v1/genpdf/image/myapp")
            .content_type("image/png")
            .bytes(axum::body::Bytes::from(std::fs::read(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("resources")
                    .join("NAVLogoRed.png"),
            )?))
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/pdf"
        );
        Ok(())
    }

    #[tokio::test]
    async fn build_router_get_pdf_returns_405_when_dev_mode_disabled() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/api/v1/genpdf/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_get_pdf_returns_404_when_dev_mode_enabled() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(true)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/api/v1/genpdf/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_post_html_returns_404_for_missing_template() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server
            .post("/api/v1/genhtml/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_get_html_returns_405_when_dev_mode_disabled() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(false)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/api/v1/genhtml/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::METHOD_NOT_ALLOWED);
        Ok(())
    }

    #[tokio::test]
    async fn build_router_get_html_returns_404_when_dev_mode_enabled() -> anyhow::Result<()> {
        let server = TestServer::new(build_router(
            make_empty_state(true)?,
            metrics::test_metrics_handle(),
        ));
        let response = server.get("/api/v1/genhtml/myapp/mytemplate").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }
}
