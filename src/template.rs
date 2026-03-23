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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // --- load_templates_from_dir ---

    #[test]
    fn test_load_templates_single_file() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("hello.typ"), "Hello Typst").unwrap();

        let templates = load_templates_from_dir(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(templates.len(), 1);
        assert_eq!(templates["hello"], "Hello Typst");
    }

    #[test]
    fn test_load_templates_nested_dir() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("myapp");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("report.typ"), "Report content").unwrap();

        let templates = load_templates_from_dir(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(templates.len(), 1);
        assert!(templates.contains_key("myapp/report"), "key should use forward slash");
        assert_eq!(templates["myapp/report"], "Report content");
    }

    #[test]
    fn test_load_templates_ignores_non_typ_files() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("template.typ"), "Typst source").unwrap();
        fs::write(dir.path().join("data.json"), "{}").unwrap();
        fs::write(dir.path().join("readme.txt"), "readme").unwrap();

        let templates = load_templates_from_dir(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(templates.len(), 1);
        assert!(templates.contains_key("template"));
    }

    #[test]
    fn test_load_templates_empty_dir() {
        let dir = TempDir::new().unwrap();

        let templates = load_templates_from_dir(dir.path().to_str().unwrap()).unwrap();

        assert!(templates.is_empty());
    }

    #[test]
    fn test_load_templates_multiple_files() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("app");
        fs::create_dir_all(&sub).unwrap();
        fs::write(dir.path().join("root.typ"), "root").unwrap();
        fs::write(sub.join("one.typ"), "one").unwrap();
        fs::write(sub.join("two.typ"), "two").unwrap();

        let templates = load_templates_from_dir(dir.path().to_str().unwrap()).unwrap();

        assert_eq!(templates.len(), 3);
        assert!(templates.contains_key("root"));
        assert!(templates.contains_key("app/one"));
        assert!(templates.contains_key("app/two"));
    }

    // --- load_test_data ---

    #[test]
    fn test_load_test_data_basic() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("mytemplate.json"), r#"{"key": "value"}"#).unwrap();

        let data = load_test_data(dir.path().to_str().unwrap());

        assert_eq!(data.len(), 1);
        let key = ("myapp".to_string(), "mytemplate".to_string());
        assert!(data.contains_key(&key));
        assert_eq!(data[&key]["key"], "value");
    }

    #[test]
    fn test_load_test_data_ignores_invalid_json() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("valid.json"), r#"{"key": "value"}"#).unwrap();
        fs::write(app_dir.join("invalid.json"), "not valid json").unwrap();

        let data = load_test_data(dir.path().to_str().unwrap());

        assert_eq!(data.len(), 1);
        assert!(data.contains_key(&("myapp".to_string(), "valid".to_string())));
    }

    #[test]
    fn test_load_test_data_ignores_non_json_files() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("template.json"), r#"{"a": 1}"#).unwrap();
        fs::write(app_dir.join("template.typ"), "typst").unwrap();

        let data = load_test_data(dir.path().to_str().unwrap());

        assert_eq!(data.len(), 1);
        assert!(data.contains_key(&("myapp".to_string(), "template".to_string())));
    }

    #[test]
    fn test_load_test_data_ignores_wrong_depth() {
        let dir = TempDir::new().unwrap();
        // depth 1: directly inside base dir – should be ignored
        fs::write(dir.path().join("toplevel.json"), r#"{"a": 1}"#).unwrap();
        // depth 3: too deep – should be ignored
        let deep = dir.path().join("app").join("sub");
        fs::create_dir_all(&deep).unwrap();
        fs::write(deep.join("deep.json"), r#"{"a": 1}"#).unwrap();

        let data = load_test_data(dir.path().to_str().unwrap());

        assert!(data.is_empty());
    }

    #[test]
    fn test_load_test_data_empty_dir() {
        let dir = TempDir::new().unwrap();

        let data = load_test_data(dir.path().to_str().unwrap());

        assert!(data.is_empty());
    }
}
