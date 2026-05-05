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
    /// Directory containing Typst template files. Defaults to `"templates"` (`TEMPLATES_DIR`).
    pub templates_dir: PathBuf,
    /// Directory containing static resource files. Defaults to `"resources"` (`RESOURCES_DIR`).
    pub resources_dir: PathBuf,
    /// Directory containing test JSON data used in dev mode. Defaults to `"data"` (`DATA_DIR`).
    pub data_dir: PathBuf,
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
            templates_dir: PathBuf::from(
                env::var("TEMPLATES_DIR").unwrap_or_else(|_| "templates".to_string()),
            ),
            resources_dir: PathBuf::from(
                env::var("RESOURCES_DIR").unwrap_or_else(|_| "resources".to_string()),
            ),
            data_dir: PathBuf::from(
                env::var("DATA_DIR").unwrap_or_else(|_| "data".to_string()),
            ),
            dev_mode: env::var("DEV_MODE")
                .map(|v| v.eq_ignore_ascii_case("true"))
                .unwrap_or(false),
        }
    }
}
