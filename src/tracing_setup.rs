use anyhow::Result;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::Sampler,
    Resource,
};
use std::time::Duration;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Custom JSON formatter that emits NAIS-compatible structured log lines.
///
/// Each log event is serialised as a single JSON object containing timestamp,
/// level, target, source location, active OpenTelemetry span/trace IDs, and
/// all recorded event fields.
struct NaisJsonFormat;

fn logger_name_for_file(file: &str) -> String {
    file.strip_suffix(".rs").unwrap_or(file).replace('/', ".")
}

impl<S, N> fmt::format::FormatEvent<S, N> for NaisJsonFormat
where
    S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    N: for<'a> fmt::FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &fmt::FmtContext<'_, S, N>,
        mut writer: fmt::format::Writer<'_>,
        event: &tracing::Event<'_>,
    ) -> std::fmt::Result {
        use opentelemetry::trace::TraceContextExt;
        use serde_json::{Map, Value};

        let meta = event.metadata();
        let mut log_object = Map::new();

        log_object.insert(
            "timestamp".to_string(),
            Value::String(chrono::Utc::now().to_rfc3339()),
        );
        log_object.insert(
            "log_level".to_string(),
            Value::String(meta.level().to_string()),
        );
        log_object.insert(
            "target".to_string(),
            Value::String(meta.target().to_string()),
        );

        if let Some(file) = meta.file() {
            log_object.insert("file".to_string(), Value::String(file.to_string()));
            let logger_name = logger_name_for_file(file);
            log_object.insert("logger_name".to_string(), Value::String(logger_name));
        }
        if let Some(line) = meta.line() {
            log_object.insert(
                "line".to_string(),
                Value::Number(serde_json::Number::from(line)),
            );
        }

        let otel_context = opentelemetry::Context::current();
        let otel_span = otel_context.span();
        let span_context = otel_span.span_context();
        if span_context.is_valid() {
            log_object.insert(
                "trace_id".to_string(),
                Value::String(span_context.trace_id().to_string()),
            );
            log_object.insert(
                "span_id".to_string(),
                Value::String(span_context.span_id().to_string()),
            );
        }

        if let Some(span) = ctx.lookup_current() {
            log_object.insert("span".to_string(), Value::String(span.name().to_string()));
        }

        struct FieldVisitor<'a> {
            map: &'a mut Map<String, Value>,
        }
        impl tracing::field::Visit for FieldVisitor<'_> {
            fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                self.map
                    .insert(field.name().to_string(), Value::String(value.to_string()));
            }
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                self.map.insert(
                    field.name().to_string(),
                    Value::String(format!("{:?}", value)),
                );
            }
        }

        let mut visitor = FieldVisitor {
            map: &mut log_object,
        };
        event.record(&mut visitor);

        let serialized = serde_json::to_string(&log_object)
            .or_else(|err| {
                serde_json::to_string(&serde_json::json!({
                    "timestamp": chrono::Utc::now().to_rfc3339(),
                    "log_level": "ERROR",
                    "target": "tracing_setup",
                    "message": "failed to serialize log object",
                    "error": err.to_string(),
                }))
            })
            .map_err(|_| std::fmt::Error)?;
        write!(&mut writer, "{serialized}")?;
        writeln!(&mut writer)
    }
}

/// Builds a gRPC OTLP span exporter when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
///
/// The endpoint and TLS/insecure settings are read automatically from the
/// standard OTEL environment variables injected by the NAIS platform. Returns
/// `None` when the variable is absent so that tracing remains a no-op in local
/// development without any additional configuration.
fn nais_otlp_exporter(endpoint: Option<&str>) -> Result<Option<SpanExporter>> {
    if endpoint.is_none() {
        return Ok(None);
    }
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_timeout(Duration::from_secs(5))
        .build()?;
    Ok(Some(exporter))
}

fn resolve_service_name(name: Option<&str>) -> String {
    name.filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "pdfgenrs".to_string())
}

/// Initialises the global tracing subscriber with OpenTelemetry and NAIS-style JSON logging.
///
/// When `OTEL_EXPORTER_OTLP_ENDPOINT` is set (injected by the NAIS platform via
/// `spec.observability.autoInstrumentation.runtime: sdk`) spans are exported via
/// gRPC. All other OTEL environment variables (`OTEL_SERVICE_NAME`,
/// `OTEL_RESOURCE_ATTRIBUTES`, `OTEL_EXPORTER_OTLP_INSECURE`, …) are consumed
/// automatically by the SDK. `service.name=pdfgenrs` is used as a fallback when
/// `OTEL_SERVICE_NAME` is not present (i.e. local development).
///
/// Log records emitted by third-party crates via the `log` crate are bridged
/// into tracing so they appear in the same JSON output.
///
/// Returns the `SdkTracerProvider` so the caller can call `.shutdown()` for a
/// graceful flush before the process exits.
pub fn setup_tracing() -> Result<opentelemetry_sdk::trace::SdkTracerProvider> {
    let exporter =
        nais_otlp_exporter(std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok().as_deref())?;
    let exporter_active = exporter.is_some();

    let builder = opentelemetry_sdk::trace::SdkTracerProvider::builder();
    let builder = if let Some(otlp_exporter) = exporter {
        builder.with_batch_exporter(otlp_exporter)
    } else {
        builder
    };

    // NAIS injects OTEL_TRACES_SAMPLER=parentbased_always_on. The Rust SDK does not read
    // this env var automatically, so we configure the sampler explicitly to match.
    let builder = builder.with_sampler(Sampler::ParentBased(Box::new(Sampler::AlwaysOn)));

    // NAIS injects OTEL_SERVICE_NAME (and OTEL_RESOURCE_ATTRIBUTES) into the pod when
    // spec.observability.autoInstrumentation.enabled=true / runtime=sdk is set.
    // Resource::builder() includes SdkProvidedResourceDetector which always produces a
    // service.name (falling back to "unknown_service" when OTEL_SERVICE_NAME is absent).
    // Because ResourceBuilder::with_attribute() merges by letting the new value win, calling
    // it unconditionally would silently override whatever OTEL_SERVICE_NAME NAIS injects.
    // We therefore read the env var ourselves and only substitute the hardcoded name as a
    // local-development fallback when the variable is absent or empty.
    let service_name = resolve_service_name(std::env::var("OTEL_SERVICE_NAME").ok().as_deref());
    let tracer_provider = builder
        .with_resource(
            Resource::builder()
                .with_attribute(KeyValue::new("service.name", service_name))
                .build(),
        )
        .build();

    // NAIS injects OTEL_PROPAGATORS=tracecontext,baggage. The Rust SDK does not read this env
    // var automatically, so both propagators are registered explicitly to match.
    global::set_text_map_propagator(TextMapCompositePropagator::new(vec![
        Box::new(TraceContextPropagator::new()),
        Box::new(BaggagePropagator::new()),
    ]));

    let fmt_layer = fmt::layer().event_format(NaisJsonFormat).with_ansi(false);

    let tracer = tracer_provider.tracer("pdfgenrs");
    // Keep a clone for the caller to shut down gracefully before exit.
    // SdkTracerProvider is backed by Arc so this is cheap.
    let provider_for_shutdown = tracer_provider.clone();
    global::set_tracer_provider(tracer_provider);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(OpenTelemetryLayer::new(tracer))
        .with(fmt_layer)
        .init();

    // Bridge `log` crate records from third-party libraries into tracing.
    if let Err(e) = tracing_log::LogTracer::init() {
        tracing::debug!("LogTracer already initialised: {e}");
    }

    tracing::info!(
        exporter_active,
        "Tracing initialised (OTEL exporter active: {exporter_active})"
    );
    Ok(provider_for_shutdown)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_name_for_file_rewrites_path_and_suffix() {
        assert_eq!(
            logger_name_for_file("src/tracing_setup.rs"),
            "src.tracing_setup"
        );
    }

    #[test]
    fn logger_name_for_file_keeps_non_rs_suffix() {
        assert_eq!(
            logger_name_for_file("src/tracing_setup"),
            "src.tracing_setup"
        );
    }

    #[test]
    fn resolve_service_name_uses_env_value() {
        assert_eq!(
            resolve_service_name(Some("custom-service")),
            "custom-service"
        );
    }

    #[test]
    fn resolve_service_name_falls_back_when_missing_or_empty() {
        assert_eq!(resolve_service_name(None), "pdfgenrs");
        assert_eq!(resolve_service_name(Some("")), "pdfgenrs");
    }

    #[test]
    fn nais_otlp_exporter_is_none_without_endpoint() -> anyhow::Result<()> {
        let exporter = nais_otlp_exporter(None)?;
        assert!(exporter.is_none());
        Ok(())
    }
}
