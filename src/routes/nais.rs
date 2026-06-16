use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::metrics::MetricsHandle;
use crate::state::AppState;

/// Builds the NAIS health check router with `/internal/is_alive`,
/// `/internal/is_ready`, and `/internal/metrics` endpoints.
pub fn nais_router(metrics_handle: MetricsHandle) -> Router<AppState> {
    Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
        .route(
            "/internal/metrics",
            get(move || std::future::ready(metrics_handle.render())),
        )
}

/// Handles `GET /internal/is_alive`.
///
/// Returns 200 OK when the application is alive, or 503 Service Unavailable otherwise.
pub async fn is_alive(State(state): State<AppState>) -> Response {
    if state.aliveness.is_alive() {
        (StatusCode::OK, "I'm alive").into_response()
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "I'm dead x_x").into_response()
    }
}

/// Handles `GET /internal/is_ready`.
///
/// Returns 200 OK when the application is ready to serve traffic, or 503 Service Unavailable otherwise.
/// In addition to the readiness flag, verifies that templates were successfully loaded.
pub async fn is_ready(State(state): State<AppState>) -> Response {
    if !state.aliveness.is_ready() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "Please wait! I'm not ready :(",
        )
            .into_response();
    }

    if state.templates.is_empty() {
        return (StatusCode::SERVICE_UNAVAILABLE, "No templates loaded").into_response();
    }

    (StatusCode::OK, "I'm ready").into_response()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use super::nais_router;
    use crate::config::Config;
    use crate::metrics;
    use crate::state::AppAliveness;
    use crate::state::AppState;
    use crate::{build_html_converter, typst_world};

    fn test_state(alive: bool, ready: bool) -> anyhow::Result<AppState> {
        test_state_with_templates(alive, ready, true)
    }

    fn test_state_with_templates(
        alive: bool,
        ready: bool,
        with_templates: bool,
    ) -> anyhow::Result<AppState> {
        let aliveness = AppAliveness::new();
        aliveness.set_alive(alive);
        aliveness.set_ready(ready);
        let cfg = Config::default();
        let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
        let mut templates = HashMap::new();
        if with_templates {
            templates.insert(
                ("app".to_string(), "test".to_string()),
                Arc::new("hello".to_string()),
            );
        }
        Ok(AppState {
            templates: Arc::new(templates),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness,
            fonts,
            html_converter: Arc::new(build_html_converter(&cfg.fonts_dir, &cfg.root_dir).0),
            root_dir: Arc::new(cfg.root_dir.clone()),
            resources_dir: Arc::new(cfg.resource_root()),
            config: cfg,
            compile_semaphore: None,
        })
    }

    #[tokio::test]
    async fn is_alive_returns_200_when_alive() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(nais_router(handle).with_state(test_state(true, false)?));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn is_alive_returns_503_when_not_alive() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(nais_router(handle).with_state(test_state(false, false)?));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_200_when_ready() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(nais_router(handle).with_state(test_state(false, true)?));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_503_when_not_ready() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(nais_router(handle).with_state(test_state(false, false)?));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_503_when_no_templates_loaded() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(
            nais_router(handle).with_state(test_state_with_templates(false, true, false)?),
        );
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(response.text(), "No templates loaded");
        Ok(())
    }

    #[tokio::test]
    async fn metrics_endpoint_returns_200() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(nais_router(handle).with_state(test_state(true, true)?));
        let response = server.get("/internal/metrics").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn metrics_endpoint_returns_prometheus_output() -> anyhow::Result<()> {
        let handle = metrics::test_metrics_handle();
        let server = TestServer::new(crate::build_router(test_state(true, true)?, handle.clone()));
        server.get("/internal/is_alive").await;
        let response = server.get("/internal/metrics").await;
        let body = response.text();
        assert!(
            body.contains("http_requests_total"),
            "expected prometheus metrics output to contain http_requests_total: {body}"
        );
        Ok(())
    }
}
