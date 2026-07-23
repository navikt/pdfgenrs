use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use metrics::histogram;
use serde_json::Value;
use tokio::sync::OwnedSemaphorePermit;
use tracing::error;

use self::error::ApiError;
use crate::state::AppState;
use crate::typst_world::Fonts;
use typst::Library;
use typst::utils::LazyHash;

pub(crate) mod error;
pub(crate) mod html;
pub(crate) mod nais;
pub(crate) mod pdf;

/// Common parameters extracted from state for template compilation.
pub(crate) struct CompileParams {
    pub source: Arc<String>,
    pub data: Value,
    pub fonts: Arc<Fonts>,
    pub pdf_library: Arc<LazyHash<Library>>,
    pub html_library: Arc<LazyHash<Library>>,
    pub root: Arc<PathBuf>,
    pub resources_dir: Arc<PathBuf>,
}

/// Looks up the template source and pre-loaded test data for the given key (used by GET handlers).
/// Returns `ApiError::NotFound` if either the template or its test data is missing.
pub(crate) async fn lookup_template_and_data(
    state: &AppState,
    template_key: &(String, String),
) -> Result<CompileParams, ApiError> {
    let template_source = state.templates.get(template_key).cloned();
    let json_data = {
        let data_map = state.data.read().await;
        data_map.get(template_key).cloned()
    };

    let (source, data) = match (template_source, json_data) {
        (Some(s), Some(d)) => (s, d),
        _ => return Err(ApiError::NotFound),
    };

    Ok(CompileParams {
        source,
        data,
        fonts: Arc::clone(&state.fonts),
        pdf_library: Arc::clone(&state.pdf_library),
        html_library: Arc::clone(&state.html_library),
        root: Arc::clone(&state.root_dir),
        resources_dir: Arc::clone(&state.resources_dir),
    })
}

/// Looks up the template source for the given key and pairs it with the provided JSON data
/// (used by POST handlers). Returns `ApiError::NotFound` if the template is missing.
pub(crate) fn lookup_template_with_data(
    state: &AppState,
    template_key: &(String, String),
    data: Value,
) -> Result<CompileParams, ApiError> {
    let source = state
        .templates
        .get(template_key)
        .cloned()
        .ok_or(ApiError::NotFound)?;

    Ok(CompileParams {
        source,
        data,
        fonts: Arc::clone(&state.fonts),
        pdf_library: Arc::clone(&state.pdf_library),
        html_library: Arc::clone(&state.html_library),
        root: Arc::clone(&state.root_dir),
        resources_dir: Arc::clone(&state.resources_dir),
    })
}

/// Acquires a compilation semaphore permit if a limit is configured.
/// When no semaphore is set (unlimited mode), returns `Ok(None)` immediately.
/// Returns `Err(ApiError::ServiceOverloaded)` if the permit cannot be acquired
/// within the configured timeout.
pub(crate) async fn acquire_compile_permit(
    state: &AppState,
) -> Result<Option<OwnedSemaphorePermit>, ApiError> {
    if let Some(ref semaphore) = state.compile_semaphore {
        let timeout_duration = Duration::from_secs(state.config.semaphore_acquire_timeout_seconds);
        match tokio::time::timeout(timeout_duration, Arc::clone(semaphore).acquire_owned()).await {
            Ok(Ok(permit)) => Ok(Some(permit)),
            // SAFETY: The semaphore lives inside an Arc in AppState for the entire application
            // lifetime and is never explicitly closed, so acquire_owned() cannot fail with a
            // closed error.
            Ok(Err(_)) => unreachable!("semaphore is never closed"),
            Err(_elapsed) => Err(ApiError::ServiceOverloaded {
                retry_after_seconds: state.config.semaphore_acquire_timeout_seconds,
            }),
        }
    } else {
        Ok(None)
    }
}

/// Runs a blocking compilation task with semaphore-gated concurrency and a timeout.
///
/// Acquires the compile permit, spawns the task on a blocking thread, and applies
/// the configured timeout. Returns the task result or an appropriate `ApiError`.
pub(crate) async fn compile_blocking<T, F>(
    state: &AppState,
    app_name: String,
    template_name: Option<String>,
    task: F,
) -> Result<T, ApiError>
where
    T: Send + 'static,
    F: FnOnce() -> anyhow::Result<T> + Send + 'static,
{
    let timeout_duration = Duration::from_secs(state.config.compile_timeout_seconds);
    let permit = acquire_compile_permit(state).await?;

    let start = Instant::now();
    let result = tokio::time::timeout(
        timeout_duration,
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            task()
        }),
    )
    .await;

    let duration = start.elapsed().as_secs_f64();
    let labels = [
        ("app_name", app_name.clone()),
        (
            "template_name",
            template_name
                .clone()
                .unwrap_or_else(|| "unknown".to_owned()),
        ),
    ];
    histogram!("template_compilation_duration_seconds", &labels).record(duration);

    match result {
        Ok(join_result) => {
            let inner = join_result.unwrap_or_else(|e| {
                error!("spawn_blocking task panicked: {e}");
                Err(anyhow::anyhow!("Task join error: {e}"))
            });
            inner.map_err(|source| ApiError::GenerationFailed {
                app_name,
                template_name,
                source,
                dev_mode: state.config.dev_mode,
            })
        }
        Err(_elapsed) => Err(ApiError::RequestTimeout {
            app_name,
            template_name,
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::sync::Arc;
    use std::time::Duration;

    use anyhow::Context;
    use tokio::sync::Semaphore;

    use metrics_exporter_prometheus::PrometheusBuilder;

    use super::compile_blocking;
    use crate::testutil::make_state;

    #[tokio::test]
    async fn compile_blocking_returns_timeout_when_task_exceeds_deadline() -> anyhow::Result<()> {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.config.compile_timeout_seconds = 1;

        let result: Result<(), _> = compile_blocking(
            &state,
            "myapp".to_string(),
            Some("mytemplate".to_string()),
            || {
                std::thread::sleep(Duration::from_secs(5));
                Ok(())
            },
        )
        .await;

        assert!(result.is_err());
        let err = match result {
            Ok(_) => anyhow::bail!("expected compile_blocking to time out"),
            Err(err) => err,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(response.status(), axum::http::StatusCode::REQUEST_TIMEOUT);
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_semaphore_limits_concurrency() -> anyhow::Result<()> {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.config.compile_timeout_seconds = 10;
        state.compile_semaphore = Some(Arc::new(Semaphore::new(1)));

        let state1 = state.clone();
        let state2 = state.clone();

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel::<()>();

        let task1 = tokio::spawn(async move {
            compile_blocking(&state1, "app".to_string(), None, move || {
                started_tx.send(()).ok();
                rx.blocking_recv().ok();
                Ok(42)
            })
            .await
        });

        started_rx
            .await
            .context("failed to receive task1 start signal")?;

        let task2 = tokio::spawn(async move {
            tokio::time::timeout(
                Duration::from_millis(100),
                compile_blocking(&state2, "app".to_string(), None, || Ok(99)),
            )
            .await
        });

        let task2_result = task2.await.context("task2 join error")?;
        assert!(
            task2_result.is_err(),
            "Expected task2 to time out while task1 holds the semaphore"
        );

        tx.send(()).ok();
        let task1_result = task1.await.context("task1 join error")?;
        let value = match task1_result {
            Ok(value) => value,
            Err(error) => anyhow::bail!("task1 failed: {error:?}"),
        };
        assert_eq!(value, 42);

        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_returns_503_when_semaphore_acquire_times_out() -> anyhow::Result<()> {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.config.compile_timeout_seconds = 10;
        state.config.semaphore_acquire_timeout_seconds = 1;
        state.compile_semaphore = Some(Arc::new(Semaphore::new(1)));

        let (tx, rx) = tokio::sync::oneshot::channel::<()>();
        let (started_tx, started_rx) = tokio::sync::oneshot::channel::<()>();

        let state1 = state.clone();
        let task1 = tokio::spawn(async move {
            compile_blocking(&state1, "app".to_string(), None, move || {
                started_tx.send(()).ok();
                rx.blocking_recv().ok();
                Ok(42)
            })
            .await
        });

        started_rx
            .await
            .context("failed to receive task1 start signal")?;

        let result: Result<(), _> =
            compile_blocking(&state, "app".to_string(), None, || Ok(())).await;

        assert!(result.is_err());
        let err = match result {
            Ok(_) => anyhow::bail!("expected compile_blocking to return ServiceOverloaded"),
            Err(err) => err,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(
            response.status(),
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );

        tx.send(()).ok();
        let _ = task1.await.context("task1 join error")?;
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_with_zero_timeout_returns_timeout_immediately() -> anyhow::Result<()>
    {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.config.compile_timeout_seconds = 0;

        let result: Result<(), _> = compile_blocking(
            &state,
            "myapp".to_string(),
            Some("mytemplate".to_string()),
            || {
                std::thread::sleep(Duration::from_millis(50));
                Ok(())
            },
        )
        .await;

        let err = match result {
            Ok(_) => anyhow::bail!("expected timeout"),
            Err(err) => err,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(response.status(), axum::http::StatusCode::REQUEST_TIMEOUT);
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_returns_500_when_task_returns_error() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;

        let result: Result<(), _> = compile_blocking(
            &state,
            "myapp".to_string(),
            Some("mytemplate".to_string()),
            || Err(anyhow::anyhow!("compilation failed")),
        )
        .await;

        let err = match result {
            Ok(_) => anyhow::bail!("expected error"),
            Err(err) => err,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(
            response.status(),
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        );
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_succeeds_without_semaphore() -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;

        let result = compile_blocking(&state, "app".to_string(), None, || Ok(42)).await;

        let value = match result {
            Ok(v) => v,
            Err(e) => anyhow::bail!("unexpected error: {e:?}"),
        };
        assert_eq!(value, 42);
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_succeeds_with_semaphore() -> anyhow::Result<()> {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.compile_semaphore = Some(Arc::new(Semaphore::new(2)));

        let result = compile_blocking(&state, "app".to_string(), None, || Ok("ok")).await;

        let value = match result {
            Ok(v) => v,
            Err(e) => anyhow::bail!("unexpected error: {e:?}"),
        };
        assert_eq!(value, "ok");
        Ok(())
    }

    #[tokio::test]
    async fn compile_blocking_allows_concurrent_tasks_within_semaphore_capacity()
    -> anyhow::Result<()> {
        let mut state = make_state(HashMap::new(), HashMap::new(), false)?;
        state.config.compile_timeout_seconds = 10;
        state.compile_semaphore = Some(Arc::new(Semaphore::new(2)));

        let state1 = state.clone();
        let state2 = state.clone();

        let (tx1, rx1) = tokio::sync::oneshot::channel::<()>();
        let (tx2, rx2) = tokio::sync::oneshot::channel::<()>();
        let (started1_tx, started1_rx) = tokio::sync::oneshot::channel::<()>();
        let (started2_tx, started2_rx) = tokio::sync::oneshot::channel::<()>();

        let task1 = tokio::spawn(async move {
            compile_blocking(&state1, "app".to_string(), None, move || {
                started1_tx.send(()).ok();
                rx1.blocking_recv().ok();
                Ok(1)
            })
            .await
        });

        let task2 = tokio::spawn(async move {
            compile_blocking(&state2, "app".to_string(), None, move || {
                started2_tx.send(()).ok();
                rx2.blocking_recv().ok();
                Ok(2)
            })
            .await
        });

        // Both tasks should start concurrently since semaphore capacity is 2
        tokio::time::timeout(Duration::from_secs(5), started1_rx)
            .await
            .context("task1 did not start in time")?
            .context("task1 start channel closed")?;
        tokio::time::timeout(Duration::from_secs(5), started2_rx)
            .await
            .context("task2 did not start in time")?
            .context("task2 start channel closed")?;

        tx1.send(()).ok();
        tx2.send(()).ok();

        let r1 = task1.await.context("task1 join error")?;
        let r2 = task2.await.context("task2 join error")?;

        let v1 = match r1 {
            Ok(v) => v,
            Err(e) => anyhow::bail!("task1 failed: {e:?}"),
        };
        let v2 = match r2 {
            Ok(v) => v,
            Err(e) => anyhow::bail!("task2 failed: {e:?}"),
        };
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        Ok(())
    }

    #[tokio::test]
    async fn lookup_template_with_data_returns_not_found_when_template_missing()
    -> anyhow::Result<()> {
        let state = make_state(HashMap::new(), HashMap::new(), false)?;
        let key = ("myapp".to_string(), "missing".to_string());
        let result =
            super::lookup_template_with_data(&state, &key, serde_json::json!({"key": "value"}));
        assert!(result.is_err());
        let err = match result {
            Ok(_) => anyhow::bail!("expected NotFound error"),
            Err(e) => e,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn lookup_template_with_data_returns_params_when_template_exists() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "tmpl".to_string()),
            "Hello".to_string(),
        );
        let state = make_state(templates, HashMap::new(), false)?;
        let key = ("myapp".to_string(), "tmpl".to_string());
        let result = super::lookup_template_with_data(&state, &key, serde_json::json!({"x": 1}));
        let params = match result {
            Ok(p) => p,
            Err(_) => anyhow::bail!("expected Ok"),
        };
        assert_eq!(*params.source, "Hello");
        assert_eq!(params.data, serde_json::json!({"x": 1}));
        Ok(())
    }

    #[tokio::test]
    async fn lookup_template_and_data_returns_not_found_when_template_missing() -> anyhow::Result<()>
    {
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "tmpl".to_string()),
            serde_json::json!({}),
        );
        let state = make_state(HashMap::new(), data, true)?;
        let key = ("myapp".to_string(), "tmpl".to_string());
        let result = super::lookup_template_and_data(&state, &key).await;
        let err = match result {
            Ok(_) => anyhow::bail!("expected NotFound error"),
            Err(e) => e,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn lookup_template_and_data_returns_not_found_when_data_missing() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "tmpl".to_string()),
            "Hello".to_string(),
        );
        let state = make_state(templates, HashMap::new(), true)?;
        let key = ("myapp".to_string(), "tmpl".to_string());
        let result = super::lookup_template_and_data(&state, &key).await;
        let err = match result {
            Ok(_) => anyhow::bail!("expected NotFound error"),
            Err(e) => e,
        };
        let response = axum::response::IntoResponse::into_response(err);
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
        Ok(())
    }

    #[tokio::test]
    async fn lookup_template_and_data_returns_params_when_both_exist() -> anyhow::Result<()> {
        let mut templates = HashMap::new();
        templates.insert(
            ("myapp".to_string(), "tmpl".to_string()),
            "Hello".to_string(),
        );
        let mut data = HashMap::new();
        data.insert(
            ("myapp".to_string(), "tmpl".to_string()),
            serde_json::json!({"key": "value"}),
        );
        let state = make_state(templates, data, true)?;
        let key = ("myapp".to_string(), "tmpl".to_string());
        let result = super::lookup_template_and_data(&state, &key).await;
        let params = match result {
            Ok(p) => p,
            Err(_) => anyhow::bail!("expected Ok"),
        };
        assert_eq!(*params.source, "Hello");
        assert_eq!(params.data, serde_json::json!({"key": "value"}));
        Ok(())
    }

    #[test]
    fn compile_blocking_records_compilation_duration_histogram() -> anyhow::Result<()> {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .context("failed to build runtime")?;
            rt.block_on(async {
                let state = make_state(HashMap::new(), HashMap::new(), false)?;

                let _ = compile_blocking(
                    &state,
                    "myapp".to_string(),
                    Some("report".to_string()),
                    || Ok(42),
                )
                .await;

                let output = handle.render();
                assert!(
                    output.contains("template_compilation_duration_seconds"),
                    "expected template_compilation_duration_seconds in output: {output}"
                );
                assert!(
                    output.contains(r#"app_name="myapp""#),
                    "expected app_name=myapp label: {output}"
                );
                assert!(
                    output.contains(r#"template_name="report""#),
                    "expected template_name=report label: {output}"
                );
                Ok::<(), anyhow::Error>(())
            })?;
            Ok::<(), anyhow::Error>(())
        })?;
        Ok(())
    }

    #[test]
    fn compile_blocking_records_unknown_template_name_when_none() -> anyhow::Result<()> {
        let recorder = PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .context("failed to build runtime")?;
            rt.block_on(async {
                let state = make_state(HashMap::new(), HashMap::new(), false)?;

                let _ = compile_blocking(&state, "myapp".to_string(), None, || Ok(42)).await;

                let output = handle.render();
                assert!(
                    output.contains(r#"template_name="unknown""#),
                    "expected template_name=unknown label when None: {output}"
                );
                Ok::<(), anyhow::Error>(())
            })?;
            Ok::<(), anyhow::Error>(())
        })?;
        Ok(())
    }
}
