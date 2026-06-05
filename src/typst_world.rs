use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// Maximum number of evictions to perform on the comemo memoization cache after
/// each compilation. This bounds memory growth while preserving frequently-used
/// cache entries to maintain a good hit rate.
const COMEMO_EVICTION_THRESHOLD: usize = 15;

use anyhow::{Context, Result};
use chrono::Datelike;
use chrono::Timelike;
use typst::foundations::Bytes;
use typst::utils::LazyHash;
use typst::{Feature, Features, Library, LibraryExt};
use typst_library::World;
use typst_library::diag::{FileError, FileResult};
use typst_library::text::{Font, FontBook};
use typst_syntax::{FileId, Source, VirtualPath};
use walkdir::WalkDir;

/// Holds the loaded fonts and the font book used by the Typst compiler.
#[derive(Clone)]
pub struct Fonts {
    /// The list of individual font faces available for rendering.
    pub fonts: Vec<Font>,
    /// The font book that indexes all available fonts for Typst's font resolver.
    pub book: LazyHash<FontBook>,
}

/// Loads fonts from the provided directory and returns a [`Fonts`] instance.
pub fn load_fonts(fonts_dir: &Path) -> Result<Fonts> {
    let font_paths = collect_font_files(fonts_dir)
        .with_context(|| format!("Failed to read font directory '{}'", fonts_dir.display()))?;

    if font_paths.is_empty() {
        return Err(anyhow::anyhow!(
            "No font files found in '{}'",
            fonts_dir.display()
        ));
    }

    let mut fonts = Vec::new();
    for font_path in font_paths {
        let font_bytes = std::fs::read(&font_path)
            .with_context(|| format!("Failed to read font file '{}'", font_path.display()))?;
        let mut parsed_fonts: Vec<Font> = Font::iter(Bytes::new(font_bytes)).collect();
        if parsed_fonts.is_empty() {
            tracing::warn!(
                path = %font_path.display(),
                "Font file did not contain any readable font faces"
            );
            continue;
        }
        fonts.append(&mut parsed_fonts);
    }

    if fonts.is_empty() {
        return Err(anyhow::anyhow!(
            "No valid font faces found in '{}'",
            fonts_dir.display()
        ));
    }

    let book = FontBook::from_fonts(&fonts);
    Ok(Fonts {
        fonts,
        book: LazyHash::new(book),
    })
}

/// Walks `dir` and collects all supported font files.
fn collect_font_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter() {
        let entry = entry.with_context(|| {
            format!("Failed to read font directory entry in '{}'", dir.display())
        })?;
        let path = entry.path();
        if entry.file_type().is_file() && is_supported_font_file(path) {
            files.push(path.to_path_buf());
        }
    }
    Ok(files)
}

/// Returns whether `path` has a supported font extension (`ttf`, `otf`, or `ttc`).
#[must_use]
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

/// A Typst [`World`] implementation used to compile templates into PDF documents.
///
/// It resolves files from a combination of in-memory virtual files (e.g. JSON
/// data injected at compile time) and the physical filesystem rooted at `root`.
pub struct PdfgenWorld {
    library: LazyHash<Library>,
    fonts: Arc<Fonts>,
    main_id: FileId,
    main_source: Source,
    virtual_files: HashMap<FileId, Bytes>,
    root: PathBuf,
    resources_dir: PathBuf,
}

impl PdfgenWorld {
    /// Creates a new `PdfgenWorld`.
    ///
    /// # Arguments
    /// - `fonts` - Shared font data to use during compilation.
    /// - `root` - Root directory for resolving physical file paths.
    /// - `main_path` - Virtual path for the main Typst source file (e.g. `"/main.typ"`).
    /// - `main_source` - Source text of the main Typst file.
    /// - `virtual_files` - Map of virtual path to byte content for in-memory files
    ///   (e.g. `"/data.json"` to JSON bytes).
    /// - `features` - In-development Typst features to enable (e.g. [`Feature::Html`]).
    pub fn new(
        fonts: Arc<Fonts>,
        root: &Path,
        resources_dir: &Path,
        main_path: &str,
        main_source: String,
        virtual_files: HashMap<String, Bytes>,
        features: Features,
    ) -> Self {
        let main_id = FileId::new(None, VirtualPath::new(main_path));
        let source = Source::new(main_id, main_source);

        let vfiles: HashMap<FileId, Bytes> = virtual_files
            .into_iter()
            .map(|(path, bytes)| (FileId::new(None, VirtualPath::new(&path)), bytes))
            .collect();

        let library = Library::builder().with_features(features).build();

        Self {
            library: LazyHash::new(library),
            fonts,
            main_id,
            main_source: source,
            virtual_files: vfiles,
            root: root.to_path_buf(),
            resources_dir: resources_dir.to_path_buf(),
        }
    }

    fn physical_path(&self, vpath: &VirtualPath) -> PathBuf {
        let rootless = vpath.as_rootless_path();
        if let Ok(resource_relative) = rootless.strip_prefix("resources") {
            self.resources_dir.join(resource_relative)
        } else {
            self.root.join(rootless)
        }
    }
}

impl World for PdfgenWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.fonts.book
    }

    fn main(&self) -> FileId {
        self.main_id
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.main_id {
            return Ok(self.main_source.clone());
        }
        if let Some(bytes) = self.virtual_files.get(&id) {
            let text = std::str::from_utf8(bytes.as_slice())
                .map_err(|_| FileError::InvalidUtf8)?
                .to_string();
            return Ok(Source::new(id, text));
        }
        let vpath = id.vpath();
        let physical = self.physical_path(vpath);
        let text =
            std::fs::read_to_string(&physical).map_err(|e| FileError::from_io(e, &physical))?;
        Ok(Source::new(id, text))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(bytes) = self.virtual_files.get(&id) {
            return Ok(bytes.clone());
        }
        let vpath = id.vpath();
        let physical = self.physical_path(vpath);
        let bytes = std::fs::read(&physical).map_err(|e| FileError::from_io(e, &physical))?;
        Ok(Bytes::new(bytes))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<typst_library::foundations::Datetime> {
        let now = chrono::Local::now();
        let naive = offset.map_or_else(
            || now.naive_local(),
            |off| {
                let utc = now.with_timezone(&chrono::Utc);
                (utc + chrono::Duration::hours(off)).naive_local()
            },
        );
        typst_library::foundations::Datetime::from_ymd(
            naive.year(),
            u8::try_from(naive.month()).ok()?,
            u8::try_from(naive.day()).ok()?,
        )
    }
}

/// Compiles a Typst source document to PDF bytes.
///
/// Virtual files (e.g. injected JSON data) are provided via `virtual_files` and
/// are resolved before falling back to the physical filesystem under `root`.
/// The resulting PDF conforms to the PDF/A-2a standard.
///
/// # Errors
/// Returns an error if Typst compilation fails or the PDF cannot be exported.
#[must_use = "this returns a Result that should be handled"]
pub fn compile_to_pdf(
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    main_path: &str,
    main_source: String,
    virtual_files: HashMap<String, Bytes>,
) -> Result<Vec<u8>> {
    let world = PdfgenWorld::new(
        fonts,
        root,
        resources_dir,
        main_path,
        main_source,
        virtual_files,
        Features::default(),
    );

    let result = typst::compile::<typst_library::layout::PagedDocument>(&world);

    comemo::evict(COMEMO_EVICTION_THRESHOLD);

    let document = result
        .output
        .map_err(|errors| format_typst_errors("compilation", &errors))?;

    log_typst_warnings(&result.warnings);

    let standards = typst_pdf::PdfStandards::new(&[typst_pdf::PdfStandard::A_2a])
        .map_err(|e| anyhow::anyhow!("Failed to configure PDF/A-2a standard: {e}"))?;

    let timestamp = build_timestamp();
    let options = typst_pdf::PdfOptions {
        standards,
        timestamp,
        ..typst_pdf::PdfOptions::default()
    };
    let pdf_bytes = typst_pdf::pdf(&document, &options)
        .map_err(|errors| format_typst_errors("PDF export", &errors))?;

    Ok(pdf_bytes)
}

/// Compiles a Typst source document to an HTML string.
///
/// Virtual files (e.g. injected JSON data) are provided via `virtual_files` and
/// are resolved before falling back to the physical filesystem under `root`.
///
/// # Errors
/// Returns an error if Typst compilation fails or the HTML cannot be exported.
pub fn compile_to_html(
    fonts: Arc<Fonts>,
    root: &Path,
    resources_dir: &Path,
    main_path: &str,
    main_source: String,
    virtual_files: HashMap<String, Bytes>,
) -> Result<String> {
    let world = PdfgenWorld::new(
        fonts,
        root,
        resources_dir,
        main_path,
        main_source,
        virtual_files,
        [Feature::Html].into_iter().collect(),
    );

    let result = typst::compile::<typst_html::HtmlDocument>(&world);

    comemo::evict(COMEMO_EVICTION_THRESHOLD);

    let document = result
        .output
        .map_err(|errors| format_typst_errors("compilation", &errors))?;

    log_typst_warnings(&result.warnings);

    typst_html::html(&document).map_err(|errors| format_typst_errors("HTML export", &errors))
}

/// Formats a slice of Typst diagnostics into a single semicolon-separated error message.
fn format_typst_errors(
    context: &str,
    errors: &[typst_library::diag::SourceDiagnostic],
) -> anyhow::Error {
    let msgs: Vec<String> = errors.iter().map(|e| e.message.to_string()).collect();
    anyhow::anyhow!("Typst {context} failed: {}", msgs.join("; "))
}

/// Logs Typst compilation warnings (if any) at the `warn` level.
fn log_typst_warnings(warnings: &[typst_library::diag::SourceDiagnostic]) {
    if !warnings.is_empty() {
        let warns: Vec<String> = warnings.iter().map(|w| w.message.to_string()).collect();
        tracing::warn!(warnings = warns.join("; "), "Typst compilation warnings");
    }
}

fn build_timestamp() -> Option<typst_pdf::Timestamp> {
    let now = chrono::Utc::now();
    let datetime = typst_library::foundations::Datetime::from_ymd_hms(
        now.year(),
        u8::try_from(now.month()).ok()?,
        u8::try_from(now.day()).ok()?,
        u8::try_from(now.hour()).ok()?,
        u8::try_from(now.minute()).ok()?,
        u8::try_from(now.second()).ok()?,
    )?;
    Some(typst_pdf::Timestamp::new_utc(datetime))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use typst_library::World;
    use typst_syntax::VirtualPath;

    fn root_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn resources_dir() -> PathBuf {
        root_dir().join("resources")
    }

    fn is_pdf(bytes: &[u8]) -> bool {
        bytes.starts_with(b"%PDF")
    }

    fn rss_kb() -> Option<u64> {
        let status = fs::read_to_string("/proc/self/status").ok()?;
        status
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|val| val.parse().ok())
    }

    #[test]
    fn fonts_load_from_directory() -> Result<()> {
        let fonts = load_fonts(&root_dir().join("fonts"))?;
        assert!(
            !fonts.fonts.is_empty(),
            "Expected at least one font face loaded from fonts directory"
        );
        Ok(())
    }

    #[test]
    fn fonts_clone_can_be_reused_across_multiple_compilations() -> Result<()> {
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);

        let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
";

        let pdf1 = compile_to_pdf(
            Arc::clone(&fonts),
            &root_dir(),
            &resources_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        )?;
        assert!(is_pdf(&pdf1), "First result is not a valid PDF");

        let pdf2 = compile_to_pdf(
            Arc::clone(&fonts),
            &root_dir(),
            &resources_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        )?;
        assert!(is_pdf(&pdf2), "Second result is not a valid PDF");

        Ok(())
    }

    #[test]
    fn compilation_succeeds_after_full_cache_eviction() -> Result<()> {
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let root = root_dir();
        let source = "#set page(margin: 1cm)\nCache eviction test.".to_string();

        comemo::evict(0);

        let pdf = compile_to_pdf(
            fonts,
            &root,
            &resources_dir(),
            "/main.typ",
            source,
            HashMap::new(),
        )?;
        assert!(
            is_pdf(&pdf),
            "Result after cache eviction is not a valid PDF"
        );
        Ok(())
    }

    #[test]
    fn load_fonts_returns_error_for_empty_directory() -> Result<()> {
        let dir = TempDir::new()?;

        let result = load_fonts(dir.path());

        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn is_supported_font_file_accepts_uppercase_extensions() {
        assert!(is_supported_font_file(Path::new("font.TTF")));
        assert!(is_supported_font_file(Path::new("font.OTF")));
        assert!(is_supported_font_file(Path::new("font.TTC")));
        assert!(!is_supported_font_file(Path::new("font.txt")));
    }

    #[test]
    fn source_returns_invalid_utf8_for_virtual_non_utf8_file() -> Result<()> {
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let world = PdfgenWorld::new(
            fonts,
            &root_dir(),
            &resources_dir(),
            "/main.typ",
            "Hello".to_string(),
            HashMap::from([("/data.json".to_string(), Bytes::new(vec![0xff, 0xfe]))]),
            Features::default(),
        );

        let file_id = FileId::new(None, VirtualPath::new("/data.json"));
        let result = world.source(file_id);

        assert!(matches!(result, Err(FileError::InvalidUtf8)));
        Ok(())
    }

    #[test]
    fn source_reads_physical_files_from_root() -> Result<()> {
        let dir = TempDir::new()?;
        fs::write(dir.path().join("snippet.typ"), "File system content")?;
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let world = PdfgenWorld::new(
            fonts,
            dir.path(),
            &resources_dir(),
            "/main.typ",
            "Main".to_string(),
            HashMap::new(),
            Features::default(),
        );

        let file_id = FileId::new(None, VirtualPath::new("/snippet.typ"));
        let source = world.source(file_id)?;

        assert_eq!(source.text(), "File system content");
        Ok(())
    }

    #[test]
    fn file_reads_resource_files_from_configured_resources_dir() -> Result<()> {
        let root = TempDir::new()?;
        let resources = TempDir::new()?;
        fs::write(resources.path().join("logo.txt"), b"resource content")?;
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let world = PdfgenWorld::new(
            fonts,
            root.path(),
            resources.path(),
            "/main.typ",
            "Main".to_string(),
            HashMap::new(),
            Features::default(),
        );

        let file_id = FileId::new(None, VirtualPath::new("/resources/logo.txt"));
        let bytes = world.file(file_id)?;

        assert_eq!(bytes.as_slice(), b"resource content");
        Ok(())
    }

    #[test]
    fn today_supports_offset_argument() -> Result<()> {
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let world = PdfgenWorld::new(
            fonts,
            &root_dir(),
            &resources_dir(),
            "/main.typ",
            "Hello".to_string(),
            HashMap::new(),
            Features::default(),
        );

        assert!(world.today(None).is_some());
        assert!(world.today(Some(2)).is_some());
        Ok(())
    }

    #[test]
    fn repeated_compilations_do_not_grow_memory_unboundedly() -> Result<()> {
        let _guard = crate::memory_sensitive_test_lock().blocking_lock();
        let fonts = Arc::new(load_fonts(&root_dir().join("fonts"))?);
        let root = root_dir();

        for i in 0..10 {
            let source = format!("#set page(margin: 1cm)\nWarmup {i}.");
            compile_to_pdf(
                Arc::clone(&fonts),
                &root,
                &resources_dir(),
                "/main.typ",
                source,
                HashMap::new(),
            )?;
        }

        let Some(rss_before) = rss_kb() else {
            return Ok(());
        };

        for i in 0..200 {
            let source = format!("#set page(margin: 1cm)\nDocument {i} with unique content.");
            let result = compile_to_pdf(
                Arc::clone(&fonts),
                &root,
                &resources_dir(),
                "/main.typ",
                source,
                HashMap::new(),
            );
            assert!(result.is_ok(), "Compilation {i} failed: {:?}", result.err());
        }

        let rss_after = rss_kb().unwrap_or(0);
        let growth_kb = rss_after.saturating_sub(rss_before);

        assert!(
            growth_kb < 90_000,
            "RSS grew by {growth_kb} KB after 200 compilations – possible memory leak. \
             Ensure comemo::evict() is called after each compilation in compile_to_pdf."
        );

        Ok(())
    }
}
