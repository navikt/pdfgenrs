use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use typst::foundations::Bytes;
use typst::{Library, LibraryExt};
use typst::utils::LazyHash;
use typst_library::diag::{FileError, FileResult};
use typst_library::text::{Font, FontBook};
use typst_library::World;
use typst_syntax::{FileId, Source, VirtualPath};

static EMBEDDED_FONTS: &[&[u8]] = &[
    include_bytes!("../fonts/SourceSansPro-Regular.ttf"),
    include_bytes!("../fonts/SourceSansPro-Bold.ttf"),
];

/// Cached font data loaded once at startup and shared across requests.
#[derive(Clone)]
pub struct FontCache {
    pub fonts: Vec<Font>,
    pub book: FontBook,
}

/// Load embedded fonts and return a [`FontCache`] suitable for sharing across requests.
pub fn load_font_cache() -> FontCache {
    let mut fonts: Vec<Font> = Vec::new();
    for &font_data in EMBEDDED_FONTS {
        let bytes = Bytes::new(font_data);
        fonts.extend(Font::iter(bytes));
    }
    let book = FontBook::from_fonts(&fonts);
    FontCache { fonts, book }
}

/// A minimal Typst World implementation that:
/// - Provides the standard library
/// - Uses embedded fonts
/// - Serves a main `.typ` source and optional data as virtual files
pub struct PdfgenWorld {
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_id: FileId,
    main_source: Source,
    /// Virtual files accessible by ID: template files and data
    virtual_files: HashMap<FileId, Bytes>,
    /// Physical file root for resolving relative paths in templates
    root: PathBuf,
}

impl PdfgenWorld {
    /// Create a new world for rendering a Typst source string with optional
    /// auxiliary files accessible via the virtual file system.
    ///
    /// `font_cache`: pre-loaded fonts (load once at startup via [`load_font_cache`])
    /// `main_path`: virtual path of the main document (e.g. `/main.typ`)
    /// `main_source`: the Typst source code to compile
    /// `virtual_files`: additional files (e.g. `data.json`) accessible by virtual path
    pub fn new(
        font_cache: FontCache,
        root: &Path,
        main_path: &str,
        main_source: String,
        virtual_files: HashMap<String, Bytes>,
    ) -> Result<Self> {
        let FontCache { fonts, book: font_book } = font_cache;

        let main_id = FileId::new(None, VirtualPath::new(main_path));
        let source = Source::new(main_id, main_source);

        let vfiles: HashMap<FileId, Bytes> = virtual_files
            .into_iter()
            .map(|(path, bytes)| (FileId::new(None, VirtualPath::new(&path)), bytes))
            .collect();

        Ok(Self {
            library: LazyHash::new(Library::default()),
            font_book: LazyHash::new(font_book),
            fonts,
            main_id,
            main_source: source,
            virtual_files: vfiles,
            root: root.to_path_buf(),
        })
    }
}

impl World for PdfgenWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.font_book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            return Ok(self.main_source.clone());
        }
        // Check virtual files first
        if let Some(bytes) = self.virtual_files.get(&id) {
            let text = std::str::from_utf8(bytes.as_slice())
                .map_err(|_| FileError::InvalidUtf8)?
                .to_string();
            return Ok(Source::new(id, text));
        }
        // Try resolving relative to the root
        let vpath = id.vpath();
        let physical = self.root.join(vpath.as_rootless_path());
        let text = std::fs::read_to_string(&physical)
            .map_err(|e| FileError::from_io(e, &physical))?;
        Ok(Source::new(id, text))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        // Check virtual files
        if let Some(bytes) = self.virtual_files.get(&id) {
            return Ok(bytes.clone());
        }
        // Try resolving relative to the root
        let vpath = id.vpath();
        let physical = self.root.join(vpath.as_rootless_path());
        let bytes = std::fs::read(&physical)
            .map_err(|e| FileError::from_io(e, &physical))?;
        Ok(Bytes::new(bytes))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<typst_library::foundations::Datetime> {
        let now = chrono::Local::now();
        let naive = if let Some(off) = offset {
            let utc = now.with_timezone(&chrono::Utc);
            (utc + chrono::Duration::hours(off)).naive_local()
        } else {
            now.naive_local()
        };
        typst_library::foundations::Datetime::from_ymd(
            naive.year(),
            naive.month() as u8,
            naive.day() as u8,
        )
    }
}

/// Compile a Typst source document to PDF bytes.
///
/// `font_cache`: pre-loaded fonts shared across requests
/// `root`: base path for resolving template file includes
/// `main_path`: virtual path for the main document  
/// `main_source`: Typst source code
/// `virtual_files`: additional virtual files (e.g. data.json)
pub fn compile_to_pdf(
    font_cache: FontCache,
    root: &Path,
    main_path: &str,
    main_source: String,
    virtual_files: HashMap<String, Bytes>,
) -> Result<Vec<u8>> {
    let world = PdfgenWorld::new(font_cache, root, main_path, main_source, virtual_files)?;

    let result = typst::compile::<typst_library::layout::PagedDocument>(&world);

    // Evict the comemo memoization cache to prevent unbounded memory growth.
    // Each compilation populates Typst's global cache; without eviction this
    // causes a memory leak under sustained load.
    comemo::evict(30);

    let document = result
        .output
        .map_err(|errors| {
            let msgs: Vec<String> = errors
                .iter()
                .map(|e| e.message.to_string())
                .collect();
            anyhow::anyhow!("Typst compilation failed: {}", msgs.join("; "))
        })?;

    if !result.warnings.is_empty() {
        let warns: Vec<String> = result.warnings.iter().map(|w| w.message.to_string()).collect();
        log::warn!("Typst warnings: {}", warns.join("; "));
    }

    let standards = typst_pdf::PdfStandards::new(&[typst_pdf::PdfStandard::A_2a])
        .map_err(|e| anyhow::anyhow!("Failed to configure PDF/A-2a standard: {}", e))?;

    let timestamp = build_timestamp();
    let options = typst_pdf::PdfOptions {
        standards,
        timestamp,
        ..typst_pdf::PdfOptions::default()
    };
    let pdf_bytes = typst_pdf::pdf(&document, &options)
        .map_err(|errors| {
            let msgs: Vec<String> = errors.iter().map(|e| e.message.to_string()).collect();
            anyhow::anyhow!("Typst PDF export failed: {}", msgs.join("; "))
        })?;

    Ok(pdf_bytes)
}

use chrono::Datelike;
use chrono::Timelike;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    /// Verifies that `load_font_cache` loads the embedded fonts.
    #[test]
    fn font_cache_loads_embedded_fonts() {
        let cache = load_font_cache();
        assert!(
            cache.fonts.len() >= EMBEDDED_FONTS.len(),
            "Expected at least as many font faces as there are embedded font files"
        );
    }

    /// Validates the core caching pattern: a single `FontCache` loaded once at
    /// startup can be cloned and reused for multiple independent compilations,
    /// matching how `AppState::fonts` is shared across requests.
    #[test]
    fn font_cache_clone_can_be_reused_across_multiple_compilations() {
        let cache = load_font_cache();

        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
"#;

        let result1 = compile_to_pdf(
            cache.clone(),
            &root_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        );
        assert!(result1.is_ok(), "First compilation failed: {:?}", result1.err());
        let pdf1 = result1.unwrap();
        assert!(is_pdf(&pdf1), "First result is not a valid PDF");

        let result2 = compile_to_pdf(
            cache.clone(),
            &root_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        );
        assert!(result2.is_ok(), "Second compilation failed: {:?}", result2.err());
        let pdf2 = result2.unwrap();
        assert!(is_pdf(&pdf2), "Second result is not a valid PDF");
    }
}

fn build_timestamp() -> Option<typst_pdf::Timestamp> {
    let now = chrono::Utc::now();
    let datetime = typst_library::foundations::Datetime::from_ymd_hms(
        now.year(),
        now.month() as u8,
        now.day() as u8,
        now.hour() as u8,
        now.minute() as u8,
        now.second() as u8,
    )?;
    Some(typst_pdf::Timestamp::new_utc(datetime))
}
