use std::sync::Arc;
use std::time::Duration;

use tokio::sync::OwnedSemaphorePermit;
use tracing::error;

use self::error::ApiError;
use crate::state::AppState;

pub mod error;
pub mod html;
pub mod nais;
pub mod pdf;

/// Acquires a compilation semaphore permit if a limit is configured.
/// When no semaphore is set (unlimited mode), returns `None` immediately.
pub(crate) async fn acquire_compile_permit(state: &AppState) -> Option<OwnedSemaphorePermit> {
    if let Some(ref semaphore) = state.compile_semaphore {
        Some(
            Arc::clone(semaphore)
                .acquire_owned()
                .await
                .unwrap_or_else(|_| unreachable!("semaphore is never closed")),
        )
    } else {
        None
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
    let permit = acquire_compile_permit(state).await;

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
