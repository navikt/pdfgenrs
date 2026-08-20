use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use criterion::{Criterion, criterion_group, criterion_main};
use pdfgenrs::html::typst_to_html;
use pdfgenrs::pdf::{
    CompileRequest, build_html_converter, html_to_pdf, image_to_pdf, typst_to_pdf,
};
use pdfgenrs::typst_world;
use typst::Library;
use typst::utils::LazyHash;
use typst::{Feature, Features};

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

fn html_library() -> Arc<LazyHash<Library>> {
    Arc::new(typst_world::build_library(
        [Feature::Html].into_iter().collect(),
    ))
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
    let data = serde_json::json!({});

    c.bench_function("typst_to_pdf_simple", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: Arc::clone(&fonts),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "bench",
                template_name: "simple",
                library: Arc::clone(&library),
                comemo_eviction_threshold: pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            });
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
    let data = serde_json::json!({
        "title": "Benchmark Document",
        "body": "This is a benchmark document with JSON data injection for performance testing."
    });

    c.bench_function("typst_to_pdf_with_data", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: Arc::clone(&fonts),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "bench",
                template_name: "template",
                library: Arc::clone(&library),
                comemo_eviction_threshold: pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            });
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
                pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            );
        });
    });
}

fn bench_typst_to_pdf_large_json(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let source = r#"#set document(date: auto)
#set page(margin: 1cm)
#let data = json("/data/bench/large.json")
= #data.at("title", default: "Report")
#for item in data.at("items", default: ()) [
  - *#item.at("name", default: "")* — #item.at("description", default: "")
]
"#;

    let items: Vec<serde_json::Value> = (0..100)
        .map(|i| {
            serde_json::json!({
                "name": format!("Item {i}"),
                "description": format!("Description for item number {i} in the benchmark payload."),
                "value": i,
                "active": i % 2 == 0,
            })
        })
        .collect();
    let data = serde_json::json!({
        "title": "Large JSON Benchmark Report",
        "items": items,
    });

    c.bench_function("typst_to_pdf_large_json", |b| {
        b.iter(|| {
            let _ = typst_to_pdf(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: Arc::clone(&fonts),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "bench",
                template_name: "large",
                library: Arc::clone(&library),
                comemo_eviction_threshold: pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            });
        });
    });
}

fn bench_typst_to_pdf_concurrent(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let source = r"#set document(date: auto)
#set page(margin: 1cm)
Hello, concurrent world!
";
    let data = Arc::new(serde_json::json!({}));

    const THREADS: usize = 8;

    c.bench_function("typst_to_pdf_concurrent", |b| {
        b.iter_custom(|iters| {
            let iters = iters as usize;
            let start = Instant::now();

            std::thread::scope(|s| {
                let handles: Vec<_> = (0..THREADS)
                    .map(|_| {
                        let fonts = Arc::clone(&fonts);
                        let library = Arc::clone(&library);
                        let data = Arc::clone(&data);
                        let root = root_dir();
                        let resources = resources_dir();
                        s.spawn(move || {
                            for _ in 0..iters {
                                let _ = typst_to_pdf(CompileRequest {
                                    template_source: source,
                                    json_data: &data,
                                    fonts: Arc::clone(&fonts),
                                    root: &root,
                                    resources_dir: &resources,
                                    app_name: "bench",
                                    template_name: "concurrent",
                                    library: Arc::clone(&library),
                                    comemo_eviction_threshold:
                                        pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
                                });
                            }
                        })
                    })
                    .collect();
                for h in handles {
                    let _ = h.join();
                }
            });

            start.elapsed() / THREADS as u32
        });
    });
}

fn bench_image_to_pdf_jpeg(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let Ok(image_bytes) = std::fs::read(root_dir().join("resources").join("NAVLogoRed.jpg")) else {
        return;
    };

    c.bench_function("image_to_pdf_jpeg", |b| {
        b.iter(|| {
            let _ = image_to_pdf(
                image_bytes.clone(),
                "/image.jpg",
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                Arc::clone(&library),
                pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            );
        });
    });
}

fn bench_image_to_pdf_svg(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = pdf_library();
    let Ok(image_bytes) = std::fs::read(root_dir().join("resources").join("pdfgenrs-logo.svg"))
    else {
        return;
    };

    c.bench_function("image_to_pdf_svg", |b| {
        b.iter(|| {
            let _ = image_to_pdf(
                image_bytes.clone(),
                "/image.svg",
                Arc::clone(&fonts),
                &root_dir(),
                &resources_dir(),
                Arc::clone(&library),
                pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            );
        });
    });
}

fn bench_typst_to_html(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = html_library();
    let source = r"#set document(date: auto)
Hello, world!
";
    let data = serde_json::json!({});

    c.bench_function("typst_to_html_simple", |b| {
        b.iter(|| {
            let _ = typst_to_html(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: Arc::clone(&fonts),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "bench",
                template_name: "simple",
                library: Arc::clone(&library),
                comemo_eviction_threshold: pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            });
        });
    });
}

fn bench_typst_to_html_with_data(c: &mut Criterion) {
    let Ok(fonts) = typst_world::load_fonts(&fonts_dir()) else {
        return;
    };
    let fonts = Arc::new(fonts);
    let library = html_library();
    let source = r#"#set document(date: auto)
#let data = json("/data/bench/template.json")
= #data.at("title", default: "Untitled")
#data.at("body", default: "")
"#;
    let data = serde_json::json!({
        "title": "Benchmark Document",
        "body": "This is a benchmark document with JSON data injection for performance testing."
    });

    c.bench_function("typst_to_html_with_data", |b| {
        b.iter(|| {
            let _ = typst_to_html(CompileRequest {
                template_source: source,
                json_data: &data,
                fonts: Arc::clone(&fonts),
                root: &root_dir(),
                resources_dir: &resources_dir(),
                app_name: "bench",
                template_name: "template",
                library: Arc::clone(&library),
                comemo_eviction_threshold: pdfgenrs::config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
            });
        });
    });
}

criterion_group!(
    benches,
    bench_typst_to_pdf,
    bench_typst_to_pdf_with_data,
    bench_typst_to_pdf_large_json,
    bench_typst_to_pdf_concurrent,
    bench_html_to_pdf,
    bench_image_to_pdf,
    bench_image_to_pdf_jpeg,
    bench_image_to_pdf_svg,
    bench_typst_to_html,
    bench_typst_to_html_with_data,
);
criterion_main!(benches);
