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

/// Loads predefined HTML font aliases from `fonts_dir`.
///
/// Each returned tuple contains `(font_family_name, font_bytes)`.
/// Files that cannot be read are skipped and logged as warnings.
pub fn load_html_font_aliases(fonts_dir: &Path) -> Vec<(String, Vec<u8>)> {
    HTML_FONT_ALIASES
        .iter()
        .filter_map(|(family, file_name)| {
            let font_path = fonts_dir.join(file_name);
            match std::fs::read(&font_path) {
                Ok(font_bytes) => Some(((*family).to_string(), font_bytes)),
                Err(error) => {
                    warn!(
                        font_path = %font_path.display(),
                        font_family = family,
                        "Failed to load HTML font alias: {error}"
                    );
                    None
                }
            }
        })
        .collect()
}

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
    resources_dir: &Path,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    vfiles.insert("/data.json".to_string(), Bytes::new(json_bytes));

    typst_world::compile_to_pdf(
        fonts,
        root,
        resources_dir,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

/// Converts an HTML document into PDF bytes.
pub fn html_to_pdf(
    html: &str,
    root: &Path,
    html_font_aliases: &[(String, Vec<u8>)],
) -> Result<Vec<u8>> {
    let mut converter = HtmlConverter::new().base_path(root);

    for (family, font_bytes) in html_font_aliases {
        converter = converter.add_font(family.as_str(), font_bytes.clone());
    }

    converter
        .convert(html)
        .context("Failed to convert HTML to PDF")
}

/// Converts a PNG or JPEG image into PDF bytes.
pub fn image_to_pdf<B>(
    image_bytes: B,
    image_path: &str,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
) -> Result<Vec<u8>>
where
    B: AsRef<[u8]> + Send + Sync + 'static,
{
    let mut vfiles = HashMap::new();
    vfiles.insert(image_path.to_string(), Bytes::new(image_bytes));

    let source = format!(
        r#"#set document(date: auto, title: "Image")
#set page(margin: 0pt)
#image("{image_path}", width: 100%, alt: "Uploaded image")
"#
    );

    typst_world::compile_to_pdf(fonts, root, resources_dir, "/main.typ", source, vfiles)
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

    fn test_fonts() -> anyhow::Result<Arc<Fonts>> {
        Ok(Arc::new(load_fonts(&fonts_dir())?))
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() -> anyhow::Result<()> {
        let source = r#"#set document(date: auto, title: "Test")
#set page(margin: 1cm)
Hello, world!
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(source, &data, test_fonts()?, &root_dir(), &resources_dir())?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_with_json_data_returns_pdf_bytes() -> anyhow::Result<()> {
        let source = r#"#set document(date: auto, title: "Test")
#let data = json("/data.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let bytes = typst_to_pdf(source, &data, test_fonts()?, &root_dir(), &resources_dir())?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() -> anyhow::Result<()> {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(source, &data, test_fonts()?, &root_dir(), &resources_dir());
        assert!(
            result.is_err(),
            "Expected an error for invalid Typst source"
        );
        Ok(())
    }

    #[test]
    fn html_to_pdf_simple_document_returns_pdf_bytes() -> anyhow::Result<()> {
        let source = "<!DOCTYPE html><html><body><h1>Hello, world!</h1></body></html>";
        let html_font_aliases = load_html_font_aliases(&fonts_dir());
        let bytes = html_to_pdf(source, &root_dir(), &html_font_aliases)?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn html_to_pdf_with_source_sans_pro_alias_returns_pdf_bytes() -> anyhow::Result<()> {
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
        let html_font_aliases = load_html_font_aliases(&fonts_dir());
        let bytes = html_to_pdf(source, &root_dir(), &html_font_aliases)?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn image_to_pdf_png_returns_pdf_bytes() -> anyhow::Result<()> {
        let image_bytes = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        let bytes = image_to_pdf(
            image_bytes,
            "/image.png",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_with_resource_image_returns_pdf_bytes() -> anyhow::Result<()> {
        let source = r#"#set document(date: auto, title: "Test")
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(source, &data, test_fonts()?, &root_dir(), &resources_dir())?;
        assert!(is_pdf(&bytes));
        Ok(())
    }
}
