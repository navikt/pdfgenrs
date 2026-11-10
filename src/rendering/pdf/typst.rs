use metrics::counter;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use typst::Library;
use typst::foundations::Bytes;
use typst::utils::LazyHash;

use super::PdfRenderError;
use crate::typst_world::{self, Fonts};

/// Bundles the arguments required to compile a Typst template with JSON data.
///
/// Used as the single parameter for [`typst_to_pdf`] and [`crate::html::typst_to_html`].
pub struct CompileRequest<'a> {
    /// The Typst source of the template.
    pub template_source: &'a str,
    /// The JSON data to inject into the template.
    pub json_data: &'a serde_json::Value,
    /// The loaded font set.
    pub fonts: Arc<Fonts>,
    /// The root directory of the Typst world.
    pub root: &'a Path,
    /// The directory containing shared resources.
    pub resources_dir: &'a Path,
    /// The application name, used to build the virtual JSON data path.
    pub app_name: &'a str,
    /// The template name, used to build the virtual JSON data path.
    pub template_name: &'a str,
    /// The Typst standard library to use for compilation.
    pub library: Arc<LazyHash<Library>>,
    /// Number of cache entries to evict from the comemo memoization cache after compilation.
    pub comemo_eviction_threshold: usize,
}

impl std::fmt::Debug for CompileRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompileRequest")
            .field("template_source", &self.template_source)
            .field("json_data", &self.json_data)
            .field("root", &self.root)
            .field("resources_dir", &self.resources_dir)
            .field("app_name", &self.app_name)
            .field("template_name", &self.template_name)
            .finish_non_exhaustive()
    }
}

/// Compiles a Typst template with JSON data and returns the resulting PDF bytes.
///
/// The JSON data is serialised and injected as a virtual file at
/// `/data/{app_name}/{template_name}.json`, which the template can read with
/// `#let data = json("/data/<app_name>/<template_name>.json")`.
///
/// # Errors
/// Returns an error if serialisation of `json_data` fails or if the Typst
/// compilation / PDF export fails.
pub fn typst_to_pdf(req: CompileRequest<'_>) -> Result<Vec<u8>, PdfRenderError> {
    let json_bytes = serde_json::to_vec(req.json_data)
        .map_err(|source| PdfRenderError::JsonSerialization { source })?;
    let data_path = format!("/data/{}/{}.json", req.app_name, req.template_name);
    let vfiles = HashMap::from([(data_path, Bytes::new(json_bytes))]);

    let result = typst_world::compile_to_pdf(
        req.fonts,
        req.root,
        req.resources_dir,
        "/main.typ",
        req.template_source,
        vfiles,
        req.library,
    );
    comemo::evict(req.comemo_eviction_threshold);
    counter!("comemo_evictions_total", &[("output", "pdf")]).increment(1);
    result.map_err(PdfRenderError::from)
}

#[cfg(test)]
mod tests {
    use super::super::testutil::*;
    use super::*;
    use anyhow::Result;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
Hello, world!
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "simple",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    /// Regression test for a PDF/A-2a and PDF/UA-1 export failure caused by
    /// Arabic-Indic digits (e.g. U+0660..U+0669) not being covered by any
    /// bundled font. Source Sans 3 and Noto Color Emoji don't include Arabic
    /// script glyphs, so Typst's font fallback had nowhere to look. Noto Sans
    /// Arabic was added to `fonts/` to cover this range.
    #[test]
    fn typst_to_pdf_arabic_indic_digits_are_rendered() -> Result<()> {
        let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
Numbers: ٠١٢٣٤٥٦٧٨٩
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "simple",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_records_comemo_evictions_total_metric() -> Result<()> {
        let recorder = metrics_exporter_prometheus::PrometheusBuilder::new().build_recorder();
        let handle = recorder.handle();
        metrics::with_local_recorder(&recorder, || -> Result<()> {
            let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
Hello, world!
"#;
            let data = serde_json::json!({});
            typst_to_pdf(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: test_fonts()?,
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "test",
                template_name: "simple",
                library: pdf_library(),
                comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            })?;
            Ok(())
        })?;

        let output = handle.render();
        assert!(
            output.contains(r#"comemo_evictions_total{output="pdf"} 1"#),
            "expected comemo_evictions_total{{output=\"pdf\"}} 1 in output: {output}"
        );
        Ok(())
    }

    #[test]
    fn typst_to_pdf_with_json_data_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(title: "Test", date: auto)
#let data = json("/data/test/app.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let bytes = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "app",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() -> Result<()> {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "invalid",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        });
        assert!(
            result.is_err(),
            "Expected an error for invalid Typst source"
        );
        Ok(())
    }

    #[test]
    fn typst_to_pdf_returns_error_when_resources_dir_is_missing() -> Result<()> {
        let source = r#"#set document(title: "Assets", date: auto)
#image("/resources/NAVLogoRed.png")
"#;
        let data = serde_json::json!({});
        let temp = TempDir::new()?;
        let missing_resources = temp.path().join("missing-resources");
        let result = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &missing_resources,
            app_name: "test",
            template_name: "assets-missing",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        });
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn typst_to_pdf_returns_error_when_resources_dir_is_not_a_directory() -> Result<()> {
        let source = r#"#set document(title: "Assets", date: auto)
#image("/resources/NAVLogoRed.png")
"#;
        let data = serde_json::json!({});
        let temp = TempDir::new()?;
        let malformed_resources = temp.path().join("not-a-directory");
        fs::write(&malformed_resources, b"file")?;
        let result = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &malformed_resources,
            app_name: "test",
            template_name: "assets-malformed",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        });
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn typst_to_pdf_with_resource_image_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(CompileRequest {
            template_source: source,
            json_data: &data,
            fonts: test_fonts()?,
            root: &root_dir(),
            resources_dir: &resources_dir(),
            app_name: "test",
            template_name: "resource",
            library: pdf_library(),
            comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        })?;
        assert!(is_pdf(&bytes));
        Ok(())
    }
}
