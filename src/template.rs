use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadErrorKind {
    WalkDir,
    InvalidPath,
    ReadFile,
    InvalidJson,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadDiagnostic {
    pub path: PathBuf,
    pub kind: LoadErrorKind,
    pub message: String,
}

#[derive(Debug, Default)]
pub struct TestDataLoadResult {
    pub data: HashMap<(String, String), Value>,
    pub diagnostics: Vec<LoadDiagnostic>,
}

impl TestDataLoadResult {
    pub fn error_summary(&self) -> HashMap<LoadErrorKind, usize> {
        let mut summary = HashMap::new();
        for diagnostic in &self.diagnostics {
            *summary.entry(diagnostic.kind.clone()).or_insert(0) += 1;
        }
        summary
    }
}

/// Recursively loads all `*.typ` template files from `templates_dir`.
///
/// Each template is keyed by its relative path with the `.typ` extension
/// stripped and path separators normalised to `/`
/// (e.g. `"myapp/invoice"` for `templates/myapp/invoice.typ`).
///
/// # Errors
/// Returns an error if any file cannot be read or a path cannot be processed.
pub fn load_templates_from_dir(templates_dir: &Path) -> anyhow::Result<HashMap<String, String>> {
    let mut templates = HashMap::new();

    for entry in WalkDir::new(templates_dir).follow_links(true).into_iter() {
        let entry = entry.context("Failed to read template directory entry")?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "typ" {
                    let relative = path
                        .strip_prefix(templates_dir)
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

/// Loads test JSON data files from a two-level directory structure under `data_dir`.
///
/// Files must be at exactly depth 2 (`<app_name>/<template_name>.json`).
/// The returned map is keyed by `(app_name, template_name)` tuples where
/// `template_name` has the `.json` extension removed.
///
/// Files that fail to load are returned as structured diagnostics.
pub fn load_test_data(data_dir: &Path) -> TestDataLoadResult {
    let mut result = TestDataLoadResult::default();

    for entry in WalkDir::new(data_dir).min_depth(2).max_depth(2).into_iter() {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                result.diagnostics.push(LoadDiagnostic {
                    path: error.path().unwrap_or(data_dir).to_path_buf(),
                    kind: LoadErrorKind::WalkDir,
                    message: error.to_string(),
                });
                continue;
            }
        };
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().map_or(true, |ext| ext != "json") {
            continue;
        }

        let relative = match path.strip_prefix(data_dir) {
            Ok(relative) => relative,
            Err(error) => {
                result.diagnostics.push(LoadDiagnostic {
                    path: path.to_path_buf(),
                    kind: LoadErrorKind::InvalidPath,
                    message: error.to_string(),
                });
                continue;
            }
        };

        let app_name = match relative
            .parent()
            .and_then(Path::file_name)
            .and_then(|part| part.to_str())
        {
            Some(app_name) => app_name,
            None => {
                result.diagnostics.push(LoadDiagnostic {
                    path: path.to_path_buf(),
                    kind: LoadErrorKind::InvalidPath,
                    message:
                        "Expected file path format '<app_name>/<template_name>.json'".to_string(),
                });
                continue;
            }
        };

        let template_name = match relative.file_stem().and_then(|part| part.to_str()) {
            Some(template_name) => template_name,
            None => {
                result.diagnostics.push(LoadDiagnostic {
                    path: path.to_path_buf(),
                    kind: LoadErrorKind::InvalidPath,
                    message: "Template file name is missing or not valid UTF-8".to_string(),
                });
                continue;
            }
        };

        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(error) => {
                result.diagnostics.push(LoadDiagnostic {
                    path: path.to_path_buf(),
                    kind: LoadErrorKind::ReadFile,
                    message: error.to_string(),
                });
                continue;
            }
        };

        let value = match serde_json::from_str::<Value>(&content) {
            Ok(value) => value,
            Err(error) => {
                result.diagnostics.push(LoadDiagnostic {
                    path: path.to_path_buf(),
                    kind: LoadErrorKind::InvalidJson,
                    message: error.to_string(),
                });
                continue;
            }
        };

        result.data.insert(
            (app_name.to_string(), template_name.to_string()),
            value,
        );
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_templates_single_file() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("hello.typ"), "Hello Typst").unwrap();

        let templates = load_templates_from_dir(dir.path()).unwrap();

        assert_eq!(templates.len(), 1);
        assert_eq!(templates["hello"], "Hello Typst");
    }

    #[test]
    fn test_load_templates_nested_dir() {
        let dir = TempDir::new().unwrap();
        let sub = dir.path().join("myapp");
        fs::create_dir_all(&sub).unwrap();
        fs::write(sub.join("report.typ"), "Report content").unwrap();

        let templates = load_templates_from_dir(dir.path()).unwrap();

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

        let templates = load_templates_from_dir(dir.path()).unwrap();

        assert_eq!(templates.len(), 1);
        assert!(templates.contains_key("template"));
    }

    #[test]
    fn test_load_templates_empty_dir() {
        let dir = TempDir::new().unwrap();

        let templates = load_templates_from_dir(dir.path()).unwrap();

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

        let templates = load_templates_from_dir(dir.path()).unwrap();

        assert_eq!(templates.len(), 3);
        assert!(templates.contains_key("root"));
        assert!(templates.contains_key("app/one"));
        assert!(templates.contains_key("app/two"));
    }

    #[test]
    fn test_load_test_data_basic() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("mytemplate.json"), r#"{"key": "value"}"#).unwrap();

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result.diagnostics.is_empty());
        let key = ("myapp".to_string(), "mytemplate".to_string());
        assert!(result.data.contains_key(&key));
        assert_eq!(result.data[&key]["key"], "value");
    }

    #[test]
    fn test_load_test_data_reports_invalid_json() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("valid.json"), r#"{"key": "value"}"#).unwrap();
        fs::write(app_dir.join("invalid.json"), "not valid json").unwrap();

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result.data.contains_key(&("myapp".to_string(), "valid".to_string())));
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].kind, LoadErrorKind::InvalidJson);

        let summary = result.error_summary();
        assert_eq!(summary.get(&LoadErrorKind::InvalidJson), Some(&1));
    }

    #[test]
    fn test_load_test_data_ignores_non_json_files() {
        let dir = TempDir::new().unwrap();
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("template.json"), r#"{"a": 1}"#).unwrap();
        fs::write(app_dir.join("template.typ"), "typst").unwrap();

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result.diagnostics.is_empty());
        assert!(result
            .data
            .contains_key(&("myapp".to_string(), "template".to_string())));
    }

    #[test]
    fn test_load_test_data_ignores_wrong_depth() {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join("toplevel.json"), r#"{"a": 1}"#).unwrap();
        let deep = dir.path().join("app").join("sub");
        fs::create_dir_all(&deep).unwrap();
        fs::write(deep.join("deep.json"), r#"{"a": 1}"#).unwrap();

        let result = load_test_data(dir.path());

        assert!(result.data.is_empty());
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_load_test_data_empty_dir() {
        let dir = TempDir::new().unwrap();

        let result = load_test_data(dir.path());

        assert!(result.data.is_empty());
        assert!(result.diagnostics.is_empty());
    }
}
