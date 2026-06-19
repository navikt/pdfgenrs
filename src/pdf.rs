use anyhow::{Context, Result};
use ironpress::HtmlConverter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tracing::warn;
use typst::foundations::Bytes;

use crate::typst_world::{self, Fonts};

const HTML_FONT_ALIASES: &[(&str, &str)] = &[
    ("Source Sans 3", "SourceSans3-Regular.ttf"),
    ("Source Sans 3__bold", "SourceSans3-Bold.ttf"),
    ("Source Sans 3__italic", "SourceSans3-Italic.ttf"),
    ("SourceSans3", "SourceSans3-Regular.ttf"),
    ("SourceSans3__bold", "SourceSans3-Bold.ttf"),
    ("SourceSans3__italic", "SourceSans3-Italic.ttf"),
    ("Noto Color Emoji", "NotoColorEmoji-Regular.ttf"),
    ("Noto Emoji", "NotoColorEmoji-Regular.ttf"),
];

/// Cached font alias bytes. Once loaded, subsequent calls to [`build_html_converter`]
/// reuse the cached bytes instead of re-reading files from disk.
type FontAliasCache = (PathBuf, Vec<(&'static str, Vec<u8>)>);
static FONT_ALIAS_CACHE: OnceLock<FontAliasCache> = OnceLock::new();

/// Loads font alias bytes from `fonts_dir`, using a process-wide cache to avoid
/// redundant file I/O on repeated calls with the same directory.
fn load_font_aliases(fonts_dir: &Path) -> &'static [(&'static str, Vec<u8>)] {
    let (_, aliases) = FONT_ALIAS_CACHE.get_or_init(|| {
        let mut loaded = Vec::new();
        for (family, file_name) in HTML_FONT_ALIASES {
            let font_path = fonts_dir.join(file_name);
            match std::fs::read(&font_path) {
                Ok(font_bytes) => {
                    loaded.push((*family, font_bytes));
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
        (fonts_dir.to_path_buf(), loaded)
    });
    aliases
}

/// Builds a pre-configured [`HtmlConverter`] with font aliases loaded from `fonts_dir`.
///
/// Font alias bytes are cached in a process-wide [`OnceLock`] so that repeated calls
/// (e.g. in tests) avoid redundant file I/O. The converter itself is constructed fresh
/// each call with the given `base_path`, but the expensive disk reads happen at most once.
///
/// Font files that cannot be read are skipped and logged as warnings (on first load only).
///
/// Returns a tuple of `(converter, count)` where `count` is the number of
/// font aliases successfully loaded.
#[must_use]
pub fn build_html_converter(fonts_dir: &Path, base_path: &Path) -> (HtmlConverter, usize) {
    let aliases = load_font_aliases(fonts_dir);
    let mut converter = HtmlConverter::new().base_path(base_path);

    for (family, font_bytes) in aliases {
        converter = converter.add_font(family, font_bytes.clone());
    }

    (converter, aliases.len())
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
#[must_use = "this returns a Result that should be handled"]
pub fn typst_to_pdf(
    template_source: String,
    json_data: &serde_json::Value,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    app_name: &str,
    template_name: &str,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let data_path = format!("/data/{app_name}/{template_name}.json");
    let vfiles = HashMap::from([(data_path, Bytes::new(json_bytes))]);

    typst_world::compile_to_pdf(
        fonts,
        root,
        resources_dir,
        "/main.typ",
        template_source,
        vfiles,
    )
}

/// Converts an HTML document into PDF bytes using a pre-built converter.
#[must_use = "this returns a Result that should be handled"]
pub fn html_to_pdf(html: &str, converter: &HtmlConverter) -> Result<Vec<u8>> {
    converter
        .convert(html)
        .context("Failed to convert HTML to PDF")
}

/// Converts a PNG or JPEG image into PDF bytes.
///
/// Landscape images (width > height) are automatically placed on a
/// landscape-oriented page so they fill the page without distortion.
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
    let is_landscape = image_dimensions(image_bytes.as_ref())
        .map(|(w, h)| w > h)
        .unwrap_or(false);

    let mut vfiles = HashMap::new();
    vfiles.insert(image_path.to_string(), Bytes::new(image_bytes));

    let flipped = if is_landscape { "flipped: true, " } else { "" };
    let source = format!(
        r#"#set document(title: "Image", date: auto)
#set page({flipped}margin: 0pt)
#image("{image_path}", width: 100%, alt: "Uploaded image")
"#
    );

    typst_world::compile_to_pdf(fonts, root, resources_dir, "/main.typ", source, vfiles)
}

/// Extracts (width, height) from PNG or JPEG image bytes by parsing headers.
///
/// Returns `None` if the format is unrecognised or the header is too short.
fn image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        png_dimensions(data)
    } else if data.starts_with(&[0xFF, 0xD8]) {
        jpeg_dimensions(data)
    } else {
        None
    }
}

fn png_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 24 {
        return None;
    }
    let width = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
    let height = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
    Some((width, height))
}

fn jpeg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let mut i = 2;
    while data.len() >= i + 2 {
        if data[i] != 0xFF {
            return None;
        }
        let marker = data[i + 1];
        if marker == 0xD9 {
            return None;
        }
        if matches!(marker, 0xC0..=0xC3) {
            if data.len() < i + 9 {
                return None;
            }
            let height = u32::from(u16::from_be_bytes([data[i + 5], data[i + 6]]));
            let width = u32::from(u16::from_be_bytes([data[i + 7], data[i + 8]]));
            return Some((width, height));
        }
        if data.len() < i + 4 {
            return None;
        }
        let seg_len = u16::from_be_bytes([data[i + 2], data[i + 3]]) as usize;
        i += 2 + seg_len;
    }
    None
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
        let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
Hello, world!
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(
            source.to_string(),
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
        let source = r#"#set document(title: "Test", date: auto)
#let data = json("/data/test/app.json")
#data.at("name", default: "")
"#;
        let data = serde_json::json!({"name": "Test User"});
        let bytes = typst_to_pdf(
            source.to_string(),
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
            source.to_string(),
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
    fn image_to_pdf_landscape_png_returns_pdf_bytes() -> Result<()> {
        let image_bytes = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        assert!(
            image_dimensions(&image_bytes).is_some_and(|(w, h)| w > h),
            "Test image should be landscape"
        );
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
    fn image_dimensions_png_parses_correctly() -> Result<()> {
        let data = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        let dims = image_dimensions(&data);
        assert_eq!(dims, Some((2201, 1386)));
        Ok(())
    }

    #[test]
    fn image_dimensions_returns_none_for_short_data() {
        assert_eq!(image_dimensions(&[0x89, 0x50, 0x4E, 0x47]), None);
        assert_eq!(image_dimensions(&[0xFF, 0xD8]), None);
        assert_eq!(image_dimensions(&[]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_unknown_format() {
        assert_eq!(image_dimensions(b"GIF89a"), None);
    }

    // --- Fuzz-style edge case tests for PNG parser ---

    #[test]
    fn png_dimensions_returns_none_for_valid_magic_but_truncated_ihdr() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn png_dimensions_handles_exactly_24_bytes() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&100u32.to_be_bytes());
        data.extend_from_slice(&200u32.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((100, 200)));
    }

    #[test]
    fn png_dimensions_zero_width_and_height() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&0u32.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((0, 0)));
    }

    #[test]
    fn png_dimensions_max_u32_values() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        data.extend_from_slice(&u32::MAX.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((u32::MAX, u32::MAX)));
    }

    #[test]
    fn png_dimensions_square_image_not_landscape() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&500u32.to_be_bytes());
        data.extend_from_slice(&500u32.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| w <= h),
            "Square image should not be detected as landscape"
        );
    }

    #[test]
    fn png_dimensions_portrait_image() {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        data.extend_from_slice(&100u32.to_be_bytes());
        data.extend_from_slice(&200u32.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| h > w),
            "Image should be portrait"
        );
    }

    // --- Fuzz-style edge case tests for JPEG parser ---

    #[test]
    fn jpeg_dimensions_returns_none_for_soi_only() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_immediate_eoi() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8, 0xFF, 0xD9]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_when_non_ff_byte_encountered() {
        assert_eq!(image_dimensions(&[0xFF, 0xD8, 0x00, 0xC0]), None);
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_sof_marker_with_truncated_data() {
        let data = [0xFF, 0xD8, 0xFF, 0xC0, 0x00, 0x11, 0x08];
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn jpeg_dimensions_parses_sof0_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((640, 480)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof1_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC1];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&100u16.to_be_bytes());
        data.extend_from_slice(&200u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((200, 100)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof2_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC2];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&300u16.to_be_bytes());
        data.extend_from_slice(&400u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((400, 300)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof3_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC3];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&768u16.to_be_bytes());
        data.extend_from_slice(&1024u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((1024, 768)));
    }

    #[test]
    fn jpeg_dimensions_skips_non_sof_segments_before_sof() {
        let mut data = vec![0xFF, 0xD8];
        data.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x10]);
        data.extend_from_slice(&[0x00; 14]);
        data.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x11, 0x08]);
        data.extend_from_slice(&1080u16.to_be_bytes());
        data.extend_from_slice(&1920u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((1920, 1080)));
    }

    #[test]
    fn jpeg_dimensions_returns_none_for_truncated_segment_length() {
        let data = [0xFF, 0xD8, 0xFF, 0xE0];
        assert_eq!(image_dimensions(&data), None);
    }

    #[test]
    fn jpeg_dimensions_zero_dimensions() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&0u16.to_be_bytes());
        data.extend_from_slice(&0u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((0, 0)));
    }

    #[test]
    fn jpeg_dimensions_landscape_detection() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC0];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert!(
            image_dimensions(&data).is_some_and(|(w, h)| w > h),
            "640x480 should be landscape"
        );
    }

    // --- Malformed image edge cases ---

    #[test]
    fn image_dimensions_returns_none_for_single_byte() {
        assert_eq!(image_dimensions(&[0xFF]), None);
        assert_eq!(image_dimensions(&[0x89]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_partial_png_magic() {
        assert_eq!(image_dimensions(b"\x89PNG\r\n"), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_all_zeros() {
        assert_eq!(image_dimensions(&[0u8; 100]), None);
    }

    #[test]
    fn image_dimensions_returns_none_for_all_0xff() {
        assert_eq!(image_dimensions(&[0xFF; 100]), None);
    }

    #[test]
    fn typst_to_pdf_with_resource_image_returns_pdf_bytes() -> Result<()> {
        let source = r#"#set document(title: "Test", date: auto)
#set page(margin: 1cm)
#image("/resources/NAVLogoRed.png", width: 50%, alt: "NAV logo")
"#;
        let data = serde_json::json!({});
        let bytes = typst_to_pdf(
            source.to_string(),
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
