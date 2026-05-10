use anyhow::Result;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::{global, KeyValue};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry::propagation::TextMapCompositePropagator;
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
        use std::fmt::Write as FmtWrite;

        let meta = event.metadata();

        write!(&mut writer, "{{")?;
        write!(
            &mut writer,
            "\"timestamp\":\"{}\"",
            chrono::Utc::now().to_rfc3339()
        )?;
        write!(&mut writer, ",\"log_level\":\"{}\"", meta.level())?;
        write!(&mut writer, ",\"target\":\"{}\"", meta.target())?;

        if let Some(file) = meta.file() {
            write!(&mut writer, ",\"file\":\"{}\"", file)?;
            let logger_name = logger_name_for_file(file);
            write!(&mut writer, ",\"logger_name\":\"{}\"", logger_name)?;
        }
        if let Some(line) = meta.line() {
            write!(&mut writer, ",\"line\":{}", line)?;
        }

        let otel_context = opentelemetry::Context::current();
        let otel_span = otel_context.span();
        let span_context = otel_span.span_context();
        if span_context.is_valid() {
            write!(
                &mut writer,
                ",\"trace_id\":\"{}\"",
                span_context.trace_id()
            )?;
            write!(
                &mut writer,
                ",\"span_id\":\"{}\"",
                span_context.span_id()
            )?;
        }

        if let Some(span) = ctx.lookup_current() {
            write!(&mut writer, ",\"span\":\"{}\"", span.name())?;
        }

        struct FieldVisitor<W> {
            writer: W,
            result: std::fmt::Result,
        }
        impl<W: FmtWrite> tracing::field::Visit for FieldVisitor<W> {
            fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
                if self.result.is_err() {
                    return;
                }
                self.result = write!(
                    &mut self.writer,
                    ",\"{}\":\"{}\"",
                    field.name(),
                    value.replace('\\', "\\\\").replace('"', "\\\"")
                );
            }
            fn record_debug(
                &mut self,
                field: &tracing::field::Field,
                value: &dyn std::fmt::Debug,
            ) {
                if self.result.is_err() {
                    return;
                }
                let s = format!("{:?}", value);
                self.result = write!(
                    &mut self.writer,
                    ",\"{}\":\"{}\"",
                    field.name(),
                    s.replace('\\', "\\\\").replace('"', "\\\"")
                );
            }
        }

        let mut visitor = FieldVisitor {
            writer: &mut writer,
            result: Ok(()),
        };
        event.record(&mut visitor);
        visitor.result?;

        write!(&mut writer, "}}")?;
        writeln!(&mut writer)
    }
}

/// Builds a gRPC OTLP span exporter when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
///
/// The endpoint and TLS/insecure settings are read automatically from the
/// standard OTEL environment variables injected by the NAIS platform. Returns
/// `None` when the variable is absent so that tracing remains a no-op in local
/// development without any additional configuration.
fn nais_otlp_exporter() -> Result<Option<SpanExporter>> {
    if std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT").is_err() {
        return Ok(None);
    }
    let exporter = SpanExporter::builder()
        .with_tonic()
        .with_timeout(Duration::from_secs(5))
        .build()?;
    Ok(Some(exporter))
}

fn resolve_service_name() -> String {
    std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .filter(|s| !s.is_empty())
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
    let exporter = nais_otlp_exporter()?;
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
    let service_name = resolve_service_name();
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

    let fmt_layer = fmt::layer()
        .event_format(NaisJsonFormat)
        .with_ansi(false);

    let tracer = tracer_provider.tracer("pdfgenrs");
    // Keep a clone for the caller to shut down gracefully before exit.
    // SdkTracerProvider is backed by Arc so this is cheap.
    let provider_for_shutdown = tracer_provider.clone();
    global::set_tracer_provider(tracer_provider);

    tracing_subscriber::registry()
        .with(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
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
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
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
        assert_eq!(logger_name_for_file("src/tracing_setup"), "src.tracing_setup");
    }

    #[test]
    fn resolve_service_name_uses_env_value() {
        let _guard = env_lock().lock().expect("lock poisoned");
        let previous = std::env::var_os("OTEL_SERVICE_NAME");
        std::env::set_var("OTEL_SERVICE_NAME", "custom-service");

        let resolved = resolve_service_name();

        match previous {
            Some(value) => std::env::set_var("OTEL_SERVICE_NAME", value),
            None => std::env::remove_var("OTEL_SERVICE_NAME"),
        }
        assert_eq!(resolved, "custom-service");
    }

    #[test]
    fn resolve_service_name_falls_back_when_missing_or_empty() {
        let _guard = env_lock().lock().expect("lock poisoned");
        let previous = std::env::var_os("OTEL_SERVICE_NAME");
        std::env::remove_var("OTEL_SERVICE_NAME");

        let resolved_missing = resolve_service_name();
        std::env::set_var("OTEL_SERVICE_NAME", "");
        let resolved_empty = resolve_service_name();

        match previous {
            Some(value) => std::env::set_var("OTEL_SERVICE_NAME", value),
            None => std::env::remove_var("OTEL_SERVICE_NAME"),
        }
        assert_eq!(resolved_missing, "pdfgenrs");
        assert_eq!(resolved_empty, "pdfgenrs");
    }

    #[test]
    fn nais_otlp_exporter_is_none_without_endpoint() {
        let _guard = env_lock().lock().expect("lock poisoned");
        let previous = std::env::var_os("OTEL_EXPORTER_OTLP_ENDPOINT");
        std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT");

        let exporter = nais_otlp_exporter().expect("exporter setup should not fail");

        match previous {
            Some(value) => std::env::set_var("OTEL_EXPORTER_OTLP_ENDPOINT", value),
            None => std::env::remove_var("OTEL_EXPORTER_OTLP_ENDPOINT"),
        }
        assert!(exporter.is_none());
    }
}
