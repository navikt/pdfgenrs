use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Categorises errors encountered while loading test data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LoadErrorKind {
    /// Failed to traverse the data directory.
    WalkDir,
    /// File path did not match the expected `<app>/<template>.json` structure.
    InvalidPath,
    /// Failed to read a JSON file from disk.
    ReadFile,
    /// File content could not be parsed as valid JSON.
    InvalidJson,
}

/// Describes a single data-loading failure with location and classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadDiagnostic {
    /// Path for the file or directory entry that failed.
    pub path: PathBuf,
    /// High-level category of the loading failure.
    pub kind: LoadErrorKind,
    /// Underlying error details for troubleshooting.
    pub message: String,
}

/// Result of loading development test data from disk.
#[derive(Debug, Default)]
pub struct TestDataLoadResult {
    /// Successfully parsed values keyed by `(app_name, template_name)`.
    pub data: HashMap<(String, String), Value>,
    /// All non-fatal loading issues encountered during traversal and parsing.
    pub diagnostics: Vec<LoadDiagnostic>,
}

impl TestDataLoadResult {
    /// Returns the number of diagnostics grouped by error kind.
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
                    let source =
                        std::fs::read_to_string(path).context("Failed to read template file")?;
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
        if path.extension().is_none_or(|ext| ext != "json") {
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
                    message: "Expected file path format '<app_name>/<template_name>.json'"
                        .to_string(),
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

        result
            .data
            .insert((app_name.to_string(), template_name.to_string()), value);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_load_templates_single_file() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("hello.typ"), "Hello Typst")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 1);
        assert_eq!(templates["hello"], "Hello Typst");
        Ok(())
    }

    #[test]
    fn test_load_templates_nested_dir() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let sub = dir.path().join("myapp");
        fs::create_dir_all(&sub)?;
        fs::write(sub.join("report.typ"), "Report content")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 1);
        assert!(
            templates.contains_key("myapp/report"),
            "key should use forward slash"
        );
        assert_eq!(templates["myapp/report"], "Report content");
        Ok(())
    }

    #[test]
    fn test_load_templates_ignores_non_typ_files() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("template.typ"), "Typst source")?;
        fs::write(dir.path().join("data.json"), "{}")?;
        fs::write(dir.path().join("readme.txt"), "readme")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 1);
        assert!(templates.contains_key("template"));
        Ok(())
    }

    #[test]
    fn test_load_templates_empty_dir() -> anyhow::Result<()> {
        let dir = TempDir::new()?;

        let templates = load_templates_from_dir(dir.path())?;

        assert!(templates.is_empty());
        Ok(())
    }

    #[test]
    fn test_load_templates_multiple_files() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let sub = dir.path().join("app");
        fs::create_dir_all(&sub)?;
        fs::write(dir.path().join("root.typ"), "root")?;
        fs::write(sub.join("one.typ"), "one")?;
        fs::write(sub.join("two.typ"), "two")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 3);
        assert!(templates.contains_key("root"));
        assert!(templates.contains_key("app/one"));
        assert!(templates.contains_key("app/two"));
        Ok(())
    }

    #[test]
    fn test_load_test_data_basic() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir)?;
        fs::write(app_dir.join("mytemplate.json"), r#"{"key": "value"}"#)?;

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result.diagnostics.is_empty());
        let key = ("myapp".to_string(), "mytemplate".to_string());
        assert!(result.data.contains_key(&key));
        assert_eq!(result.data[&key]["key"], "value");
        Ok(())
    }

    #[test]
    fn test_load_test_data_reports_invalid_json() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir)?;
        fs::write(app_dir.join("valid.json"), r#"{"key": "value"}"#)?;
        fs::write(app_dir.join("invalid.json"), "not valid json")?;

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result
            .data
            .contains_key(&("myapp".to_string(), "valid".to_string())));
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].kind, LoadErrorKind::InvalidJson);

        let summary = result.error_summary();
        assert_eq!(summary.get(&LoadErrorKind::InvalidJson), Some(&1));
        Ok(())
    }

    #[test]
    fn test_load_test_data_ignores_non_json_files() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app_dir = dir.path().join("myapp");
        fs::create_dir_all(&app_dir)?;
        fs::write(app_dir.join("template.json"), r#"{"a": 1}"#)?;
        fs::write(app_dir.join("template.typ"), "typst")?;

        let result = load_test_data(dir.path());

        assert_eq!(result.data.len(), 1);
        assert!(result.diagnostics.is_empty());
        assert!(result
            .data
            .contains_key(&("myapp".to_string(), "template".to_string())));
        Ok(())
    }

    #[test]
    fn test_load_test_data_ignores_wrong_depth() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("toplevel.json"), r#"{"a": 1}"#)?;
        let deep = dir.path().join("app").join("sub");
        fs::create_dir_all(&deep)?;
        fs::write(deep.join("deep.json"), r#"{"a": 1}"#)?;

        let result = load_test_data(dir.path());

        assert!(result.data.is_empty());
        assert!(result.diagnostics.is_empty());
        Ok(())
    }

    #[test]
    fn test_load_test_data_empty_dir() -> anyhow::Result<()> {
        let dir = TempDir::new()?;

        let result = load_test_data(dir.path());

        assert!(result.data.is_empty());
        assert!(result.diagnostics.is_empty());
        Ok(())
    }
}
