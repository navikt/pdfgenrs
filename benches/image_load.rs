//! Load test for the image-to-PDF endpoint with large images.
//!
//! This is deliberately *not* part of `benches/performance.rs`. That benchmark is a
//! fast CI regression guard with fixed millisecond thresholds; this one allocates
//! hundreds of megabytes and is meant to be run deliberately when tuning
//! `MAX_IMAGE_PIXELS`, `MAX_CONCURRENT_COMPILATIONS` and pod memory limits.
//!
//! Run with:
//!
//! ```text
//! IMAGE_LOAD_BENCH=1 cargo bench --bench image_load
//! ```
//!
//! To find the actual OOM threshold rather than estimating it, run the benchmark
//! under a memory-limited container. Build and run must be separate steps: the
//! memory limit is meant for the benchmark, and compiling `typst-library` at
//! `opt-level=3` needs several gigabytes on its own, so building under `-m 3g`
//! gets the compiler OOM-killed before the benchmark ever starts.
//!
//! ```text
//! # 1. Build with full memory available. A separate target dir avoids thrashing
//! #    the host's macOS/Windows artifacts in the same directory.
//! docker run --rm -v "$PWD:/src" -w /src -e CARGO_TARGET_DIR=/src/target/linux \
//!   rust:latest cargo bench --bench image_load --no-run
//!
//! # 2. Run only the benchmark binary under the memory limit.
//! docker run --rm -m 3g -v "$PWD:/src" -w /src rust:latest \
//!   bash -c 'IMAGE_LOAD_BENCH=1 exec $(find target/linux/release/deps -type f -name "image_load-*" ! -name "*.d")'
//! ```
//!
//! Peak RSS is only measured on Linux, since it is read from `/proc/self/status`.
//! Numbers from macOS are latency-only and do not reflect behaviour under cgroup
//! memory limits.

use std::collections::HashMap;
use std::future::IntoFuture;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

use image::ExtendedColorType;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::ImageEncoder;
use pdfgenrs::{build_html_converter, build_router, config, metrics, state, typst_world};
use reqwest::header;
use tokio::sync::{RwLock, Semaphore};
use tokio::task::JoinSet;
use typst::{Feature, Features};

/// Env var that must be set for the benchmark to do any work.
///
/// Without this gate, a plain `cargo bench` would allocate hundreds of megabytes
/// of fixture data in CI.
const ENABLE_ENV: &str = "IMAGE_LOAD_BENCH";

/// Requests fired per (fixture, concurrency) combination.
const REQUESTS_PER_COMBINATION: usize = 12;

/// Concurrency settings to compare. This is the matrix that answers
/// "does lowering MAX_CONCURRENT_COMPILATIONS actually cost throughput?".
const CONCURRENCY_MATRIX: [usize; 3] = [1, 2, 4];

/// Body limit used for the benchmark, matching the 50 MB target.
const BODY_LIMIT_BYTES: usize = 50 * 1024 * 1024;
const MAX_IMAGE_DIMENSION_PIXELS: u32 = 16_384;
const MAX_IMAGE_PIXELS: u64 = 100_000_000;
const COMPILE_TIMEOUT_SECONDS: u64 = 60;

struct Fixture {
    name: &'static str,
    content_type: &'static str,
    path: PathBuf,
    width: u32,
    height: u32,
    file_len: u64,
}

struct Measurement {
    fixture: &'static str,
    concurrency: usize,
    ok: usize,
    rejected: usize,
    total_ms: u128,
    p50_ms: u128,
    p95_ms: u128,
    max_ms: u128,
    peak_rss_growth_kb: Option<u64>,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    if std::env::var(ENABLE_ENV).is_err() {
        println!(
            "image_load benchmark skipped. Set {ENABLE_ENV}=1 to run it.\n\
             It generates ~500 MB of fixtures and drives large concurrent compilations."
        );
        return Ok(());
    }

    let fixtures = ensure_fixtures()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?;

    let mut measurements = Vec::new();
    for fixture in &fixtures {
        if fixture.file_len > BODY_LIMIT_BYTES as u64 {
            println!(
                "SKIP {}: {:.1} MiB exceeds the {} MiB body limit",
                fixture.name,
                fixture.file_len as f64 / (1024.0 * 1024.0),
                BODY_LIMIT_BYTES / (1024 * 1024)
            );
            continue;
        }
        for concurrency in CONCURRENCY_MATRIX {
            measurements.push(runtime.block_on(measure(fixture, concurrency))?);
        }
    }

    print_report(&fixtures, &measurements);
    Ok(())
}

/// Generates the fixture images once and caches them under `target/`.
///
/// Fixtures are generated rather than committed: a 25-50 MB binary would stay in
/// the git history permanently. `target/` is already gitignored.
fn ensure_fixtures() -> anyhow::Result<Vec<Fixture>> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("image-load-fixtures");
    std::fs::create_dir_all(&dir)?;

    let mut fixtures = Vec::new();

    // The reported failing image: 13583x5417 = 73.6 MP as JPEG. Exercises the
    // DCTDecode passthrough path, where memory tracks file size, not pixel count.
    fixtures.push(write_jpeg(
        &dir,
        "jpeg-73mp-q85",
        13_583,
        5_417,
        85,
        Content::Photo,
    )?);

    // Same dimensions as flat-colour PNG: a small file that still forces a ~294 MB
    // RGBA decode buffer. This is the worst case a body size limit cannot catch.
    fixtures.push(write_png(
        &dir,
        "png-73mp-flat",
        13_583,
        5_417,
        Content::Flat,
    )?);

    // Photo-like PNG at 32 MP. Compresses poorly, so this is the case where the
    // body limit binds before the pixel limit does.
    fixtures.push(write_png(&dir, "png-32mp-photo", 8_000, 4_000, Content::Photo)?);

    Ok(fixtures)
}

#[derive(Clone, Copy)]
enum Content {
    /// Large uniform areas with a few bands. Compresses extremely well.
    Flat,
    /// Smooth gradients plus deterministic noise, approximating photographic data.
    Photo,
}

/// Fills an RGB8 buffer.
///
/// Uses a deterministic LCG rather than a random number generator so that repeated
/// runs produce byte-identical fixtures and therefore comparable file sizes.
fn rgb_buffer(width: u32, height: u32, content: Content) -> Vec<u8> {
    let pixels = width as usize * height as usize;
    let mut buf = vec![0u8; pixels * 3];
    let mut seed: u32 = 0x1234_5678;

    for y in 0..height as usize {
        for x in 0..width as usize {
            let i = (y * width as usize + x) * 3;
            match content {
                Content::Flat => {
                    let band = (y / 512) % 4;
                    let (r, g, b) = match band {
                        0 => (250, 250, 250),
                        1 => (200, 40, 40),
                        2 => (250, 250, 250),
                        _ => (20, 20, 60),
                    };
                    buf[i] = r;
                    buf[i + 1] = g;
                    buf[i + 2] = b;
                }
                Content::Photo => {
                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    let noise = (seed >> 24) as u8 / 8;
                    let gx = ((x * 255) / width as usize) as u8;
                    let gy = ((y * 255) / height as usize) as u8;
                    buf[i] = gx.wrapping_add(noise);
                    buf[i + 1] = gy.wrapping_add(noise);
                    buf[i + 2] = gx.wrapping_add(gy).wrapping_add(noise);
                }
            }
        }
    }
    buf
}

fn write_jpeg(
    dir: &Path,
    name: &'static str,
    width: u32,
    height: u32,
    quality: u8,
    content: Content,
) -> anyhow::Result<Fixture> {
    let path = dir.join(format!("{name}.jpg"));
    if !path.exists() {
        println!("generating fixture {name} ({width}x{height})...");
        let buf = rgb_buffer(width, height, content);
        let mut file = std::io::BufWriter::new(std::fs::File::create(&path)?);
        JpegEncoder::new_with_quality(&mut file, quality).write_image(
            &buf,
            width,
            height,
            ExtendedColorType::Rgb8,
        )?;
    }
    let file_len = std::fs::metadata(&path)?.len();
    Ok(Fixture {
        name,
        content_type: "image/jpeg",
        path,
        width,
        height,
        file_len,
    })
}

fn write_png(
    dir: &Path,
    name: &'static str,
    width: u32,
    height: u32,
    content: Content,
) -> anyhow::Result<Fixture> {
    let path = dir.join(format!("{name}.png"));
    if !path.exists() {
        println!("generating fixture {name} ({width}x{height})...");
        let buf = rgb_buffer(width, height, content);
        let mut file = std::io::BufWriter::new(std::fs::File::create(&path)?);
        // Fast compression keeps fixture generation from dominating the run time.
        // Flat content still compresses to a tiny file, which is the point.
        PngEncoder::new_with_quality(&mut file, CompressionType::Fast, FilterType::Adaptive)
            .write_image(&buf, width, height, ExtendedColorType::Rgb8)?;
    }
    let file_len = std::fs::metadata(&path)?.len();
    Ok(Fixture {
        name,
        content_type: "image/png",
        path,
        width,
        height,
        file_len,
    })
}

fn bench_state(concurrency: usize) -> anyhow::Result<state::AppState> {
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let cfg = config::Config {
        port: 0,
        root_dir: root_dir.clone(),
        templates_dir: root_dir.join("templates"),
        resources_dir: root_dir.join("resources"),
        data_dir: root_dir.join("data"),
        fonts_dir: root_dir.join("fonts"),
        dev_mode: false,
        request_body_limit_bytes: BODY_LIMIT_BYTES,
        compile_timeout_seconds: COMPILE_TIMEOUT_SECONDS,
        shutdown_drain_seconds: 0,
        max_concurrent_compilations: concurrency,
        semaphore_acquire_timeout_seconds: 10,
        comemo_eviction_threshold: config::DEFAULT_COMEMO_EVICTION_THRESHOLD,
        max_image_dimension_pixels: MAX_IMAGE_DIMENSION_PIXELS,
        max_image_pixels: MAX_IMAGE_PIXELS,
    };

    let fonts = Arc::new(typst_world::load_fonts(&cfg.font_dir())?);

    Ok(state::AppState {
        templates: Arc::new(HashMap::new()),
        data: Arc::new(RwLock::new(HashMap::new())),
        aliveness: state::AppAliveness::new(),
        fonts,
        pdf_library: Arc::new(typst_world::build_library(Features::default())),
        html_library: Arc::new(typst_world::build_library(
            [Feature::Html].into_iter().collect(),
        )),
        html_converter: Arc::new(build_html_converter(&cfg.font_dir(), &cfg.root_dir).0),
        root_dir: Arc::new(cfg.root_dir.clone()),
        resources_dir: Arc::new(cfg.resource_root()),
        // The existing performance benchmark sets this to None, which disables the
        // semaphore entirely and makes MAX_CONCURRENT_COMPILATIONS untestable.
        compile_semaphore: Some(Arc::new(Semaphore::new(concurrency))),
        config: cfg,
    })
}

async fn measure(fixture: &Fixture, concurrency: usize) -> anyhow::Result<Measurement> {
    let app_state = bench_state(concurrency)?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let port = listener.local_addr()?.port();
    let url = format!("http://127.0.0.1:{port}/api/v1/genpdf/image/bench");

    let server = tokio::spawn(
        axum::serve(
            listener,
            build_router(app_state, metrics::test_metrics_handle()),
        )
        .into_future(),
    );

    let image_bytes = axum::body::Bytes::from(std::fs::read(&fixture.path)?);
    let client = Arc::new(
        reqwest::Client::builder()
            .timeout(Duration::from_secs(300))
            .build()?,
    );

    let baseline_rss = rss_kb();
    let peak_rss = Arc::new(AtomicU64::new(0));
    let sampling = Arc::new(AtomicBool::new(true));
    let sampler = spawn_rss_sampler(Arc::clone(&peak_rss), Arc::clone(&sampling));

    let start = Instant::now();
    let mut join_set = JoinSet::new();
    for _ in 0..REQUESTS_PER_COMBINATION {
        let client = Arc::clone(&client);
        let url = url.clone();
        // Cloning `Bytes` is a refcount bump, so the client side does not add a
        // copy of the body per request and pollute the RSS measurement.
        let body = image_bytes.clone();
        let content_type = fixture.content_type;
        join_set.spawn(async move {
            let request_start = Instant::now();
            let response = client
                .post(&url)
                .header(header::CONTENT_TYPE, content_type)
                .body(body)
                .send()
                .await?;
            let status = response.status();
            let bytes = response.bytes().await?;
            anyhow::Ok((status, bytes.len(), request_start.elapsed().as_millis()))
        });
    }

    let mut durations = Vec::with_capacity(REQUESTS_PER_COMBINATION);
    let mut ok = 0;
    let mut rejected = 0;
    while let Some(joined) = join_set.join_next().await {
        let (status, len, elapsed_ms) = joined??;
        if status.is_success() && len > 0 {
            ok += 1;
        } else {
            rejected += 1;
            println!("  request returned {status}");
        }
        durations.push(elapsed_ms);
    }
    let total_ms = start.elapsed().as_millis();

    sampling.store(false, Ordering::Relaxed);
    sampler.await?;
    server.abort();

    durations.sort_unstable();
    let peak = peak_rss.load(Ordering::Relaxed);
    let peak_rss_growth_kb = baseline_rss.map(|base| peak.saturating_sub(base));

    Ok(Measurement {
        fixture: fixture.name,
        concurrency,
        ok,
        rejected,
        total_ms,
        p50_ms: percentile(&durations, 50),
        p95_ms: percentile(&durations, 95),
        max_ms: durations.last().copied().unwrap_or(0),
        peak_rss_growth_kb,
    })
}

fn percentile(sorted: &[u128], p: usize) -> u128 {
    if sorted.is_empty() {
        return 0;
    }
    let idx = (sorted.len() * p / 100).min(sorted.len() - 1);
    sorted[idx]
}

fn spawn_rss_sampler(peak: Arc<AtomicU64>, sampling: Arc<AtomicBool>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        while sampling.load(Ordering::Relaxed) {
            if let Some(current) = rss_kb() {
                peak.fetch_max(current, Ordering::Relaxed);
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
    })
}

#[cfg(target_os = "linux")]
fn rss_kb() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    status
        .lines()
        .find(|line| line.starts_with("VmRSS:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse().ok())
}

#[cfg(not(target_os = "linux"))]
fn rss_kb() -> Option<u64> {
    None
}

fn print_report(fixtures: &[Fixture], measurements: &[Measurement]) {
    println!("\n## Fixtures\n");
    println!("| fixture | dimensions | megapixels | file size | RGBA buffer if decoded |");
    println!("| --- | --- | --- | --- | --- |");
    for f in fixtures {
        let px = u64::from(f.width) * u64::from(f.height);
        println!(
            "| {} | {}x{} | {:.1} MP | {:.1} MiB | {:.0} MiB |",
            f.name,
            f.width,
            f.height,
            px as f64 / 1e6,
            f.file_len as f64 / (1024.0 * 1024.0),
            (px * 4) as f64 / (1024.0 * 1024.0)
        );
    }

    println!("\n## Load results ({REQUESTS_PER_COMBINATION} requests per row)\n");
    println!("| fixture | concurrency | ok | rejected | total ms | p50 ms | p95 ms | max ms | peak RSS growth |");
    println!("| --- | --- | --- | --- | --- | --- | --- | --- | --- |");
    for m in measurements {
        let rss = match m.peak_rss_growth_kb {
            Some(kb) => format!("{:.0} MiB", kb as f64 / 1024.0),
            None => "n/a (not Linux)".to_string(),
        };
        println!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            m.fixture,
            m.concurrency,
            m.ok,
            m.rejected,
            m.total_ms,
            m.p50_ms,
            m.p95_ms,
            m.max_ms,
            rss
        );
    }

    if rss_kb().is_none() {
        println!(
            "\nNote: peak RSS is only sampled on Linux. Run under \
             `docker run -m 3g` to measure memory and find the OOM threshold."
        );
    }

    println!(
        "\nNote: the semaphore gates compilation, not body buffering. Axum buffers each \
         request body in full before the handler runs, so {REQUESTS_PER_COMBINATION} concurrent \
         uploads hold {REQUESTS_PER_COMBINATION} bodies in memory regardless of \
         MAX_CONCURRENT_COMPILATIONS. Budget for that separately from the RGBA decode buffers."
    );

    println!(
        "\nNote: fixture generation itself allocates a full RGB buffer (~220 MiB for the \
         73 MP fixtures) and is included in the process RSS if it ran in this invocation. \
         Re-run to measure against cached fixtures."
    );
}
