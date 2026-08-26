use anyhow::{Context, Result};
use metrics::counter;
use std::collections::HashMap;
use typst::foundations::Bytes;

use crate::pdf::CompileRequest;
use crate::typst_world;

/// Compiles a Typst template with JSON data and returns the resulting HTML string.
///
/// The JSON data is serialised and injected as a virtual file at
/// `/data/{app_name}/{template_name}.json`, which the template can read with
/// `#let data = json("/data/<app_name>/<template_name>.json")`.
///
/// # Errors
/// Returns an error if serialisation of `json_data` fails or if the Typst
/// compilation / HTML export fails.
pub fn typst_to_html(req: CompileRequest<'_>) -> Result<String> {
    let json_bytes = serde_json::to_vec(req.json_data).context("Failed to serialize JSON data")?;
    let data_path = format!("/data/{}/{}.json", req.app_name, req.template_name);
    let vfiles = HashMap::from([(data_path, Bytes::new(json_bytes))]);

    let result = typst_world::compile_to_html(
        req.fonts,
        req.root,
        req.resources_dir,
        "/main.typ",
        req.template_source,
        vfiles,
        req.library,
    );
    comemo::evict(req.comemo_eviction_threshold);
    counter!("comemo_evictions_total", &[("output", "html")]).increment(1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdf::CompileRequest;
    use crate::typst_world::{build_library, load_fonts};
    use std::path::PathBuf;
    use std::sync::Arc;
    use typst::Feature;
    use typst::Library;
    use typst::utils::LazyHash;

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn fonts_dir() -> PathBuf {
        root_dir().join("fonts")
    }

    fn resources_dir() -> PathBuf {
        root_dir().join("resources")
    }

    fn html_library() -> Arc<LazyHash<Library>> {
        Arc::new(build_library([Feature::Html].into_iter().collect()))
    }

    #[test]
    fn typst_to_html_simple_template_returns_html_string() -> Result<()> {
        let source = "Hello, world!\n";
        let data = serde_json::json!({});
        let html = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "simple",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(
            html.contains("<!DOCTYPE html>") && html.contains("<html"),
            "Expected HTML document"
        );
        assert!(html.contains("Hello, world!"));
        Ok(())
    }

    #[test]
    fn typst_to_html_with_json_data_returns_html_with_data() -> Result<()> {
        let source = r#"#let data = json("/data/test/app.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let html = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "app",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(html.contains("Test User"));
        Ok(())
    }

    #[test]
    fn typst_to_html_invalid_source_returns_error() -> Result<()> {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "invalid",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        });
        assert!(
            result.is_err(),
            "Expected an error for invalid Typst source"
        );
        Ok(())
    }

    #[test]
    fn typst_to_html_with_nested_json_data() -> Result<()> {
        let source = r#"#let data = json("/data/test/nested.json")
#data.at("user").at("name", default: "")
"#;
        let data = serde_json::json!({
            "user": {
                "name": "Alice",
                "age": 30
            }
        });
        let html = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "nested",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(html.contains("Alice"));
        Ok(())
    }

    #[test]
    fn typst_to_html_with_array_json_data() -> Result<()> {
        let source = r#"#let data = json("/data/test/array.json")
#for item in data.at("items", default: ()) [
  #item
]
"#;
        let data = serde_json::json!({
            "items": ["alpha", "beta", "gamma"]
        });
        let html = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "array",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(html.contains("alpha"));
        assert!(html.contains("beta"));
        assert!(html.contains("gamma"));
        Ok(())
    }

    #[test]
    fn typst_to_html_with_empty_json_object() -> Result<()> {
        let source = r#"#let data = json("/data/test/empty.json")
Empty: #data.keys().len()
"#;
        let data = serde_json::json!({});
        let html = typst_to_html(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: Arc::new(load_fonts(&fonts_dir())?),
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "empty",
            library: html_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(html.contains("<!DOCTYPE html>"));
        Ok(())
    }
}
