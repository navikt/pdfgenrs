use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use serde_json::Value;
use tokio::sync::OwnedSemaphorePermit;
use tracing::error;

use self::error::ApiError;
use crate::state::AppState;
use crate::typst_world::Fonts;

pub mod error;
pub mod html;
pub mod nais;
pub mod pdf;

/// Common parameters extracted from state for template compilation.
pub(crate) struct CompileParams {
    pub source: Arc<String>,
    pub data: Value,
    pub fonts: Arc<Fonts>,
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
            Ok(Err(_)) => unreachable!("semaphore is never closed"),
            Err(_elapsed) => Err(ApiError::ServiceOverloaded),
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

    let result = tokio::time::timeout(
        timeout_duration,
        tokio::task::spawn_blocking(move || {
            let _permit = permit;
            task()
        }),
    )
    .await;

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
}
