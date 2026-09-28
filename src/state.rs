use std::collections::{HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use ironpress::HtmlConverter;
use serde_json::Value;
use tokio::sync::{RwLock, Semaphore};

use crate::config;
use crate::typst_world::Fonts;
use typst::Library;
use typst::utils::LazyHash;

/// Shared, cheaply-cloneable map of pre-loaded test JSON data keyed by `(app_name, template_name)`.
pub type DataMap = Arc<RwLock<HashMap<(String, String), Arc<Value>>>>;

#[derive(Debug)]
struct HtmlPdfCacheEntry {
    html: Arc<str>,
    pdf_bytes: Arc<Vec<u8>>,
}

#[derive(Debug, Default)]
struct HtmlPdfCacheInner {
    entries: HashMap<u64, Vec<HtmlPdfCacheEntry>>,
    order: VecDeque<(u64, Arc<str>)>,
}

/// Bounded in-memory cache for HTML-to-PDF conversion results.
#[derive(Debug)]
pub struct HtmlPdfCache {
    converter_key: u64,
    max_entries: usize,
    inner: Mutex<HtmlPdfCacheInner>,
}

impl HtmlPdfCache {
    #[must_use]
    pub fn new(converter_key: u64, max_entries: usize) -> Self {
        Self {
            converter_key,
            max_entries,
            inner: Mutex::new(HtmlPdfCacheInner::default()),
        }
    }

    fn hash_html(&self, html: &str) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.converter_key.hash(&mut hasher);
        html.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns cached PDF bytes for `html` when present.
    #[must_use]
    pub fn get(&self, html: &str) -> Option<Vec<u8>> {
        let key = self.hash_html(html);
        let cache = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let bucket = cache.entries.get(&key)?;
        let entry = bucket.iter().find(|entry| entry.html.as_ref() == html)?;
        Some((*entry.pdf_bytes).clone())
    }

    /// Inserts PDF bytes for `html`, evicting the oldest entry when full.
    pub fn insert(&self, html: &str, pdf_bytes: Vec<u8>) {
        if self.max_entries == 0 {
            return;
        }

        let key = self.hash_html(html);
        let mut cache = self
            .inner
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        if let Some(bucket) = cache.entries.get_mut(&key)
            && let Some(existing) = bucket.iter_mut().find(|entry| entry.html.as_ref() == html)
        {
            existing.pdf_bytes = Arc::new(pdf_bytes);
            return;
        }

        while cache.order.len() >= self.max_entries {
            let Some((evict_key, evict_html)) = cache.order.pop_front() else {
                break;
            };
            if let Some(bucket) = cache.entries.get_mut(&evict_key) {
                bucket.retain(|entry| !Arc::ptr_eq(&entry.html, &evict_html));
                if bucket.is_empty() {
                    cache.entries.remove(&evict_key);
                }
            }
        }

        let html = Arc::<str>::from(html);
        cache
            .entries
            .entry(key)
            .or_default()
            .push(HtmlPdfCacheEntry {
                html: Arc::clone(&html),
                pdf_bytes: Arc::new(pdf_bytes),
            });
        cache.order.push_back((key, html));
    }
}

#[derive(Clone)]
pub struct AppState {
    /// Pre-loaded Typst templates keyed by `(app_name, template_name)`.
    pub templates: Arc<HashMap<(String, String), Arc<str>>>,
    /// Test JSON data keyed by `(app_name, template_name)`, used in dev mode.
    pub data: DataMap,
    /// Liveness / readiness flags exposed via the NAIS health endpoints.
    pub aliveness: AppAliveness,
    /// Server configuration derived from environment variables.
    pub config: config::Config,
    /// Shared font data used by the Typst compiler.
    pub fonts: Arc<Fonts>,
    /// Pre-built Typst library for PDF compilation (default features).
    pub pdf_library: Arc<LazyHash<Library>>,
    /// Pre-built Typst library for HTML compilation (HTML feature enabled).
    pub html_library: Arc<LazyHash<Library>>,
    /// Pre-built HTML-to-PDF converter with font aliases loaded at startup.
    pub html_converter: Arc<HtmlConverter>,
    /// Optional bounded cache for HTML-to-PDF conversion output.
    pub html_pdf_cache: Option<Arc<HtmlPdfCache>>,
    /// Semaphore to limit the number of concurrent compilation tasks.
    /// When `None`, no limit is enforced.
    pub compile_semaphore: Option<Arc<Semaphore>>,
    /// Optional dedicated semaphore to limit concurrent HTML-to-PDF conversions.
    pub html_pdf_semaphore: Option<Arc<Semaphore>>,
    /// Pre-computed root directory path, shared via Arc to avoid per-request cloning.
    pub root_dir: Arc<PathBuf>,
    /// Pre-computed resource root path, shared via Arc to avoid per-request cloning.
    pub resources_dir: Arc<PathBuf>,
}

/// Tracks the liveness and readiness state of the application.
///
/// Both flags are stored as atomic booleans and can be shared across threads
/// via [`Clone`]. Cloning this struct creates a new handle to the same shared state.
#[derive(Clone, Debug, Default)]
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

impl std::fmt::Debug for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppState")
            .field("templates", &self.templates)
            .field("data", &self.data)
            .field("aliveness", &self.aliveness)
            .field("config", &self.config)
            .field("fonts", &self.fonts)
            .field("pdf_library", &"LazyHash<Library>")
            .field("html_library", &"LazyHash<Library>")
            .field("html_converter", &"HtmlConverter")
            .field("html_pdf_cache", &self.html_pdf_cache)
            .field("compile_semaphore", &self.compile_semaphore)
            .field("html_pdf_semaphore", &self.html_pdf_semaphore)
            .field("root_dir", &self.root_dir)
            .field("resources_dir", &self.resources_dir)
            .finish()
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

    #[test]
    fn html_pdf_cache_round_trips_inserted_value() {
        let cache = HtmlPdfCache::new(123, 2);
        cache.insert("<p>hello</p>", vec![1, 2, 3]);

        assert_eq!(cache.get("<p>hello</p>"), Some(vec![1, 2, 3]));
    }

    #[test]
    fn html_pdf_cache_evicts_oldest_entry_when_full() {
        let cache = HtmlPdfCache::new(123, 1);
        cache.insert("first", vec![1]);
        cache.insert("second", vec![2]);

        assert_eq!(cache.get("first"), None);
        assert_eq!(cache.get("second"), Some(vec![2]));
    }

    #[test]
    fn html_pdf_cache_does_nothing_when_disabled() {
        let cache = HtmlPdfCache::new(123, 0);
        cache.insert("first", vec![1]);

        assert_eq!(cache.get("first"), None);
    }
}
