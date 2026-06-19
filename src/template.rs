use anyhow::Context;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

/// Categorises errors encountered while loading test data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
            *summary.entry(diagnostic.kind).or_insert(0) += 1;
        }
        summary
    }
}

/// Recursively loads all `*.typ` template files from `templates_dir`.
///
/// Each template must be at path format `<app_name>/<template_name>.typ` and is
/// keyed by `(app_name, template_name)`.
///
/// # Errors
/// Returns an error if any file cannot be read or a path cannot be processed.
pub fn load_templates_from_dir(
    templates_dir: &Path,
) -> anyhow::Result<HashMap<(String, String), Arc<String>>> {
    let mut templates = HashMap::new();

    for entry in WalkDir::new(templates_dir).follow_links(true).into_iter() {
        let entry = entry.context("Failed to read template directory entry")?;
        let path = entry.path();
        if path.is_file()
            && let Some(ext) = path.extension()
            && ext == "typ"
        {
            let relative = path
                .strip_prefix(templates_dir)
                .context("Failed to strip prefix")?;
            let relative_no_ext = relative.with_extension("");
            let mut parts = relative_no_ext.iter();
            let app_name = parts
                .next()
                .and_then(|part| part.to_str())
                .context("Template path must start with '<app_name>/' and use valid UTF-8")?;
            let template_name = parts.next().and_then(|part| part.to_str()).context(
                "Template path must include '<template_name>.typ' under app_name and use valid UTF-8",
            )?;
            if parts.next().is_some() {
                return Err(anyhow::anyhow!(
                    "Template path must be exactly '<app_name>/<template_name>.typ': {}",
                    relative.display()
                ));
            }
            let source = std::fs::read_to_string(path).context("Failed to read template file")?;
            templates.insert(
                (app_name.to_string(), template_name.to_string()),
                Arc::new(source),
            );
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
        let app = dir.path().join("myapp");
        fs::create_dir_all(&app)?;
        fs::write(app.join("hello.typ"), "Hello Typst")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 1);
        assert_eq!(
            templates[&("myapp".to_string(), "hello".to_string())].as_str(),
            "Hello Typst"
        );
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
        let key = ("myapp".to_string(), "report".to_string());
        assert!(templates.contains_key(&key));
        assert_eq!(templates[&key].as_str(), "Report content");
        Ok(())
    }

    #[test]
    fn test_load_templates_ignores_non_typ_files() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app = dir.path().join("myapp");
        fs::create_dir_all(&app)?;
        fs::write(app.join("template.typ"), "Typst source")?;
        fs::write(app.join("data.json"), "{}")?;
        fs::write(app.join("readme.txt"), "readme")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 1);
        assert!(templates.contains_key(&("myapp".to_string(), "template".to_string())));
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
        let app1 = dir.path().join("app1");
        let app2 = dir.path().join("app2");
        fs::create_dir_all(&app1)?;
        fs::create_dir_all(&app2)?;
        fs::write(app1.join("one.typ"), "one")?;
        fs::write(app2.join("two.typ"), "two")?;

        let templates = load_templates_from_dir(dir.path())?;

        assert_eq!(templates.len(), 2);
        assert!(templates.contains_key(&("app1".to_string(), "one".to_string())));
        assert!(templates.contains_key(&("app2".to_string(), "two".to_string())));
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
        assert!(
            result
                .data
                .contains_key(&("myapp".to_string(), "valid".to_string()))
        );
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
        assert!(
            result
                .data
                .contains_key(&("myapp".to_string(), "template".to_string()))
        );
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

    #[test]
    fn test_load_templates_follows_symlinked_file() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app = dir.path().join("myapp");
        fs::create_dir_all(&app)?;
        fs::write(app.join("real.typ"), "Real template")?;

        let link_app = dir.path().join("linkapp");
        fs::create_dir_all(&link_app)?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(app.join("real.typ"), link_app.join("linked.typ"))?;
        #[cfg(not(unix))]
        {
            fs::write(link_app.join("linked.typ"), "Real template")?;
        }

        let templates = load_templates_from_dir(dir.path())?;

        assert!(templates.contains_key(&("myapp".to_string(), "real".to_string())));
        assert!(templates.contains_key(&("linkapp".to_string(), "linked".to_string())));
        assert_eq!(
            templates[&("linkapp".to_string(), "linked".to_string())].as_str(),
            "Real template"
        );
        Ok(())
    }

    #[test]
    fn test_load_templates_follows_symlinked_directory() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let real_app = dir.path().join("realapp");
        fs::create_dir_all(&real_app)?;
        fs::write(real_app.join("template.typ"), "From real dir")?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(&real_app, dir.path().join("symlinkapp"))?;
        #[cfg(not(unix))]
        {
            let sym_app = dir.path().join("symlinkapp");
            fs::create_dir_all(&sym_app)?;
            fs::write(sym_app.join("template.typ"), "From real dir")?;
        }

        let templates = load_templates_from_dir(dir.path())?;

        assert!(templates.contains_key(&("realapp".to_string(), "template".to_string())));
        assert!(templates.contains_key(&("symlinkapp".to_string(), "template".to_string())));
        assert_eq!(
            templates[&("symlinkapp".to_string(), "template".to_string())].as_str(),
            "From real dir"
        );
        Ok(())
    }

    #[test]
    fn test_load_templates_rejects_too_deep_nesting() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let deep = dir.path().join("app").join("sub").join("deep");
        fs::create_dir_all(&deep)?;
        fs::write(deep.join("template.typ"), "Deep template")?;

        let result = load_templates_from_dir(dir.path());

        let err = match result {
            Ok(_) => anyhow::bail!("expected error for deeply nested template"),
            Err(e) => e,
        };
        let err_msg = err.to_string();
        assert!(
            err_msg.contains("exactly"),
            "Error should mention path structure: {err_msg}"
        );
        Ok(())
    }

    #[test]
    fn test_load_templates_dangling_symlink_returns_error() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app = dir.path().join("myapp");
        fs::create_dir_all(&app)?;

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("/nonexistent/file.typ", app.join("dangling.typ"))?;
            let result = load_templates_from_dir(dir.path());
            assert!(
                result.is_err(),
                "Dangling symlink should cause an error during traversal"
            );
        }

        Ok(())
    }

    #[test]
    fn test_load_test_data_follows_symlinked_json() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        let app = dir.path().join("myapp");
        fs::create_dir_all(&app)?;
        fs::write(app.join("real.json"), r#"{"source": "real"}"#)?;

        let link_app = dir.path().join("linkapp");
        fs::create_dir_all(&link_app)?;

        #[cfg(unix)]
        std::os::unix::fs::symlink(app.join("real.json"), link_app.join("linked.json"))?;
        #[cfg(not(unix))]
        {
            fs::write(link_app.join("linked.json"), r#"{"source": "real"}"#)?;
        }

        let result = load_test_data(dir.path());

        assert!(result.diagnostics.is_empty());
        assert!(
            result
                .data
                .contains_key(&("linkapp".to_string(), "linked".to_string()))
        );
        assert_eq!(
            result.data[&("linkapp".to_string(), "linked".to_string())]["source"],
            "real"
        );
        Ok(())
    }

    #[test]
    fn test_error_summary_groups_multiple_error_kinds() {
        let result = TestDataLoadResult {
            data: HashMap::new(),
            diagnostics: vec![
                LoadDiagnostic {
                    path: PathBuf::from("data/app/invalid.json"),
                    kind: LoadErrorKind::InvalidJson,
                    message: "invalid json".to_string(),
                },
                LoadDiagnostic {
                    path: PathBuf::from("data/app/missing.json"),
                    kind: LoadErrorKind::ReadFile,
                    message: "permission denied".to_string(),
                },
                LoadDiagnostic {
                    path: PathBuf::from("data/app/another-invalid.json"),
                    kind: LoadErrorKind::InvalidJson,
                    message: "invalid json".to_string(),
                },
            ],
        };

        let summary = result.error_summary();

        assert_eq!(summary.get(&LoadErrorKind::InvalidJson), Some(&2));
        assert_eq!(summary.get(&LoadErrorKind::ReadFile), Some(&1));
        assert_eq!(summary.len(), 2);
    }
}
