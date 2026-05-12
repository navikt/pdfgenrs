use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

use crate::{html as gen_html, AppState};

/// Handles `GET /api/v1/genhtml/{app_name}/{template}` (dev mode only).
///
/// Looks up the template source and pre-loaded test JSON data for the given
/// `app_name` / `template` combination and returns an HTML response.
/// Returns `404` if the template or its test data cannot be found.
pub async fn get_html(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Response {
    let tmpl_name = format!("{app_name}/{template_name}");

    let template_source = state.templates.get(&tmpl_name).cloned();
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
            match tokio::task::spawn_blocking(move || {
                gen_html::typst_to_html(&source, &data, fonts, &root)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
            {
                Err(e) => {
                    error!(template = %tmpl_name, error = %e, "HTML generation failed");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
                Ok(html_string) => html_response(html_string),
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
    let tmpl_name = format!("{app_name}/{template_name}");

    let Some(template_source) = state.templates.get(&tmpl_name).cloned() else {
        return (StatusCode::NOT_FOUND, "Template or application not found").into_response();
    };

    let fonts = Arc::clone(&state.fonts);
    let root = state.config.root_dir.clone();
    match tokio::task::spawn_blocking(move || {
        gen_html::typst_to_html(&template_source, &json_data, fonts, &root)
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!(template = %tmpl_name, error = %e, "HTML generation failed");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        Ok(html_string) => {
            info!(template = %tmpl_name, duration_ms = start.elapsed().as_millis(), "Done generating HTML");
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

    use axum::http::StatusCode;
    use axum::routing::{get, post};
    use axum::Router;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use super::{get_html, post_html};
    use crate::{config, state, typst_world, AppState};

    const SIMPLE_TEMPLATE: &str = "Hello!\n";
    const INVALID_TEMPLATE: &str = "#this-is-not-valid-typst-syntax(((";

    fn make_state(
        templates: HashMap<String, String>,
        data: HashMap<(String, String), serde_json::Value>,
        dev_mode: bool,
    ) -> AppState {
        AppState {
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
            },
            fonts: Arc::new(
                typst_world::load_fonts(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"))
                    .expect("test fonts should load"),
            ),
        }
    }

    fn make_router(state: AppState, dev_mode: bool) -> Router {
        let mut router = Router::new().route("/{app_name}/{template}", post(post_html));
        if dev_mode {
            router = router.route("/{app_name}/{template}", get(get_html));
        }
        router.with_state(state)
    }

    fn is_html(body: &str) -> bool {
        body.contains("<!DOCTYPE html>") && body.contains("<html")
    }

    #[tokio::test]
    async fn post_html_returns_html_for_valid_template() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), SIMPLE_TEMPLATE.to_string());
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false),
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html"));
        assert!(is_html(response.text().as_str()));
    }

    #[tokio::test]
    async fn post_html_returns_404_when_template_missing() {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false),
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_html_returns_500_for_invalid_template() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), INVALID_TEMPLATE.to_string());
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false),
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn get_html_returns_html_when_template_and_data_exist() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), SIMPLE_TEMPLATE.to_string());
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            serde_json::json!({}),
        );
        let server = TestServer::new(make_router(make_state(templates, data, true), true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(response
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("text/html"));
        assert!(is_html(response.text().as_str()));
    }

    #[tokio::test]
    async fn get_html_returns_404_when_data_missing() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), SIMPLE_TEMPLATE.to_string());
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), true),
            true,
        ));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_html_returns_404_when_template_missing() {
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            serde_json::json!({}),
        );
        let server = TestServer::new(make_router(make_state(HashMap::new(), data, true), true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
