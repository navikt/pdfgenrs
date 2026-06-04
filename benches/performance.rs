use std::future::IntoFuture;
use std::sync::Arc;

use axum_test::TestServer;
use pdfgenrs::{build_html_converter, build_router, config, metrics, state, template, typst_world};
use reqwest::header;
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::info;

const BENCH_HTML_BODY: &str = r#"<!DOCTYPE html>
<html>
<head><style>body { font-family: "Source Sans 3", sans-serif; }</style></head>
<body><h1>Benchmark HTML to PDF</h1><p>This is a performance test document.</p></body>
</html>"#;

const BENCH_MAX_TOTAL_MS_MULTI_THREAD: u128 = 600;
const BENCH_MAX_TOTAL_MS_SINGLE_THREAD: u128 = 600;
const BENCH_MAX_TOTAL_MS_HTML_MULTI_THREAD: u128 = 5000;
const BENCH_MAX_TOTAL_MS_HTML_SINGLE_THREAD: u128 = 60000;
const BENCH_MAX_TOTAL_MS_IMAGE_MULTI_THREAD: u128 = 5000;
const BENCH_MAX_TOTAL_MS_IMAGE_SINGLE_THREAD: u128 = 60000;

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
        html_converter: Arc::new(build_html_converter(&cfg.fonts_dir, &cfg.root_dir).0),
        config: cfg,
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
        eprintln!("Failed to write GitHub step summary to {summary_file}: {e}");
    }
}

fn fail_if_total_too_long(
    results: &[BenchResult],
    mode: &str,
    default_max_ms: u128,
    html_max_ms: u128,
    image_max_ms: u128,
) -> anyhow::Result<()> {
    let slow_results: Vec<String> = results
        .iter()
        .filter(|result| {
            let max = match result.app.as_str() {
                "html" => html_max_ms,
                "image" => image_max_ms,
                _ => default_max_ms,
            };
            result.duration_ms > max
        })
        .map(|result| {
            let max = match result.app.as_str() {
                "html" => html_max_ms,
                "image" => image_max_ms,
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
        BENCH_MAX_TOTAL_MS_HTML_MULTI_THREAD,
        BENCH_MAX_TOTAL_MS_IMAGE_MULTI_THREAD,
    )?;
    fail_if_total_too_long(
        &st_results,
        "Single-thread",
        BENCH_MAX_TOTAL_MS_SINGLE_THREAD,
        BENCH_MAX_TOTAL_MS_HTML_SINGLE_THREAD,
        BENCH_MAX_TOTAL_MS_IMAGE_SINGLE_THREAD,
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
    let passes = 30;
    let mut results = Vec::new();

    for (app_name, template_name) in app_state.templates.keys() {
        let app_name = app_name.clone();
        let template_name = template_name.clone();

        let json_data = {
            let data = app_state.data.read().await;
            data.get(&(app_name.clone(), template_name.clone()))
                .cloned()
                .unwrap_or_else(|| serde_json::json!({}))
        };

        let start = std::time::Instant::now();

        let mut join_set = JoinSet::new();
        for _ in 0..passes {
            let client = Arc::clone(&client);
            let url = format!("{base_url}/api/v1/genpdf/{app_name}/{template_name}");
            let data = json_data.clone();
            join_set.spawn(async move {
                let response = client.post(&url).json(&data).send().await?;
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
            app: "html".to_string(),
            template: "bench".to_string(),
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

    server_handle.abort();
    Ok(results)
}

async fn performance_single_thread() -> anyhow::Result<Vec<BenchResult>> {
    let app_state = create_bench_state()?;
    let server = TestServer::new(build_router(
        app_state.clone(),
        metrics::test_metrics_handle(),
    ));

    let passes = 30;
    let mut results = Vec::new();

    for (app_name, template_name) in app_state.templates.keys() {
        let app_name = app_name.clone();
        let template_name = template_name.clone();

        let json_data = {
            let data = app_state.data.read().await;
            data.get(&(app_name.clone(), template_name.clone()))
                .cloned()
                .unwrap_or_else(|| serde_json::json!({}))
        };

        let start = std::time::Instant::now();

        for _ in 0..passes {
            let url = format!("/api/v1/genpdf/{app_name}/{template_name}");
            let response = server.post(&url).json(&json_data).await;
            response.assert_status_success();
            assert!(!response.as_bytes().is_empty());
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
            app: "html".to_string(),
            template: "bench".to_string(),
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

    Ok(results)
}
