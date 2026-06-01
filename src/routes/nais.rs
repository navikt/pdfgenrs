use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};

use crate::state::AppState;

/// Builds the NAIS health check router with `/internal/is_alive` and
/// `/internal/is_ready` endpoints.
pub fn nais_router() -> Router<AppState> {
    Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
}

/// Handles `GET /internal/is_alive`.
///
/// Returns 200 OK when the application is alive, or 500 otherwise.
pub async fn is_alive(State(state): State<AppState>) -> Response {
    if state.aliveness.is_alive() {
        (StatusCode::OK, "I'm alive").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x").into_response()
    }
}

/// Handles `GET /internal/is_ready`.
///
/// Returns 200 OK when the application is ready to serve traffic, or 500 otherwise.
/// In addition to the readiness flag, this checks that fonts and templates were
/// loaded successfully (i.e. are non-empty).
pub async fn is_ready(State(state): State<AppState>) -> Response {
    if !state.aliveness.is_ready() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Please wait! I'm not ready :(",
        )
            .into_response();
    }

    if state.fonts.fonts.is_empty() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "No fonts loaded").into_response();
    }

    if state.templates.is_empty() {
        return (StatusCode::INTERNAL_SERVER_ERROR, "No templates loaded").into_response();
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
    use crate::state::AppAliveness;
    use crate::state::AppState;
    use crate::{build_html_converter, typst_world};

    fn test_state(alive: bool, ready: bool) -> anyhow::Result<AppState> {
        let aliveness = AppAliveness::new();
        aliveness.set_alive(alive);
        aliveness.set_ready(ready);
        let cfg = Config::default();
        let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
        let mut templates = HashMap::new();
        templates.insert(
            ("app".to_string(), "template".to_string()),
            Arc::new("dummy".to_string()),
        );
        Ok(AppState {
            templates: Arc::new(templates),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness,
            fonts,
            html_converter: Arc::new(build_html_converter(&cfg.fonts_dir, &cfg.root_dir).0),
            config: cfg,
        })
    }

    fn test_state_no_templates(alive: bool, ready: bool) -> anyhow::Result<AppState> {
        let aliveness = AppAliveness::new();
        aliveness.set_alive(alive);
        aliveness.set_ready(ready);
        let cfg = Config::default();
        let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
        Ok(AppState {
            templates: Arc::new(HashMap::new()),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness,
            fonts,
            html_converter: Arc::new(build_html_converter(&cfg.fonts_dir, &cfg.root_dir).0),
            config: cfg,
        })
    }

    #[tokio::test]
    async fn is_alive_returns_200_when_alive() -> anyhow::Result<()> {
        let server = TestServer::new(nais_router().with_state(test_state(true, false)?));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn is_alive_returns_500_when_not_alive() -> anyhow::Result<()> {
        let server = TestServer::new(nais_router().with_state(test_state(false, false)?));
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_200_when_ready() -> anyhow::Result<()> {
        let server = TestServer::new(nais_router().with_state(test_state(false, true)?));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_500_when_not_ready() -> anyhow::Result<()> {
        let server = TestServer::new(nais_router().with_state(test_state(false, false)?));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn is_ready_returns_500_when_no_templates() -> anyhow::Result<()> {
        let server =
            TestServer::new(nais_router().with_state(test_state_no_templates(false, true)?));
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }
}
