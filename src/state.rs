use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use ironpress::HtmlConverter;
use serde_json::Value;
use tokio::sync::{RwLock, Semaphore};

use crate::config;
use crate::typst_world::Fonts;

#[derive(Clone)]
pub struct AppState {
    /// Pre-loaded Typst templates keyed by `(app_name, template_name)`.
    pub templates: Arc<std::collections::HashMap<(String, String), Arc<String>>>,
    /// Test JSON data keyed by `(app_name, template_name)`, used in dev mode.
    pub data: Arc<RwLock<std::collections::HashMap<(String, String), Value>>>,
    /// Liveness / readiness flags exposed via the NAIS health endpoints.
    pub aliveness: AppAliveness,
    /// Server configuration derived from environment variables.
    pub config: config::Config,
    /// Shared font data used by the Typst compiler.
    pub fonts: Arc<Fonts>,
    /// Pre-built HTML-to-PDF converter with font aliases loaded at startup.
    pub html_converter: Arc<HtmlConverter>,
    /// Semaphore to limit the number of concurrent compilation tasks.
    /// When `None`, no limit is enforced.
    pub compile_semaphore: Option<Arc<Semaphore>>,
}

/// Tracks the liveness and readiness state of the application.
///
/// Both flags are stored as atomic booleans and can be shared across threads
/// via [`Clone`]. Cloning this struct creates a new handle to the same shared state.
#[derive(Clone, Default)]
pub struct AppAliveness {
    /// Whether the application process is alive (i.e. not shutting down).
    alive: Arc<AtomicBool>,
    /// Whether the application is ready to serve traffic.
    ready: Arc<AtomicBool>,
}

impl AppAliveness {
    /// Creates a new `AppAliveness` with both flags set to `false`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the liveness flag to `val`.
    pub fn set_alive(&self, val: bool) {
        self.alive.store(val, Ordering::Relaxed);
    }

    /// Sets the readiness flag to `val`.
    pub fn set_ready(&self, val: bool) {
        self.ready.store(val, Ordering::Relaxed);
    }

    /// Returns `true` if the application is currently alive.
    #[inline]
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::Relaxed)
    }

    /// Returns `true` if the application is currently ready to serve traffic.
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_defaults_to_not_alive_and_not_ready() {
        let a = AppAliveness::new();
        assert!(!a.is_alive());
        assert!(!a.is_ready());
    }

    #[test]
    fn set_alive_and_is_alive_round_trip() {
        let a = AppAliveness::new();
        a.set_alive(true);
        assert!(a.is_alive());
        a.set_alive(false);
        assert!(!a.is_alive());
    }

    #[test]
    fn set_ready_and_is_ready_round_trip() {
        let a = AppAliveness::new();
        a.set_ready(true);
        assert!(a.is_ready());
        a.set_ready(false);
        assert!(!a.is_ready());
    }

    #[test]
    fn alive_and_ready_are_independent() {
        let a = AppAliveness::new();
        a.set_alive(true);
        assert!(a.is_alive());
        assert!(!a.is_ready());

        a.set_ready(true);
        assert!(a.is_alive());
        assert!(a.is_ready());
    }

    #[test]
    fn clone_shares_state() {
        let a = AppAliveness::new();
        let b = a.clone();
        a.set_alive(true);
        assert!(b.is_alive());
        b.set_ready(true);
        assert!(a.is_ready());
    }
}
