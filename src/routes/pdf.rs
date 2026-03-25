use axum::{
    body::{Body, Bytes},
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::Value;
use std::path::PathBuf;
use ::log::{error, info};

use crate::{pdf as gen_pdf, AppState};

/// GET /api/v1/genpdf/{applicationName}/{template}
/// Renders a PDF from a Typst template using pre-loaded test data.
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
            let font_cache = (*state.fonts).clone();
            let root = PathBuf::from(&state.config.templates_dir);
            match tokio::task::spawn_blocking(move || {
                gen_pdf::typst_to_pdf(&source, &data, font_cache, &root)
            })
            .await
            .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
            {
                Err(e) => {
                    error!("PDF generation failed: {e}");
                    (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
                Ok(pdf_bytes) => pdf_response(pdf_bytes),
            }
        }
    }
}

/// POST /api/v1/genpdf/{applicationName}/{template}
/// Renders a PDF from a Typst template with JSON data from the request body.
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

    let font_cache = (*state.fonts).clone();
    let root = PathBuf::from(&state.config.templates_dir);
    match tokio::task::spawn_blocking(move || {
        gen_pdf::typst_to_pdf(&template_source, &json_data, font_cache, &root)
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!("PDF generation failed for {tmpl_name}: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
        Ok(pdf_bytes) => {
            info!("Done generating PDF in {}ms", start.elapsed().as_millis());
            pdf_response(pdf_bytes)
        }
    }
}

/// POST /api/v1/genpdf/html/{applicationName}
/// Converts a raw HTML string to PDF using Typst.
pub async fn post_html_to_pdf(
    State(state): State<AppState>,
    Path(_app_name): Path<String>,
    body: String,
) -> Response {
    let font_cache = (*state.fonts).clone();
    let root = PathBuf::from(&state.config.templates_dir);
    match tokio::task::spawn_blocking(move || gen_pdf::html_to_pdf(&body, font_cache, &root))
        .await
        .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!("HTML-to-PDF conversion failed: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
        Ok(pdf_bytes) => pdf_response(pdf_bytes),
    }
}

/// POST /api/v1/genpdf/image/{applicationName}
/// Converts a JPEG or PNG image to PDF using Typst.
pub async fn post_image_to_pdf(
    State(state): State<AppState>,
    Path(_app_name): Path<String>,
    request: axum::http::Request<Body>,
) -> Response {
    let content_type = request
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !content_type.contains("jpeg") && !content_type.contains("png") && !content_type.contains("jpg") {
        return StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response();
    }

    let body_bytes = match axum::body::to_bytes(request.into_body(), usize::MAX).await {
        Ok(b) => b,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
        }
    };

    let font_cache = (*state.fonts).clone();
    let root = PathBuf::from(&state.config.templates_dir);
    match tokio::task::spawn_blocking(move || {
        gen_pdf::image_to_pdf(&body_bytes, &content_type, font_cache, &root)
    })
    .await
    .unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {e}")))
    {
        Err(e) => {
            error!("Image-to-PDF conversion failed: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
        Ok(pdf_bytes) => pdf_response(pdf_bytes),
    }
}

fn pdf_response(pdf_bytes: Vec<u8>) -> Response {
    (
        [(header::CONTENT_TYPE, "application/pdf")],
        Bytes::from(pdf_bytes),
    )
        .into_response()
}
