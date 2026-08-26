use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::Span;
use uuid::Uuid;

static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

tokio::task_local! {
    pub(crate) static CURRENT_REQUEST_ID: String;
}

/// Returns the request ID associated with the currently executing task, if any.
pub(crate) fn current_request_id() -> Option<String> {
    CURRENT_REQUEST_ID.try_with(|id| id.clone()).ok()
}

/// Middleware that propagates or generates an `X-Request-Id` header.
///
/// If the incoming request contains an `X-Request-Id` header with a valid
/// value, it is echoed back in the response. Otherwise a new UUID v4 is
/// generated and attached to the response.
///
/// The resolved request ID is stored in a task-local so that error handlers
/// can embed it in problem responses without requiring access to the request.
pub(crate) async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(&X_REQUEST_ID)
        .filter(|v| v.to_str().is_ok())
        .cloned()
        .or_else(|| HeaderValue::from_str(&Uuid::new_v4().to_string()).ok())
        .unwrap_or_else(|| HeaderValue::from_static("unknown"));

    let id_string = request_id.to_str().unwrap_or("unknown").to_string();
    Span::current().record("request_id", &id_string);

    let mut response = CURRENT_REQUEST_ID.scope(id_string, next.run(request)).await;
    response
        .headers_mut()
        .insert(X_REQUEST_ID.clone(), request_id);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use axum::http::StatusCode;
    use axum::{Router, middleware, routing::get};
    use axum_test::TestServer;

    async fn handler() -> StatusCode {
        StatusCode::OK
    }

    fn test_app() -> Router {
        Router::new()
            .route("/", get(handler))
            .layer(middleware::from_fn(request_id_middleware))
    }

    #[tokio::test]
    async fn generates_request_id_when_not_provided() -> anyhow::Result<()> {
        let server = TestServer::new(test_app());
        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let header = response
            .headers()
            .get("x-request-id")
            .ok_or_else(|| anyhow!("expected x-request-id header in response"))?;
        let value = header.to_str()?;
        assert!(
            Uuid::parse_str(value).is_ok(),
            "expected valid UUID, got: {value}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn propagates_request_id_from_request() -> anyhow::Result<()> {
        let server = TestServer::new(test_app());
        let response = server
            .get("/")
            .add_header(
                X_REQUEST_ID.clone(),
                HeaderValue::from_static("my-custom-id"),
            )
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let header = response
            .headers()
            .get("x-request-id")
            .ok_or_else(|| anyhow!("expected x-request-id header in response"))?;
        let value = header.to_str()?;
        assert_eq!(value, "my-custom-id");
        Ok(())
    }

    #[tokio::test]
    async fn replaces_invalid_request_id_from_request() -> anyhow::Result<()> {
        let server = TestServer::new(test_app());
        let response = server
            .get("/")
            .add_header(X_REQUEST_ID.clone(), HeaderValue::from_bytes(b"\xFF")?)
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let header = response
            .headers()
            .get("x-request-id")
            .ok_or_else(|| anyhow!("expected x-request-id header in response"))?;
        let value = header.to_str()?;
        assert!(
            Uuid::parse_str(value).is_ok(),
            "expected valid UUID, got: {value}"
        );
        Ok(())
    }
}
