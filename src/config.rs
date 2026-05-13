use std::env;
use std::path::{Path, PathBuf};

const SERVER_PORT_ENV: &str = "SERVER_PORT";
const ROOT_DIR_ENV: &str = "ROOT_DIR";
const TEMPLATES_DIR_ENV: &str = "TEMPLATES_DIR";
const RESOURCES_DIR_ENV: &str = "RESOURCES_DIR";
const DATA_DIR_ENV: &str = "DATA_DIR";
const FONTS_DIR_ENV: &str = "FONTS_DIR";
const DEV_MODE_ENV: &str = "DEV_MODE";

const DEFAULT_PORT: u16 = 8080;
const DEFAULT_ROOT_DIR: &str = ".";
const DEFAULT_TEMPLATES_DIR: &str = "templates";
const DEFAULT_RESOURCES_DIR: &str = "resources";
const DEFAULT_DATA_DIR: &str = "data";
const DEFAULT_FONTS_DIR: &str = "fonts";

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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: env_u16(SERVER_PORT_ENV).unwrap_or(DEFAULT_PORT),
            root_dir: env_path(ROOT_DIR_ENV, DEFAULT_ROOT_DIR),
            templates_dir: env_path(TEMPLATES_DIR_ENV, DEFAULT_TEMPLATES_DIR),
            resources_dir: env_path(RESOURCES_DIR_ENV, DEFAULT_RESOURCES_DIR),
            data_dir: env_path(DATA_DIR_ENV, DEFAULT_DATA_DIR),
            fonts_dir: env_path(FONTS_DIR_ENV, DEFAULT_FONTS_DIR),
            dev_mode: env_bool(DEV_MODE_ENV),
        }
    }
}

impl Config {
    /// Returns the absolute resource directory used to resolve `/resources/...` Typst paths.
    pub fn resource_root(&self) -> PathBuf {
        resolve_from_root(&self.root_dir, &self.resources_dir)
    }
}

fn env_path(key: &str, default: &str) -> PathBuf {
    PathBuf::from(env::var(key).unwrap_or_else(|_| default.to_owned()))
}

fn resolve_from_root(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn env_u16(key: &str) -> Option<u16> {
    env::var(key).ok()?.parse().ok()
}

fn env_bool(key: &str) -> bool {
    env::var(key)
        .map(|value| value.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvGuard {
        saved: Vec<(&'static str, Option<String>)>,
    }

    impl EnvGuard {
        fn set(vars: &[(&'static str, Option<&str>)]) -> Self {
            let saved = vars
                .iter()
                .map(|(key, value)| {
                    let previous = env::var(key).ok();
                    match value {
                        Some(value) => env::set_var(key, value),
                        None => env::remove_var(key),
                    }
                    (*key, previous)
                })
                .collect();
            Self { saved }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            for (key, value) in &self.saved {
                match value {
                    Some(value) => env::set_var(key, value),
                    None => env::remove_var(key),
                }
            }
        }
    }

    #[test]
    fn default_uses_fallback_values_when_env_is_missing() {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvGuard::set(&[
            (SERVER_PORT_ENV, None),
            (ROOT_DIR_ENV, None),
            (TEMPLATES_DIR_ENV, None),
            (RESOURCES_DIR_ENV, None),
            (DATA_DIR_ENV, None),
            (FONTS_DIR_ENV, None),
            (DEV_MODE_ENV, None),
        ]);

        let config = Config::default();

        assert_eq!(config.port, DEFAULT_PORT);
        assert_eq!(config.root_dir, PathBuf::from(DEFAULT_ROOT_DIR));
        assert_eq!(config.templates_dir, PathBuf::from(DEFAULT_TEMPLATES_DIR));
        assert_eq!(config.resources_dir, PathBuf::from(DEFAULT_RESOURCES_DIR));
        assert_eq!(config.data_dir, PathBuf::from(DEFAULT_DATA_DIR));
        assert_eq!(config.fonts_dir, PathBuf::from(DEFAULT_FONTS_DIR));
        assert!(!config.dev_mode);
    }

    #[test]
    fn default_reads_values_from_env() {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvGuard::set(&[
            (SERVER_PORT_ENV, Some("9090")),
            (ROOT_DIR_ENV, Some("/tmp/root")),
            (TEMPLATES_DIR_ENV, Some("/tmp/templates")),
            (RESOURCES_DIR_ENV, Some("/tmp/resources")),
            (DATA_DIR_ENV, Some("/tmp/data")),
            (FONTS_DIR_ENV, Some("/tmp/fonts")),
            (DEV_MODE_ENV, Some("TrUe")),
        ]);

        let config = Config::default();

        assert_eq!(config.port, 9090);
        assert_eq!(config.root_dir, PathBuf::from("/tmp/root"));
        assert_eq!(config.templates_dir, PathBuf::from("/tmp/templates"));
        assert_eq!(config.resources_dir, PathBuf::from("/tmp/resources"));
        assert_eq!(config.data_dir, PathBuf::from("/tmp/data"));
        assert_eq!(config.fonts_dir, PathBuf::from("/tmp/fonts"));
        assert!(config.dev_mode);
    }

    #[test]
    fn default_falls_back_to_default_port_for_invalid_env_value() {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvGuard::set(&[(SERVER_PORT_ENV, Some("not-a-port"))]);

        let config = Config::default();

        assert_eq!(config.port, DEFAULT_PORT);
    }

    #[test]
    fn default_treats_non_true_dev_mode_values_as_false() {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvGuard::set(&[(DEV_MODE_ENV, Some("FALSE"))]);

        let config = Config::default();

        assert!(!config.dev_mode);
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
        };

        assert_eq!(
            config.resource_root(),
            PathBuf::from("/tmp/shared/resources")
        );
    }
}
