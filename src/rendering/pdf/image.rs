use metrics::counter;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use typst::Library;
use typst::foundations::Bytes;
use typst::utils::LazyHash;

use super::{PdfRenderError, validate_image};
use crate::typst_world::{self, Fonts};

/// Converts a PNG, JPEG, WebP, or SVG image into PDF bytes.
///
/// Landscape images (width > height) are automatically placed on a
/// landscape-oriented page. The image is centred and scaled down to fit the page
/// while preserving its aspect ratio; images smaller than the page keep their
/// natural size and are never scaled up.
pub fn image_to_pdf<B>(
    image_bytes: B,
    image_path: &str,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    library: Arc<LazyHash<Library>>,
    comemo_eviction_threshold: usize,
) -> Result<Vec<u8>, PdfRenderError>
where
    B: AsRef<[u8]> + Send + Sync + 'static,
{
    image_to_pdf_with_limits(
        image_bytes,
        image_path,
        fonts,
        root,
        resources_dir,
        library,
        comemo_eviction_threshold,
        crate::config::DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
        crate::config::DEFAULT_MAX_IMAGE_PIXELS,
    )
}

pub(crate) struct ValidatedImageRenderRequest<'a> {
    pub image_path: &'a str,
    pub width: u32,
    pub height: u32,
    pub fonts: Arc<Fonts>,
    pub root: &'a Path,
    pub resources_dir: &'a Path,
    pub library: Arc<LazyHash<Library>>,
    pub comemo_eviction_threshold: usize,
}

pub(crate) fn image_to_pdf_with_validated_dimensions<B>(
    image_bytes: B,
    req: ValidatedImageRenderRequest<'_>,
) -> Result<Vec<u8>, PdfRenderError>
where
    B: AsRef<[u8]> + Send + Sync + 'static,
{
    let mut vfiles = HashMap::new();
    vfiles.insert(req.image_path.to_string(), Bytes::new(image_bytes));

    let source = image_typst_source(req.image_path, req.width, req.height);

    let result = typst_world::compile_to_pdf(
        req.fonts,
        req.root,
        req.resources_dir,
        "/main.typ",
        &source,
        vfiles,
        req.library,
    );
    comemo::evict(req.comemo_eviction_threshold);
    counter!("comemo_evictions_total", &[("output", "image")]).increment(1);
    result.map_err(PdfRenderError::from)
}

/// Converts an image into PDF bytes using configured image dimension limits.
///
/// Validation is repeated here even when the caller has already run
/// [`validate_image`]. The check only reads the image header, so the cost is
/// negligible, and keeping it guarantees the limits cannot be bypassed by a future
/// call site that forgets to validate first.
#[allow(clippy::too_many_arguments)]
pub fn image_to_pdf_with_limits<B>(
    image_bytes: B,
    image_path: &str,
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    library: Arc<LazyHash<Library>>,
    comemo_eviction_threshold: usize,
    max_image_dimension_pixels: u32,
    max_image_pixels: u64,
) -> Result<Vec<u8>, PdfRenderError>
where
    B: AsRef<[u8]> + Send + Sync + 'static,
{
    let (width, height) = validate_image(
        image_bytes.as_ref(),
        image_path,
        max_image_dimension_pixels,
        max_image_pixels,
    )
    .map_err(|source| PdfRenderError::InvalidImage { source })?;
    image_to_pdf_with_validated_dimensions(
        image_bytes,
        ValidatedImageRenderRequest {
            image_path,
            width,
            height,
            fonts,
            root,
            resources_dir,
            library,
            comemo_eviction_threshold,
        },
    )
}

/// A4 edge lengths in millimetres, matching the page size Typst uses by default.
const A4_SHORT_EDGE_MM: f64 = 210.0;
const A4_LONG_EDGE_MM: f64 = 297.0;

/// Physical size of one image pixel, treating pixels as PostScript points (72 dpi).
const MM_PER_PIXEL: f64 = 25.4 / 72.0;

/// Returns the rendered size in millimetres for an image placed on an A4 page.
///
/// The image is scaled down to fit within the page while preserving its aspect
/// ratio, and is never scaled up: an image smaller than the page keeps its natural
/// size rather than being stretched and blurred.
///
/// Computing this here rather than leaving it to Typst means the result does not
/// depend on how Typst interprets image DPI metadata, and can be tested directly.
fn image_render_size_mm(width: u32, height: u32, is_landscape: bool) -> (f64, f64) {
    let (page_width, page_height) = if is_landscape {
        (A4_LONG_EDGE_MM, A4_SHORT_EDGE_MM)
    } else {
        (A4_SHORT_EDGE_MM, A4_LONG_EDGE_MM)
    };

    let natural_width = f64::from(width) * MM_PER_PIXEL;
    let natural_height = f64::from(height) * MM_PER_PIXEL;

    let scale = (page_width / natural_width)
        .min(page_height / natural_height)
        .min(1.0);

    (natural_width * scale, natural_height * scale)
}

/// Builds the Typst source that renders a single image on its own page.
///
/// Landscape images (width > height) are placed on a landscape-oriented page so
/// they fill more of it.
fn image_typst_source(image_path: &str, width: u32, height: u32) -> String {
    let is_landscape = width > height;
    let (render_width, render_height) = image_render_size_mm(width, height, is_landscape);
    let flipped = if is_landscape { "flipped: true, " } else { "" };

    // `place` keeps the image out of the flow. A rendered size equal to the page
    // size can therefore not round up and spill onto a second page, which is what
    // an in-flow block combined with `fr` spacers would risk.
    format!(
        r#"#set document(title: "Image", date: auto)
#set page({flipped}margin: 0pt)
#place(center + horizon, image("{image_path}", width: {render_width:.4}mm, height: {render_height:.4}mm, fit: "contain", alt: "Uploaded image"))
"#
    )
}

#[cfg(test)]
mod tests {
    use super::super::image_header::image_dimensions;
    use super::super::testutil::*;
    use super::*;
    use anyhow::Result;
    use std::fs;

    #[test]
    fn image_to_pdf_png_returns_pdf_bytes() -> Result<()> {
        let image_bytes = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
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
        let image_bytes = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
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
    fn image_to_pdf_with_validated_dimensions_png_returns_pdf_bytes() -> Result<()> {
        let image_bytes = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
        let (width, height) = validate_image(
            &image_bytes,
            "/image.png",
            crate::config::DEFAULT_MAX_IMAGE_DIMENSION_PIXELS,
            crate::config::DEFAULT_MAX_IMAGE_PIXELS,
        )?;
        let bytes = image_to_pdf_with_validated_dimensions(
            image_bytes,
            ValidatedImageRenderRequest {
                image_path: "/image.png",
                width,
                height,
                fonts: test_fonts()?,
                root: &root_dir(),
                resources_dir: &resources_dir(),
                library: pdf_library(),
                comemo_eviction_threshold: crate::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            },
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

    // --- Page fitting: images must never be cropped, and never upscaled ---

    /// Both A4 edges, so a rendered size may equal but never exceed either.
    fn assert_fits_a4(width: u32, height: u32) {
        let is_landscape = width > height;
        let (w_mm, h_mm) = image_render_size_mm(width, height, is_landscape);
        let (page_w, page_h) = if is_landscape {
            (297.0, 210.0)
        } else {
            (210.0, 297.0)
        };
        assert!(
            w_mm <= page_w + 1e-6 && h_mm <= page_h + 1e-6,
            "{width}x{height} rendered to {w_mm}x{h_mm} mm, which does not fit {page_w}x{page_h} mm"
        );
    }

    /// The reported bug: tall images were cropped because width was pinned to 100%.
    #[test]
    fn tall_image_fits_within_page() {
        assert_fits_a4(1_000, 3_000);
    }

    /// The second, unreported case: landscape images narrower than the A4 ratio
    /// overflowed the flipped page.
    #[test]
    fn moderately_wide_image_fits_within_page() {
        assert_fits_a4(1_200, 1_000);
    }

    #[test]
    fn extreme_aspect_ratios_fit_within_page() {
        for (w, h) in [
            (13_583, 5_417),
            (1_000, 1_000),
            (8_000, 100),
            (100, 8_000),
            (1, 8_192),
            (8_192, 1),
        ] {
            assert_fits_a4(w, h);
        }
    }

    #[test]
    fn aspect_ratio_is_preserved_when_scaling_down() {
        let (w_mm, h_mm) = image_render_size_mm(1_000, 3_000, false);
        let source_ratio = 1_000.0 / 3_000.0;
        assert!(
            ((w_mm / h_mm) - source_ratio).abs() < 1e-9,
            "aspect ratio changed: {w_mm}x{h_mm} mm from a 1000x3000 image"
        );
    }

    #[test]
    fn image_smaller_than_page_keeps_natural_size() {
        // 300x200 px at 72 dpi is 105.8 x 70.6 mm, well inside a landscape A4.
        let (w_mm, h_mm) = image_render_size_mm(300, 200, true);
        assert!(
            (w_mm - 300.0 * MM_PER_PIXEL).abs() < 1e-9,
            "width was scaled"
        );
        assert!(
            (h_mm - 200.0 * MM_PER_PIXEL).abs() < 1e-9,
            "height was scaled"
        );
    }

    #[test]
    fn tiny_image_is_not_upscaled() {
        let (w_mm, h_mm) = image_render_size_mm(32, 32, false);
        let expected = 32.0 * MM_PER_PIXEL;
        assert!((w_mm - expected).abs() < 1e-9 && (h_mm - expected).abs() < 1e-9);
        assert!(w_mm < 12.0, "a 32 px icon should stay small, got {w_mm} mm");
    }

    /// An image exactly the size of the page must be left alone, not scaled by a
    /// factor marginally above or below 1.
    #[test]
    fn image_exactly_page_sized_is_not_scaled() {
        let width = (A4_SHORT_EDGE_MM / MM_PER_PIXEL).round() as u32;
        let height = (A4_LONG_EDGE_MM / MM_PER_PIXEL).round() as u32;
        let (w_mm, h_mm) = image_render_size_mm(width, height, false);
        assert!(w_mm <= A4_SHORT_EDGE_MM + 1e-6);
        assert!(h_mm <= A4_LONG_EDGE_MM + 1e-6);
    }

    #[test]
    fn typst_source_flips_page_for_landscape_images_only() {
        assert!(image_typst_source("/image.png", 1_200, 800).contains("flipped: true"));
        assert!(!image_typst_source("/image.png", 800, 1_200).contains("flipped: true"));
        assert!(
            !image_typst_source("/image.png", 800, 800).contains("flipped: true"),
            "square images should stay portrait"
        );
    }

    #[test]
    fn typst_source_centres_the_image_and_sets_explicit_size() {
        let source = image_typst_source("/image.png", 1_000, 3_000);
        assert!(source.contains("center + horizon"), "image is not centred");
        assert!(
            !source.contains("width: 100%"),
            "explicit millimetre sizes must replace the relative width that caused cropping"
        );
        assert!(
            source.contains("mm,"),
            "expected millimetre sizes: {source}"
        );
    }

    // --- SVG-to-PDF integration test ---

    #[test]
    fn image_to_pdf_svg_returns_pdf_bytes() -> Result<()> {
        let image_bytes = fs::read(root_dir().join("resources").join("pdfgenrs-logo.svg"))?;
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
        let image_bytes = fs::read(root_dir().join("resources").join("NAVLogoRed.png"))?;
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
        let image_bytes = fs::read(root_dir().join("resources").join("NAVLogoRed.jpg"))?;
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
}
