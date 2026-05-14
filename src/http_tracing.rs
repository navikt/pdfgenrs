use axum::Router;

#[cfg(not(test))]
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
        app.layer(TraceLayer::new_for_http().make_span_with(make_otel_span))
    }
}

#[cfg(test)]
mod imp {
    use axum::Router;

    pub(super) fn apply_http_tracing_layer(app: Router) -> Router {
        app
    }
}

pub(crate) fn apply_http_tracing_layer(app: Router) -> Router {
    imp::apply_http_tracing_layer(app)
}
