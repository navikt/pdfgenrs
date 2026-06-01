use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use notify_debouncer_mini::{DebouncedEventKind, new_debouncer};
use tokio::sync::RwLock;
use tracing::{error, info, warn};

use crate::template;

type TemplateMap = Arc<RwLock<HashMap<(String, String), Arc<String>>>>;
type DataMap = Arc<RwLock<HashMap<(String, String), serde_json::Value>>>;

/// Spawns a background task that watches the templates and data directories for
/// changes and reloads them automatically. Only intended for dev mode.
///
/// Returns the `JoinHandle` for the spawned watcher task so the caller can
/// abort it during shutdown if desired.
pub fn spawn_watcher(
    templates_dir: &Path,
    data_dir: &Path,
    templates: TemplateMap,
    data: DataMap,
) -> tokio::task::JoinHandle<()> {
    let templates_dir = templates_dir.to_path_buf();
    let data_dir = data_dir.to_path_buf();

    tokio::spawn(async move {
        if let Err(e) = watch_loop(&templates_dir, &data_dir, templates, data).await {
            error!(error = %e, "File watcher terminated unexpectedly");
        }
    })
}

async fn watch_loop(
    templates_dir: &Path,
    data_dir: &Path,
    templates: TemplateMap,
    data: DataMap,
) -> anyhow::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(16);

    let mut debouncer = new_debouncer(Duration::from_millis(500), move |result| {
        let tx = tx.clone();
        // Ignore send errors — the receiver may have been dropped during shutdown.
        let _ = tx.blocking_send(result);
    })?;

    debouncer
        .watcher()
        .watch(templates_dir, notify::RecursiveMode::Recursive)?;
    debouncer
        .watcher()
        .watch(data_dir, notify::RecursiveMode::Recursive)?;

    info!(
        templates_dir = %templates_dir.display(),
        data_dir = %data_dir.display(),
        "File watcher started — templates and data will be reloaded on change"
    );

    while let Some(result) = rx.recv().await {
        match result {
            Ok(events) => {
                let has_template_change = events.iter().any(|e| {
                    e.kind == DebouncedEventKind::Any && e.path.starts_with(templates_dir)
                });
                let has_data_change = events
                    .iter()
                    .any(|e| e.kind == DebouncedEventKind::Any && e.path.starts_with(data_dir));

                if has_template_change {
                    reload_templates(templates_dir, &templates).await;
                }
                if has_data_change {
                    reload_data(data_dir, &data).await;
                }
            }
            Err(errors) => {
                warn!(error = %errors, "File watcher error");
            }
        }
    }

    Ok(())
}

async fn reload_templates(templates_dir: &Path, templates: &TemplateMap) {
    info!(path = %templates_dir.display(), "Reloading templates");
    match template::load_templates_from_dir(templates_dir) {
        Ok(new_templates) => {
            let count = new_templates.len();
            let mut lock = templates.write().await;
            *lock = new_templates;
            info!(count, "Templates reloaded successfully");
        }
        Err(e) => {
            error!(error = %e, "Failed to reload templates — keeping previous state");
        }
    }
}

async fn reload_data(data_dir: &Path, data: &DataMap) {
    info!(path = %data_dir.display(), "Reloading test data");
    let result = template::load_test_data(data_dir);
    for diagnostic in &result.diagnostics {
        warn!(
            path = %diagnostic.path.display(),
            kind = ?diagnostic.kind,
            error = %diagnostic.message,
            "Failed to load test data file during reload"
        );
    }
    let count = result.data.len();
    let mut lock = data.write().await;
    *lock = result.data;
    info!(count, "Test data reloaded successfully");
}
