use std::env;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub templates_dir: PathBuf,
    pub resources_dir: PathBuf,
    pub data_dir: PathBuf,
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
