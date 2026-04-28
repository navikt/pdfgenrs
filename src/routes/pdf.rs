use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::path::PathBuf;
use log::{error, info};

use crate::{pdf as gen_pdf, AppState};

pub async fn get_pdf(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Response {
    let tmpl_name = format!("{}/{}", app_name, template_name);

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
            let fonts = (*state.fonts).clone();
            let root = PathBuf::from(&state.config.templates_dir);
            match tokio::task::spawn_blocking(move || {
                gen_pdf::typst_to_pdf(&source, &data, fonts, &root)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
            {
                Err(e) => {
                    error!("PDF generation failed: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
                Ok(pdf_bytes) => pdf_response(pdf_bytes),
            }
        }
    }
}

pub async fn post_pdf(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
    Json(json_data): Json<Value>,
) -> Response {
    let start = std::time::Instant::now();
    let tmpl_name = format!("{}/{}", app_name, template_name);

    let template_source = match state.templates.get(&tmpl_name).cloned() {
        Some(s) => s,
        None => {
            return (StatusCode::NOT_FOUND, "Template or application not found").into_response();
        }
    };

    let fonts = (*state.fonts).clone();
    let root = PathBuf::from(&state.config.templates_dir);
    match tokio::task::spawn_blocking(move || {
        gen_pdf::typst_to_pdf(&template_source, &json_data, fonts, &root)
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!("PDF generation failed for {tmpl_name}: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        Ok(pdf_bytes) => {
            info!("Done generating PDF in {}ms", start.elapsed().as_millis());
            pdf_response(pdf_bytes)
        }
    }
}

fn pdf_response(pdf_bytes: Vec<u8>) -> Response {
    (
        [(header::CONTENT_TYPE, "application/pdf")],
        Bytes::from(pdf_bytes),
    )
        .into_response()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum::routing::{get, post};
    use axum::Router;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use crate::{config, state, typst_world, AppState};
    use super::{get_pdf, post_pdf};

    const SIMPLE_TEMPLATE: &str = "#set document(date: auto)\n#set page(margin: 1cm)\nHello!\n";
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
                templates_dir: "templates".to_string(),
                resources_dir: "resources".to_string(),
                data_dir: "data".to_string(),
                dev_mode,
            },
            fonts: Arc::new(typst_world::load_fonts()),
        }
    }

    fn make_router(state: AppState, dev_mode: bool) -> Router {
        let mut router = Router::new()
            .route("/{app_name}/{template}", post(post_pdf));
        if dev_mode {
            router = router.route("/{app_name}/{template}", get(get_pdf));
        }
        router.with_state(state)
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    #[tokio::test]
    async fn post_pdf_returns_pdf_for_valid_template() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), SIMPLE_TEMPLATE.to_string());
        let server = TestServer::new(make_router(make_state(templates, HashMap::new(), false), false));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
    }

    #[tokio::test]
    async fn post_pdf_returns_404_when_template_missing() {
        let server = TestServer::new(make_router(make_state(HashMap::new(), HashMap::new(), false), false));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_pdf_returns_500_for_invalid_template() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), INVALID_TEMPLATE.to_string());
        let server = TestServer::new(make_router(make_state(templates, HashMap::new(), false), false));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn get_pdf_returns_pdf_when_template_and_data_exist() {
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
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
    }

    #[tokio::test]
    async fn get_pdf_returns_404_when_data_missing() {
        let mut templates = HashMap::new();
        templates.insert("myapp/mytemplate".to_string(), SIMPLE_TEMPLATE.to_string());
        let server = TestServer::new(make_router(make_state(templates, HashMap::new(), true), true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_pdf_returns_404_when_template_missing() {
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
