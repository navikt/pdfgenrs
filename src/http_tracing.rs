use axum::Router;

pub(crate) fn apply_http_tracing_layer(app: Router) -> Router {
    imp::apply_http_tracing_layer(app)
}

mod imp {
    use axum::body::Body;
    use axum::http::{HeaderMap, Request};
    use axum::Router;
    use opentelemetry::{global, propagation::Extractor};
    use tower_http::trace::TraceLayer;
    use tracing_opentelemetry::OpenTelemetrySpanExt;

    /// Implements [`opentelemetry::propagation::Extractor`] for an Axum [`HeaderMap`] so that
    /// the global W3C TraceContext + Baggage propagators can extract an incoming parent trace context
    /// from request headers.
    struct HeaderExtractor<'a>(&'a HeaderMap);

    impl<'a> Extractor for HeaderExtractor<'a> {
        fn get(&self, key: &str) -> Option<&str> {
            self.0.get(key).and_then(|v| v.to_str().ok())
        }

        fn keys(&self) -> Vec<&str> {
            self.0.keys().map(|k| k.as_str()).collect()
        }
    }

    /// Creates a tracing span for an HTTP request and sets the remote span as OTel parent.
    ///
    /// This is passed to [`TraceLayer::make_span_with`].  Extracting the trace context and calling
    /// [`OpenTelemetrySpanExt::set_parent`] here (synchronously, before any `.await`) avoids the
    /// `!Send` constraint of [`opentelemetry::ContextGuard`] and correctly parents the new span to
    /// the caller's distributed trace when a `traceparent` header is present.
    fn make_otel_span(request: &Request<Body>) -> tracing::Span {
        let span = tracing::info_span!(
            "HTTP request",
            http.method = %request.method(),
            http.uri = %request.uri(),
            http.version = ?request.version(),
            otel.kind = "server",
            otel.status_code = tracing::field::Empty,
            http.status_code = tracing::field::Empty,
        );
        let parent_cx = global::get_text_map_propagator(|propagator| {
            propagator.extract(&HeaderExtractor(request.headers()))
        });
        span.set_parent(parent_cx).ok();
        span
    }

    pub(super) fn apply_http_tracing_layer(app: Router) -> Router {
        if cfg!(test) {
            app
        } else {
            app.layer(TraceLayer::new_for_http().make_span_with(make_otel_span))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use axum::http::{HeaderValue, Method, Uri, Version};

        #[test]
        fn header_extractor_reads_valid_headers_and_skips_non_utf8_values() {
            let mut headers = HeaderMap::new();
            headers.insert("traceparent", HeaderValue::from_static("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01"));
            headers.insert("x-invalid", HeaderValue::from_bytes(b"\xFF").expect("opaque header value"));

            let extractor = HeaderExtractor(&headers);

            assert_eq!(
                extractor.get("traceparent"),
                Some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
            );
            assert_eq!(extractor.get("x-invalid"), None);

            let keys = extractor.keys();
            assert!(keys.contains(&"traceparent"));
            assert!(keys.contains(&"x-invalid"));
        }

        #[test]
        fn make_otel_span_creates_http_request_span() {
            let request = Request::builder()
                .method(Method::POST)
                .uri(Uri::from_static("/api/v1/genpdf"))
                .version(Version::HTTP_11)
                .header(
                    "traceparent",
                    "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
                )
                .body(Body::empty())
                .expect("request");

            let span = make_otel_span(&request);
            let metadata = span.metadata().expect("span metadata");

            assert_eq!(metadata.name(), "HTTP request");
            assert_eq!(metadata.level(), &tracing::Level::INFO);
        }
    }
}
