use axum::{
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use opentelemetry::trace::TraceContextExt;
use tracing::error;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// Centralized error type for API route handlers.
///
/// Each variant maps to a specific HTTP status code and carries enough context
/// for structured logging while returning a safe message to the client.
///
/// Responses use the [RFC 9457](https://www.rfc-editor.org/rfc/rfc9457) Problem
/// Details format (`application/problem+json`).
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
            Self::GenerationFailed {
                app_name,
                template_name,
                ..
            } => {
                write!(f, "GenerationFailed({app_name}, {template_name:?})")
            }
            Self::UnsupportedMediaType => write!(f, "UnsupportedMediaType"),
            Self::RequestTimeout {
                app_name,
                template_name,
            } => {
                write!(f, "RequestTimeout({app_name}, {template_name:?})")
            }
        }
    }
}

/// Returns the current OpenTelemetry trace ID if one is active, or `None`.
fn current_trace_id() -> Option<String> {
    let context = tracing::Span::current().context();
    let span_ref = context.span();
    let span_context = span_ref.span_context();
    if span_context.is_valid() {
        Some(format!("{:032x}", span_context.trace_id()))
    } else {
        None
    }
}

/// Builds an RFC 9457 Problem Details JSON response.
///
/// The `type` member is always `"about:blank"` which signals that the `title`
/// carries the same semantics as the HTTP status phrase (§4.2.1 of RFC 9457).
fn problem_response(status: StatusCode, detail: &str) -> Response {
    let mut body = serde_json::json!({
        "type": "about:blank",
        "title": status.canonical_reason().unwrap_or("Error"),
        "status": status.as_u16(),
        "detail": detail,
    });

    if let Some(trace_id) = current_trace_id() {
        body["trace_id"] = serde_json::Value::String(trace_id);
    }

    (
        status,
        [(
            header::CONTENT_TYPE,
            "application/problem+json; charset=utf-8",
        )],
        body.to_string(),
    )
        .into_response()
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound => {
                problem_response(StatusCode::NOT_FOUND, "Template or application not found")
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
                problem_response(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            Self::UnsupportedMediaType => {
                problem_response(StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported media type")
            }
            Self::RequestTimeout {
                ref app_name,
                ref template_name,
            } => {
                if let Some(tmpl) = template_name {
                    error!(app_name = %app_name, template_name = %tmpl, "Compilation timed out");
                } else {
                    error!(app_name = %app_name, "Compilation timed out");
                }
                problem_response(StatusCode::REQUEST_TIMEOUT, "Request timed out")
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

    async fn status_and_body(error: ApiError) -> (StatusCode, serde_json::Value) {
        let response = error.into_response();
        let status = response.status();
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .unwrap()
                .to_str()
                .unwrap(),
            "application/problem+json; charset=utf-8"
        );
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).expect("response body must be valid JSON");
        (status, json)
    }

    #[tokio::test]
    async fn not_found_returns_404() {
        let (status, body) = status_and_body(ApiError::NotFound).await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Not Found");
        assert_eq!(body["status"], 404);
        assert_eq!(body["detail"], "Template or application not found");
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
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Internal Server Error");
        assert_eq!(body["status"], 500);
        assert_eq!(body["detail"], "Internal server error");
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
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Internal Server Error");
        assert_eq!(body["status"], 500);
        assert_eq!(body["detail"], "Internal server error");
    }

    #[tokio::test]
    async fn unsupported_media_type_returns_415() {
        let (status, body) = status_and_body(ApiError::UnsupportedMediaType).await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Unsupported Media Type");
        assert_eq!(body["status"], 415);
        assert_eq!(body["detail"], "Unsupported media type");
    }

    #[tokio::test]
    async fn request_timeout_returns_408_with_template() {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Request Timeout");
        assert_eq!(body["status"], 408);
        assert_eq!(body["detail"], "Request timed out");
    }

    #[tokio::test]
    async fn request_timeout_returns_408_without_template() {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: None,
        };
        let (status, body) = status_and_body(error).await;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body["type"], "about:blank");
        assert_eq!(body["title"], "Request Timeout");
        assert_eq!(body["status"], 408);
        assert_eq!(body["detail"], "Request timed out");
    }
}
