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
    /// The compilation task exceeded the configured timeout.
    RequestTimeout {
        app_name: String,
        template_name: Option<String>,
    },
}

#[cfg(test)]
impl std::fmt::Debug for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "NotFound"),
            Self::GenerationFailed { app_name, template_name, .. } => {
                write!(f, "GenerationFailed({app_name}, {template_name:?})")
            }
            Self::UnsupportedMediaType => write!(f, "UnsupportedMediaType"),
            Self::RequestTimeout { app_name, template_name } => {
                write!(f, "RequestTimeout({app_name}, {template_name:?})")
            }
        }
    }
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
            Self::RequestTimeout {
                ref app_name,
                ref template_name,
            } => {
                if let Some(tmpl) = template_name {
                    error!(app_name = %app_name, template_name = %tmpl, "Compilation timed out");
                } else {
                    error!(app_name = %app_name, "Compilation timed out");
                }
                (StatusCode::REQUEST_TIMEOUT, "Request timed out").into_response()
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use http_body_util::BodyExt;

    async fn status_and_body(error: ApiError) -> (StatusCode, String) {
        let response = error.into_response();
        let status = response.status();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, String::from_utf8(body.to_vec()).unwrap())
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let (status, body) = status_and_body(ApiError::NotFound).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body, "Template or application not found");
    }

    #[tokio::test]
    async fn generation_failed_returns_500_with_template() {
        let error = ApiError::GenerationFailed {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
            source: anyhow::anyhow!("render error"),
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body, "Internal server error");
    }

    #[tokio::test]
    async fn generation_failed_returns_500_without_template() {
        let error = ApiError::GenerationFailed {
            app_name: "myapp".to_string(),
            template_name: None,
            source: anyhow::anyhow!("render error"),
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body, "Internal server error");
    }

    #[tokio::test]
    async fn unsupported_media_type_returns_415() {
        let (status, body) = status_and_body(ApiError::UnsupportedMediaType).await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(body, "");
    }

    #[tokio::test]
    async fn request_timeout_returns_408_with_template() {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body, "Request timed out");
    }

    #[tokio::test]
    async fn request_timeout_returns_408_without_template() {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: None,
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body, "Request timed out");
    }
}
