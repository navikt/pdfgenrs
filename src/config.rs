use clap::Parser;
use std::path::{Path, PathBuf};
use tracing::warn;

const REQUEST_BODY_LIMIT_BYTES_ENV: &str = "REQUEST_BODY_LIMIT_BYTES";
const COMPILE_TIMEOUT_SECONDS_ENV: &str = "COMPILE_TIMEOUT_SECONDS";
const SHUTDOWN_DRAIN_SECONDS_ENV: &str = "SHUTDOWN_DRAIN_SECONDS";
const SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV: &str = "SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS";

pub const DEFAULT_COMEMO_EVICTION_THRESHOLD: usize = 15;
pub const DEFAULT_MAX_IMAGE_DIMENSION_PIXELS: u32 = 8_192;
pub const DEFAULT_MAX_IMAGE_PIXELS: u64 = 25_000_000;
pub const DEFAULT_REQUEST_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;

/// pdfgenrs: a PDF generation API server. It supports PDF/A-2a and PDF/UA-1 simultaneously.
#[derive(Clone, Debug, Parser)]
pub struct Config {
    /// TCP port the server listens on.
    #[arg(long, env = "SERVER_PORT", default_value_t = 8080)]
    pub port: u16,
    /// Root directory used as the Typst filesystem root. Templates can reference
    /// resources with absolute paths such as `/resources/logo.png`.
    #[arg(long, env = "ROOT_DIR", default_value_os_t = PathBuf::from("."))]
    pub root_dir: PathBuf,
    /// Directory containing Typst template files.
    #[arg(long, env = "TEMPLATES_DIR", default_value_os_t = PathBuf::from("templates"))]
    pub templates_dir: PathBuf,
    /// Directory containing static resource files.
    #[arg(long, env = "RESOURCES_DIR", default_value_os_t = PathBuf::from("resources"))]
    pub resources_dir: PathBuf,
    /// Directory containing test JSON data used in dev mode.
    #[arg(long, env = "DATA_DIR", default_value_os_t = PathBuf::from("data"))]
    pub data_dir: PathBuf,
    /// Directory containing font files used by Typst.
    #[arg(long, env = "FONTS_DIR", default_value_os_t = PathBuf::from("fonts"))]
    pub fonts_dir: PathBuf,
    /// When `true`, the GET PDF endpoint is enabled and test data is pre-loaded.
    #[arg(long, env = "DEV_MODE")]
    pub dev_mode: bool,
    /// Maximum accepted request body size in bytes.
    #[arg(long, env = REQUEST_BODY_LIMIT_BYTES_ENV, default_value_t = DEFAULT_REQUEST_BODY_LIMIT_BYTES)]
    pub request_body_limit_bytes: usize,
    /// Maximum time in seconds allowed for a single compilation task (Typst to PDF/HTML).
    /// Requests exceeding this timeout will be aborted with a `408 Request Timeout`.
    #[arg(long, env = COMPILE_TIMEOUT_SECONDS_ENV, default_value_t = 30)]
    pub compile_timeout_seconds: u64,
    /// Duration in seconds to wait between marking the application as not ready and
    /// marking it as not alive during shutdown. This allows Kubernetes to stop routing
    /// new traffic before existing connections are drained.
    #[arg(long, env = SHUTDOWN_DRAIN_SECONDS_ENV, default_value_t = 5)]
    pub shutdown_drain_seconds: u64,
    /// Maximum number of concurrent compilation tasks allowed. Defaults to `4`; set to `0`
    /// to disable the limit.
    #[arg(long, env = "MAX_CONCURRENT_COMPILATIONS", default_value_t = 4)]
    pub max_concurrent_compilations: usize,
    /// Maximum time in seconds to wait for a compilation semaphore permit.
    /// When the timeout is exceeded, the server responds with `503 Service Unavailable`.
    #[arg(long, env = SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV, default_value_t = 10)]
    pub semaphore_acquire_timeout_seconds: u64,
    /// Maximum number of cache entries to evict from the comemo memoization cache after
    /// each compilation. Higher values free more memory at the cost of cache hit rate.
    /// Set to `0` to evict the entire cache.
    #[arg(long, env = "COMEMO_EVICTION_THRESHOLD", default_value_t = DEFAULT_COMEMO_EVICTION_THRESHOLD)]
    pub comemo_eviction_threshold: usize,
    /// Maximum width or height for an uploaded image.
    #[arg(long, env = "MAX_IMAGE_DIMENSION_PIXELS", default_value_t = DEFAULT_MAX_IMAGE_DIMENSION_PIXELS)]
    pub max_image_dimension_pixels: u32,
    /// Maximum total number of pixels for an uploaded image.
    #[arg(long, env = "MAX_IMAGE_PIXELS", default_value_t = DEFAULT_MAX_IMAGE_PIXELS)]
    pub max_image_pixels: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::parse()
    }
}

impl Config {
    /// Logs warnings for configuration values that are technically valid but likely
    /// degenerate (e.g. zero timeouts that would cause immediate failures).
    /// Call this at startup so operators are alerted to potential misconfigurations.
    pub fn warn_degenerate_values(&self) {
        if self.compile_timeout_seconds == 0 {
            warn!(
                env = COMPILE_TIMEOUT_SECONDS_ENV,
                value = self.compile_timeout_seconds,
                "compile_timeout_seconds is 0, all compilations will time out immediately"
            );
        }
        if self.semaphore_acquire_timeout_seconds == 0 {
            warn!(
                env = SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV,
                value = self.semaphore_acquire_timeout_seconds,
                "semaphore_acquire_timeout_seconds is 0, semaphore acquisition will time out immediately"
            );
        }
        if self.shutdown_drain_seconds == 0 {
            warn!(
                env = SHUTDOWN_DRAIN_SECONDS_ENV,
                value = self.shutdown_drain_seconds,
                "shutdown_drain_seconds is 0, no graceful drain period before shutdown"
            );
        }
        if self.request_body_limit_bytes == 0 {
            warn!(
                env = REQUEST_BODY_LIMIT_BYTES_ENV,
                value = self.request_body_limit_bytes,
                "request_body_limit_bytes is 0, all requests with a body will be rejected"
            );
        }
        if self.max_image_pixels < u64::from(self.max_image_dimension_pixels) {
            warn!(
                max_image_pixels = self.max_image_pixels,
                max_image_dimension_pixels = self.max_image_dimension_pixels,
                "MAX_IMAGE_PIXELS is lower than MAX_IMAGE_DIMENSION_PIXELS, images within the dimension limit may still be rejected by the pixel limit"
            );
        }
    }

    /// Returns the absolute resource directory used to resolve `/resources/...` Typst paths.
    /// Relative paths in `resources_dir` are resolved from `root_dir`.
    #[must_use]
    pub fn resource_root(&self) -> PathBuf {
        resolve_from_root(&self.root_dir, &self.resources_dir)
    }

    /// Returns the absolute font directory.
    /// Relative paths in `fonts_dir` are resolved from `root_dir`.
    #[must_use]
    pub fn font_dir(&self) -> PathBuf {
        resolve_from_root(&self.root_dir, &self.fonts_dir)
    }
}

#[must_use]
fn resolve_from_root(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_dir_joins_relative_fonts_dir_to_root_dir() {
        let mut config = Config::default();
        config.root_dir = PathBuf::from("/tmp/root");
        assert_eq!(config.font_dir(), PathBuf::from("/tmp/root/fonts"));
    }

    #[test]
    fn font_dir_keeps_absolute_fonts_dir() {
        let mut config = Config::default();
        config.root_dir = PathBuf::from("/tmp/root");
        config.fonts_dir = PathBuf::from("/tmp/shared/fonts");
        assert_eq!(config.font_dir(), PathBuf::from("/tmp/shared/fonts"));
    }

    #[test]
    fn resource_root_joins_relative_resources_dir_to_root_dir() {
        let mut config = Config::default();
        config.root_dir = PathBuf::from("/tmp/root");
        assert_eq!(config.resource_root(), PathBuf::from("/tmp/root/resources"));
    }

    #[test]
    fn resource_root_keeps_absolute_resources_dir() {
        let mut config = Config::default();
        config.root_dir = PathBuf::from("/tmp/root");
        config.resources_dir = PathBuf::from("/tmp/shared/resources");
        assert_eq!(
            config.resource_root(),
            PathBuf::from("/tmp/shared/resources")
        );
    }

    #[test]
    fn warn_degenerate_values_does_not_panic_with_zero_values() {
        let mut config = Config::default();
        config.compile_timeout_seconds = 0;
        config.semaphore_acquire_timeout_seconds = 0;
        config.shutdown_drain_seconds = 0;
        config.request_body_limit_bytes = 0;
        config.warn_degenerate_values();
    }

    #[test]
    fn warn_degenerate_values_does_not_panic_with_defaults() {
        let config = Config::default();
        config.warn_degenerate_values();
    }

    #[test]
    fn warn_degenerate_values_handles_conflicting_image_limits() {
        let mut config = Config::default();
        config.max_image_dimension_pixels = 100;
        config.max_image_pixels = 99;
        config.warn_degenerate_values();
    }
}
