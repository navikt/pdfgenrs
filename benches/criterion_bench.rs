use std::path::PathBuf;
use std::sync::Arc;

use criterion::{Criterion, criterion_group, criterion_main};
use pdfgenrs::pdf::{build_html_converter, html_to_pdf, image_to_pdf, typst_to_pdf};
use pdfgenrs::typst_world;

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fonts_dir() -> PathBuf {
    root_dir().join("fonts")
}

fn resources_dir() -> PathBuf {
    root_dir().join("resources")
}

fn bench_typst_to_pdf(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
";
    let data = serde_json::json!({});

    c.bench_function("typst_to_pdf_simple", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(
                source,
                &data,
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                "bench",
                "simple",
            );
        });
    });
}

fn bench_typst_to_pdf_with_data(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let source = r#"#set document(date: auto)
#set page(margin: 1cm)
#let data = json("/data/bench/template.json")
= #data.at("title", default: "Untitled")
#data.at("body", default: "")
"#;
    let data = serde_json::json!({
        "title": "Benchmark Document",
        "body": "This is a benchmark document with JSON data injection for performance testing."
    });

    c.bench_function("typst_to_pdf_with_data", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(
                source,
                &data,
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                "bench",
                "template",
            );
        });
    });
}

fn bench_html_to_pdf(c: &mut Criterion) {
    let (converter, _) = build_html_converter(&fonts_dir(), &root_dir());
    let html = r#"<!DOCTYPE html>
<html>
<head><style>body { font-family: "Source Sans 3", sans-serif; }</style></head>
<body><h1>Benchmark HTML to PDF</h1><p>This is a performance test document.</p></body>
</html>"#;

    c.bench_function("html_to_pdf", |b| {
        b.iter(|| {
            let _ = html_to_pdf(html, &converter);
        });
    });
}

fn bench_image_to_pdf(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let Ok(image_bytes) = std::fs::read(root_dir().join("resources").join("NAVLogoRed.png")) else {
        return;
    };

    c.bench_function("image_to_pdf_png", |b| {
        b.iter(|| {
            let _ = image_to_pdf(
                image_bytes.clone(),
                "/image.png",
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
            );
        });
    });
}

criterion_group!(
    benches,
    bench_typst_to_pdf,
    bench_typst_to_pdf_with_data,
    bench_html_to_pdf,
    bench_image_to_pdf,
);
criterion_main!(benches);
