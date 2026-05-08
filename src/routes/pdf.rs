use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use tracing::{error, info};

use crate::{pdf as gen_pdf, AppState};

/// Handles `GET /api/v1/genpdf/{app_name}/{template}` (dev mode only).
///
/// Looks up the template source and pre-loaded test JSON data for the given
/// `app_name` / `template` combination and returns a PDF response.
/// Returns `404` if the template or its test data cannot be found.
pub async fn get_pdf(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Response {
    let start = std::time::Instant::now();
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
                gen_pdf::typst_to_pdf(&source, &data, fonts, &root)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
            {
                Err(e) => {
                    error!("PDF generation failed: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                }
                Ok(pdf_bytes) => {
                    info!("Done generating PDF in {}ms", start.elapsed().as_millis());
                    pdf_response(pdf_bytes)
                }
            }
        }
    }
}
/// Handles `POST /api/v1/genpdf/{app_name}/{template}`.
///
/// Accepts a JSON body and compiles the named Typst template with that data,
/// returning the result as `application/pdf`.
/// Returns `404` if the template is not found, or `500` if compilation fails.
pub async fn post_pdf(
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

/// Handles `POST /api/v1/genpdf/html/{app_name}`.
///
/// Accepts an HTML body and converts it to PDF.
pub async fn post_pdf_from_html(
    State(state): State<AppState>,
    Path(app_name): Path<String>,
    html: String,
) -> Response {
    let start = std::time::Instant::now();
    let root = state.config.root_dir.clone();
    let fonts_dir = root.join(&state.config.fonts_dir);

    match tokio::task::spawn_blocking(move || gen_pdf::html_to_pdf(&html, &root, &fonts_dir))
        .await
        .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!("HTML-to-PDF generation failed for {app_name}: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
        Ok(pdf_bytes) => {
            info!(
                "Done generating PDF from HTML for {app_name} in {}ms",
                start.elapsed().as_millis()
            );
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
    use std::path::PathBuf;
    use std::sync::Arc;

    use axum::http::StatusCode;
    use axum::routing::{get, post};
    use axum::Router;
    use axum_test::TestServer;
    use tokio::sync::RwLock;

    use super::{get_pdf, post_pdf, post_pdf_from_html};
    use crate::{config, state, typst_world, AppState};

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
        let mut router = Router::new()
            .route("/{app_name}/{template}", post(post_pdf))
            .route("/html/{app_name}", post(post_pdf_from_html));
        if dev_mode {
            router = router.route("/{app_name}/{template}", get(get_pdf));
        }
        router.with_state(state)
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    // Keeps the request-level check slightly above the 90 MB compile-only guard in typst_world,
    // leaving room for Axum/TestServer/request handling overhead while still catching sustained
    // RSS growth across a long run of PDF requests.
    const MAX_REQUEST_RSS_GROWTH_KB: u64 = 110_000;
    const WARMUP_REQUEST_COUNT: usize = 10;
    const MEMORY_REGRESSION_REQUEST_COUNT: usize = 200;

    #[cfg(target_os = "linux")]
    fn rss_kb() -> u64 {
        let status = std::fs::read_to_string("/proc/self/status")
            .expect("Failed to read /proc/self/status for RSS measurement");
        status
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse().ok())
            .expect("Failed to parse VmRSS from /proc/self/status")
    }

    #[tokio::test]
    async fn post_pdf_returns_pdf_for_valid_template() {
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
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
    }

    #[tokio::test]
    async fn post_pdf_returns_404_when_template_missing() {
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
    async fn post_pdf_returns_500_for_invalid_template() {
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
    async fn post_pdf_from_html_returns_pdf() {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false),
            false,
        ));

        let response = server
            .post("/html/myapp")
            .text(
                r#"<!DOCTYPE html>
<html>
<head>
    <style>
        h1 {
            font-family: "Source Sans Pro" !important;
        }
    </style>
</head>
<body><h1>Hello</h1></body>
</html>"#,
            )
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
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
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), true),
            true,
        ));

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

    #[tokio::test]
    async fn post_pdf_can_reference_image_from_resources_folder() {
        const TEMPLATE_WITH_IMAGE: &str = r#"#set document(date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let mut templates = HashMap::new();
        templates.insert(
            "myapp/mytemplate".to_string(),
            TEMPLATE_WITH_IMAGE.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false),
            false,
        ));

        let response = server
            .post("/myapp/mytemplate")
            .json(&serde_json::json!({}))
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(is_pdf(response.as_bytes()));
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn post_pdf_repeated_requests_do_not_grow_memory_unboundedly() {
        let _guard = crate::memory_sensitive_test_lock().lock().unwrap();
        const TEMPLATE_WITH_JSON: &str = r#"#set document(date: auto)
#set page(margin: 1cm)
#let data = json("/data.json")
#data.at("message", default: "")
"#;

        let mut templates = HashMap::new();
        templates.insert(
            "myapp/mytemplate".to_string(),
            TEMPLATE_WITH_JSON.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false),
            false,
        ));

        for i in 0..WARMUP_REQUEST_COUNT {
            let response = server
                .post("/myapp/mytemplate")
                .json(&serde_json::json!({ "message": format!("warmup-{i}") }))
                .await;
            response.assert_status_success();
            assert!(is_pdf(response.as_bytes()));
        }

        let rss_before = rss_kb();

        for _ in 0..MEMORY_REGRESSION_REQUEST_COUNT {
            let response = server
                .post("/myapp/mytemplate")
                .json(&serde_json::json!({ "message": "steady-request" }))
                .await;
            response.assert_status_success();
            assert!(is_pdf(response.as_bytes()));
        }

        let rss_after = rss_kb();
        let growth_kb = rss_after.saturating_sub(rss_before);

        assert!(
            growth_kb < MAX_REQUEST_RSS_GROWTH_KB,
            "RSS grew by {growth_kb} KB after {MEMORY_REGRESSION_REQUEST_COUNT} requests – possible memory leak."
        );
    }

    #[cfg(not(target_os = "linux"))]
    #[tokio::test]
    #[ignore = "requires Linux RSS metrics from /proc/self/status"]
    async fn post_pdf_repeated_requests_do_not_grow_memory_unboundedly() {
        // Intentionally empty: this regression check only runs on Linux.
    }
}
