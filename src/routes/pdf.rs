use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, header},
    response::{IntoResponse, Response},
};
use serde_json::Value;
use std::sync::Arc;
use tracing::info;

use super::error::ApiError;
use super::{compile_blocking, lookup_template_and_data, lookup_template_with_data};
use crate::pdf as gen_pdf;
use crate::state::AppState;

/// Handles `GET /api/v1/genpdf/{app_name}/{template}` (dev mode only).
///
/// Looks up the template source and pre-loaded test JSON data for the given
/// `app_name` / `template` combination and returns a PDF response.
/// Returns `404` if the template or its test data cannot be found.
pub(crate) async fn get_pdf(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let params = lookup_template_and_data(&state, &template_key).await?;

    let pdf_bytes = compile_blocking(
        &state,
        template_key.0.clone(),
        Some(template_key.1.clone()),
        move || {
            gen_pdf::typst_to_pdf(
                Arc::unwrap_or_clone(params.source),
                &params.data,
                params.fonts,
                &params.root,
                &params.resources_dir,
                &app_name,
                &template_name,
                params.pdf_library,
                params.file_cache,
            )
        },
    )
    .await?;

    info!(app_name = %template_key.0, template_name = %template_key.1, duration_ms = start.elapsed().as_millis(), "Done generating PDF");
    Ok(pdf_response(pdf_bytes))
}

/// Handles `POST /api/v1/genpdf/{app_name}/{template}`.
///
/// Accepts a JSON body and compiles the named Typst template with that data,
/// returning the result as `application/pdf`.
/// Returns `404` if the template is not found, or `500` if compilation fails.
pub(crate) async fn post_pdf(
    State(state): State<AppState>,
    Path((app_name, template_name)): Path<(String, String)>,
    Json(json_data): Json<Value>,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let template_key = (app_name.clone(), template_name.clone());

    let params = lookup_template_with_data(&state, &template_key, json_data)?;

    let pdf_bytes = compile_blocking(
        &state,
        template_key.0.clone(),
        Some(template_key.1.clone()),
        move || {
            gen_pdf::typst_to_pdf(
                Arc::unwrap_or_clone(params.source),
                &params.data,
                params.fonts,
                &params.root,
                &params.resources_dir,
                &app_name,
                &template_name,
                params.pdf_library,
                params.file_cache,
            )
        },
    )
    .await?;

    info!(app_name = %template_key.0, template_name = %template_key.1, duration_ms = start.elapsed().as_millis(), "Done generating PDF");
    Ok(pdf_response(pdf_bytes))
}

/// Handles `POST /api/v1/genpdf/html/{app_name}`.
///
/// Accepts an HTML body and converts it to PDF.
pub(crate) async fn post_pdf_from_html(
    State(state): State<AppState>,
    Path(app_name): Path<String>,
    html: String,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let html_converter = Arc::clone(&state.html_converter);

    let pdf_bytes = compile_blocking(&state, app_name.clone(), None, move || {
        gen_pdf::html_to_pdf(&html, &html_converter)
    })
    .await?;

    info!(app_name = %app_name, duration_ms = start.elapsed().as_millis(), "Done generating PDF from HTML");
    Ok(pdf_response(pdf_bytes))
}

/// Handles `POST /api/v1/genpdf/image/{app_name}`.
///
/// Accepts a PNG, JPEG, WebP, or SVG body and converts it to PDF.
pub(crate) async fn post_pdf_from_image(
    State(state): State<AppState>,
    Path(app_name): Path<String>,
    headers: HeaderMap,
    image_bytes: Bytes,
) -> Result<Response, ApiError> {
    let start = std::time::Instant::now();
    let Some(image_path) = image_virtual_path(headers.get(header::CONTENT_TYPE)) else {
        return Err(ApiError::UnsupportedMediaType);
    };

    let fonts = Arc::clone(&state.fonts);
    let root = Arc::clone(&state.root_dir);
    let resources_dir = Arc::clone(&state.resources_dir);
    let library = Arc::clone(&state.pdf_library);
    let file_cache = state.file_cache.clone();

    let pdf_bytes = compile_blocking(&state, app_name.clone(), None, move || {
        gen_pdf::image_to_pdf(
            image_bytes,
            image_path,
            fonts,
            &root,
            &resources_dir,
            library,
            file_cache,
        )
    })
    .await?;

    info!(app_name = %app_name, duration_ms = start.elapsed().as_millis(), "Done generating PDF from image");
    Ok(pdf_response(pdf_bytes))
}

fn pdf_response(pdf_bytes: Vec<u8>) -> Response {
    let content_length = pdf_bytes.len().to_string();
    (
        [
            (header::CONTENT_TYPE, "application/pdf"),
            (header::CONTENT_DISPOSITION, "inline"),
            (header::CONTENT_LENGTH, content_length.as_str()),
        ],
        Bytes::from(pdf_bytes),
    )
        .into_response()
}

#[must_use]
fn image_virtual_path(content_type: Option<&HeaderValue>) -> Option<&'static str> {
    let content_type = content_type
        .and_then(|value| value.to_str().ok())?
        .split(';')
        .next()?
        .trim();

    match content_type {
        "image/png" => Some("/image.png"),
        "image/jpeg" => Some("/image.jpg"),
        "image/webp" => Some("/image.webp"),
        "image/svg+xml" => Some("/image.svg"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;

    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::{IntoResponse, Response};
    use axum::routing::{get, post};
    use axum::{Json, Router};
    use axum_test::TestServer;
    use serde_json::Value;
    use tokio::time::{Duration, timeout};

    use axum::body::Bytes;
    use axum::http::HeaderValue;

    use super::{get_pdf, image_virtual_path, post_pdf, post_pdf_from_html, post_pdf_from_image};
    use crate::state::AppState;
    use crate::testutil::make_state;

    const SIMPLE_TEMPLATE: &str =
        "#set document(title: \"Test\", date: auto)\n#set page(margin: 1cm)\nHello!\n";
    const INVALID_TEMPLATE: &str = "#this-is-not-valid-typst-syntax(((";
    const OVERSIZED_PAYLOAD_SIZE_BYTES: usize = 3 * 1024 * 1024;
    const DELAYED_REQUEST_DURATION_MS: u64 = 200;
    const CLIENT_TIMEOUT_DURATION_MS: u64 = 50;

    fn make_router(state: AppState, dev_mode: bool) -> Router {
        let mut router = Router::new()
            .route("/{app_name}/{template}", post(post_pdf))
            .route("/html/{app_name}", post(post_pdf_from_html))
            .route("/image/{app_name}", post(post_pdf_from_image));
        if dev_mode {
            router = router.route("/{app_name}/{template}", get(get_pdf));
        }
        router.with_state(state)
    }

    async fn delayed_post_pdf(
        State(state): State<AppState>,
        Path((app_name, template_name)): Path<(String, String)>,
        Json(json_data): Json<Value>,
    ) -> Response {
        tokio::time::sleep(Duration::from_millis(DELAYED_REQUEST_DURATION_MS)).await;
        post_pdf(
            State(state),
            Path((app_name, template_name)),
            Json(json_data),
        )
        .await
        .into_response()
    }

    fn make_router_with_delayed_post(state: AppState) -> Router {
        Router::new()
            .route("/{app_name}/{template}", post(delayed_post_pdf))
            .with_state(state)
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    // Keeps the request-level check slightly above the 90 MB compile-only guard in typst_world,
    // leaving room for Axum/TestServer/request handling overhead while still catching sustained
    // RSS growth across a long run of PDF requests.
    #[cfg(target_os = "linux")]
    const MAX_REQUEST_RSS_GROWTH_KB: u64 = 110_000;
    #[cfg(target_os = "linux")]
    const WARMUP_REQUEST_COUNT: usize = 10;
    #[cfg(target_os = "linux")]
    const MEMORY_REGRESSION_REQUEST_COUNT: usize = 200;

    #[cfg(target_os = "linux")]
    fn rss_kb() -> anyhow::Result<u64> {
        let status = std::fs::read_to_string("/proc/self/status")?;
        status
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|value| value.parse().ok())
            .ok_or_else(|| anyhow::anyhow!("Failed to parse VmRSS from /proc/self/status"))
    }

    #[tokio::test]
    async fn post_pdf_returns_pdf_for_valid_template() -> anyhow::Result<()> {
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
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_returns_404_when_template_missing() -> anyhow::Result<()> {
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
    async fn post_pdf_returns_500_for_invalid_template() -> anyhow::Result<()> {
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
    async fn post_pdf_returns_413_for_oversized_json_body() -> anyhow::Result<()> {
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
    async fn post_pdf_client_timeout_cancels_request_and_followup_still_succeeds()
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
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_html_returns_pdf() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
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
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_pdf_for_png() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/png")
            .bytes(Bytes::from(std::fs::read(
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
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_accepts_jpeg_content_type() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/jpeg")
            .bytes(Bytes::from(std::fs::read(
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("resources")
                    .join("NAVLogoRed.jpg"),
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
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_415_for_unsupported_media_type() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/gif")
            .bytes(Bytes::from_static(b"gif"))
            .await;

        assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        Ok(())
    }

    #[test]
    fn image_virtual_path_supports_png_and_jpeg() {
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/png"))),
            Some("/image.png")
        );
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/jpeg"))),
            Some("/image.jpg")
        );
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/webp"))),
            Some("/image.webp")
        );
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/svg+xml"))),
            Some("/image.svg")
        );
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/png; charset=utf-8"))),
            Some("/image.png")
        );
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("image/gif"))),
            None
        );
    }

    #[tokio::test]
    async fn get_pdf_returns_pdf_when_template_and_data_exist() -> anyhow::Result<()> {
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
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .ok_or_else(|| anyhow::anyhow!("missing content-type header"))?,
            "application/pdf"
        );
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn get_pdf_returns_404_when_data_missing() -> anyhow::Result<()> {
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
    async fn get_pdf_returns_404_when_template_missing() -> anyhow::Result<()> {
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

    #[tokio::test]
    async fn post_pdf_can_reference_image_from_resources_folder() -> anyhow::Result<()> {
        const TEMPLATE_WITH_IMAGE: &str = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            TEMPLATE_WITH_IMAGE.to_string(),
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
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn post_pdf_repeated_requests_do_not_grow_memory_unboundedly() -> anyhow::Result<()> {
        let _guard = crate::memory_sensitive_test_lock().lock().await;
        const TEMPLATE_WITH_JSON: &str = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
#let data = json("/data/myapp/mytemplate.json")
#data.at("message", default: "")
"#;

        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            TEMPLATE_WITH_JSON.to_string(),
        );
        let server = TestServer::new(make_router(
            make_state(templates, HashMap::new(), false)?,
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

        let rss_before = rss_kb()?;

        for _ in 0..MEMORY_REGRESSION_REQUEST_COUNT {
            let response = server
                .post("/myapp/mytemplate")
                .json(&serde_json::json!({ "message": "steady-request" }))
                .await;
            response.assert_status_success();
            assert!(is_pdf(response.as_bytes()));
        }

        let rss_after = rss_kb()?;
        let growth_kb = rss_after.saturating_sub(rss_before);

        assert!(
            growth_kb < MAX_REQUEST_RSS_GROWTH_KB,
            "RSS grew by {growth_kb} KB after {MEMORY_REGRESSION_REQUEST_COUNT} requests – possible memory leak."
        );

        Ok(())
    }

    #[cfg(not(target_os = "linux"))]
    #[tokio::test]
    #[ignore = "requires Linux RSS metrics from /proc/self/status"]
    async fn post_pdf_repeated_requests_do_not_grow_memory_unboundedly() {
        // Intentionally empty: this regression check only runs on Linux.
    }

    // --- Unit tests for image_virtual_path edge cases ---

    #[test]
    fn image_virtual_path_returns_none_for_no_header() {
        assert_eq!(image_virtual_path(None), None);
    }

    #[test]
    fn image_virtual_path_returns_none_for_empty_value() {
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static(""))),
            None
        );
    }

    #[test]
    fn image_virtual_path_returns_none_for_text_plain() {
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("text/plain"))),
            None
        );
    }

    #[test]
    fn image_virtual_path_returns_none_for_application_pdf() {
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static("application/pdf"))),
            None
        );
    }

    #[test]
    fn image_virtual_path_trims_whitespace_in_content_type() {
        assert_eq!(
            image_virtual_path(Some(&HeaderValue::from_static(" image/png "))),
            Some("/image.png")
        );
    }

    // --- Unit tests for pdf_response helper ---

    #[tokio::test]
    async fn pdf_response_sets_correct_headers() {
        use super::pdf_response;
        use http_body_util::BodyExt;

        let response = pdf_response(b"%PDF-1.4 fake content".to_vec());
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .map(|v| v.to_str().ok()),
            Some(Some("application/pdf"))
        );
        assert_eq!(
            response
                .headers()
                .get("content-disposition")
                .map(|v| v.to_str().ok()),
            Some(Some("inline"))
        );

        let body = response.into_body().collect().await.map(|b| b.to_bytes());
        assert!(body.is_ok());
        assert!(body.ok().is_some_and(|b| b.starts_with(b"%PDF")));
    }

    #[tokio::test]
    async fn pdf_response_handles_empty_bytes() {
        use super::pdf_response;
        use http_body_util::BodyExt;

        let response = pdf_response(Vec::new());
        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.map(|b| b.to_bytes());
        assert!(body.is_ok());
        assert!(body.ok().is_some_and(|b| b.is_empty()));
    }

    // --- HTML-to-PDF error path tests ---

    #[tokio::test]
    async fn post_pdf_from_html_returns_pdf_for_empty_html() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server.post("/html/myapp").text("").await;

        // Empty HTML still produces a PDF (the converter renders a blank page)
        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_html_returns_pdf_for_minimal_html() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server.post("/html/myapp").text("<p>Hello</p>").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .map(|v| v.to_str().ok()),
            Some(Some("application/pdf"))
        );
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    // --- Image-to-PDF error path tests ---

    #[tokio::test]
    async fn post_pdf_from_image_returns_500_for_corrupted_png() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/png")
            .bytes(Bytes::from_static(b"not a valid png file"))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_500_for_corrupted_jpeg() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/jpeg")
            .bytes(Bytes::from_static(b"not a valid jpeg file"))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_500_for_empty_image_bytes() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/png")
            .bytes(Bytes::from_static(b""))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_415_without_content_type() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .bytes(Bytes::from_static(b"some bytes"))
            .await;

        assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        Ok(())
    }

    // --- Timeout tests for HTML and image endpoints ---

    async fn delayed_post_pdf_from_html(
        State(state): State<AppState>,
        Path(app_name): Path<String>,
        html: String,
    ) -> Response {
        tokio::time::sleep(Duration::from_millis(DELAYED_REQUEST_DURATION_MS)).await;
        post_pdf_from_html(State(state), Path(app_name), html)
            .await
            .into_response()
    }

    async fn delayed_post_pdf_from_image(
        State(state): State<AppState>,
        Path(app_name): Path<String>,
        headers: axum::http::HeaderMap,
        image_bytes: Bytes,
    ) -> Response {
        tokio::time::sleep(Duration::from_millis(DELAYED_REQUEST_DURATION_MS)).await;
        post_pdf_from_image(State(state), Path(app_name), headers, image_bytes)
            .await
            .into_response()
    }

    #[tokio::test]
    async fn post_pdf_from_html_client_timeout_cancels_and_followup_succeeds() -> anyhow::Result<()>
    {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let router = Router::new()
            .route("/html/{app_name}", post(delayed_post_pdf_from_html))
            .with_state(state);
        let server = TestServer::new(router);

        let timed_out = timeout(
            Duration::from_millis(CLIENT_TIMEOUT_DURATION_MS),
            server.post("/html/myapp").text("<p>Hello</p>"),
        )
        .await;
        assert!(timed_out.is_err());

        let response = server.post("/html/myapp").text("<p>Hello</p>").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_client_timeout_cancels_and_followup_succeeds() -> anyhow::Result<()>
    {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let router = Router::new()
            .route("/image/{app_name}", post(delayed_post_pdf_from_image))
            .with_state(state);
        let server = TestServer::new(router);

        let image_bytes = Bytes::from(std::fs::read(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("resources")
                .join("NAVLogoRed.png"),
        )?);

        let timed_out = timeout(
            Duration::from_millis(CLIENT_TIMEOUT_DURATION_MS),
            server
                .post("/image/myapp")
                .content_type("image/png")
                .bytes(image_bytes.clone()),
        )
        .await;
        assert!(timed_out.is_err());

        let response = server
            .post("/image/myapp")
            .content_type("image/png")
            .bytes(image_bytes)
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        assert!(is_pdf(response.as_bytes()));
        Ok(())
    }

    // --- get_pdf error path: invalid template ---

    #[tokio::test]
    async fn get_pdf_returns_500_for_invalid_template() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            INVALID_TEMPLATE.to_string(),
        );
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "mytemplate".to_string()),
            serde_json::json!({}),
        );
        let server = TestServer::new(make_router(make_state(templates, data, true)?, true));

        let response = server.get("/myapp/mytemplate").await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_returns_422_for_non_json_content_type() -> anyhow::Result<()> {
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
            .content_type("text/plain")
            .bytes(Bytes::from_static(b"not json"))
            .await;

        assert!(
            response.status_code().is_client_error(),
            "Non-JSON content type should return a client error, got {}",
            response.status_code()
        );
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_returns_422_for_malformed_json() -> anyhow::Result<()> {
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
            .content_type("application/json")
            .bytes(Bytes::from_static(b"{invalid json"))
            .await;

        assert!(
            response.status_code().is_client_error(),
            "Malformed JSON should return a client error, got {}",
            response.status_code()
        );
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_415_for_text_html_content_type() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("text/html")
            .bytes(Bytes::from_static(b"<html></html>"))
            .await;

        assert_eq!(response.status_code(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        Ok(())
    }

    #[tokio::test]
    async fn post_pdf_from_image_returns_500_for_corrupted_webp() -> anyhow::Result<()> {
        let server = TestServer::new(make_router(
            make_state(HashMap::new(), HashMap::new(), false)?,
            false,
        ));

        let response = server
            .post("/image/myapp")
            .content_type("image/webp")
            .bytes(Bytes::from_static(b"not a valid webp file"))
            .await;

        assert_eq!(response.status_code(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(())
    }
}
