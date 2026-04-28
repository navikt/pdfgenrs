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

#[derive(Clone)]
pub struct Fonts {
    pub fonts: Vec<Font>,
    pub book: FontBook,
}

pub fn load_fonts() -> Fonts {
    let mut fonts: Vec<Font> = Vec::new();
    for &font_data in EMBEDDED_FONTS {
        let bytes = Bytes::new(font_data);
        fonts.extend(Font::iter(bytes));
    }
    let book = FontBook::from_fonts(&fonts);
    Fonts { fonts, book }
}

pub struct PdfgenWorld {
    library: LazyHash<Library>,
    font_book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main_id: FileId,
    main_source: Source,
    virtual_files: HashMap<FileId, Bytes>,
    root: PathBuf,
}

impl PdfgenWorld {
    pub fn new(
        fonts: Fonts,
        root: &Path,
        main_path: &str,
        main_source: String,
        virtual_files: HashMap<String, Bytes>,
    ) -> Result<Self> {
        let Fonts { fonts, book: font_book } = fonts;

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
        if let Some(bytes) = self.virtual_files.get(&id) {
            let text = std::str::from_utf8(bytes.as_slice())
                .map_err(|_| FileError::InvalidUtf8)?
                .to_string();
            return Ok(Source::new(id, text));
        }
        let vpath = id.vpath();
        let physical = self.root.join(vpath.as_rootless_path());
        let text = std::fs::read_to_string(&physical)
            .map_err(|e| FileError::from_io(e, &physical))?;
        Ok(Source::new(id, text))
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(bytes) = self.virtual_files.get(&id) {
            return Ok(bytes.clone());
        }
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

pub fn compile_to_pdf(
    fonts: Fonts,
    root: &Path,
    main_path: &str,
    main_source: String,
    virtual_files: HashMap<String, Bytes>,
) -> Result<Vec<u8>> {
    let world = PdfgenWorld::new(fonts, root, main_path, main_source, virtual_files)?;

    let result = typst::compile::<typst_library::layout::PagedDocument>(&world);

    comemo::evict(15);

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

    fn rss_kb() -> Option<u64> {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        status
            .lines()
            .find(|line| line.starts_with("VmRSS:"))
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|val| val.parse().ok())
    }

    #[test]
    fn fonts_loads_embedded_fonts() {
        let fonts = load_fonts();
        assert!(
            fonts.fonts.len() >= EMBEDDED_FONTS.len(),
            "Expected at least as many font faces as there are embedded font files"
        );
    }

    #[test]
    fn fonts_clone_can_be_reused_across_multiple_compilations() {
        let fonts = load_fonts();

        let source = r#"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
"#;

        let result1 = compile_to_pdf(
            fonts.clone(),
            &root_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        );
        assert!(result1.is_ok(), "First compilation failed: {:?}", result1.err());
        let pdf1 = result1.unwrap();
        assert!(is_pdf(&pdf1), "First result is not a valid PDF");

        let result2 = compile_to_pdf(
            fonts.clone(),
            &root_dir(),
            "/main.typ",
            source.to_string(),
            HashMap::new(),
        );
        assert!(result2.is_ok(), "Second compilation failed: {:?}", result2.err());
        let pdf2 = result2.unwrap();
        assert!(is_pdf(&pdf2), "Second result is not a valid PDF");
    }

    #[test]
    fn compilation_succeeds_after_full_cache_eviction() {
        let fonts = load_fonts();
        let root = root_dir();
        let source = "#set page(margin: 1cm)\nCache eviction test.".to_string();

        comemo::evict(0);

        let result = compile_to_pdf(fonts, &root, "/main.typ", source, HashMap::new());
        assert!(
            result.is_ok(),
            "Compilation after full cache eviction failed: {:?}",
            result.err()
        );
        assert!(is_pdf(&result.unwrap()), "Result after cache eviction is not a valid PDF");
    }

    #[test]
    fn repeated_compilations_do_not_grow_memory_unboundedly() {
        let fonts = load_fonts();
        let root = root_dir();

        for i in 0..10 {
            let source = format!("#set page(margin: 1cm)\nWarmup {i}.");
            compile_to_pdf(fonts.clone(), &root, "/main.typ", source, HashMap::new())
                .expect("warmup compilation should succeed");
        }

        let Some(rss_before) = rss_kb() else {
            return;
        };

        for i in 0..200 {
            let source = format!("#set page(margin: 1cm)\nDocument {i} with unique content.");
            let result =
                compile_to_pdf(fonts.clone(), &root, "/main.typ", source, HashMap::new());
            assert!(result.is_ok(), "Compilation {i} failed: {:?}", result.err());
        }

        let rss_after = rss_kb().unwrap_or(0);
        let growth_kb = rss_after.saturating_sub(rss_before);

        assert!(
            growth_kb < 65_536,
            "RSS grew by {growth_kb} KB after 200 compilations – possible memory leak. \
             Ensure comemo::evict() is called after each compilation in compile_to_pdf."
        );
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
