use anyhow::{Context, Result};
use ironpress::HtmlConverter;
use metrics::counter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tracing::warn;
use typst::foundations::Bytes;
use typst_library::text::Font;
use walkdir::WalkDir;

use crate::typst_world::{self, Fonts};
use typst::Library;
use typst::utils::LazyHash;

/// Cached font data discovered from the fonts directory. Once loaded, subsequent
/// calls to [`build_html_converter`] reuse the cached data instead of re-reading
/// files from disk. The `Arc<Vec<u8>>` wrapping lets multiple font faces from the
/// same collection file share one allocation.
type FontCache = (PathBuf, Vec<(String, Arc<Vec<u8>>)>);
static FONT_CACHE: OnceLock<FontCache> = OnceLock::new();

/// Derives a CSS-friendly font name from a font's family and variant.
///
/// For regular weight (400) and normal style, returns just the family name.
/// For other variants, appends the weight/style descriptor (e.g. "bold", "italic").
fn css_font_name(family: &str, variant: &typst_library::text::FontVariant) -> String {
    use typst_library::text::{FontStyle, FontWeight};

    let weight_suffix = match variant.weight {
        FontWeight::THIN => Some("thin"),
        FontWeight::EXTRALIGHT => Some("extra light"),
        FontWeight::LIGHT => Some("light"),
        FontWeight::REGULAR => None,
        FontWeight::MEDIUM => Some("medium"),
        FontWeight::SEMIBOLD => Some("semi bold"),
        FontWeight::BOLD => Some("bold"),
        FontWeight::EXTRABOLD => Some("extra bold"),
        FontWeight::BLACK => Some("black"),
        _ => None,
    };

    let style_suffix = match variant.style {
        FontStyle::Normal => None,
        FontStyle::Italic => Some("italic"),
        FontStyle::Oblique => Some("oblique"),
    };

    match (weight_suffix, style_suffix) {
        (None, None) => family.to_string(),
        (Some(w), None) => format!("{family} {w}"),
        (None, Some(s)) => format!("{family} {s}"),
        (Some(w), Some(s)) => format!("{family} {w} {s}"),
    }
}

/// Discovers all fonts in `fonts_dir` and returns `(css_name, font_bytes)` pairs,
/// using a process-wide cache to avoid redundant file I/O.
fn discover_fonts(fonts_dir: &Path) -> &'static [(String, Arc<Vec<u8>>)] {
    let (cached_dir, fonts) = FONT_CACHE.get_or_init(|| {
        let mut loaded: Vec<(String, Arc<Vec<u8>>)> = Vec::new();

        let entries = match WalkDir::new(fonts_dir)
            .into_iter()
            .collect::<std::result::Result<Vec<_>, _>>()
        {
            Ok(entries) => entries,
            Err(error) => {
                warn!(
                    fonts_dir = %fonts_dir.display(),
                    "Failed to read fonts directory: {error}"
                );
                return (fonts_dir.to_path_buf(), loaded);
            }
        };

        for entry in entries {
            let path = entry.path();
            if !entry.file_type().is_file() || !is_supported_font_file(path) {
                continue;
            }

            let font_bytes = match std::fs::read(path) {
                Ok(bytes) => bytes,
                Err(error) => {
                    warn!(
                        font_path = %path.display(),
                        "Failed to read font file: {error}"
                    );
                    continue;
                }
            };

            let font_bytes = Arc::new(font_bytes);
            let fonts_from_file: Vec<Font> =
                Font::iter(Bytes::new((*font_bytes).clone())).collect();

            if fonts_from_file.is_empty() {
                warn!(
                    font_path = %path.display(),
                    "Font file did not contain any readable font faces"
                );
                continue;
            }

            for font in &fonts_from_file {
                let info = font.info();
                let name = css_font_name(&info.family, &info.variant);
                loaded.push((name, Arc::clone(&font_bytes)));
            }
        }

        (fonts_dir.to_path_buf(), loaded)
    });
    if cached_dir != fonts_dir {
        warn!(
            cached = %cached_dir.display(),
            requested = %fonts_dir.display(),
            "Font cache was initialised from a different directory; reusing cached fonts"
        );
    }
    fonts
}

/// Returns whether `path` has a supported font extension (`ttf`, `otf`, or `ttc`).
fn is_supported_font_file(path: &Path) -> bool {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some(ext) => {
            ext.eq_ignore_ascii_case("ttf")
                || ext.eq_ignore_ascii_case("otf")
                || ext.eq_ignore_ascii_case("ttc")
        }
        None => false,
    }
}

/// Builds a pre-configured [`HtmlConverter`] with fonts discovered from `fonts_dir`.
///
/// Font data is cached in a process-wide [`OnceLock`] so that repeated calls
/// (e.g. in tests) avoid redundant file I/O. The converter itself is constructed fresh
/// each call with the given `base_path`, but the expensive disk reads happen at most once.
///
/// Font files that cannot be read are skipped and logged as warnings (on first load only).
///
/// Returns a tuple of `(converter, count)` where `count` is the number of
/// fonts successfully loaded.
#[must_use]
pub fn build_html_converter(fonts_dir: &Path, base_path: &Path) -> (HtmlConverter, usize) {
    let fonts = discover_fonts(fonts_dir);
    let mut converter = HtmlConverter::new().base_path(base_path);

    for (name, font_bytes) in fonts {
        converter = converter.add_font(name, (**font_bytes).clone());
    }

    (converter, fonts.len())
}

const MAX_IMAGE_DIMENSION_PIXELS: u32 = 8_192;
const MAX_IMAGE_PIXELS: u64 = 25_000_000;

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
pub fn typst_to_pdf(req: CompileRequest<'_>) -> Result<Vec<u8>> {
    typst_to_pdf_with_resources(req, &HashMap::new())
}

/// Compiles a Typst template with JSON data and approved external virtual files.
///
/// External files are supplied by the server rather than the template, so Typst
/// compilation remains unable to initiate network requests.
pub fn typst_to_pdf_with_resources(
    req: CompileRequest<'_>,
    external_resources: &HashMap<String, Bytes>,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(req.json_data).context("Failed to serialize JSON data")?;
    let data_path = format!("/data/{}/{}.json", req.app_name, req.template_name);
    let mut vfiles = external_resources.clone();
    vfiles.insert(data_path, Bytes::new(json_bytes));

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
    result
}

/// Converts an HTML document into PDF bytes using a pre-built converter.
#[must_use = "this returns a Result that should be handled"]
pub fn html_to_pdf(html: &str, converter: &HtmlConverter) -> Result<Vec<u8>> {
    converter
        .convert(html)
        .context("Failed to convert HTML to PDF")
}

/// Converts a PNG, JPEG, WebP, or SVG image into PDF bytes.
///
/// Landscape images (width > height) are automatically placed on a
/// landscape-oriented page so they fill the page without distortion.
pub fn image_to_pdf<B>(
    image_bytes: B,
    image_path: &str,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    library: Arc<LazyHash<Library>>,
    comemo_eviction_threshold: usize,
) -> Result<Vec<u8>>
where
    B: AsRef<[u8]> + Send + Sync + 'static,
{
    let data = image_bytes.as_ref();
    let declared_ext = image_path.rsplit('.').next().unwrap_or("");
    let detected_ext = detect_image_format(data).ok_or_else(|| {
        anyhow::anyhow!(
            "Unsupported or corrupted image '{image_path}': unable to determine dimensions"
        )
    })?;
    if detected_ext != declared_ext {
        return Err(anyhow::anyhow!(
            "Image format mismatch for '{image_path}': declared '{declared_ext}' but bytes are '{detected_ext}'"
        ));
    }
    let (w, h) = image_dimensions_by_format(data, detected_ext).with_context(|| {
        format!("Unsupported or corrupted image '{image_path}': unable to determine dimensions")
    })?;
    validate_image_dimensions(w, h, image_path)?;
    let is_landscape = w > h;

    let mut vfiles = HashMap::new();
    vfiles.insert(image_path.to_string(), Bytes::new(image_bytes));

    let flipped = if is_landscape { "flipped: true, " } else { "" };
    let source = format!(
        r#"#set document(title: "Image", date: auto)
#set page({flipped}margin: 0pt)
#image("{image_path}", width: 100%, alt: "Uploaded image")
"#
    );

    let result = typst_world::compile_to_pdf(
        fonts,
        root,
        resources_dir,
        "/main.typ",
        &source,
        vfiles,
        library,
    );
    comemo::evict(comemo_eviction_threshold);
    counter!("comemo_evictions_total", &[("output", "image")]).increment(1);
    result
}

fn validate_image_dimensions(width: u32, height: u32, image_path: &str) -> Result<()> {
    if width == 0 || height == 0 {
        anyhow::bail!("Image '{image_path}' has invalid dimensions: {width}x{height}");
    }
    if width > MAX_IMAGE_DIMENSION_PIXELS || height > MAX_IMAGE_DIMENSION_PIXELS {
        anyhow::bail!(
            "Image '{image_path}' dimensions exceed the maximum of {MAX_IMAGE_DIMENSION_PIXELS} pixels: {width}x{height}"
        );
    }

    let pixels = u64::from(width) * u64::from(height);
    if pixels > MAX_IMAGE_PIXELS {
        anyhow::bail!(
            "Image '{image_path}' has {pixels} pixels, exceeding the maximum of {MAX_IMAGE_PIXELS}"
        );
    }
    Ok(())
}

/// Detects the image format from magic bytes and returns its canonical file extension.
///
/// Returns `None` if the format is unrecognised.
fn detect_image_format(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("png")
    } else if data.starts_with(&[0xFF, 0xD8]) {
        Some("jpg")
    } else if data.starts_with(b"RIFF") && data.len() >= 30 && &data[8..12] == b"WEBP" {
        Some("webp")
    } else if is_svg(data) {
        Some("svg")
    } else {
        None
    }
}

/// Extracts (width, height) from PNG, JPEG, WebP, or SVG image bytes by parsing headers.
///
/// Returns `None` if the format is unrecognised or the header is too short.
#[cfg(test)]
fn image_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let fmt = detect_image_format(data)?;
    image_dimensions_by_format(data, fmt)
}

/// Extracts (width, height) for a known image format without re-detecting the format.
///
/// `fmt` must be one of `"png"`, `"jpg"`, `"webp"`, or `"svg"` as returned by
/// [`detect_image_format`]. Returns `None` if the header is too short or malformed.
fn image_dimensions_by_format(data: &[u8], fmt: &str) -> Option<(u32, u32)> {
    match fmt {
        "png" => png_dimensions(data),
        "jpg" => jpeg_dimensions(data),
        "webp" => webp_dimensions(data),
        "svg" => svg_dimensions(data),
        _ => None,
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
        if matches!(marker, 0xC0..=0xC3 | 0xC9..=0xCB) {
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

fn webp_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    if data.len() < 30 {
        return None;
    }
    // VP8 lossy format
    if &data[12..16] == b"VP8 " {
        // VP8 bitstream header starts at offset 20 (after the chunk header)
        // Frame tag at bytes 23..26 contains width/height
        let width = u32::from(u16::from_le_bytes([data[26], data[27]])) & 0x3FFF;
        let height = u32::from(u16::from_le_bytes([data[28], data[29]])) & 0x3FFF;
        return Some((width, height));
    }
    // VP8L lossless format
    if &data[12..16] == b"VP8L" && data.len() >= 25 && data[20] == 0x2F {
        // Signature byte at offset 20 verified; width/height packed in next 4 bytes
        let b0 = u32::from(data[21]);
        let b1 = u32::from(data[22]);
        let b2 = u32::from(data[23]);
        let b3 = u32::from(data[24]);
        let width = (b0 | (b1 << 8)) & 0x3FFF;
        let height = ((b1 >> 6) | (b2 << 2) | (b3 << 10)) & 0x3FFF;
        return Some((width + 1, height + 1));
    }
    // VP8X extended format
    if &data[12..16] == b"VP8X" && data.len() >= 30 {
        let width = u32::from(data[24]) | (u32::from(data[25]) << 8) | (u32::from(data[26]) << 16);
        let height = u32::from(data[27]) | (u32::from(data[28]) << 8) | (u32::from(data[29]) << 16);
        return Some((width + 1, height + 1));
    }
    None
}

/// Checks whether bytes start with an SVG document (XML declaration or `<svg` tag).
fn is_svg(data: &[u8]) -> bool {
    let trimmed = trim_leading_whitespace(data);
    trimmed.starts_with(b"<?xml") || trimmed.starts_with(b"<svg")
}

/// Extracts width and height from an SVG root element.
///
/// Parses the `viewBox`, `width`, and `height` attributes. If `width`/`height`
/// are present as unitless numbers or pixel values they take priority; otherwise
/// falls back to the viewBox dimensions.
fn svg_dimensions(data: &[u8]) -> Option<(u32, u32)> {
    let text = std::str::from_utf8(data).ok()?;
    let svg_start = text.find("<svg")?;
    let svg_tag_end = text[svg_start..].find('>')? + svg_start;
    let svg_tag = &text[svg_start..=svg_tag_end];

    let width_attr = extract_svg_attr(svg_tag, "width");
    let height_attr = extract_svg_attr(svg_tag, "height");

    // Try width/height attributes first (only unitless or px values)
    if let (Some(w), Some(h)) = (
        width_attr.and_then(parse_svg_length),
        height_attr.and_then(parse_svg_length),
    ) {
        return Some((w, h));
    }

    // Fall back to viewBox (handles both space-separated and comma-separated values)
    if let Some(vb) = extract_svg_attr(svg_tag, "viewBox") {
        let parts: Vec<&str> = vb
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|s| !s.is_empty())
            .collect();
        if parts.len() == 4 {
            let w: f64 = parts[2].parse().ok()?;
            let h: f64 = parts[3].parse().ok()?;
            if w > 0.0 && h > 0.0 {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                return Some((w as u32, h as u32));
            }
        }
    }

    None
}

/// Extracts the value of an attribute from an SVG/XML tag string.
fn extract_svg_attr<'a>(tag: &'a str, attr_name: &str) -> Option<&'a str> {
    // Match attr_name followed by = and a quoted value
    let search_len = attr_name.len() + 1; // attr_name + '='
    let mut start = 0;
    let pos = loop {
        let rel = tag[start..].find(attr_name)?;
        let abs = start + rel;
        // Verify word boundary: the character before the match must be whitespace or '<'
        let prev = tag[..abs].chars().next_back();
        if prev.is_none_or(|c| c.is_ascii_whitespace() || c == '<') {
            // Verify the character immediately after attr_name is '='
            if tag.as_bytes().get(abs + attr_name.len()) == Some(&b'=') {
                break abs;
            }
            // Boundary passes but '=' fails: skip past the full attr_name to avoid
            // redundant re-scanning of the same occurrence.
            start = abs + attr_name.len() + 1;
        } else {
            // Boundary fails: advance by 1 to search for the next candidate.
            start = abs + 1;
        }
    };
    let after_eq = &tag[pos + search_len..];
    let quote = after_eq.as_bytes().first()?;
    if *quote != b'"' && *quote != b'\'' {
        return None;
    }
    let value_start = 1;
    let value_end = after_eq[value_start..].find(*quote as char)? + value_start;
    Some(&after_eq[value_start..value_end])
}

/// Parses a unitless or pixel SVG length value to an u32.
fn parse_svg_length(value: &str) -> Option<u32> {
    let trimmed = value.trim();
    let numeric = if let Some(stripped) = trimmed.strip_suffix("px") {
        stripped.trim()
    } else if trimmed.ends_with(|c: char| c.is_alphabetic() || c == '%') {
        // Other units (em, pt, cm, etc.) - can't reliably convert to pixels
        return None;
    } else {
        trimmed
    };
    let f: f64 = numeric.parse().ok()?;
    if f > 0.0 {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        Some(f as u32)
    } else {
        None
    }
}

/// Trims leading ASCII whitespace bytes.
fn trim_leading_whitespace(data: &[u8]) -> &[u8] {
    let start = data
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(data.len());
    &data[start..]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_world::{build_library, load_fonts};
    use std::path::PathBuf;
    use std::sync::Arc;
    use typst::Features;

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

    fn pdf_library() -> Arc<LazyHash<Library>> {
        Arc::new(build_library(Features::default()))
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
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
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
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_for_unsupported_image_type() -> Result<()> {
        let result = image_to_pdf(
            b"not a valid image".to_vec(),
            "/image.png",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        assert!(
            result.as_ref().err().is_some(),
            "Expected an error for unsupported image type"
        );
        if let Err(err) = result {
            assert!(
                err.to_string().contains(
                    "Unsupported or corrupted image '/image.png': unable to determine dimensions"
                ),
                "Unexpected error: {err:#}"
            );
        }
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
    fn validate_image_dimensions_accepts_limits() -> Result<()> {
        validate_image_dimensions(8_192, 3_051, "/image.png")
    }

    #[test]
    fn validate_image_dimensions_rejects_zero_dimensions() -> Result<()> {
        let error = validate_image_dimensions(0, 1, "/image.png").err();
        let error = error.context("zero dimensions should be rejected")?;
        assert!(error.to_string().contains("invalid dimensions"));
        Ok(())
    }

    #[test]
    fn validate_image_dimensions_rejects_excessive_dimension() -> Result<()> {
        let error = validate_image_dimensions(8_193, 1, "/image.png").err();
        let error = error.context("excessive dimensions should be rejected")?;
        assert!(error.to_string().contains("dimensions exceed"));
        Ok(())
    }

    #[test]
    fn validate_image_dimensions_rejects_excessive_pixel_count() -> Result<()> {
        let error = validate_image_dimensions(8_192, 3_052, "/image.png").err();
        let error = error.context("excessive pixel count should be rejected")?;
        assert!(error.to_string().contains("pixels, exceeding"));
        Ok(())
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
    fn jpeg_dimensions_parses_sof9_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xC9];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&480u16.to_be_bytes());
        data.extend_from_slice(&640u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((640, 480)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof10_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xCA];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&100u16.to_be_bytes());
        data.extend_from_slice(&200u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((200, 100)));
    }

    #[test]
    fn jpeg_dimensions_parses_sof11_marker() {
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xCB];
        data.extend_from_slice(&[0x00, 0x11]);
        data.push(0x08);
        data.extend_from_slice(&300u16.to_be_bytes());
        data.extend_from_slice(&400u16.to_be_bytes());
        assert_eq!(image_dimensions(&data), Some((400, 300)));
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

    // --- WebP dimension tests ---

    #[test]
    fn webp_vp8_lossy_dimensions() {
        // Minimal VP8 lossy WebP: RIFF header + VP8 chunk
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&100u32.to_le_bytes()); // file size (not validated)
        data.extend_from_slice(b"WEBP");
        data.extend_from_slice(b"VP8 ");
        data.extend_from_slice(&18u32.to_le_bytes()); // chunk size
        // VP8 bitstream: 3-byte frame tag
        data.extend_from_slice(&[0x00, 0x00, 0x00]); // frame tag (offset 20-22)
        // 3-byte sync code
        data.extend_from_slice(&[0x9D, 0x01, 0x2A]); // sync code (offset 23-25)
        // width and height (little-endian 14 bits each)
        data.extend_from_slice(&320u16.to_le_bytes()); // width=320 (offset 26-27)
        data.extend_from_slice(&240u16.to_le_bytes()); // height=240 (offset 28-29)
        assert_eq!(image_dimensions(&data), Some((320, 240)));
    }

    #[test]
    fn webp_vp8x_extended_dimensions() {
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(b"WEBP");
        data.extend_from_slice(b"VP8X");
        data.extend_from_slice(&10u32.to_le_bytes()); // chunk size
        data.extend_from_slice(&[0u8; 4]); // flags (offset 20..24)
        // Canvas width - 1 (3 bytes LE): 799 = width 800
        let w_minus_1: u32 = 800 - 1;
        data.push((w_minus_1 & 0xFF) as u8);
        data.push(((w_minus_1 >> 8) & 0xFF) as u8);
        data.push(((w_minus_1 >> 16) & 0xFF) as u8);
        // Canvas height - 1 (3 bytes LE): 599 = height 600
        let h_minus_1: u32 = 600 - 1;
        data.push((h_minus_1 & 0xFF) as u8);
        data.push(((h_minus_1 >> 8) & 0xFF) as u8);
        data.push(((h_minus_1 >> 16) & 0xFF) as u8);
        assert_eq!(image_dimensions(&data), Some((800, 600)));
    }

    #[test]
    fn webp_returns_none_for_truncated_data() {
        let data = b"RIFF\x00\x00\x00\x00WEBP";
        assert_eq!(image_dimensions(data), None);
    }

    // --- SVG dimension tests ---

    #[test]
    fn svg_dimensions_from_width_height_attrs() {
        let svg = br#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((800, 600)));
    }

    #[test]
    fn svg_dimensions_from_viewbox() {
        let svg = br#"<svg viewBox="0 0 1024 768" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((1024, 768)));
    }

    #[test]
    fn svg_dimensions_with_px_suffix() {
        let svg = br#"<svg width="400px" height="300px" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((400, 300)));
    }

    #[test]
    fn svg_dimensions_with_xml_declaration() {
        let svg = br#"<?xml version="1.0" encoding="UTF-8"?><svg width="200" height="100"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((200, 100)));
    }

    #[test]
    fn svg_dimensions_with_leading_whitespace() {
        let svg = b"  \n  <svg width=\"50\" height=\"75\"></svg>";
        assert_eq!(image_dimensions(svg), Some((50, 75)));
    }

    #[test]
    fn svg_dimensions_returns_none_for_em_units() {
        let svg = br#"<svg width="10em" height="10em"></svg>"#;
        assert_eq!(image_dimensions(svg), None);
    }

    #[test]
    fn svg_dimensions_falls_back_to_viewbox_when_units_present() {
        let svg =
            br#"<svg width="10cm" height="5cm" viewBox="0 0 300 150" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((300, 150)));
    }

    #[test]
    fn svg_landscape_detected() {
        let svg = br#"<svg width="1920" height="1080"></svg>"#;
        assert!(
            image_dimensions(svg).is_some_and(|(w, h)| w > h),
            "Wide SVG should be landscape"
        );
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

    #[test]
    fn typst_to_pdf_with_external_resource_returns_pdf_bytes() -> Result<()> {
        let data = serde_json::json!({});
        let external_resources = HashMap::from([(
            "/external/logo.svg".to_string(),
            Bytes::new(
                br#"<svg width="10" height="10" xmlns="http://www.w3.org/2000/svg"><rect width="10" height="10"/></svg>"#
                    .to_vec(),
            ),
        )]);
        let bytes = typst_to_pdf_with_resources(
            CompileRequest {
                template_source: r#"#set document(title: "External resource")
#image("/external/logo.svg", alt: "Logo")"#,
                json_data: &data,
                fonts: Arc::new(load_fonts(&fonts_dir())?),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "test",
                template_name: "external-resource",
                library: pdf_library(),
                comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            },
            &external_resources,
        )?;

        assert!(bytes.starts_with(b"%PDF-"));
        Ok(())
    }

    // --- SVG-to-PDF integration test ---

    #[test]
    fn image_to_pdf_svg_returns_pdf_bytes() -> Result<()> {
        let image_bytes = std::fs::read(root_dir().join("resources").join("pdfgenrs-logo.svg"))?;
        let bytes = image_to_pdf(
            image_bytes,
            "/image.svg",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        )?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    // --- SVG dimension edge cases ---

    #[test]
    fn svg_dimensions_with_comma_separated_viewbox() {
        let svg = br#"<svg viewBox="0,0,640,480" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((640, 480)));
    }

    #[test]
    fn svg_dimensions_returns_none_for_empty_svg_tag() {
        let svg = br#"<svg></svg>"#;
        assert_eq!(image_dimensions(svg), None);
    }

    #[test]
    fn svg_dimensions_with_single_quotes() {
        let svg = b"<svg width='120' height='80'></svg>";
        assert_eq!(image_dimensions(svg), Some((120, 80)));
    }

    #[test]
    fn svg_dimensions_no_partial_attr_name_match() {
        // stroke-width= must not be matched when searching for width=
        let svg =
            br#"<svg stroke-width="5" viewBox="0 0 200 100" xmlns="http://www.w3.org/2000/svg"></svg>"#;
        assert_eq!(image_dimensions(svg), Some((200, 100)));
    }

    // --- Corrupted/truncated image tests for image_to_pdf ---

    #[test]
    fn image_to_pdf_returns_error_for_truncated_png_with_valid_magic() -> Result<()> {
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]);
        // Only 16 bytes of IHDR data — not enough for width+height (needs 24 total)
        let result = image_to_pdf(
            data,
            "/image.png",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        assert!(
            result.is_err(),
            "Truncated PNG with valid magic should fail"
        );
        let err_msg = result
            .as_ref()
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        assert!(
            err_msg.contains("Unsupported or corrupted image"),
            "Error should mention corrupted image: {err_msg}"
        );
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_for_truncated_jpeg_with_valid_soi() -> Result<()> {
        // Valid SOI + SOF0 marker but truncated before dimensions
        let data = vec![0xFF, 0xD8, 0xFF, 0xC0, 0x00, 0x11, 0x08];
        let result = image_to_pdf(
            data,
            "/image.jpg",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        assert!(result.is_err(), "Truncated JPEG with valid SOI should fail");
        let err_msg = result
            .as_ref()
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        assert!(
            err_msg.contains("Unsupported or corrupted image"),
            "Error should mention corrupted image: {err_msg}"
        );
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_for_truncated_webp_with_valid_riff_header() -> Result<()> {
        // Valid RIFF + WEBP header but no VP8 chunk data
        let mut data = Vec::new();
        data.extend_from_slice(b"RIFF");
        data.extend_from_slice(&100u32.to_le_bytes());
        data.extend_from_slice(b"WEBP");
        // Only 12 bytes — no VP8 chunk follows
        let result = image_to_pdf(
            data,
            "/image.webp",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        assert!(
            result.is_err(),
            "Truncated WebP with valid RIFF header should fail"
        );
        let err_msg = result
            .as_ref()
            .err()
            .map(|e| e.to_string())
            .unwrap_or_default();
        assert!(
            err_msg.contains("Unsupported or corrupted image"),
            "Error should mention corrupted image: {err_msg}"
        );
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_for_png_with_valid_dimensions_but_corrupted_body() -> Result<()> {
        // Valid PNG header with dimensions but no actual image data
        let mut data = b"\x89PNG\r\n\x1a\n".to_vec();
        data.extend_from_slice(&[0u8; 8]); // IHDR chunk length/type placeholder
        data.extend_from_slice(&100u32.to_be_bytes()); // width
        data.extend_from_slice(&200u32.to_be_bytes()); // height
        // Dimensions are parseable but the image data is garbage — Typst should fail to render
        let result = image_to_pdf(
            data,
            "/image.png",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        assert!(
            result.is_err(),
            "PNG with valid header but corrupted body should fail during compilation"
        );
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_when_png_bytes_sent_with_jpeg_path() -> Result<()> {
        let image_bytes = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        let result = image_to_pdf(
            image_bytes,
            "/image.jpg",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        match result {
            Ok(_) => anyhow::bail!("PNG bytes with JPEG path should have failed"),
            Err(e) => {
                let err_msg = e.to_string();
                assert!(
                    err_msg.contains("format mismatch"),
                    "Error should mention format mismatch: {err_msg}"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn image_to_pdf_returns_error_when_jpeg_bytes_sent_with_png_path() -> Result<()> {
        let image_bytes = std::fs::read(root_dir().join("resources").join("NAVLogoRed.jpg"))?;
        let result = image_to_pdf(
            image_bytes,
            "/image.png",
            test_fonts()?,
            &root_dir(),
            &resources_dir(),
            pdf_library(),
            crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        );
        match result {
            Ok(_) => anyhow::bail!("JPEG bytes with PNG path should have failed"),
            Err(e) => {
                let err_msg = e.to_string();
                assert!(
                    err_msg.contains("format mismatch"),
                    "Error should mention format mismatch: {err_msg}"
                );
            }
        }
        Ok(())
    }

    #[test]
    fn detect_image_format_returns_correct_format_for_each_type() -> Result<()> {
        let png = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        assert_eq!(detect_image_format(&png), Some("png"));

        let jpg = std::fs::read(root_dir().join("resources").join("NAVLogoRed.jpg"))?;
        assert_eq!(detect_image_format(&jpg), Some("jpg"));

        let webp = std::fs::read(root_dir().join("resources").join("test.webp"))?;
        assert_eq!(detect_image_format(&webp), Some("webp"));

        let svg = std::fs::read(root_dir().join("resources").join("pdfgenrs-logo.svg"))?;
        assert_eq!(detect_image_format(&svg), Some("svg"));

        assert_eq!(detect_image_format(b"not an image"), None);
        Ok(())
    }

    // --- css_font_name tests ---

    #[test]
    fn css_font_name_regular_weight_normal_style_returns_family_only() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::REGULAR,
            style: FontStyle::Normal,
            ..Default::default()
        };
        assert_eq!(css_font_name("Source Sans 3", &variant), "Source Sans 3");
    }

    #[test]
    fn css_font_name_bold_weight_returns_family_with_bold() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::BOLD,
            style: FontStyle::Normal,
            ..Default::default()
        };
        assert_eq!(
            css_font_name("Source Sans 3", &variant),
            "Source Sans 3 bold"
        );
    }

    #[test]
    fn css_font_name_italic_style_returns_family_with_italic() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::REGULAR,
            style: FontStyle::Italic,
            ..Default::default()
        };
        assert_eq!(
            css_font_name("Source Sans 3", &variant),
            "Source Sans 3 italic"
        );
    }

    #[test]
    fn css_font_name_bold_italic_returns_family_with_both() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::BOLD,
            style: FontStyle::Italic,
            ..Default::default()
        };
        assert_eq!(
            css_font_name("Source Sans 3", &variant),
            "Source Sans 3 bold italic"
        );
    }

    #[test]
    fn css_font_name_light_weight() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::LIGHT,
            style: FontStyle::Normal,
            ..Default::default()
        };
        assert_eq!(
            css_font_name("Source Sans 3", &variant),
            "Source Sans 3 light"
        );
    }

    #[test]
    fn css_font_name_semibold_oblique() {
        use typst_library::text::{FontStyle, FontVariant, FontWeight};
        let variant = FontVariant {
            weight: FontWeight::SEMIBOLD,
            style: FontStyle::Oblique,
            ..Default::default()
        };
        assert_eq!(
            css_font_name("MyFont", &variant),
            "MyFont semi bold oblique"
        );
    }

    // --- is_supported_font_file tests ---

    #[test]
    fn is_supported_font_file_accepts_ttf() {
        assert!(is_supported_font_file(Path::new("font.ttf")));
        assert!(is_supported_font_file(Path::new("font.TTF")));
    }

    #[test]
    fn is_supported_font_file_accepts_otf() {
        assert!(is_supported_font_file(Path::new("font.otf")));
        assert!(is_supported_font_file(Path::new("font.OTF")));
    }

    #[test]
    fn is_supported_font_file_accepts_ttc() {
        assert!(is_supported_font_file(Path::new("font.ttc")));
        assert!(is_supported_font_file(Path::new("font.TTC")));
    }

    #[test]
    fn is_supported_font_file_rejects_unsupported_extensions() {
        assert!(!is_supported_font_file(Path::new("font.woff")));
        assert!(!is_supported_font_file(Path::new("font.woff2")));
        assert!(!is_supported_font_file(Path::new("font.svg")));
        assert!(!is_supported_font_file(Path::new("readme.txt")));
    }

    #[test]
    fn is_supported_font_file_rejects_no_extension() {
        assert!(!is_supported_font_file(Path::new("font")));
        assert!(!is_supported_font_file(Path::new(".")));
    }
}
