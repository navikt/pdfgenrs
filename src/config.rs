use std::env;
use std::path::PathBuf;

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
            port: env::var("SERVER_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            root_dir: PathBuf::from(env::var("ROOT_DIR").unwrap_or_else(|_| ".".to_string())),
            templates_dir: PathBuf::from(
                env::var("TEMPLATES_DIR").unwrap_or_else(|_| "templates".to_string()),
            ),
            resources_dir: PathBuf::from(
                env::var("RESOURCES_DIR").unwrap_or_else(|_| "resources".to_string()),
            ),
            data_dir: PathBuf::from(env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string())),
            fonts_dir: PathBuf::from(env::var("FONTS_DIR").unwrap_or_else(|_| "fonts".to_string())),
            dev_mode: env::var("DEV_MODE")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        }
    }
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
            ("SERVER_PORT", None),
            ("ROOT_DIR", None),
            ("TEMPLATES_DIR", None),
            ("RESOURCES_DIR", None),
            ("DATA_DIR", None),
            ("FONTS_DIR", None),
            ("DEV_MODE", None),
        ]);

        let config = Config::default();

        assert_eq!(config.port, 8080);
        assert_eq!(config.root_dir, PathBuf::from("."));
        assert_eq!(config.templates_dir, PathBuf::from("templates"));
        assert_eq!(config.resources_dir, PathBuf::from("resources"));
        assert_eq!(config.data_dir, PathBuf::from("data"));
        assert_eq!(config.fonts_dir, PathBuf::from("fonts"));
        assert!(!config.dev_mode);
    }

    #[test]
    fn default_reads_values_from_env() {
        let _guard = env_lock()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let _env = EnvGuard::set(&[
            ("SERVER_PORT", Some("9090")),
            ("ROOT_DIR", Some("/tmp/root")),
            ("TEMPLATES_DIR", Some("/tmp/templates")),
            ("RESOURCES_DIR", Some("/tmp/resources")),
            ("DATA_DIR", Some("/tmp/data")),
            ("FONTS_DIR", Some("/tmp/fonts")),
            ("DEV_MODE", Some("TrUe")),
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
        let _env = EnvGuard::set(&[("SERVER_PORT", Some("not-a-port"))]);

        let config = Config::default();

        assert_eq!(config.port, 8080);
    }
}
