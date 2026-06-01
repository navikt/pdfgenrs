use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use typst::foundations::Bytes;

use crate::typst_world::{self, Fonts};

/// Compiles a Typst template with JSON data and returns the resulting HTML string.
///
/// The JSON data is serialised and injected as a virtual file at
/// `/data/{app_name}/{template_name}.json`, which the template can read with
/// `#let data = json("/data/<app_name>/<template_name>.json")`.
///
/// # Errors
/// Returns an error if serialisation of `json_data` fails or if the Typst
/// compilation / HTML export fails.
pub fn typst_to_html(
    template_source: &str,
    json_data: &serde_json::Value,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    app_name: &str,
    template_name: &str,
) -> Result<String> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    let data_path = format!("/data/{app_name}/{template_name}.json");
    vfiles.insert(data_path, Bytes::new(json_bytes));

    typst_world::compile_to_html(
        fonts,
        root,
        resources_dir,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_world::load_fonts;
    use std::path::PathBuf;
    use std::sync::Arc;

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn fonts_dir() -> PathBuf {
        root_dir().join("fonts")
    }

    fn resources_dir() -> PathBuf {
        root_dir().join("resources")
    }

    #[test]
    fn typst_to_html_simple_template_returns_html_string() -> Result<()> {
        let source = "Hello, world!\n";
        let data = serde_json::json!({});
        let html = typst_to_html(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir())?),
            &root_dir(),
            &resources_dir(),
            "test",
            "simple",
        )?;
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
        let html = typst_to_html(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir())?),
            &root_dir(),
            &resources_dir(),
            "test",
            "app",
        )?;
        assert!(html.contains("Test User"));
        Ok(())
    }

    #[test]
    fn typst_to_html_invalid_source_returns_error() -> Result<()> {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_html(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir())?),
            &root_dir(),
            &resources_dir(),
            "test",
            "invalid",
        );
        assert!(
            result.is_err(),
            "Expected an error for invalid Typst source"
        );
        Ok(())
    }
}
