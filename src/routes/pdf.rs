use axum::{
    body::Bytes,
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

fn pdf_response(pdf_bytes: Vec<u8>) -> Response {
    (
        [(header::CONTENT_TYPE, "application/pdf")],
        Bytes::from(pdf_bytes),
    )
        .into_response()
}
