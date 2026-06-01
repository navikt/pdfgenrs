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

/// Builds a pre-configured [`HtmlConverter`] with font aliases loaded from `fonts_dir`.
///
/// The converter is constructed once and can be reused across requests via shared
/// reference, avoiding per-request cloning of font byte vectors.
/// Font files that cannot be read are skipped and logged as warnings.
///
/// Returns a tuple of `(converter, count)` where `count` is the number of
/// font aliases successfully loaded.
pub fn build_html_converter(fonts_dir: &Path, base_path: &Path) -> (HtmlConverter, usize) {
    let mut converter = HtmlConverter::new().base_path(base_path);
    let mut count = 0;

    for (family, file_name) in HTML_FONT_ALIASES {
        let font_path = fonts_dir.join(file_name);
        match std::fs::read(&font_path) {
            Ok(font_bytes) => {
                converter = converter.add_font(family, font_bytes);
                count += 1;
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

    (converter, count)
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
pub fn typst_to_pdf(
    template_source: &str,
    json_data: &serde_json::Value,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    app_name: &str,
    template_name: &str,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    let data_path = format!("/data/{app_name}/{template_name}.json");
    vfiles.insert(data_path, Bytes::new(json_bytes));

    typst_world::compile_to_pdf(
        fonts,
        root,
        resources_dir,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

/// Converts an HTML document into PDF bytes using a pre-built converter.
pub fn html_to_pdf(html: &str, converter: &HtmlConverter) -> Result<Vec<u8>> {
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
        r#"#set document(date: auto)
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

    fn test_fonts() -> Result<Arc<Fonts>> {
        Ok(Arc::new(load_fonts(&fonts_dir())?))
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() -> Result<()> {
        let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
";
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(
            source,
            &data,
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            "test",
            "simple",
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_with_json_data_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(date: auto)
#let data = json("/data/test/app.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let bytes = typst_to_pdf(
            source,
            &data,
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            "test",
            "app",
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() -> Result<()> {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(
            source,
            &data,
            test_fonts()?,
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

    #[test]
    fn html_to_pdf_simple_document_returns_pdf_bytes() -> Result<()> {
        let source = "<!DOCTYPE html><html><body><h1>Hello, world!</h1></body></html>";
        let (converter, _) = build_html_converter(&fonts_dir(), &root_dir());
        let bytes = html_to_pdf(source, &converter)?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn html_to_pdf_with_source_sans_pro_alias_returns_pdf_bytes() -> Result<()> {
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
        let (converter, _) = build_html_converter(&fonts_dir(), &root_dir());
        let bytes = html_to_pdf(source, &converter)?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn image_to_pdf_png_returns_pdf_bytes() -> Result<()> {
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
    fn typst_to_pdf_with_resource_image_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(
            source,
            &data,
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            "test",
            "resource",
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }
}
