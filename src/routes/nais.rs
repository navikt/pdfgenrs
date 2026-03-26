use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::AppState;

pub fn nais_router() -> Router<AppState> {
    Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
}

pub async fn is_alive(State(state): State<AppState>) -> Response {
    if state.aliveness.is_alive() {
        (StatusCode::OK, "I'm alive").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "I'm dead x_x").into_response()
    }
}

pub async fn is_ready(State(state): State<AppState>) -> Response {
    if state.aliveness.is_ready() {
        (StatusCode::OK, "I'm ready").into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Please wait! I'm not ready :(",
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use crate::config::Config;
    use crate::state::AppAliveness;
    use crate::{typst_world, AppState};
    use super::nais_router;

    fn test_state(alive: bool, ready: bool) -> AppState {
        let aliveness = AppAliveness::new();
        aliveness.set_alive(alive);
        aliveness.set_ready(ready);
        let cfg = Config::default();
        let fonts = Arc::new(typst_world::load_font_cache(&cfg.fonts_dir));
        AppState {
            templates: Arc::new(HashMap::new()),
            data: Arc::new(RwLock::new(HashMap::new())),
            aliveness,
            fonts,
            config: cfg,
        }
    }

    #[tokio::test]
    async fn is_alive_returns_200_when_alive() {
        let server = TestServer::new(nais_router().with_state(test_state(true, false))).unwrap();
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn is_alive_returns_500_when_not_alive() {
        let server = TestServer::new(nais_router().with_state(test_state(false, false))).unwrap();
        let response = server.get("/internal/is_alive").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn is_ready_returns_200_when_ready() {
        let server = TestServer::new(nais_router().with_state(test_state(false, true))).unwrap();
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::OK);
    }

    #[tokio::test]
    async fn is_ready_returns_500_when_not_ready() {
        let server = TestServer::new(nais_router().with_state(test_state(false, false))).unwrap();
        let response = server.get("/internal/is_ready").await;
        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
