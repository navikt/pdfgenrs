use std::future::IntoFuture;
use std::sync::Arc;

use axum_test::TestServer;
use pdfgenrs::{build_html_converter, build_router, config, metrics, state, template, typst_world};
use reqwest::header;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::info;
use typst::{Feature, Features};

const BENCH_MAX_TOTAL_MS_MULTI_THREAD: u128 = 700;
const BENCH_MAX_TOTAL_MS_SINGLE_THREAD: u128 = 700;
const BENCH_MAX_TOTAL_MS_IMAGE_MULTI_THREAD: u128 = 600;
const BENCH_MAX_TOTAL_MS_IMAGE_SINGLE_THREAD: u128 = 600;
const BENCH_MAX_TOTAL_MS_HTML_MULTI_THREAD: u128 = 700;
const BENCH_MAX_TOTAL_MS_HTML_SINGLE_THREAD: u128 = 700;
const BENCH_MAX_TOTAL_MS_LARGE_OUTPUT_MULTI_THREAD: u128 = 30_000;
const BENCH_MAX_TOTAL_MS_LARGE_OUTPUT_SINGLE_THREAD: u128 = 30_000;
const DEFAULT_PASSES: u32 = 30;
const LARGE_OUTPUT_PASSES: u32 = 3;
const LARGE_OUTPUT_PDF_MIN_BYTES: usize = 512 * 1024;
const LARGE_OUTPUT_ITEM_COUNT: usize = 350;
const LARGE_OUTPUT_ITEM_BODY_BYTES: usize = 4_096;

const BENCH_HTML_BODY: &str = r#"<!DOCTYPE html>
<html>
<head><style>body { font-family: "Source Sans 3", sans-serif; }</style></head>
<body><h1>Benchmark HTML to PDF</h1><p>This is a performance test document.</p></body>
</html>"#;

#[derive(Clone, Debug)]
struct BenchResult {
    app: String,
    template: String,
    passes: u32,
    duration_ms: u128,
}

fn create_bench_state() -> anyhow::Result<state::AppState> {
    let cfg = config::Config::default();
    let templates =
        Arc::new(template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_default());
    let data = template::load_test_data(&cfg.data_dir).data;
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
    Ok(state::AppState {
        templates,
        data: Arc::new(RwLock::new(data)),
        aliveness: state::AppAliveness::new(),
        fonts,
        pdf_library: Arc::new(typst_world::build_library(Features::default())),
        html_library: Arc::new(typst_world::build_library(
            [Feature::Html].into_iter().collect(),
        )),
        html_converter: Arc::new(build_html_converter(&cfg.fonts_dir, &cfg.root_dir).0),
        root_dir: Arc::new(cfg.root_dir.clone()),
        resources_dir: Arc::new(cfg.resource_root()),
        config: cfg,
        compile_semaphore: None,
    })
}

fn write_github_summary(mt_results: &[BenchResult], st_results: &[BenchResult]) {
    let summary_file = match std::env::var("GITHUB_STEP_SUMMARY") {
        Ok(path) => path,
        Err(_) => return,
    };

    let mut md = String::new();
    md.push_str("## Performance benchmark results\n\n");

    md.push_str("### Multi-thread (8 workers, 30 passes)\n\n");
    md.push_str("| App | Template | Total (ms) | Avg per request (ms) |\n");
    md.push_str("|-----|----------|-----------|----------------------|\n");
    for r in mt_results {
        let avg = if r.passes > 0 {
            r.duration_ms as f64 / r.passes as f64
        } else {
            0.0
        };
        md.push_str(&format!(
            "| {} | {} | {} | {:.1} |\n",
            r.app, r.template, r.duration_ms, avg
        ));
    }

    md.push('\n');
    md.push_str("### Single-thread (30 passes)\n\n");
    md.push_str("| App | Template | Total (ms) | Avg per request (ms) |\n");
    md.push_str("|-----|----------|-----------|----------------------|\n");
    for r in st_results {
        let avg = if r.passes > 0 {
            r.duration_ms as f64 / r.passes as f64
        } else {
            0.0
        };
        md.push_str(&format!(
            "| {} | {} | {} | {:.1} |\n",
            r.app, r.template, r.duration_ms, avg
        ));
    }

    if let Err(e) = std::fs::write(&summary_file, &md) {
        tracing::warn!(
            path = %summary_file,
            error = %e,
            "Failed to write GitHub step summary"
        );
    }
}

fn html_bench_json_data() -> serde_json::Value {
    serde_json::json!({
        "title": "HTML Benchmark Document",
        "body": "This document is used to benchmark HTML generation performance.",
        "items": [
            { "name": "Item 1", "value": "Value one" },
            { "name": "Item 2", "value": "Value two" },
            { "name": "Item 3", "value": "Value three" }
        ]
    })
}

fn large_output_json_data() -> serde_json::Value {
    let items = (0..LARGE_OUTPUT_ITEM_COUNT)
        .map(|item| {
            let mut body = String::with_capacity(LARGE_OUTPUT_ITEM_BODY_BYTES);
            let mut state = item as u64 + 1;

            while body.len() < LARGE_OUTPUT_ITEM_BODY_BYTES {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1);
                let word_len = 5 + (state % 8) as usize;
                for shift in 0..word_len {
                    let letter = ((state >> (shift * 5)) % 26) as u8;
                    body.push(char::from(b'a' + letter));
                }
                body.push(' ');
            }
            body.truncate(LARGE_OUTPUT_ITEM_BODY_BYTES);

            serde_json::json!({
                "title": format!("Section {item}"),
                "body": body,
            })
        })
        .collect::<Vec<_>>();

    serde_json::json!({
        "title": "Large PDF output load benchmark",
        "items": items,
    })
}

fn is_large_output_benchmark(app_name: &str, template_name: &str) -> bool {
    app_name == "bench" && template_name == "large-output"
}

fn fail_if_total_too_long(
    results: &[BenchResult],
    mode: &str,
    default_max_ms: u128,
    image_max_ms: u128,
    html_max_ms: u128,
    large_output_max_ms: u128,
) -> anyhow::Result<()> {
    let slow_results: Vec<String> = results
        .iter()
        .filter(|result| {
            let max = match result.app.as_str() {
                "bench" if result.template == "large-output" => large_output_max_ms,
                "image" => image_max_ms,
                "html" | "html-to-pdf" => html_max_ms,
                _ => default_max_ms,
            };
            result.duration_ms > max
        })
        .map(|result| {
            let max = match result.app.as_str() {
                "bench" if result.template == "large-output" => large_output_max_ms,
                "image" => image_max_ms,
                "html" | "html-to-pdf" => html_max_ms,
                _ => default_max_ms,
            };
            format!(
                "{}/{}: {}ms (limit: {}ms)",
                result.app, result.template, result.duration_ms, max
            )
        })
        .collect();

    if slow_results.is_empty() {
        return Ok(());
    }

    anyhow::bail!(
        "{} benchmark exceeded max Total (ms) threshold: {}",
        mode,
        slow_results.join(", ")
    );
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let mt_runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(8)
        .enable_all()
        .build()?;
    let mt_results = mt_runtime.block_on(performance_multi_thread())?;

    let st_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    let st_results = st_runtime.block_on(performance_single_thread())?;

    write_github_summary(&mt_results, &st_results);
    fail_if_total_too_long(
        &mt_results,
        "Multi-thread",
        BENCH_MAX_TOTAL_MS_MULTI_THREAD,
        BENCH_MAX_TOTAL_MS_IMAGE_MULTI_THREAD,
        BENCH_MAX_TOTAL_MS_HTML_MULTI_THREAD,
        BENCH_MAX_TOTAL_MS_LARGE_OUTPUT_MULTI_THREAD,
    )?;
    fail_if_total_too_long(
        &st_results,
        "Single-thread",
        BENCH_MAX_TOTAL_MS_SINGLE_THREAD,
        BENCH_MAX_TOTAL_MS_IMAGE_SINGLE_THREAD,
        BENCH_MAX_TOTAL_MS_HTML_SINGLE_THREAD,
        BENCH_MAX_TOTAL_MS_LARGE_OUTPUT_SINGLE_THREAD,
    )?;

    Ok(())
}

async fn performance_multi_thread() -> anyhow::Result<Vec<BenchResult>> {
    let app_state = create_bench_state()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let base_url = format!("http://127.0.0.1:{port}");

    let server_handle = tokio::spawn(
        axum::serve(
            listener,
            build_router(app_state.clone(), metrics::test_metrics_handle()),
        )
        .into_future(),
    );

    let client = Arc::new(reqwest::Client::new());
    let mut results = Vec::new();

    for (app_name, template_name) in app_state.templates.keys() {
        let app_name = app_name.clone();
        let template_name = template_name.clone();

        let is_large_output = is_large_output_benchmark(&app_name, &template_name);
        let passes = if is_large_output {
            LARGE_OUTPUT_PASSES
        } else {
            DEFAULT_PASSES
        };
        let json_data = if is_large_output {
            Arc::new(large_output_json_data())
        } else {
            let data = app_state.data.read().await;
            data.get(&(app_name.clone(), template_name.clone()))
                .cloned()
                .unwrap_or_else(|| Arc::new(serde_json::json!({})))
        };

        let start = std::time::Instant::now();

        let mut join_set = JoinSet::new();
        for _ in 0..passes {
            let client = Arc::clone(&client);
            let url = format!("{base_url}/api/v1/genpdf/{app_name}/{template_name}");
            let data = json_data.clone();
            join_set.spawn(async move {
                let response = client.post(&url).json(data.as_ref()).send().await?;
                assert!(response.status().is_success());
                let bytes = response.bytes().await?;
                assert!(!bytes.is_empty());
                if is_large_output {
                    assert!(
                        bytes.len() >= LARGE_OUTPUT_PDF_MIN_BYTES,
                        "large output PDF was {} bytes, expected at least {LARGE_OUTPUT_PDF_MIN_BYTES}",
                        bytes.len()
                    );
                }
                anyhow::Ok(())
            });
        }

        while let Some(task_result) = join_set.join_next().await {
            task_result??;
        }

        let duration_ms = start.elapsed().as_millis();
        info!(
            template = %template_name,
            app = %app_name,
            duration_ms,
            "Multi-thread performance benchmark completed"
        );
        results.push(BenchResult {
            app: app_name,
            template: template_name,
            passes,
            duration_ms,
        });
    }

    // Benchmark image-to-PDF
    {
        let image_bytes = std::fs::read(
            app_state
                .config
                .root_dir
                .join("resources")
                .join("NAVLogoRed.png"),
        )?;
        let start = std::time::Instant::now();
        let mut join_set = JoinSet::new();
        for _ in 0..passes {
            let client = Arc::clone(&client);
            let url = format!("{base_url}/api/v1/genpdf/image/bench");
            let data = image_bytes.clone();
            join_set.spawn(async move {
                let response = client
                    .post(&url)
                    .header(header::CONTENT_TYPE, "image/png")
                    .body(data)
                    .send()
                    .await?;
                assert!(response.status().is_success());
                let bytes = response.bytes().await?;
                assert!(!bytes.is_empty());
                anyhow::Ok(())
            });
        }
        while let Some(task_result) = join_set.join_next().await {
            task_result??;
        }
        let duration_ms = start.elapsed().as_millis();
        info!(duration_ms, "Multi-thread image-to-PDF benchmark completed");
        results.push(BenchResult {
            app: "image".to_string(),
            template: "bench".to_string(),
            passes,
            duration_ms,
        });
    }

    // Benchmark HTML-to-PDF
    {
        let start = std::time::Instant::now();
        let mut join_set = JoinSet::new();
        for _ in 0..passes {
            let client = Arc::clone(&client);
            let url = format!("{base_url}/api/v1/genpdf/html/bench");
            let body = BENCH_HTML_BODY.to_string();
            join_set.spawn(async move {
                let response = client
                    .post(&url)
                    .header(header::CONTENT_TYPE, "text/html")
                    .body(body)
                    .send()
                    .await?;
                assert!(response.status().is_success());
                let bytes = response.bytes().await?;
                assert!(!bytes.is_empty());
                anyhow::Ok(())
            });
        }
        while let Some(task_result) = join_set.join_next().await {
            task_result??;
        }
        let duration_ms = start.elapsed().as_millis();
        info!(duration_ms, "Multi-thread HTML-to-PDF benchmark completed");
        results.push(BenchResult {
            app: "html-to-pdf".to_string(),
            template: "bench".to_string(),
            passes,
            duration_ms,
        });
    }

    // Benchmark HTML generation
    {
        let json_data = Arc::new(html_bench_json_data());
        let start = std::time::Instant::now();
        let mut join_set = JoinSet::new();
        for _ in 0..passes {
            let client = Arc::clone(&client);
            let url = format!("{base_url}/api/v1/genhtml/bench/html-bench");
            let data = json_data.clone();
            join_set.spawn(async move {
                let response = client.post(&url).json(data.as_ref()).send().await?;
                assert!(response.status().is_success());
                let bytes = response.bytes().await?;
                assert!(!bytes.is_empty());
                anyhow::Ok(())
            });
        }
        while let Some(task_result) = join_set.join_next().await {
            task_result??;
        }
        let duration_ms = start.elapsed().as_millis();
        info!(
            duration_ms,
            "Multi-thread HTML generation benchmark completed"
        );
        results.push(BenchResult {
            app: "html".to_string(),
            template: "html-bench".to_string(),
            passes,
            duration_ms,
        });
    }

    server_handle.abort();
    Ok(results)
}

async fn performance_single_thread() -> anyhow::Result<Vec<BenchResult>> {
    let app_state = create_bench_state()?;
    let server = TestServer::new(build_router(
        app_state.clone(),
        metrics::test_metrics_handle(),
    ));

    let mut results = Vec::new();

    for (app_name, template_name) in app_state.templates.keys() {
        let app_name = app_name.clone();
        let template_name = template_name.clone();

        let is_large_output = is_large_output_benchmark(&app_name, &template_name);
        let passes = if is_large_output {
            LARGE_OUTPUT_PASSES
        } else {
            DEFAULT_PASSES
        };
        let json_data = if is_large_output {
            Arc::new(large_output_json_data())
        } else {
            let data = app_state.data.read().await;
            data.get(&(app_name.clone(), template_name.clone()))
                .cloned()
                .unwrap_or_else(|| Arc::new(serde_json::json!({})))
        };

        let start = std::time::Instant::now();

        for _ in 0..passes {
            let url = format!("/api/v1/genpdf/{app_name}/{template_name}");
            let response = server.post(&url).json(json_data.as_ref()).await;
            response.assert_status_success();
            assert!(!response.as_bytes().is_empty());
            if is_large_output {
                assert!(
                    response.as_bytes().len() >= LARGE_OUTPUT_PDF_MIN_BYTES,
                    "large output PDF was {} bytes, expected at least {LARGE_OUTPUT_PDF_MIN_BYTES}",
                    response.as_bytes().len()
                );
            }
        }

        let duration_ms = start.elapsed().as_millis();
        info!(
            template = %template_name,
            app = %app_name,
            duration_ms,
            "Single-thread performance benchmark completed"
        );
        results.push(BenchResult {
            app: app_name,
            template: template_name,
            passes,
            duration_ms,
        });
    }

    // Benchmark image-to-PDF
    {
        let image_bytes = std::fs::read(
            app_state
                .config
                .root_dir
                .join("resources")
                .join("NAVLogoRed.png"),
        )?;
        let start = std::time::Instant::now();
        for _ in 0..passes {
            let response = server
                .post("/api/v1/genpdf/image/bench")
                .content_type("image/png")
                .bytes(axum::body::Bytes::from(image_bytes.clone()))
                .await;
            response.assert_status_success();
            assert!(!response.as_bytes().is_empty());
        }
        let duration_ms = start.elapsed().as_millis();
        info!(
            duration_ms,
            "Single-thread image-to-PDF benchmark completed"
        );
        results.push(BenchResult {
            app: "image".to_string(),
            template: "bench".to_string(),
            passes,
            duration_ms,
        });
    }

    // Benchmark HTML-to-PDF
    {
        let start = std::time::Instant::now();
        for _ in 0..passes {
            let response = server
                .post("/api/v1/genpdf/html/bench")
                .content_type("text/html")
                .bytes(axum::body::Bytes::from(BENCH_HTML_BODY))
                .await;
            response.assert_status_success();
            assert!(!response.as_bytes().is_empty());
        }
        let duration_ms = start.elapsed().as_millis();
        info!(duration_ms, "Single-thread HTML-to-PDF benchmark completed");
        results.push(BenchResult {
            app: "html-to-pdf".to_string(),
            template: "bench".to_string(),
            passes,
            duration_ms,
        });
    }

    // Benchmark HTML generation
    {
        let json_data = html_bench_json_data();
        let start = std::time::Instant::now();
        for _ in 0..passes {
            let response = server
                .post("/api/v1/genhtml/bench/html-bench")
                .json(&json_data)
                .await;
            response.assert_status_success();
            assert!(!response.as_bytes().is_empty());
        }
        let duration_ms = start.elapsed().as_millis();
        info!(
            duration_ms,
            "Single-thread HTML generation benchmark completed"
        );
        results.push(BenchResult {
            app: "html".to_string(),
            template: "html-bench".to_string(),
            passes,
            duration_ms,
        });
    }

    Ok(results)
}
