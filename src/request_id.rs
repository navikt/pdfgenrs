use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};
use tracing::Span;
use uuid::Uuid;

static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Middleware that propagates or generates an `X-Request-Id` header.
///
/// If the incoming request contains an `X-Request-Id` header with a valid
/// value, it is echoed back in the response. Otherwise a new UUID v4 is
/// generated and attached to the response.
pub(crate) async fn request_id_middleware(request: Request, next: Next) -> Response {
    let request_id = request
        .headers()
        .get(&X_REQUEST_ID)
        .cloned()
        .or_else(|| HeaderValue::from_str(&Uuid::new_v4().to_string()).ok())
        .unwrap_or_else(|| HeaderValue::from_static("unknown"));

    if let Ok(id_str) = request_id.to_str() {
        Span::current().record("request_id", id_str);
    }

    let mut response = next.run(request).await;
    response
        .headers_mut()
        .insert(X_REQUEST_ID.clone(), request_id);
    response
}

#[cfg(test)]
mod tests {
    use super::*;
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
    async fn generates_request_id_when_not_provided() {
        let server = TestServer::new(test_app());
        let response = server.get("/").await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let header = match response.headers().get("x-request-id") {
            Some(v) => v,
            None => panic!("expected x-request-id header in response"),
        };
        let value = match header.to_str() {
            Ok(v) => v,
            Err(e) => panic!("expected valid header value: {e}"),
        };
        assert!(
            Uuid::parse_str(value).is_ok(),
            "expected valid UUID, got: {value}"
        );
    }

    #[tokio::test]
    async fn propagates_request_id_from_request() {
        let server = TestServer::new(test_app());
        let response = server
            .get("/")
            .add_header(
                X_REQUEST_ID.clone(),
                HeaderValue::from_static("my-custom-id"),
            )
            .await;
        assert_eq!(response.status_code(), StatusCode::OK);
        let header = match response.headers().get("x-request-id") {
            Some(v) => v,
            None => panic!("expected x-request-id header in response"),
        };
        let value = match header.to_str() {
            Ok(v) => v,
            Err(e) => panic!("expected valid header value: {e}"),
        };
        assert_eq!(value, "my-custom-id");
    }
}
