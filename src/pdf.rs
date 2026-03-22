use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use typst::foundations::Bytes;

use crate::typst_world;

static PDF_REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Convert HTML string to PDF bytes using headless Chromium.
///
/// Renders the HTML document with a headless Chromium browser and exports
/// the result as PDF, producing visually rendered output equivalent to the
/// Kotlin pdfgen implementation (OpenHTMLtoPDF).
///
/// `_fonts_dir` and `_root` are accepted for interface consistency with
/// `typst_to_pdf` / `image_to_pdf`; font discovery is handled via fontconfig.
pub fn html_to_pdf(html: &str, _fonts_dir: &str, _root: &Path) -> Result<Vec<u8>> {
    let id = PDF_REQUEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let tmp = std::env::temp_dir();

    let html_file = tmp.join(format!("pdfgen-{pid}-{id}.html"));
    let pdf_file = tmp.join(format!("pdfgen-{pid}-{id}.pdf"));

    std::fs::write(&html_file, html).context("Failed to write HTML to temp file")?;

    let chromium = find_chromium_binary()?;
    // --no-sandbox is required when running as non-root inside a container
    // where user namespaces are not available.
    let result = std::process::Command::new(chromium)
        .args([
            "--headless",
            "--no-sandbox",
            "--disable-gpu",
            "--disable-dev-shm-usage",
            "--run-all-compositor-stages-before-draw",
            &format!("--print-to-pdf={}", pdf_file.display()),
            &format!("file://{}", html_file.display()),
        ])
        .output();

    let _ = std::fs::remove_file(&html_file);

    match result {
        Err(e) => Err(anyhow::anyhow!("Failed to launch {chromium}: {e}")),
        Ok(output) if !output.status.success() => {
            let _ = std::fs::remove_file(&pdf_file);
            Err(anyhow::anyhow!(
                "{chromium} exited with status {}: {}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            ))
        }
        Ok(_) => {
            let pdf = std::fs::read(&pdf_file).context("Failed to read PDF output")?;
            let _ = std::fs::remove_file(&pdf_file);
            Ok(pdf)
        }
    }
}

/// Return the path of the first Chromium/Chrome binary found in common locations,
/// or an error if none is available.
fn find_chromium_binary() -> Result<&'static str> {
    const CANDIDATES: &[&str] = &[
        "chromium",
        "chromium-browser",
        "google-chrome",
        "google-chrome-stable",
    ];
    const SEARCH_DIRS: &[&str] = &["/usr/bin", "/usr/local/bin"];
    for &name in CANDIDATES {
        for &dir in SEARCH_DIRS {
            if std::path::Path::new(dir).join(name).exists() {
                return Ok(name);
            }
        }
    }
    Err(anyhow::anyhow!(
        "No Chromium/Chrome binary found. Install chromium or google-chrome."
    ))
}

/// Render a Typst template to PDF bytes with JSON data injected as data.json.
///
/// The template can access the data via `#let data = json("data.json")`.
#[allow(dead_code)]
pub fn typst_to_pdf(
    template_source: &str,
    json_data: &serde_json::Value,
    fonts_dir: &str,
    root: &Path,
) -> Result<Vec<u8>> {
    let json_bytes = serde_json::to_vec(json_data).context("Failed to serialize JSON data")?;
    let mut vfiles = HashMap::new();
    vfiles.insert("/data.json".to_string(), Bytes::new(json_bytes));

    typst_world::compile_to_pdf(
        fonts_dir,
        root,
        "/main.typ",
        template_source.to_string(),
        vfiles,
    )
}

/// Wrap an image (JPEG/PNG) in a Typst document and convert to PDF.
pub fn image_to_pdf(
    image_bytes: &[u8],
    content_type: &str,
    fonts_dir: &str,
    root: &Path,
) -> Result<Vec<u8>> {
    let fmt = if content_type.contains("png") { "png" } else { "jpg" };
    let typst_source = format!(
        r#"#set page(margin: 0pt, width: auto, height: auto)
#let img-data = read("/image-data", encoding: none)
#image.decode(img-data, format: "{fmt}")
"#
    );

    let mut vfiles = HashMap::new();
    vfiles.insert("/image-data".to_string(), Bytes::new(image_bytes.to_vec()));

    typst_world::compile_to_pdf(fonts_dir, root, "/main.typ", typst_source, vfiles)
}
