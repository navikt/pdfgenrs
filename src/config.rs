use std::env;
use std::path::{Path, PathBuf};

const SERVER_PORT_ENV: &str = "SERVER_PORT";
const ROOT_DIR_ENV: &str = "ROOT_DIR";
const TEMPLATES_DIR_ENV: &str = "TEMPLATES_DIR";
const RESOURCES_DIR_ENV: &str = "RESOURCES_DIR";
const DATA_DIR_ENV: &str = "DATA_DIR";
const FONTS_DIR_ENV: &str = "FONTS_DIR";
const DEV_MODE_ENV: &str = "DEV_MODE";
const REQUEST_BODY_LIMIT_BYTES_ENV: &str = "REQUEST_BODY_LIMIT_BYTES";

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_ROOT_DIR: &str = ".";
const DEFAULT_TEMPLATES_DIR: &str = "templates";
const DEFAULT_RESOURCES_DIR: &str = "resources";
const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_FONTS_DIR: &str = "fonts";
const DEFAULT_REQUEST_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;

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
        let parse_u16 = |key: &str| env_var(key)?.parse::<u16>().ok();
        let parse_usize = |key: &str| env_var(key)?.parse::<usize>().ok();
        let path_or = |key: &str, default: &str| {
            PathBuf::from(env_var(key).unwrap_or_else(|| default.to_owned()))
        };
        let bool_var = |key: &str| {
            env_var(key)
                .map(|value| value.eq_ignore_ascii_case("true"))
                .unwrap_or(false)
        };

        Self {
            port: parse_u16(SERVER_PORT_ENV).unwrap_or(DEFAULT_PORT),
            root_dir: path_or(ROOT_DIR_ENV, DEFAULT_ROOT_DIR),
            templates_dir: path_or(TEMPLATES_DIR_ENV, DEFAULT_TEMPLATES_DIR),
            resources_dir: path_or(RESOURCES_DIR_ENV, DEFAULT_RESOURCES_DIR),
            data_dir: path_or(DATA_DIR_ENV, DEFAULT_DATA_DIR),
            fonts_dir: path_or(FONTS_DIR_ENV, DEFAULT_FONTS_DIR),
            dev_mode: bool_var(DEV_MODE_ENV),
            request_body_limit_bytes: parse_usize(REQUEST_BODY_LIMIT_BYTES_ENV)
                .unwrap_or(DEFAULT_REQUEST_BODY_LIMIT_BYTES),
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
        ]));

        assert_eq!(config.port, 9090);
        assert_eq!(config.root_dir, PathBuf::from("/tmp/root"));
        assert_eq!(config.templates_dir, PathBuf::from("/tmp/templates"));
        assert_eq!(config.resources_dir, PathBuf::from("/tmp/resources"));
        assert_eq!(config.data_dir, PathBuf::from("/tmp/data"));
        assert_eq!(config.fonts_dir, PathBuf::from("/tmp/fonts"));
        assert!(config.dev_mode);
        assert_eq!(config.request_body_limit_bytes, 4 * 1024 * 1024);
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
        };

        assert_eq!(
            config.resource_root(),
            PathBuf::from("/tmp/shared/resources")
        );
    }
}
