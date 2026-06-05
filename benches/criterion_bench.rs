#![allow(clippy::expect_used)]

use std::sync::Arc;

use criterion::{Criterion, criterion_group, criterion_main};
use pdfgenrs::{build_html_converter, config, pdf, template, typst_world};

const BENCH_HTML_BODY: &str = r#"<!DOCTYPE html>
<html>
<head><style>body { font-family: "Source Sans 3", sans-serif; }</style></head>
<body><h1>Benchmark HTML to PDF</h1><p>This is a performance test document.</p></body>
</html>"#;

fn bench_typst_to_pdf(c: &mut Criterion) {
    let cfg = config::Config::default();
    let templates =
        Arc::new(template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_default());
    let data = template::load_test_data(&cfg.data_dir).data;
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir).expect("failed to load fonts"));

    let mut group = c.benchmark_group("typst_to_pdf");

    for ((app_name, template_name), template_source) in templates.iter() {
        let json_data = data
            .get(&(app_name.clone(), template_name.clone()))
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));

        let bench_id = format!("{app_name}/{template_name}");
        let fonts = Arc::clone(&fonts);
        let root = cfg.root_dir.clone();
        let resources_dir = cfg.resources_dir.clone();

        group.bench_function(&bench_id, |b| {
            b.iter(|| {
                let result = pdf::typst_to_pdf(
                    template_source,
                    &json_data,
                    Arc::clone(&fonts),
                    &root,
                    &resources_dir,
                    app_name,
                    template_name,
                );
                assert!(result.is_ok());
            });
        });
    }

    group.finish();
}

fn bench_html_to_pdf(c: &mut Criterion) {
    let cfg = config::Config::default();
    let (converter, _) = build_html_converter(&cfg.fonts_dir, &cfg.root_dir);

    c.benchmark_group("html_to_pdf")
        .bench_function("html_bench", |b| {
            b.iter(|| {
                let result = pdf::html_to_pdf(BENCH_HTML_BODY, &converter);
                assert!(result.is_ok());
            });
        });
}

fn bench_image_to_pdf(c: &mut Criterion) {
    let cfg = config::Config::default();
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir).expect("failed to load fonts"));
    let image_bytes = std::fs::read(cfg.root_dir.join("resources").join("NAVLogoRed.png"))
        .expect("failed to read test image");

    c.benchmark_group("image_to_pdf")
        .bench_function("image_bench", |b| {
            let fonts = Arc::clone(&fonts);
            let root = cfg.root_dir.clone();
            let resources_dir = cfg.resources_dir.clone();
            b.iter(|| {
                let result = pdf::image_to_pdf(
                    image_bytes.clone(),
                    "/image.png",
                    Arc::clone(&fonts),
                    &root,
                    &resources_dir,
                );
                assert!(result.is_ok());
            });
        });
}

criterion_group!(
    benches,
    bench_typst_to_pdf,
    bench_html_to_pdf,
    bench_image_to_pdf
);
criterion_main!(benches);
