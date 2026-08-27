use std::env;
use std::path::{Path, PathBuf};

use tracing::warn;

const SERVER_PORT_ENV: &str = "SERVER_PORT";
const ROOT_DIR_ENV: &str = "ROOT_DIR";
const TEMPLATES_DIR_ENV: &str = "TEMPLATES_DIR";
const RESOURCES_DIR_ENV: &str = "RESOURCES_DIR";
const DATA_DIR_ENV: &str = "DATA_DIR";
const FONTS_DIR_ENV: &str = "FONTS_DIR";
const DEV_MODE_ENV: &str = "DEV_MODE";
const REQUEST_BODY_LIMIT_BYTES_ENV: &str = "REQUEST_BODY_LIMIT_BYTES";
const COMPILE_TIMEOUT_SECONDS_ENV: &str = "COMPILE_TIMEOUT_SECONDS";
const SHUTDOWN_DRAIN_SECONDS_ENV: &str = "SHUTDOWN_DRAIN_SECONDS";
const MAX_CONCURRENT_COMPILATIONS_ENV: &str = "MAX_CONCURRENT_COMPILATIONS";
const SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV: &str = "SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS";
const COMEMO_EVICTION_THRESHOLD_ENV: &str = "COMEMO_EVICTION_THRESHOLD";
const MAX_IMAGE_DIMENSION_PIXELS_ENV: &str = "MAX_IMAGE_DIMENSION_PIXELS";
const MAX_IMAGE_PIXELS_ENV: &str = "MAX_IMAGE_PIXELS";

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_ROOT_DIR: &str = ".";
const DEFAULT_TEMPLATES_DIR: &str = "templates";
const DEFAULT_RESOURCES_DIR: &str = "resources";
const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_FONTS_DIR: &str = "fonts";
pub(crate) const DEFAULT_REQUEST_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;
const DEFAULT_COMPILE_TIMEOUT_SECONDS: u64 = 30;
const DEFAULT_SHUTDOWN_DRAIN_SECONDS: u64 = 5;
const DEFAULT_MAX_CONCURRENT_COMPILATIONS: usize = 4;
const DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS: u64 = 10;
pub const DEFAULT_COMEMO_EVICTION_THRESHOLD: usize = 15;
/// Maximum allowed width or height of an uploaded image, in pixels.
pub const DEFAULT_MAX_IMAGE_DIMENSION_PIXELS: u32 = 8_192;
/// Maximum allowed total pixel count (width × height) of an uploaded image.
///
/// This is the primary memory guard for the image endpoint: PNG and WebP are
/// decoded to RGBA (4 bytes per pixel) before being embedded, so peak memory
/// scales with the pixel count rather than with the compressed file size.
pub const DEFAULT_MAX_IMAGE_PIXELS: u64 = 25_000_000;

/// Runtime configuration for the pdfgenrs server.
///
/// All fields are populated from environment variables when `Config::default()` is
/// called, falling back to sensible defaults when the variables are absent.
#[derive(Clone, Debug)]
pub struct Config {
    /// TCP port the server listens on. Defaults to `8080` (`SERVER_PORT`).
    pub port: u16,
    /// Root directory used as the Typst filesystem root. Templates can reference
    /// resources with absolute paths such as `/resources/logo.png`. Defaults to
    /// `"."` (`ROOT_DIR`).
    pub root_dir: PathBuf,
    /// Directory containing Typst template files. Defaults to `"templates"` (`TEMPLATES_DIR`).
    pub templates_dir: PathBuf,
    /// Directory containing static resource files. Defaults to `"resources"` (`RESOURCES_DIR`).
    pub resources_dir: PathBuf,
    /// Directory containing test JSON data used in dev mode. Defaults to `"data"` (`DATA_DIR`).
    pub data_dir: PathBuf,
    /// Directory containing font files used by Typst. Defaults to `"fonts"` (`FONTS_DIR`).
    pub fonts_dir: PathBuf,
    /// When `true`, the GET PDF endpoint is enabled and test data is pre-loaded.
    /// Defaults to `false` (`DEV_MODE`).
    pub dev_mode: bool,
    /// Maximum accepted request body size in bytes. Defaults to `2097152` (2 MiB)
    /// (`REQUEST_BODY_LIMIT_BYTES`).
    pub request_body_limit_bytes: usize,
    /// Maximum time in seconds allowed for a single compilation task (Typst to PDF/HTML).
    /// Requests exceeding this timeout will be aborted with a `408 Request Timeout`.
    /// Defaults to `30` (`COMPILE_TIMEOUT_SECONDS`).
    pub compile_timeout_seconds: u64,
    /// Duration in seconds to wait between marking the application as not ready and
    /// marking it as not alive during shutdown. This allows Kubernetes to stop routing
    /// new traffic before existing connections are drained. Defaults to `5`
    /// (`SHUTDOWN_DRAIN_SECONDS`).
    pub shutdown_drain_seconds: u64,
    /// Maximum number of concurrent compilation tasks allowed. Defaults to `4`; set to `0`
    /// to disable the limit. Configurable via `MAX_CONCURRENT_COMPILATIONS`.
    pub max_concurrent_compilations: usize,
    /// Maximum time in seconds to wait for a compilation semaphore permit.
    /// When the timeout is exceeded, the server responds with `503 Service Unavailable`.
    /// Defaults to `10` (`SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS`).
    pub semaphore_acquire_timeout_seconds: u64,
    /// Maximum number of cache entries to evict from the comemo memoization cache after
    /// each compilation. Higher values free more memory at the cost of cache hit rate.
    /// Set to `0` to evict the entire cache. Defaults to `15` (`COMEMO_EVICTION_THRESHOLD`).
    pub comemo_eviction_threshold: usize,
    /// Maximum accepted width or height of an uploaded image, in pixels. Defaults to
    /// `8192` (`MAX_IMAGE_DIMENSION_PIXELS`).
    pub max_image_dimension_pixels: u32,
    /// Maximum accepted total pixel count (width × height) of an uploaded image.
    /// This is the primary memory guard for the image endpoint, since PNG and WebP
    /// are decoded to RGBA before embedding. Defaults to `25000000` (`MAX_IMAGE_PIXELS`).
    pub max_image_pixels: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self::from_env_fn(|key| env::var(key).ok())
    }
}

impl Config {
    /// Build a `Config` by reading environment variables through the provided
    /// lookup function. This avoids direct `env::set_var` / `env::remove_var`
    /// calls in tests — callers can supply a closure backed by a `HashMap`
    /// instead of mutating the process environment.
    fn from_env_fn(env_var: impl Fn(&str) -> Option<String>) -> Self {
        let parse_u16 = |key: &str| {
            let raw = env_var(key)?;
            match raw.parse::<u16>() {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!(env = key, value = %raw, error = %e, "Invalid env value, falling back to default");
                    None
                }
            }
        };
        let parse_usize = |key: &str| {
            let raw = env_var(key)?;
            match raw.parse::<usize>() {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!(env = key, value = %raw, error = %e, "Invalid env value, falling back to default");
                    None
                }
            }
        };
        let parse_u64 = |key: &str| {
            let raw = env_var(key)?;
            match raw.parse::<u64>() {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!(env = key, value = %raw, error = %e, "Invalid env value, falling back to default");
                    None
                }
            }
        };
        let parse_u32 = |key: &str| {
            let raw = env_var(key)?;
            match raw.parse::<u32>() {
                Ok(v) => Some(v),
                Err(e) => {
                    warn!(env = key, value = %raw, error = %e, "Invalid env value, falling back to default");
                    None
                }
            }
        };
        let path_or = |key: &str, default: &str| {
            PathBuf::from(env_var(key).unwrap_or_else(|| default.to_owned()))
        };
        let bool_var = |key: &str| {
            env_var(key)
                .map(|value| value.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };

        let request_body_limit_bytes =
            parse_usize(REQUEST_BODY_LIMIT_BYTES_ENV).unwrap_or(DEFAULT_REQUEST_BODY_LIMIT_BYTES);

        Self {
            port: parse_u16(SERVER_PORT_ENV).unwrap_or(DEFAULT_PORT),
            root_dir: path_or(ROOT_DIR_ENV, DEFAULT_ROOT_DIR),
            templates_dir: path_or(TEMPLATES_DIR_ENV, DEFAULT_TEMPLATES_DIR),
            resources_dir: path_or(RESOURCES_DIR_ENV, DEFAULT_RESOURCES_DIR),
            data_dir: path_or(DATA_DIR_ENV, DEFAULT_DATA_DIR),
            fonts_dir: path_or(FONTS_DIR_ENV, DEFAULT_FONTS_DIR),
            dev_mode: bool_var(DEV_MODE_ENV),
            request_body_limit_bytes,
            compile_timeout_seconds: parse_u64(COMPILE_TIMEOUT_SECONDS_ENV)
                .unwrap_or(DEFAULT_COMPILE_TIMEOUT_SECONDS),
            shutdown_drain_seconds: parse_u64(SHUTDOWN_DRAIN_SECONDS_ENV)
                .unwrap_or(DEFAULT_SHUTDOWN_DRAIN_SECONDS),
            max_concurrent_compilations: parse_usize(MAX_CONCURRENT_COMPILATIONS_ENV)
                .unwrap_or(DEFAULT_MAX_CONCURRENT_COMPILATIONS),
            semaphore_acquire_timeout_seconds: parse_u64(SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV)
                .unwrap_or(DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS),
            comemo_eviction_threshold: parse_usize(COMEMO_EVICTION_THRESHOLD_ENV)
                .unwrap_or(DEFAULT_COMEMO_EVICTION_THRESHOLD),
            max_image_dimension_pixels: parse_u32(MAX_IMAGE_DIMENSION_PIXELS_ENV)
                .unwrap_or(DEFAULT_MAX_IMAGE_DIMENSION_PIXELS),
            max_image_pixels: parse_u64(MAX_IMAGE_PIXELS_ENV).unwrap_or(DEFAULT_MAX_IMAGE_PIXELS),
        }
    }

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
        if self.max_image_dimension_pixels == 0 {
            warn!(
                env = MAX_IMAGE_DIMENSION_PIXELS_ENV,
                value = self.max_image_dimension_pixels,
                "max_image_dimension_pixels is 0, all images will be rejected"
            );
        }
        if self.max_image_pixels == 0 {
            warn!(
                env = MAX_IMAGE_PIXELS_ENV,
                value = self.max_image_pixels,
                "max_image_pixels is 0, all images will be rejected"
            );
        }
        // PNG and WebP are decoded to RGBA (4 bytes per pixel) before embedding, so this
        // is the dominant term in peak memory per in-flight compilation.
        let estimated_peak_bytes = self.max_image_pixels.saturating_mul(4);
        if estimated_peak_bytes > 400_000_000 {
            warn!(
                env = MAX_IMAGE_PIXELS_ENV,
                value = self.max_image_pixels,
                estimated_rgba_bytes = estimated_peak_bytes,
                max_concurrent_compilations = self.max_concurrent_compilations,
                "max_image_pixels allows large RGBA decode buffers; verify pod memory limits and MAX_CONCURRENT_COMPILATIONS"
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
    use std::collections::HashMap;

    fn env_from(entries: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let map: HashMap<String, String> = entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        move |key: &str| map.get(key).cloned()
    }

    #[test]
    fn default_uses_fallback_values_when_env_is_missing() {
        let config = Config::from_env_fn(|_| None);

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.root_dir, PathBuf::from(DEFAULT_ROOT_DIR));
        assert_eq!(config.templates_dir, PathBuf::from(DEFAULT_TEMPLATES_DIR));
        assert_eq!(config.resources_dir, PathBuf::from(DEFAULT_RESOURCES_DIR));
        assert_eq!(config.data_dir, PathBuf::from(DEFAULT_DATA_DIR));
        assert_eq!(config.fonts_dir, PathBuf::from(DEFAULT_FONTS_DIR));
        assert!(!config.dev_mode);
        assert_eq!(
            config.request_body_limit_bytes,
            DEFAULT_REQUEST_BODY_LIMIT_BYTES
        );
        assert_eq!(
            config.compile_timeout_seconds,
            DEFAULT_COMPILE_TIMEOUT_SECONDS
        );
        assert_eq!(
            config.shutdown_drain_seconds,
            DEFAULT_SHUTDOWN_DRAIN_SECONDS
        );
        assert_eq!(
            config.max_concurrent_compilations,
            DEFAULT_MAX_CONCURRENT_COMPILATIONS
        );
        assert_ne!(config.max_concurrent_compilations, 0);
        assert_eq!(
            config.semaphore_acquire_timeout_seconds,
            DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS
        );
        assert_eq!(
            config.comemo_eviction_threshold,
            DEFAULT_COMEMO_EVICTION_THRESHOLD
        );
        assert_eq!(
            config.max_image_dimension_pixels,
            DEFAULT_MAX_IMAGE_DIMENSION_PIXELS
        );
        assert_eq!(config.max_image_pixels, DEFAULT_MAX_IMAGE_PIXELS);
    }

    #[test]
    fn default_reads_values_from_env() {
        let config = Config::from_env_fn(env_from(&[
            (SERVER_PORT_ENV, "9090"),
            (ROOT_DIR_ENV, "/tmp/root"),
            (TEMPLATES_DIR_ENV, "/tmp/templates"),
            (RESOURCES_DIR_ENV, "/tmp/resources"),
            (DATA_DIR_ENV, "/tmp/data"),
            (FONTS_DIR_ENV, "/tmp/fonts"),
            (DEV_MODE_ENV, "TrUe"),
            (REQUEST_BODY_LIMIT_BYTES_ENV, "4194304"),
            (COMPILE_TIMEOUT_SECONDS_ENV, "60"),
            (SHUTDOWN_DRAIN_SECONDS_ENV, "10"),
            (MAX_CONCURRENT_COMPILATIONS_ENV, "4"),
            (SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV, "15"),
            (COMEMO_EVICTION_THRESHOLD_ENV, "30"),
            (MAX_IMAGE_DIMENSION_PIXELS_ENV, "16384"),
            (MAX_IMAGE_PIXELS_ENV, "100000000"),
        ]));

        assert_eq!(config.port, 9090);
        assert_eq!(config.root_dir, PathBuf::from("/tmp/root"));
        assert_eq!(config.templates_dir, PathBuf::from("/tmp/templates"));
        assert_eq!(config.resources_dir, PathBuf::from("/tmp/resources"));
        assert_eq!(config.data_dir, PathBuf::from("/tmp/data"));
        assert_eq!(config.fonts_dir, PathBuf::from("/tmp/fonts"));
        assert!(config.dev_mode);
        assert_eq!(config.request_body_limit_bytes, 4 * 1024 * 1024);
        assert_eq!(config.compile_timeout_seconds, 60);
        assert_eq!(config.shutdown_drain_seconds, 10);
        assert_eq!(config.max_concurrent_compilations, 4);
        assert_eq!(config.semaphore_acquire_timeout_seconds, 15);
        assert_eq!(config.comemo_eviction_threshold, 30);
        assert_eq!(config.max_image_dimension_pixels, 16_384);
        assert_eq!(config.max_image_pixels, 100_000_000);
    }

    #[test]
    fn image_limits_fall_back_to_defaults_for_invalid_env_values() {
        let config = Config::from_env_fn(env_from(&[
            (MAX_IMAGE_DIMENSION_PIXELS_ENV, "not-a-number"),
            (MAX_IMAGE_PIXELS_ENV, "-1"),
        ]));

        assert_eq!(
            config.max_image_dimension_pixels,
            DEFAULT_MAX_IMAGE_DIMENSION_PIXELS
        );
        assert_eq!(config.max_image_pixels, DEFAULT_MAX_IMAGE_PIXELS);
    }

    #[test]
    fn default_falls_back_to_default_port_for_invalid_env_value() {
        let config = Config::from_env_fn(env_from(&[(SERVER_PORT_ENV, "not-a-port")]));

        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn default_treats_non_true_dev_mode_values_as_false() {
        let config = Config::from_env_fn(env_from(&[(DEV_MODE_ENV, "FALSE")]));

        assert!(!config.dev_mode);
    }

    #[test]
    fn default_falls_back_to_default_request_body_limit_for_invalid_env_value() {
        let config =
            Config::from_env_fn(env_from(&[(REQUEST_BODY_LIMIT_BYTES_ENV, "not-a-number")]));

        assert_eq!(
            config.request_body_limit_bytes,
            DEFAULT_REQUEST_BODY_LIMIT_BYTES
        );
    }

    #[test]
    fn default_falls_back_to_default_compile_timeout_for_invalid_env_value() {
        let config =
            Config::from_env_fn(env_from(&[(COMPILE_TIMEOUT_SECONDS_ENV, "not-a-number")]));

        assert_eq!(
            config.compile_timeout_seconds,
            DEFAULT_COMPILE_TIMEOUT_SECONDS
        );
    }

    #[test]
    fn default_falls_back_to_default_shutdown_drain_for_invalid_env_value() {
        let config = Config::from_env_fn(env_from(&[(SHUTDOWN_DRAIN_SECONDS_ENV, "not-a-number")]));

        assert_eq!(
            config.shutdown_drain_seconds,
            DEFAULT_SHUTDOWN_DRAIN_SECONDS
        );
    }

    #[test]
    fn default_falls_back_to_default_max_concurrent_compilations_for_invalid_env_value() {
        let config = Config::from_env_fn(env_from(&[(
            MAX_CONCURRENT_COMPILATIONS_ENV,
            "not-a-number",
        )]));

        assert_eq!(
            config.max_concurrent_compilations,
            DEFAULT_MAX_CONCURRENT_COMPILATIONS
        );
    }

    #[test]
    fn default_falls_back_to_default_semaphore_acquire_timeout_for_invalid_env_value() {
        let config = Config::from_env_fn(env_from(&[(
            SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV,
            "not-a-number",
        )]));

        assert_eq!(
            config.semaphore_acquire_timeout_seconds,
            DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS
        );
    }

    #[test]
    fn comemo_eviction_threshold_reads_from_env() {
        let config = Config::from_env_fn(env_from(&[(COMEMO_EVICTION_THRESHOLD_ENV, "50")]));

        assert_eq!(config.comemo_eviction_threshold, 50);
    }

    #[test]
    fn comemo_eviction_threshold_falls_back_to_default_for_invalid_env_value() {
        let config =
            Config::from_env_fn(env_from(&[(COMEMO_EVICTION_THRESHOLD_ENV, "not-a-number")]));

        assert_eq!(
            config.comemo_eviction_threshold,
            DEFAULT_COMEMO_EVICTION_THRESHOLD
        );
    }

    #[test]
    fn zero_comemo_eviction_threshold_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(COMEMO_EVICTION_THRESHOLD_ENV, "0")]));

        assert_eq!(config.comemo_eviction_threshold, 0);
    }

    #[test]
    fn font_dir_joins_relative_fonts_dir_to_root_dir() {
        let config = Config {
            port: DEFAULT_PORT,
            root_dir: PathBuf::from("/tmp/root"),
            templates_dir: PathBuf::from(DEFAULT_TEMPLATES_DIR),
            resources_dir: PathBuf::from(DEFAULT_RESOURCES_DIR),
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            fonts_dir: PathBuf::from(DEFAULT_FONTS_DIR),
            dev_mode: false,
            request_body_limit_bytes: DEFAULT_REQUEST_BODY_LIMIT_BYTES,
            compile_timeout_seconds: DEFAULT_COMPILE_TIMEOUT_SECONDS,
            shutdown_drain_seconds: DEFAULT_SHUTDOWN_DRAIN_SECONDS,
            max_concurrent_compilations: DEFAULT_MAX_CONCURRENT_COMPILATIONS,
            semaphore_acquire_timeout_seconds: DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS,
            comemo_eviction_threshold: DEFAULT_COMEMO_EVICTION_THRESHOLD,
            max_image_dimension_pixels: DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
            max_image_pixels: DEFAULT_MAX_IMAGE_PIXELS,
        };

        assert_eq!(config.font_dir(), PathBuf::from("/tmp/root/fonts"));
    }

    #[test]
    fn font_dir_keeps_absolute_fonts_dir() {
        let config = Config {
            port: DEFAULT_PORT,
            root_dir: PathBuf::from("/tmp/root"),
            templates_dir: PathBuf::from(DEFAULT_TEMPLATES_DIR),
            resources_dir: PathBuf::from(DEFAULT_RESOURCES_DIR),
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            fonts_dir: PathBuf::from("/tmp/shared/fonts"),
            dev_mode: false,
            request_body_limit_bytes: DEFAULT_REQUEST_BODY_LIMIT_BYTES,
            compile_timeout_seconds: DEFAULT_COMPILE_TIMEOUT_SECONDS,
            shutdown_drain_seconds: DEFAULT_SHUTDOWN_DRAIN_SECONDS,
            max_concurrent_compilations: DEFAULT_MAX_CONCURRENT_COMPILATIONS,
            semaphore_acquire_timeout_seconds: DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS,
            comemo_eviction_threshold: DEFAULT_COMEMO_EVICTION_THRESHOLD,
            max_image_dimension_pixels: DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
            max_image_pixels: DEFAULT_MAX_IMAGE_PIXELS,
        };

        assert_eq!(config.font_dir(), PathBuf::from("/tmp/shared/fonts"));
    }

    #[test]
    fn resource_root_joins_relative_resources_dir_to_root_dir() {
        let config = Config {
            port: DEFAULT_PORT,
            root_dir: PathBuf::from("/tmp/root"),
            templates_dir: PathBuf::from(DEFAULT_TEMPLATES_DIR),
            resources_dir: PathBuf::from(DEFAULT_RESOURCES_DIR),
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            fonts_dir: PathBuf::from(DEFAULT_FONTS_DIR),
            dev_mode: false,
            request_body_limit_bytes: DEFAULT_REQUEST_BODY_LIMIT_BYTES,
            compile_timeout_seconds: DEFAULT_COMPILE_TIMEOUT_SECONDS,
            shutdown_drain_seconds: DEFAULT_SHUTDOWN_DRAIN_SECONDS,
            max_concurrent_compilations: DEFAULT_MAX_CONCURRENT_COMPILATIONS,
            semaphore_acquire_timeout_seconds: DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS,
            comemo_eviction_threshold: DEFAULT_COMEMO_EVICTION_THRESHOLD,
            max_image_dimension_pixels: DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
            max_image_pixels: DEFAULT_MAX_IMAGE_PIXELS,
        };

        assert_eq!(config.resource_root(), PathBuf::from("/tmp/root/resources"));
    }

    #[test]
    fn resource_root_keeps_absolute_resources_dir() {
        let config = Config {
            port: DEFAULT_PORT,
            root_dir: PathBuf::from("/tmp/root"),
            templates_dir: PathBuf::from(DEFAULT_TEMPLATES_DIR),
            resources_dir: PathBuf::from("/tmp/shared/resources"),
            data_dir: PathBuf::from(DEFAULT_DATA_DIR),
            fonts_dir: PathBuf::from(DEFAULT_FONTS_DIR),
            dev_mode: false,
            request_body_limit_bytes: DEFAULT_REQUEST_BODY_LIMIT_BYTES,
            compile_timeout_seconds: DEFAULT_COMPILE_TIMEOUT_SECONDS,
            shutdown_drain_seconds: DEFAULT_SHUTDOWN_DRAIN_SECONDS,
            max_concurrent_compilations: DEFAULT_MAX_CONCURRENT_COMPILATIONS,
            semaphore_acquire_timeout_seconds: DEFAULT_SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS,
            comemo_eviction_threshold: DEFAULT_COMEMO_EVICTION_THRESHOLD,
            max_image_dimension_pixels: DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
            max_image_pixels: DEFAULT_MAX_IMAGE_PIXELS,
        };

        assert_eq!(
            config.resource_root(),
            PathBuf::from("/tmp/shared/resources")
        );
    }

    #[test]
    fn zero_compile_timeout_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(COMPILE_TIMEOUT_SECONDS_ENV, "0")]));

        assert_eq!(config.compile_timeout_seconds, 0);
    }

    #[test]
    fn zero_shutdown_drain_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(SHUTDOWN_DRAIN_SECONDS_ENV, "0")]));

        assert_eq!(config.shutdown_drain_seconds, 0);
    }

    #[test]
    fn zero_semaphore_acquire_timeout_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV, "0")]));

        assert_eq!(config.semaphore_acquire_timeout_seconds, 0);
    }

    #[test]
    fn zero_request_body_limit_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(REQUEST_BODY_LIMIT_BYTES_ENV, "0")]));

        assert_eq!(config.request_body_limit_bytes, 0);
    }

    #[test]
    fn zero_port_is_accepted() {
        let config = Config::from_env_fn(env_from(&[(SERVER_PORT_ENV, "0")]));

        assert_eq!(config.port, 0);
    }

    #[test]
    fn negative_values_fall_back_to_defaults() {
        let config = Config::from_env_fn(env_from(&[
            (COMPILE_TIMEOUT_SECONDS_ENV, "-1"),
            (SHUTDOWN_DRAIN_SECONDS_ENV, "-5"),
            (REQUEST_BODY_LIMIT_BYTES_ENV, "-100"),
            (SERVER_PORT_ENV, "-1"),
        ]));

        assert_eq!(
            config.compile_timeout_seconds,
            DEFAULT_COMPILE_TIMEOUT_SECONDS
        );
        assert_eq!(
            config.shutdown_drain_seconds,
            DEFAULT_SHUTDOWN_DRAIN_SECONDS
        );
        assert_eq!(
            config.request_body_limit_bytes,
            DEFAULT_REQUEST_BODY_LIMIT_BYTES
        );
        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn empty_string_env_values_use_defaults_for_numeric_fields() {
        let config = Config::from_env_fn(env_from(&[
            (SERVER_PORT_ENV, ""),
            (COMPILE_TIMEOUT_SECONDS_ENV, ""),
            (REQUEST_BODY_LIMIT_BYTES_ENV, ""),
        ]));

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(
            config.compile_timeout_seconds,
            DEFAULT_COMPILE_TIMEOUT_SECONDS
        );
        assert_eq!(
            config.request_body_limit_bytes,
            DEFAULT_REQUEST_BODY_LIMIT_BYTES
        );
    }

    #[test]
    fn empty_string_root_dir_creates_empty_path() {
        let config = Config::from_env_fn(env_from(&[(ROOT_DIR_ENV, "")]));

        assert_eq!(config.root_dir, PathBuf::from(""));
    }

    #[test]
    fn dev_mode_empty_string_is_false() {
        let config = Config::from_env_fn(env_from(&[(DEV_MODE_ENV, "")]));

        assert!(!config.dev_mode);
    }

    #[test]
    fn dev_mode_yes_is_false() {
        let config = Config::from_env_fn(env_from(&[(DEV_MODE_ENV, "yes")]));

        assert!(!config.dev_mode);
    }

    #[test]
    fn dev_mode_1_is_false() {
        let config = Config::from_env_fn(env_from(&[(DEV_MODE_ENV, "1")]));

        assert!(!config.dev_mode);
    }

    #[test]
    fn warn_degenerate_values_does_not_panic_with_zero_values() {
        let config = Config::from_env_fn(env_from(&[
            (COMPILE_TIMEOUT_SECONDS_ENV, "0"),
            (SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS_ENV, "0"),
            (SHUTDOWN_DRAIN_SECONDS_ENV, "0"),
            (REQUEST_BODY_LIMIT_BYTES_ENV, "0"),
        ]));

        config.warn_degenerate_values();
    }

    #[test]
    fn warn_degenerate_values_does_not_panic_with_defaults() {
        let config = Config::from_env_fn(|_| None);

        config.warn_degenerate_values();
    }
}
