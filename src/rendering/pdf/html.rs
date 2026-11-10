use anyhow::Context;
use ironpress::HtmlConverter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use tracing::warn;
use typst::foundations::Bytes;
use typst_library::text::Font;
use walkdir::WalkDir;

use super::PdfRenderError;

/// Cached font data discovered from each fonts directory. The `Arc<Vec<u8>>`
/// wrapping lets multiple font faces from the same collection file share one
/// allocation.
type FontCache = HashMap<PathBuf, Arc<Vec<(String, Arc<Vec<u8>>)>>>;
static FONT_CACHE: OnceLock<Mutex<FontCache>> = OnceLock::new();

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
/// caching fonts by their canonicalized directory.
fn discover_fonts(fonts_dir: &Path) -> Arc<Vec<(String, Arc<Vec<u8>>)>> {
    let cache_key = match fonts_dir.canonicalize() {
        Ok(path) => path,
        Err(error) => {
            warn!(
                fonts_dir = %fonts_dir.display(),
                "Failed to canonicalize fonts directory: {error}"
            );
            fonts_dir.to_path_buf()
        }
    };
    let cache = FONT_CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    let mut cache = cache.lock().unwrap_or_else(|error| {
        warn!("Font cache lock was poisoned: {error}; continuing with recovered cache");
        error.into_inner()
    });

    cache
        .entry(cache_key)
        .or_insert_with(|| {
            let mut loaded: Vec<(String, Arc<Vec<u8>>)> = Vec::new();
            let entries = match WalkDir::new(fonts_dir)
                .into_iter()
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(entries) => entries,
                Err(error) => {
                    warn!(
                        fonts_dir = %fonts_dir.display(),
                        "Failed to read fonts directory: {error}"
                    );
                    return Arc::new(loaded);
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

            Arc::new(loaded)
        })
        .clone()
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
/// Font data is cached by canonicalized directory so that repeated calls with
/// the same directory avoid redundant file I/O. The converter itself is constructed
/// fresh each call with the given `base_path`.
///
/// Font files that cannot be read are skipped and logged as warnings (on first load only).
///
/// Returns a tuple of `(converter, count)` where `count` is the number of
/// fonts successfully loaded.
#[must_use]
pub fn build_html_converter(fonts_dir: &Path, base_path: &Path) -> (HtmlConverter, usize) {
    let fonts = discover_fonts(fonts_dir);
    let mut converter = HtmlConverter::new().base_path(base_path);

    for (name, font_bytes) in fonts.iter() {
        converter = converter.add_font(name, (**font_bytes).clone());
    }

    (converter, fonts.len())
}

/// Converts an HTML document into PDF bytes using a pre-built converter.
pub fn html_to_pdf(html: &str, converter: &HtmlConverter) -> Result<Vec<u8>, PdfRenderError> {
    converter
        .convert(html)
        .context("Failed to convert HTML to PDF")
        .map_err(|source| PdfRenderError::HtmlToPdf { source })
}

#[cfg(test)]
mod tests {
    use super::super::testutil::{fonts_dir, is_pdf, root_dir};
    use super::*;
    use anyhow::Result;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn html_to_pdf_simple_document_returns_pdf_bytes() -> Result<()> {
        let source = "<!DOCTYPE html><html><body><h1>Hello, world!</h1></body></html>";
        let (converter, _) = build_html_converter(&fonts_dir(), &root_dir());
        let bytes = html_to_pdf(source, &converter)?;
        assert!(is_pdf(&bytes));
        Ok(())
    }

    #[test]
    fn build_html_converter_uses_fonts_from_each_directory() -> Result<()> {
        let populated_fonts_dir = TempDir::new()?;
        let empty_fonts_dir = TempDir::new()?;
        fs::copy(
            fonts_dir().join("SourceSans3-SemiBold.ttf"),
            populated_fonts_dir.path().join("SourceSans3-SemiBold.ttf"),
        )?;

        let (_, populated_count) = build_html_converter(populated_fonts_dir.path(), &root_dir());
        let (_, empty_count) = build_html_converter(empty_fonts_dir.path(), &root_dir());
        let (_, repeated_populated_count) =
            build_html_converter(populated_fonts_dir.path(), &root_dir());

        assert!(populated_count > 0);
        assert_eq!(empty_count, 0);
        assert_eq!(repeated_populated_count, populated_count);
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
