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
pub(crate) enum ApiError {
    /// The requested template or application was not found.
    NotFound,
    /// An internal error occurred during document generation.
    GenerationFailed {
        app_name: String,
        template_name: Option<String>,
        source: anyhow::Error,
        /// When `true`, the error detail is included in the response body.
        dev_mode: bool,
    },
    /// The request body content type is not supported.
    UnsupportedMediaType,
    /// The compilation task exceeded the configured timeout.
    RequestTimeout {
        app_name: String,
        template_name: Option<String>,
    },
    /// The server is overloaded and cannot accept more compilation requests right now.
    ServiceOverloaded {
        /// Suggested number of seconds the client should wait before retrying.
        retry_after_seconds: u64,
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
            Self::ServiceOverloaded { .. } => write!(f, "ServiceOverloaded"),
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
/// The `type` member is a machine-readable URI identifying the problem type,
/// allowing clients to programmatically distinguish error categories.
fn problem_response(status: StatusCode, problem_type: &str, detail: &str) -> Response {
    let mut body = serde_json::json!({
        "type": problem_type,
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
            Self::NotFound => problem_response(
                StatusCode::NOT_FOUND,
                "urn:pdfgenrs:error:not-found",
                "Template or application not found",
            ),
            Self::GenerationFailed {
                ref app_name,
                ref template_name,
                ref source,
                dev_mode,
            } => {
                if let Some(tmpl) = template_name {
                    error!(app_name = %app_name, template_name = %tmpl, error = %source, "Document generation failed");
                } else {
                    error!(app_name = %app_name, error = %source, "Document generation failed");
                }
                let detail = if dev_mode {
                    format!("{source:#}")
                } else {
                    "Internal server error".to_string()
                };
                problem_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "urn:pdfgenrs:error:generation-failed",
                    &detail,
                )
            }
            Self::UnsupportedMediaType => problem_response(
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "urn:pdfgenrs:error:unsupported-media-type",
                "Unsupported media type",
            ),
            Self::RequestTimeout {
                ref app_name,
                ref template_name,
            } => {
                if let Some(tmpl) = template_name {
                    error!(app_name = %app_name, template_name = %tmpl, "Compilation timed out");
                } else {
                    error!(app_name = %app_name, "Compilation timed out");
                }
                problem_response(
                    StatusCode::REQUEST_TIMEOUT,
                    "urn:pdfgenrs:error:timeout",
                    "Request timed out",
                )
            }
            Self::ServiceOverloaded {
                retry_after_seconds,
            } => {
                error!("Semaphore acquisition timed out; server is overloaded");
                let mut response = problem_response(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "urn:pdfgenrs:error:overloaded",
                    "Service is overloaded, try again later",
                );
                if let Ok(value) = retry_after_seconds.to_string().parse() {
                    response.headers_mut().insert(header::RETRY_AFTER, value);
                }
                response
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, anyhow};
    use axum::http::StatusCode;
    use http_body_util::BodyExt;

    async fn status_and_body(error: ApiError) -> anyhow::Result<(StatusCode, serde_json::Value)> {
        let response = error.into_response();
        let status = response.status();
        let content_type = response
            .headers()
            .get("content-type")
            .ok_or_else(|| anyhow!("missing content-type header"))?
            .to_str()
            .context("invalid content-type header")?;
        assert_eq!(content_type, "application/problem+json; charset=utf-8");
        let body = response
            .into_body()
            .collect()
            .await
            .context("failed to read response body")?
            .to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).context("response body must be valid JSON")?;
        Ok((status, json))
    }

    #[tokio::test]
    async fn not_found_returns_404() -> anyhow::Result<()> {
        let (status, body) = status_and_body(ApiError::NotFound).await?;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body["type"], "urn:pdfgenrs:error:not-found");
        assert_eq!(body["title"], "Not Found");
        assert_eq!(body["status"], 404);
        assert_eq!(body["detail"], "Template or application not found");
        Ok(())
    }

    #[tokio::test]
    async fn generation_failed_returns_500_with_template() -> anyhow::Result<()> {
        let error = ApiError::GenerationFailed {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
            source: anyhow!("render error"),
            dev_mode: false,
        };
        let (status, body) = status_and_body(error).await?;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body["type"], "urn:pdfgenrs:error:generation-failed");
        assert_eq!(body["title"], "Internal Server Error");
        assert_eq!(body["status"], 500);
        assert_eq!(body["detail"], "Internal server error");
        Ok(())
    }

    #[tokio::test]
    async fn generation_failed_returns_500_without_template() -> anyhow::Result<()> {
        let error = ApiError::GenerationFailed {
            app_name: "myapp".to_string(),
            template_name: None,
            source: anyhow!("render error"),
            dev_mode: false,
        };
        let (status, body) = status_and_body(error).await?;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body["type"], "urn:pdfgenrs:error:generation-failed");
        assert_eq!(body["title"], "Internal Server Error");
        assert_eq!(body["status"], 500);
        assert_eq!(body["detail"], "Internal server error");
        Ok(())
    }

    #[tokio::test]
    async fn generation_failed_returns_error_detail_in_dev_mode() -> anyhow::Result<()> {
        let error = ApiError::GenerationFailed {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
            source: anyhow!("typst compilation error: missing field"),
            dev_mode: true,
        };
        let (status, body) = status_and_body(error).await?;
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(body["type"], "urn:pdfgenrs:error:generation-failed");
        assert_eq!(body["detail"], "typst compilation error: missing field");
        Ok(())
    }

    #[tokio::test]
    async fn unsupported_media_type_returns_415() -> anyhow::Result<()> {
        let (status, body) = status_and_body(ApiError::UnsupportedMediaType).await?;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(body["type"], "urn:pdfgenrs:error:unsupported-media-type");
        assert_eq!(body["title"], "Unsupported Media Type");
        assert_eq!(body["status"], 415);
        assert_eq!(body["detail"], "Unsupported media type");
        Ok(())
    }

    #[tokio::test]
    async fn request_timeout_returns_408_with_template() -> anyhow::Result<()> {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: Some("template1".to_string()),
        };
        let (status, body) = status_and_body(error).await?;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body["type"], "urn:pdfgenrs:error:timeout");
        assert_eq!(body["title"], "Request Timeout");
        assert_eq!(body["status"], 408);
        assert_eq!(body["detail"], "Request timed out");
        Ok(())
    }

    #[tokio::test]
    async fn request_timeout_returns_408_without_template() -> anyhow::Result<()> {
        let error = ApiError::RequestTimeout {
            app_name: "myapp".to_string(),
            template_name: None,
        };
        let (status, body) = status_and_body(error).await?;
        assert_eq!(status, StatusCode::REQUEST_TIMEOUT);
        assert_eq!(body["type"], "urn:pdfgenrs:error:timeout");
        assert_eq!(body["title"], "Request Timeout");
        assert_eq!(body["status"], 408);
        assert_eq!(body["detail"], "Request timed out");
        Ok(())
    }

    #[tokio::test]
    async fn service_overloaded_returns_503_with_retry_after() -> anyhow::Result<()> {
        let error = ApiError::ServiceOverloaded {
            retry_after_seconds: 10,
        };
        let response = error.into_response();
        let status = response.status();
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);

        let retry_after = response
            .headers()
            .get("retry-after")
            .ok_or_else(|| anyhow!("missing retry-after header"))?
            .to_str()
            .context("invalid retry-after header")?;
        assert_eq!(retry_after, "10");

        let body = response
            .into_body()
            .collect()
            .await
            .context("failed to read response body")?
            .to_bytes();
        let json: serde_json::Value =
            serde_json::from_slice(&body).context("response body must be valid JSON")?;
        assert_eq!(json["type"], "urn:pdfgenrs:error:overloaded");
        assert_eq!(json["title"], "Service Unavailable");
        assert_eq!(json["status"], 503);
        assert_eq!(json["detail"], "Service is overloaded, try again later");
        Ok(())
    }
}
