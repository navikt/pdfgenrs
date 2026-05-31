use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

/// Centralized error type for API route handlers.
///
/// Each variant maps to a specific HTTP status code and carries enough context
/// for structured logging while returning a safe message to the client.
pub enum ApiError {
    /// The requested template or application was not found.
    NotFound,
    /// An internal error occurred during document generation.
    GenerationFailed {
        app_name: String,
        template_name: Option<String>,
        source: anyhow::Error,
    },
    /// The request body content type is not supported.
    UnsupportedMediaType,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => {
                (StatusCode::NOT_FOUND, "Template or application not found").into_response()
            }
            Self::GenerationFailed {
                ref app_name,
                ref template_name,
                ref source,
            } => {
                if let Some(tmpl) = template_name {
                    error!(app_name = %app_name, template_name = %tmpl, error = %source, "Document generation failed");
                } else {
                    error!(app_name = %app_name, error = %source, "Document generation failed");
                }
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            Self::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response(),
        }
    }
}
