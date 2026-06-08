use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

use super::error::ApiError;
use super::{compile_blocking, lookup_template_and_data, lookup_template_with_data};
use crate::html as gen_html;
use crate::state::AppState;

/// Handles `GET /api/v1/genhtml/{app_name}/{template}` (dev mode only).
///
/// Looks up the template source and preloaded test JSON data for the given
/// `app_name` / `template` combination and returns an HTML response.
/// Returns `404` if the template or its test data cannot be found.
pub async fn get_html(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let params = lookup_template_and_data(&state, &template_key).await?;

    let html_string = compile_blocking(
        &state,
        template_key.0.clone(),
        Some(template_key.1.clone()),
        move || {
            gen_html::typst_to_html(
                Arc::unwrap_or_clone(params.source),
                &params.data,
                params.fonts,
                &params.root,
                &params.resources_dir,
                &app_name,
                &template_name,
            )
        },
    )
    .await?;

    info!(app_name = %template_key.0, template_name = %template_key.1, duration_ms = start.elapsed().as_millis(), "Done generating HTML");
    Ok(html_response(html_string))
}

/// Handles `POST /api/v1/genhtml/{app_name}/{template}`.
///
/// Accepts a JSON body and compiles the named Typst template with that data,
/// returning the result as `text/html`.
/// Returns `404` if the template is not found, or `500` if compilation fails.
pub async fn post_html(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
    Json(json_data): Json<Value>,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let params = lookup_template_with_data(&state, &template_key, json_data)?;

    let html_string = compile_blocking(
        &state,
        template_key.0.clone(),
        Some(template_key.1.clone()),
        move || {
            gen_html::typst_to_html(
                Arc::unwrap_or_clone(params.source),
                &params.data,
                params.fonts,
                &params.root,
                &params.resources_dir,
                &app_name,
                &template_name,
            )
        },
    )
    .await?;

    info!(app_name = %template_key.0, template_name = %template_key.1, duration_ms = start.elapsed().as_millis(), "Done generating HTML");
    Ok(html_response(html_string))
}

fn html_response(html: String) -> Response {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        Body::from(html),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::body::Bytes;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use axum_test::TestServer;
    use serde_json::Value;
    use tokio::time::{Duration, timeout};

    use super::{get_html, post_html};
    use crate::state::AppState;
    use crate::testutil::make_state;

    const SIMPLE_TEMPLATE: &str = "Hello!\n";
    const INVALID_TEMPLATE: &str = "#this-is-not-valid-typst-syntax(((";
    const OVERSIZED_PAYLOAD_SIZE_BYTES: usize = 3 * 1024 * 1024;
    const DELAYED_REQUEST_DURATION_MS: u64 = 200;
    const CLIENT_TIMEOUT_DURATION_MS: u64 = 50;

    fn make_router(state: AppState, dev_mode: bool) -> Router {
        let mut router = Router::new().route("/{app_name}/{template}", post(post_html));
        if dev_mode {
            router = router.route("/{app_name}/{template}", get(get_html));
        }
        router.with_state(state)
    }

    async fn delayed_post_html(
        State(state): State<AppState>,
        Path((app_name, template_name)): Path<(String, String)>,
        Json(json_data): Json<Value>,
    ) -> Response {
        tokio::time::sleep(Duration::from_millis(DELAYED_REQUEST_DURATION_MS)).await;
        post_html(
            State(state),
            Path((app_name, template_name)),
            Json(json_data),
        )
        .await
        .into_response()
    }

    fn make_router_with_delayed_post(state: AppState) -> Router {
        Router::new()
            .route("/{app_name}/{template}", post(delayed_post_html))
            .with_state(state)
    }

    fn is_html(body: &str) -> bool {
        body.contains("<!DOCTYPE html>") && body.contains("<html")
    }

    #[tokio::test]
    async fn post_html_returns_html_for_valid_template() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            SIMPLE_TEMPLATE.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?
                .to_str()?
                .starts_with("text/html")
        );
        assert!(is_html(response.text().as_str()));
        Ok(())
    }

    #[tokio::test]
    async fn post_html_returns_404_when_template_missing() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn post_html_returns_500_for_invalid_template() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            INVALID_TEMPLATE.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn post_html_returns_413_for_oversized_json_body() -> anyhow::Result<()> {
        let oversized_payload = format!(
            r#"{{"data":"{}"}}"#,
            "a".repeat(OVERSIZED_PAYLOAD_SIZE_BYTES)
        );
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .content_type("application/json")
            .bytes(Bytes::from(oversized_payload))
            .await;

        assert_eq!(response.status_code(), StatusCode::PAYLOAD_TOO_LARGE);
        Ok(())
    }

    #[tokio::test]
    async fn post_html_client_timeout_cancels_request_and_followup_still_succeeds()
    -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            SIMPLE_TEMPLATE.to_string(),
        );
        let server = TestServer::new(make_router_with_delayed_post(make_state(
            templates,
            HashMap::new(),
            false,
        )?));

        let timed_out = timeout(
            Duration::from_millis(CLIENT_TIMEOUT_DURATION_MS),
            server
                .post("/myapp/mytemplate")
                .json(&serde_json::json!({})),
        )
        .await;
        assert!(timed_out.is_err());

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(is_html(response.text().as_str()));
        Ok(())
    }

    #[tokio::test]
    async fn get_html_returns_html_when_template_and_data_exist() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            SIMPLE_TEMPLATE.to_string(),
        );
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            serde_json::json!({}),
        );
        let server = TestServer::new(make_router(make_state(templates, data, true)?, true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?
                .to_str()?
                .starts_with("text/html")
        );
        assert!(is_html(response.text().as_str()));
        Ok(())
    }

    #[tokio::test]
    async fn get_html_returns_404_when_data_missing() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            SIMPLE_TEMPLATE.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), true)?,
            true,
        ));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn get_html_returns_404_when_template_missing() -> anyhow::Result<()> {
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            serde_json::json!({}),
        );
        let server = TestServer::new(make_router(make_state(HashMap::new(), data, true)?, true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        Ok(())
    }
}
