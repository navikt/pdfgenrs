use super::image_header::{detect_image_format, image_dimensions_by_format};

/// Why an uploaded image was rejected before compilation started.
///
/// The variants separate malformed input (which maps to `400 Bad Request`) from
/// input that is well-formed but exceeds a configured limit (`413 Content Too
/// Large`). All variants describe a client error: none of them indicate a server
/// fault.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageRejection {
    /// The bytes do not match any supported image format.
    UndetectableFormat { image_path: String },
    /// The detected format contradicts the declared content type.
    FormatMismatch {
        image_path: String,
        declared: String,
        detected: String,
    },
    /// The header could not be parsed into usable dimensions.
    UnreadableDimensions { image_path: String },
    /// The image reports a zero width or height.
    ZeroDimension {
        image_path: String,
        width: u32,
        height: u32,
    },
    /// Width or height exceeds the configured maximum.
    DimensionTooLarge {
        image_path: String,
        width: u32,
        height: u32,
        max: u32,
    },
    /// The total pixel count exceeds the configured maximum.
    TooManyPixels {
        image_path: String,
        pixels: u64,
        max: u64,
    },
}

impl ImageRejection {
    /// Returns a stable, low-cardinality label identifying the rejection cause.
    ///
    /// Suitable as a metric label: the set of values is fixed and does not depend
    /// on request content.
    #[must_use]
    pub fn reason(&self) -> &'static str {
        match self {
            Self::UndetectableFormat { .. } => "undetectable_format",
            Self::FormatMismatch { .. } => "format_mismatch",
            Self::UnreadableDimensions { .. } => "unreadable_dimensions",
            Self::ZeroDimension { .. } => "zero_dimension",
            Self::DimensionTooLarge { .. } => "dimension_too_large",
            Self::TooManyPixels { .. } => "too_many_pixels",
        }
    }

    /// Returns whether the image was rejected for exceeding a size limit rather
    /// than for being malformed.
    #[must_use]
    pub fn is_too_large(&self) -> bool {
        matches!(
            self,
            Self::DimensionTooLarge { .. } | Self::TooManyPixels { .. }
        )
    }
}

impl std::fmt::Display for ImageRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UndetectableFormat { image_path } => write!(
                f,
                "Unsupported or corrupted image '{image_path}': unable to determine dimensions"
            ),
            Self::FormatMismatch {
                image_path,
                declared,
                detected,
            } => write!(
                f,
                "Image format mismatch for '{image_path}': declared '{declared}' but bytes are '{detected}'"
            ),
            Self::UnreadableDimensions { image_path } => write!(
                f,
                "Unsupported or corrupted image '{image_path}': unable to determine dimensions"
            ),
            Self::ZeroDimension {
                image_path,
                width,
                height,
            } => write!(
                f,
                "Image '{image_path}' has invalid dimensions: {width}x{height}"
            ),
            Self::DimensionTooLarge {
                image_path,
                width,
                height,
                max,
            } => write!(
                f,
                "Image '{image_path}' dimensions exceed the maximum of {max} pixels: {width}x{height}"
            ),
            Self::TooManyPixels {
                image_path,
                pixels,
                max,
            } => write!(
                f,
                "Image '{image_path}' has {pixels} pixels, exceeding the maximum of {max}"
            ),
        }
    }
}

impl std::error::Error for ImageRejection {}

/// Validates an uploaded image from its header alone and returns its dimensions.
///
/// Only the format magic bytes and the dimension fields are inspected, so this is
/// cheap enough to run before acquiring a compilation permit. No pixel data is
/// decoded and no allocation proportional to the image size is made.
///
/// # Errors
/// Returns [`ImageRejection`] describing which check failed. Every variant denotes
/// a client error.
pub fn validate_image(
    data: &[u8],
    image_path: &str,
    max_image_dimension_pixels: u32,
    max_image_pixels: u64,
) -> Result<(u32, u32), ImageRejection> {
    let declared_ext = image_path.rsplit('.').next().unwrap_or("");
    let detected_ext =
        detect_image_format(data).ok_or_else(|| ImageRejection::UndetectableFormat {
            image_path: image_path.to_string(),
        })?;
    if detected_ext != declared_ext {
        return Err(ImageRejection::FormatMismatch {
            image_path: image_path.to_string(),
            declared: declared_ext.to_string(),
            detected: detected_ext.to_string(),
        });
    }
    let (width, height) = image_dimensions_by_format(data, detected_ext).ok_or_else(|| {
        ImageRejection::UnreadableDimensions {
            image_path: image_path.to_string(),
        }
    })?;

    validate_image_dimensions(
        width,
        height,
        image_path,
        max_image_dimension_pixels,
        max_image_pixels,
    )?;

    Ok((width, height))
}

/// Applies the configured dimension and pixel-count limits to already-parsed
/// image dimensions.
fn validate_image_dimensions(
    width: u32,
    height: u32,
    image_path: &str,
    max_image_dimension_pixels: u32,
    max_image_pixels: u64,
) -> Result<(), ImageRejection> {
    if width == 0 || height == 0 {
        return Err(ImageRejection::ZeroDimension {
            image_path: image_path.to_string(),
            width,
            height,
        });
    }
    if width > max_image_dimension_pixels || height > max_image_dimension_pixels {
        return Err(ImageRejection::DimensionTooLarge {
            image_path: image_path.to_string(),
            width,
            height,
            max: max_image_dimension_pixels,
        });
    }

    let pixels = u64::from(width) * u64::from(height);
    if pixels > max_image_pixels {
        return Err(ImageRejection::TooManyPixels {
            image_path: image_path.to_string(),
            pixels,
            max: max_image_pixels,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};

    #[test]
    fn validate_image_dimensions_accepts_limits() {
        assert!(validate_image_dimensions(8_192, 3_051, "/image.png", 8_192, 25_000_000).is_ok());
    }

    #[test]
    fn validate_image_dimensions_rejects_zero_dimensions() -> Result<()> {
        let error = validate_image_dimensions(0, 1, "/image.png", 8_192, 25_000_000).err();
        let error = error.context("zero dimensions should be rejected")?;
        assert!(matches!(
            error,
            ImageRejection::ZeroDimension {
                width: 0,
                height: 1,
                ..
            }
        ));
        assert_eq!(error.reason(), "zero_dimension");
        assert!(
            !error.is_too_large(),
            "zero dimensions is a malformed image"
        );
        assert!(error.to_string().contains("invalid dimensions"));
        Ok(())
    }

    #[test]
    fn validate_image_dimensions_rejects_excessive_dimension() -> Result<()> {
        let error = validate_image_dimensions(8_193, 1, "/image.png", 8_192, 25_000_000).err();
        let error = error.context("excessive dimensions should be rejected")?;
        assert!(matches!(
            error,
            ImageRejection::DimensionTooLarge {
                width: 8_193,
                max: 8_192,
                ..
            }
        ));
        assert_eq!(error.reason(), "dimension_too_large");
        assert!(error.is_too_large());
        assert!(error.to_string().contains("dimensions exceed"));
        Ok(())
    }

    #[test]
    fn validate_image_dimensions_rejects_excessive_pixel_count() -> Result<()> {
        let error = validate_image_dimensions(8_192, 3_052, "/image.png", 8_192, 25_000_000).err();
        let error = error.context("excessive pixel count should be rejected")?;
        assert!(matches!(
            error,
            ImageRejection::TooManyPixels {
                pixels: 25_001_984,
                max: 25_000_000,
                ..
            }
        ));
        assert_eq!(error.reason(), "too_many_pixels");
        assert!(error.is_too_large());
        assert!(error.to_string().contains("pixels, exceeding"));
        Ok(())
    }

    /// The dimension guard is checked before the pixel guard, so an image that
    /// violates both is reported as a dimension failure.
    #[test]
    fn validate_image_dimensions_reports_dimension_before_pixel_count() -> Result<()> {
        let error =
            validate_image_dimensions(20_000, 20_000, "/image.png", 16_384, 64_000_000).err();
        let error = error.context("both limits are exceeded")?;
        assert_eq!(error.reason(), "dimension_too_large");
        Ok(())
    }

    #[test]
    fn validate_image_rejects_format_mismatch() -> Result<()> {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        png.extend_from_slice(&[0u8; 8]);
        png.extend_from_slice(&100u32.to_be_bytes());
        png.extend_from_slice(&100u32.to_be_bytes());

        let error = validate_image(&png, "/image.jpg", 16_384, 64_000_000).err();
        let error = error.context("PNG bytes declared as JPEG should be rejected")?;
        assert_eq!(error.reason(), "format_mismatch");
        assert!(!error.is_too_large(), "a mismatch is a malformed request");
        Ok(())
    }

    #[test]
    fn validate_image_accepts_valid_png_and_returns_dimensions() -> Result<()> {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        png.extend_from_slice(&[0u8; 8]);
        png.extend_from_slice(&640u32.to_be_bytes());
        png.extend_from_slice(&480u32.to_be_bytes());

        let (width, height) = validate_image(&png, "/image.png", 16_384, 64_000_000)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        assert_eq!((width, height), (640, 480));
        Ok(())
    }

    /// Exactly hitting the pixel limit is allowed; the guard rejects only strictly
    /// greater counts.
    #[test]
    fn validate_image_dimensions_accepts_exact_pixel_limit() {
        assert!(validate_image_dimensions(8_000, 8_000, "/image.png", 16_384, 64_000_000).is_ok());
    }
}
