use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::collections::HashMap;

use crate::{metrics, AppState};

pub fn nais_router() -> Router<AppState> {
    Router::new()
        .route("/internal/is_alive", get(is_alive))
        .route("/internal/is_ready", get(is_ready))
        .route("/internal/prometheus", get(prometheus_metrics))
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

pub async fn prometheus_metrics(
    Query(params): Query<HashMap<String, String>>,
) -> Response {
    let names: Vec<String> = params
        .iter()
        .filter(|(k, _)| k.as_str() == "name[]")
        .map(|(_, v)| v.clone())
        .collect();

    match metrics::gather_metrics(&names) {
        Ok(output) => {
            let mut resp = output.into_response();
            resp.headers_mut().insert(
                "Content-Type",
                axum::http::HeaderValue::from_static(
                    "text/plain; version=0.0.4; charset=utf-8",
                ),
            );
            resp
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to gather metrics: {e}"),
        )
            .into_response(),
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

    #[tokio::test]
    async fn prometheus_metrics_returns_200() {
        let server = TestServer::new(nais_router().with_state(test_state(true, true))).unwrap();
        let response = server.get("/internal/prometheus").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/plain; version=0.0.4; charset=utf-8"
        );
    }
}
