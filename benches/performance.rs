use std::future::IntoFuture;
use std::sync::Arc;

use axum_test::TestServer;
use pdfgenrs::{build_html_converter, build_router, config, state, template, typst_world};
use tokio::sync::RwLock;
use tokio::task::JoinSet;
use tracing::info;

const BENCH_MAX_TOTAL_MS_MULTI_THREAD: u128 = 600;
const BENCH_MAX_TOTAL_MS_SINGLE_THREAD: u128 = 600;

#[derive(Clone, Debug)]
struct BenchResult {
    app: String,
    template: String,
    passes: u32,
    duration_ms: u128,
}

fn create_bench_state() -> anyhow::Result<pdfgenrs::state::AppState> {
    let cfg = config::Config::default();
    let templates =
        Arc::new(template::load_templates_from_dir(&cfg.templates_dir).unwrap_or_default());
    let data = template::load_test_data(&cfg.data_dir).data;
    let fonts = Arc::new(typst_world::load_fonts(&cfg.fonts_dir)?);
    Ok(pdfgenrs::state::AppState {
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

    md.push_str("### Multi-thread (8 workers, 20 passes)\n\n");
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
    max_total_ms: u128,
) -> anyhow::Result<()> {
    let slow_results: Vec<String> = results
        .iter()
        .filter(|result| result.duration_ms > max_total_ms)
        .map(|result| {
            format!(
                "{}/{}: {}ms (limit: {}ms)",
                result.app, result.template, result.duration_ms, max_total_ms
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
    fail_if_total_too_long(&mt_results, "Multi-thread", BENCH_MAX_TOTAL_MS_MULTI_THREAD)?;
    fail_if_total_too_long(
        &st_results,
        "Single-thread",
        BENCH_MAX_TOTAL_MS_SINGLE_THREAD,
    )?;

    Ok(())
}

async fn performance_multi_thread() -> anyhow::Result<Vec<BenchResult>> {
    let app_state = create_bench_state()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let base_url = format!("http://127.0.0.1:{port}");

    let server_handle =
        tokio::spawn(axum::serve(listener, build_router(app_state.clone())).into_future());

    let client = Arc::new(reqwest::Client::new());
    let passes = 20;
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

    server_handle.abort();
    Ok(results)
}

async fn performance_single_thread() -> anyhow::Result<Vec<BenchResult>> {
    let app_state = create_bench_state()?;
    let server = TestServer::new(build_router(app_state.clone()));

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

    Ok(results)
}
