use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;

/// Loads all `.typ` Typst templates from the templates directory recursively.
/// Returns a map from template name (e.g. `app/template`) to Typst source code.
/// Templates receive request JSON data via the virtual file `/data.json`,
/// accessible in Typst as `#let data = json("/data.json")`.
pub fn load_templates_from_dir(templates_dir: &str) -> anyhow::Result<HashMap<String, String>> {
    let mut templates = HashMap::new();
    let base = Path::new(templates_dir);

    for entry in WalkDir::new(templates_dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "typ" {
                    let relative = path
                        .strip_prefix(base)
                        .context("Failed to strip prefix")?;
                    let name = relative
                        .with_extension("")
                        .to_string_lossy()
                        .replace('\\', "/");
                    let source = std::fs::read_to_string(path)
                        .context("Failed to read template file")?;
                    templates.insert(name, source);
                }
            }
        }
    }
    Ok(templates)
}

/// Loads all test data JSON files from the data directory.
pub fn load_test_data(data_dir: &str) -> HashMap<(String, String), Value> {
    let mut data = HashMap::new();
    let base = Path::new(data_dir);

    for entry in WalkDir::new(data_dir)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    if let (Ok(relative), Ok(content)) =
                        (path.strip_prefix(base), std::fs::read_to_string(path))
                    {
                        if let Ok(value) = serde_json::from_str::<Value>(&content) {
                            let parts: Vec<&str> = relative
                                .components()
                                .map(|c| c.as_os_str().to_str().unwrap_or(""))
                                .collect();
                            if parts.len() == 2 {
                                let app = parts[0].to_string();
                                let template = Path::new(parts[1])
                                    .with_extension("")
                                    .to_string_lossy()
                                    .to_string();
                                data.insert((app, template), value);
                            }
                        }
                    }
                }
            }
        }
    }
    data
}
