mod html;
mod image;
mod image_header;
mod image_validation;
#[cfg(test)]
mod testutil;
mod typst;

pub use html::{build_html_converter, html_to_pdf};
pub(crate) use image::{ValidatedImageRenderRequest, image_to_pdf_with_validated_dimensions};
pub use image::{image_to_pdf, image_to_pdf_with_limits};
pub use image_validation::{ImageRejection, validate_image};
pub use typst::{CompileRequest, typst_to_pdf};

use crate::typst_world;

/// Errors returned by PDF rendering helpers.
#[derive(Debug)]
pub enum PdfRenderError {
    JsonSerialization {
        source: serde_json::Error,
    },
    TypstWorld {
        source: typst_world::TypstWorldError,
    },
    HtmlToPdf {
        source: anyhow::Error,
    },
    InvalidImage {
        source: ImageRejection,
    },
}

impl std::fmt::Display for PdfRenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::JsonSerialization { .. } => write!(f, "Failed to serialize JSON data"),
            Self::TypstWorld { source } => write!(f, "{source}"),
            Self::HtmlToPdf { source } => write!(f, "{source}"),
            Self::InvalidImage { source } => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for PdfRenderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::JsonSerialization { source } => Some(source),
            Self::TypstWorld { source } => Some(source),
            Self::HtmlToPdf { source } => Some(source.as_ref()),
            Self::InvalidImage { source } => Some(source),
        }
    }
}

impl From<typst_world::TypstWorldError> for PdfRenderError {
    fn from(source: typst_world::TypstWorldError) -> Self {
        Self::TypstWorld { source }
    }
}
