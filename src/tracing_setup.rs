use anyhow::Result;
use opentelemetry::propagation::TextMapCompositePropagator;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{KeyValue, global};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    Resource,
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::Sampler,
};
use std::time::Duration;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
            fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
                // serde_json rejects NaN/infinity; fall back to 0 rather than
                // emitting a tracing warning here (which would be reentrant).
                let num = serde_json::Number::from_f64(value)
                    .unwrap_or_else(|| serde_json::Number::from(0));
                self.map
                    .insert(field.name().to_string(), Value::Number(num));
            }
            fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
                self.map
                    .insert(field.name().to_string(), Value::Number(value.into()));
            }
            fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
                self.map
                    .insert(field.name().to_string(), Value::Number(value.into()));
            }
            fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
                self.map
                    .insert(field.name().to_string(), Value::Bool(value));
            }
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
pub(crate) fn setup_tracing() -> Result<opentelemetry_sdk::trace::SdkTracerProvider> {
    setup_tracing_with(
        std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").ok().as_deref(),
        std::env::var("OTEL_SERVICE_NAME").ok().as_deref(),
    )
}

fn setup_tracing_with(
    otlp_endpoint: Option<&str>,
    service_name_env: Option<&str>,
) -> Result<opentelemetry_sdk::trace::SdkTracerProvider> {
    let exporter = nais_otlp_exporter(otlp_endpoint)?;
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
    let service_name = resolve_service_name(service_name_env);
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
#[allow(clippy::panic)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::io;
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::{layer::SubscriberExt, registry::Registry};

    #[derive(Clone, Default)]
    struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

    impl SharedBuffer {
        fn lines(&self) -> Vec<String> {
            let bytes = self
                .0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .clone();
            let output = match String::from_utf8(bytes) {
                Ok(output) => output,
                Err(error) => panic!("expected valid utf-8 log output: {error}"),
            };
            output.lines().map(str::to_string).collect()
        }
    }

    struct SharedWriter(Arc<Mutex<Vec<u8>>>);

    impl<'a> fmt::MakeWriter<'a> for SharedBuffer {
        type Writer = SharedWriter;

        fn make_writer(&'a self) -> Self::Writer {
            SharedWriter(self.0.clone())
        }
    }

    impl io::Write for SharedWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn parse_single_log_line(buffer: &SharedBuffer) -> Value {
        let lines = buffer.lines();
        assert_eq!(lines.len(), 1);
        match serde_json::from_str(&lines[0]) {
            Ok(value) => value,
            Err(error) => panic!("expected JSON log line: {error}"),
        }
    }

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
    fn nais_otlp_exporter_is_none_without_endpoint() -> Result<()> {
        let exporter = nais_otlp_exporter(None)?;
        assert!(exporter.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn nais_otlp_exporter_is_some_with_endpoint() -> Result<()> {
        let exporter = nais_otlp_exporter(Some("http://127.0.0.1:4317"))?;
        assert!(exporter.is_some());
        Ok(())
    }

    /// Verifies that numeric and boolean values are serialised as the correct
    /// JSON types (number / bool) rather than being quoted as strings.
    #[test]
    fn field_visitor_numeric_and_bool_produce_json_numbers_and_bools() {
        use serde_json::{Map, Value};

        let mut map: Map<String, Value> = Map::new();

        // u64 → JSON number
        map.insert("count".to_string(), Value::Number(42u64.into()));
        assert!(map["count"].is_number());
        assert_eq!(map["count"].as_u64(), Some(42));

        // i64 → JSON number
        map.insert("delta".to_string(), Value::Number((-7i64).into()));
        assert!(map["delta"].is_number());
        assert_eq!(map["delta"].as_i64(), Some(-7));

        // f64 → JSON number
        let f64_num =
            serde_json::Number::from_f64(1.5).unwrap_or_else(|| serde_json::Number::from(0));
        map.insert("ratio".to_string(), Value::Number(f64_num));
        assert!(map["ratio"].is_number());
        assert_eq!(map["ratio"].as_f64(), Some(1.5));

        // bool → JSON bool
        map.insert("active".to_string(), Value::Bool(true));
        assert_eq!(map["active"], Value::Bool(true));

        // NaN f64 falls back to 0
        let nan_fallback =
            serde_json::Number::from_f64(f64::NAN).unwrap_or_else(|| serde_json::Number::from(0));
        assert_eq!(nan_fallback, serde_json::Number::from(0));
    }

    #[test]
    fn nais_json_format_serializes_span_and_event_fields() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::info_span!("request-span");
            let _entered = span.enter();
            tracing::info!(
                string_field = "value",
                count = 42u64,
                delta = -7i64,
                ratio = 1.5f64,
                active = true,
                debug_field = ?vec![1, 2, 3],
                "formatted log line"
            );
        });

        let log_line = parse_single_log_line(&buffer);
        let file = match log_line["file"].as_str() {
            Some(file) => file,
            None => panic!("expected file field"),
        };

        assert!(log_line["timestamp"].as_str().is_some());
        assert_eq!(log_line["log_level"], Value::String("INFO".to_string()));
        assert_eq!(
            log_line["target"],
            Value::String(module_path!().to_string())
        );
        assert_eq!(log_line["file"], Value::String(file.to_string()));
        assert_eq!(
            log_line["logger_name"],
            Value::String(logger_name_for_file(file))
        );
        assert!(log_line["line"].as_u64().is_some());
        assert_eq!(log_line["span"], Value::String("request-span".to_string()));
        assert_eq!(
            log_line["message"],
            Value::String("formatted log line".to_string())
        );
        assert_eq!(log_line["string_field"], Value::String("value".to_string()));
        assert_eq!(log_line["count"], Value::Number(42u64.into()));
        assert_eq!(log_line["delta"], Value::Number((-7i64).into()));
        let ratio = match serde_json::Number::from_f64(1.5) {
            Some(ratio) => ratio,
            None => panic!("expected finite number"),
        };
        assert_eq!(log_line["ratio"], Value::Number(ratio));
        assert_eq!(log_line["active"], Value::Bool(true));
        assert_eq!(
            log_line["debug_field"],
            Value::String("[1, 2, 3]".to_string())
        );
    }

    #[test]
    fn nais_json_format_falls_back_for_nan_values() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(ratio = f64::NAN, "nan value");
        });

        let log_line = parse_single_log_line(&buffer);
        assert_eq!(log_line["ratio"], Value::Number(0.into()));
        assert!(log_line.get("span").is_none());
    }

    #[test]
    fn nais_json_format_falls_back_for_infinity_values() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!(
                pos_inf = f64::INFINITY,
                neg_inf = f64::NEG_INFINITY,
                "infinity values"
            );
        });

        let log_line = parse_single_log_line(&buffer);
        assert_eq!(log_line["pos_inf"], Value::Number(0.into()));
        assert_eq!(log_line["neg_inf"], Value::Number(0.into()));
    }

    #[test]
    fn nais_json_format_timestamp_is_rfc3339() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("timestamp check");
        });

        let log_line = parse_single_log_line(&buffer);
        let ts = match log_line["timestamp"].as_str() {
            Some(ts) => ts,
            None => panic!("timestamp should be a string"),
        };
        // Validate it parses as a valid RFC3339 / ISO8601 datetime
        assert!(
            chrono::DateTime::parse_from_rfc3339(ts).is_ok(),
            "timestamp should be valid RFC3339: {ts}"
        );
    }

    #[test]
    fn nais_json_format_omits_trace_and_span_ids_without_otel_context() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("no otel context");
        });

        let log_line = parse_single_log_line(&buffer);
        assert!(
            log_line.get("trace_id").is_none(),
            "trace_id should be absent without valid OTel span context"
        );
        assert!(
            log_line.get("span_id").is_none(),
            "span_id should be absent without valid OTel span context"
        );
    }

    #[test]
    fn nais_json_format_omits_span_field_without_active_span() {
        let buffer = SharedBuffer::default();
        let subscriber = Registry::default().with(
            fmt::layer()
                .event_format(NaisJsonFormat)
                .with_ansi(false)
                .with_writer(buffer.clone()),
        );

        tracing::subscriber::with_default(subscriber, || {
            tracing::info!("no span active");
        });

        let log_line = parse_single_log_line(&buffer);
        assert!(
            log_line.get("span").is_none(),
            "span field should be absent when no tracing span is entered"
        );
    }

    #[test]
    fn setup_tracing_initializes_without_otlp_exporter() -> Result<()> {
        let provider = setup_tracing_with(None, Some("pdfgenrs-test"))?;
        tracing::info!(test_case = "setup_tracing", "subscriber initialized");
        provider.shutdown()?;
        Ok(())
    }
}
