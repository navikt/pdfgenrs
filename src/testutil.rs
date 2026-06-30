use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use serde_json::Value;
use tokio::sync::RwLock;

use crate::pdf::build_html_converter;
use crate::state::AppState;
use crate::{config, state, typst_world};
use typst::{Feature, Features};

/// Creates an [`AppState`] for use in tests.
///
/// Accepts pre-built template and data maps as well as a `dev_mode` flag.
/// Templates and data may be empty when not needed by the test.
pub fn make_state(
    templates: HashMap<(String, String), String>,
    data: HashMap<(String, String), Value>,
    dev_mode: bool,
) -> anyhow::Result<AppState> {
    let templates = templates
        .into_iter()
        .map(|(k, v)| (k, Arc::new(v)))
        .collect();
    let cfg = config::Config {
        port: 8080,
        root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        templates_dir: PathBuf::from("templates"),
        resources_dir: PathBuf::from("resources"),
        data_dir: PathBuf::from("data"),
        fonts_dir: PathBuf::from("fonts"),
        dev_mode,
        request_body_limit_bytes: 2 * 1024 * 1024,
        compile_timeout_seconds: 30,
        shutdown_drain_seconds: 5,
        max_concurrent_compilations: 0,
        semaphore_acquire_timeout_seconds: 10,
    };
    Ok(AppState {
        templates: Arc::new(templates),
        data: Arc::new(RwLock::new(data)),
        aliveness: state::AppAliveness::new(),
        root_dir: Arc::new(cfg.root_dir.clone()),
        resources_dir: Arc::new(cfg.resource_root()),
        config: cfg,
        fonts: typst_world::cached_fonts(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"))?,
        pdf_library: Arc::new(typst_world::build_library(Features::default())),
        html_library: Arc::new(typst_world::build_library(
            [Feature::Html].into_iter().collect(),
        )),
        compile_semaphore: None,
        html_converter: Arc::new(
            build_html_converter(
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts"),
                &PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            )
            .0,
        ),
    })
}
