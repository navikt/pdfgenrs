use anyhow::{Context, Result};
use ironpress::HtmlConverter;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tracing::warn;
use typst::foundations::Bytes;

use crate::typst_world::{self, Fonts};

const HTML_FONT_ALIASES: &[(&str, &str)] = &[
    ("Source Sans Pro", "SourceSans3-Regular.ttf"),
    ("Source Sans Pro__bold", "SourceSans3-Bold.ttf"),
    ("Source Sans Pro__italic", "SourceSans3-Italic.ttf"),
    ("Source Sans 3", "SourceSans3-Regular.ttf"),
    ("Source Sans 3__bold", "SourceSans3-Bold.ttf"),
    ("Source Sans 3__italic", "SourceSans3-Italic.ttf"),
    ("SourceSans3", "SourceSans3-Regular.ttf"),
    ("SourceSans3__bold", "SourceSans3-Bold.ttf"),
    ("SourceSans3__italic", "SourceSans3-Italic.ttf"),
    ("Noto Color Emoji", "NotoColorEmoji-Regular.ttf"),
    ("Noto Emoji", "NotoColorEmoji-Regular.ttf"),
];

/// Compiles a Typst template with JSON data and returns the resulting PDF bytes.
///
/// The JSON data is serialised and injected as a virtual file at `/data.json`,
/// which the template can read with `#let data = json("/data.json")`.
///
/// # Errors
/// Returns an error if serialisation of `json_data` fails or if the Typst
/// compilation / PDF export fails.
pub fn typst_to_pdf(
    template_source: &str,
    json_data: &serde_json::Value,
    fonts: Arc<Fonts>,
    root: &Path,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    vfiles.insert("/data.json".to_string(), Bytes::new(json_bytes));

    typst_world::compile_to_pdf(
        fonts,
        root,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

/// Converts an HTML document into PDF bytes.
pub fn html_to_pdf(html: &str, root: &Path, fonts_dir: &Path) -> Result<Vec<u8>> {
    let mut converter = HtmlConverter::new().base_path(root);

    for (family, file_name) in HTML_FONT_ALIASES {
        let font_path = fonts_dir.join(file_name);
        match std::fs::read(&font_path) {
            Ok(font_bytes) => {
                converter = converter.add_font(family, font_bytes);
            }
            Err(error) => {
                warn!(
                    font_path = %font_path.display(),
                    font_family = family,
                    "Failed to load HTML font alias: {error}"
                );
            }
        }
    }

    converter
        .convert(html)
        .context("Failed to convert HTML to PDF")
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

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() {
        let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
";
        let data = serde_json::json!({});
        let result = typst_to_pdf(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir()).expect("test fonts should load")),
            &root_dir(),
        );
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
        let result = typst_to_pdf(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir()).expect("test fonts should load")),
            &root_dir(),
        );
        assert!(
            result.is_ok(),
            "typst_to_pdf with JSON data failed: {:?}",
            result.err()
        );
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir()).expect("test fonts should load")),
            &root_dir(),
        );
        assert!(
            result.is_err(),
            "Expected an error for invalid Typst source"
        );
    }

    #[test]
    fn html_to_pdf_simple_document_returns_pdf_bytes() {
        let source = "<!DOCTYPE html><html><body><h1>Hello, world!</h1></body></html>";
        let result = html_to_pdf(source, &root_dir(), &fonts_dir());
        assert!(result.is_ok(), "html_to_pdf failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn html_to_pdf_with_source_sans_pro_alias_returns_pdf_bytes() {
        let source = r#"<!DOCTYPE html>
<html>
<head>
    <style>
        h1 {
            font-family: "Source Sans Pro" !important;
        }
    </style>
</head>
<body>
    <h1>Hello, world!</h1>
</body>
</html>"#;
        let result = html_to_pdf(source, &root_dir(), &fonts_dir());
        assert!(
            result.is_ok(),
            "html_to_pdf with Source Sans Pro failed: {:?}",
            result.err()
        );
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_with_resource_image_returns_pdf_bytes() {
        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let data = serde_json::json!({});
        let result = typst_to_pdf(
            source,
            &data,
            Arc::new(load_fonts(&fonts_dir()).expect("test fonts should load")),
            &root_dir(),
        );
        assert!(
            result.is_ok(),
            "typst_to_pdf with image failed: {:?}",
            result.err()
        );
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }
}
