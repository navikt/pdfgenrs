use std::path::PathBuf;
use std::sync::Arc;

use criterion::{Criterion, criterion_group, criterion_main};
use pdfgenrs::pdf::{image_to_pdf, typst_to_pdf};
use pdfgenrs::typst_world;
use typst::Features;
use typst::Library;
use typst::utils::LazyHash;

fn root_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fonts_dir() -> PathBuf {
    root_dir().join("fonts")
}

fn resources_dir() -> PathBuf {
    root_dir().join("resources")
}

fn pdf_library() -> Arc<LazyHash<Library>> {
    Arc::new(typst_world::build_library(Features::default()))
}

fn bench_typst_to_pdf(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, world!
";
    let data = b"{}".to_vec();

    c.bench_function("typst_to_pdf_simple", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(
                source.to_string(),
                &data,
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                "bench",
                "simple",
                Arc::clone(&library),
            );
        });
    });
}

fn bench_typst_to_pdf_with_data(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let source = r#"#set document(date: auto)
#set page(margin: 1cm)
#let data = json("/data/bench/template.json")
= #data.at("title", default: "Untitled")
#data.at("body", default: "")
"#;
    let data = br#"{"title":"Benchmark Document","body":"This is a benchmark document with JSON data injection for performance testing."}"#.to_vec();

    c.bench_function("typst_to_pdf_with_data", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(
                source.to_string(),
                &data,
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                "bench",
                "template",
                Arc::clone(&library),
            );
        });
    });
}

fn bench_image_to_pdf(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
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
                Arc::clone(&library),
            );
        });
    });
}

criterion_group!(
    benches,
    bench_typst_to_pdf,
    bench_typst_to_pdf_with_data,
    bench_image_to_pdf,
);
criterion_main!(benches);
