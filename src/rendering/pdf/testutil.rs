use std::path::PathBuf;
use std::sync::Arc;
use typst::utils::LazyHash;
use typst::{Features, Library};

use crate::typst_world::{Fonts, build_library, load_fonts};
use anyhow::Result;

pub(super) fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub(super) fn fonts_dir() -> PathBuf {
    root_dir().join("fonts")
}

pub(super) fn resources_dir() -> PathBuf {
    root_dir().join("resources")
}

pub(super) fn test_fonts() -> Result<Arc<Fonts>> {
    Ok(Arc::new(load_fonts(&fonts_dir())?))
}

pub(super) fn pdf_library() -> Arc<LazyHash<Library>> {
    Arc::new(build_library(Features::default()))
}

pub(super) fn is_pdf(bytes: &[u8]) -> bool {
    bytes.starts_with(b"%PDF")
}
