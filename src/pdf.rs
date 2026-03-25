use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use typst::foundations::Bytes;

use crate::typst_world::{self, FontCache};


pub fn html_to_pdf(html: &str, font_cache: FontCache, root: &Path) -> Result<Vec<u8>> {
    // Build a Typst document that displays the HTML content as a raw block
    // This allows PDF generation without an external browser.
    let typst_source = r#"#set document(title: "pdfgenrs", date: auto)
#set page(margin: (top: 1cm, bottom: 1cm, left: 1cm, right: 1cm))
#let content = read("/html-content", encoding: none)
#raw(str(content), lang: "html")
"#
    .to_string();

    let mut vfiles = HashMap::new();
    vfiles.insert("/html-content".to_string(), Bytes::new(html.as_bytes().to_vec()));

    typst_world::compile_to_pdf(font_cache, root, "/main.typ", typst_source, vfiles)
}


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

pub fn image_to_pdf(
    image_bytes: &[u8],
    content_type: &str,
    font_cache: FontCache,
    root: &Path,
) -> Result<Vec<u8>> {
    let fmt = if content_type.contains("png") { "png" } else { "jpg" };
    let typst_source = format!(
        r#"#set document(date: auto)
#set page(margin: 0pt, width: auto, height: auto)
#let img-data = read("/image-data", encoding: none)
#image.decode(img-data, format: "{fmt}", alt: "image")
"#
    );

    let mut vfiles = HashMap::new();
    vfiles.insert("/image-data".to_string(), Bytes::new(image_bytes.to_vec()));

    typst_world::compile_to_pdf(font_cache, root, "/main.typ", typst_source, vfiles)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_world::load_font_cache;
    use std::path::PathBuf;

    fn fonts_dir() -> String {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("fonts")
            .to_string_lossy()
            .into_owned()
    }

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    // Minimal 1×1 white PNG (deflate-compressed, valid ICC-free sRGB PNG)
    fn minimal_png() -> Vec<u8> {
        vec![
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
            0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR length + type
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // width=1, height=1
            0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, // 8-bit RGB, CRC
            0xDE, 0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, // IDAT length + type
            0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F, // compressed pixel data
            0x00, 0x05, 0xFE, 0x02, 0xFE, 0xDC, 0xCC, 0x59, // CRC
            0xE7, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, // IEND length + type
            0x44, 0xAE, 0x42, 0x60, 0x82, // IEND CRC
        ]
    }

    #[test]
    fn html_to_pdf_returns_pdf_bytes() {
        let html = "<h1>Hello</h1><p>Unit test</p>";
        let result = html_to_pdf(html, load_font_cache(&fonts_dir()), &root_dir());
        assert!(result.is_ok(), "html_to_pdf failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes), "Output does not start with %PDF");
    }

    #[test]
    fn html_to_pdf_empty_html_returns_pdf_bytes() {
        let result = html_to_pdf("", load_font_cache(&fonts_dir()), &root_dir());
        assert!(result.is_ok(), "html_to_pdf with empty input failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_simple_template_returns_pdf_bytes() {
        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
"#;
        let data = serde_json::json!({});
        let result = typst_to_pdf(source, &data, load_font_cache(&fonts_dir()), &root_dir());
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
        let result = typst_to_pdf(source, &data, load_font_cache(&fonts_dir()), &root_dir());
        assert!(result.is_ok(), "typst_to_pdf with JSON data failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }

    #[test]
    fn typst_to_pdf_invalid_source_returns_error() {
        let source = "#this-is-not-valid-typst-syntax(((";
        let data = serde_json::json!({});
        let result = typst_to_pdf(source, &data, load_font_cache(&fonts_dir()), &root_dir());
        assert!(result.is_err(), "Expected an error for invalid Typst source");
    }

    #[test]
    fn image_to_pdf_png_returns_pdf_bytes() {
        let png = minimal_png();
        let result = image_to_pdf(&png, "image/png", load_font_cache(&fonts_dir()), &root_dir());
        assert!(result.is_ok(), "image_to_pdf (PNG) failed: {:?}", result.err());
        let bytes = result.unwrap();
        assert!(is_pdf(&bytes));
    }
}
