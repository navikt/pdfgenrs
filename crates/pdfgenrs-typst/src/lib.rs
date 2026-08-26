//! Typst compilation primitives and template loading for `pdfgenrs`.
//!
//! This crate provides:
//! - [`typst_world`]: Typst world, font loading, and compilation utilities.
//! - [`template`]: Template and development data loading helpers.

pub mod template;
pub mod typst_world;

#[cfg(test)]
pub(crate) fn memory_sensitive_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}
