use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

use crate::html as gen_html;
use crate::state::AppState;

/// Handles `GET /api/v1/genhtml/{app_name}/{template}` (dev mode only).
///
/// Looks up the template source and pre-loaded test JSON data for the given
/// `app_name` / `template` combination and returns an HTML response.
/// Returns `404` if the template or its test data cannot be found.
pub async fn get_html(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Response {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let template_source = state.templates.get(&template_key).cloned();
    let json_data = {
        let data_map = state.data.read().await;
        data_map
            .get(&(app_name.clone(), template_name.clone()))
            .cloned()
    };

    match (template_source, json_data) {
        (None, _) | (_, None) => {
            (StatusCode::NOT_FOUND, "Template or application not found").into_response()
        }
        (Some(source), Some(data)) => {
            let fonts = Arc::clone(&state.fonts);
            let root = state.config.root_dir.clone();
            let resources_dir = state.config.resource_root();
            match tokio::task::spawn_blocking(move || {
                gen_html::typst_to_html(&source, &data, fonts, &root, &resources_dir)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
            {
                Err(e) => {
                    error!(app_name = %app_name, template_name = %template_name, error = %e, "HTML generation failed");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
                Ok(html_string) => {
                    info!(app_name = %app_name, template_name = %template_name, duration_ms = start.elapsed().as_millis(), "Done generating HTML");
                    html_response(html_string)
                }
            }
        }
    }
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
) -> Response {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let Some(template_source) = state.templates.get(&template_key).cloned() else {
        return (StatusCode::NOT_FOUND, "Template or application not found").into_response();
    };

    let fonts = Arc::clone(&state.fonts);
    let root = state.config.root_dir.clone();
    let resources_dir = state.config.resource_root();
    match tokio::task::spawn_blocking(move || {
        gen_html::typst_to_html(&template_source, &json_data, fonts, &root, &resources_dir)
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!(app_name = %app_name, template_name = %template_name, error = %e, "HTML generation failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        Ok(html_string) => {
            info!(app_name = %app_name, template_name = %template_name, duration_ms = start.elapsed().as_millis(), "Done generating HTML");
            html_response(html_string)
        }
    }
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
    use std::path::PathBuf;
    use std::sync::Arc;

    use axum::body::Bytes;
    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::Response;
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use axum_test::TestServer;
    use serde_json::Value;
    use tokio::sync::RwLock;
    use tokio::time::{Duration, timeout};

    use super::{get_html, post_html};
    use crate::state::AppState;
    use crate::{config, load_html_font_aliases, state, typst_world};

    const SIMPLE_TEMPLATE: &str = "Hello!\n";
    const INVALID_TEMPLATE: &str = "#this-is-not-valid-typst-syntax(((";
    const OVERSIZED_PAYLOAD_SIZE_BYTES: usize = 3 * 1024 * 1024;
    const DELAYED_REQUEST_DURATION_MS: u64 = 200;
    const CLIENT_TIMEOUT_DURATION_MS: u64 = 50;

    fn make_state(
        templates: HashMap<(String, String), String>,
        data: HashMap<(String, String), serde_json::Value>,
        dev_mode: bool,
    ) -> anyhow::Result<AppState> {
        Ok(AppState {
            templates: Arc::new(templates),
            data: Arc::new(RwLock::new(data)),
            aliveness: state::AppAliveness::new(),
            config: config::Config {
                port: 8080,
                root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
                templates_dir: PathBuf::from("templates"),
                resources_dir: PathBuf::from("resources"),
                data_dir: PathBuf::from("data"),
                fonts_dir: PathBuf::from("fonts"),
                dev_mode,
                request_body_limit_bytes: 2 * 1024 * 1024,
            },
            fonts: Arc::new(typst_world::load_fonts(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"),
            )?),
            html_font_aliases: Arc::new(load_html_font_aliases(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"),
            )),
        })
    }

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
