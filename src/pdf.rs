use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use typst::foundations::Bytes;

use crate::typst_world::{self, FontCache};


pub fn typst_to_pdf(
    template_source: &str,
    json_data: &serde_json::Value,
    font_cache: FontCache,
    root: &Path,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    vfiles.insert("/data.json".to_string(), Bytes::new(json_bytes));

    typst_world::compile_to_pdf(
        font_cache,
        root,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_world::load_font_cache;
    use std::path::PathBuf;

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() {
        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
"#;
        let data = serde_json::json!({});
        let result = typst_to_pdf(source, &data, load_font_cache(), &root_dir());
        assert!(result.is_ok(), "typst_to_pdf failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_with_json_data_returns_pdf_bytes() {
        let source = r#"#set document(date: auto)
#let data = json("/data.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let result = typst_to_pdf(source, &data, load_font_cache(), &root_dir());
        assert!(result.is_ok(), "typst_to_pdf with JSON data failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(source, &data, load_font_cache(), &root_dir());
        assert!(result.is_err(), "Expected an error for invalid Typst source");
    }
}
