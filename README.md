# pdfgenrs

![pdfgenrs logo](resources/pdfgenrs-logo.svg)

[![Build main](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml)
![GitHub Release](https://img.shields.io/github/v/release/navikt/pdfgenrs)

`pdfgenrs` is a Rust application for generating PDFs through an API. It supports PDF/A-2a and PDF/UA-1 standards at once.

> **Note:** This project is tailored to Nav's [NAIS platform](https://doc.nais.io/), but it can be adapted to suit other environments and needs with small changes (e.g., adjusting health-check paths, removing OpenTelemetry/NAIS-specific configuration).

## Table of contents

- [Quick start](#quick-start)
- [Technologies and tools](#technologies-and-tools)
- [Folder structure](#folder-structure)
- [API](#api)
- [Applications that use pdfgenrs](#applications-that-use-pdfgenrs)
- [Environment variables](#environment-variables)
- [Developing pdfgenrs](#developing-pdfgenrs)
- [Benchmark reports](#benchmark-reports)
- [Release](#release)
- [Contact](#contact)
- [Contributing](#contributing)

## Quick start

Most teams use `pdfgenrs` as a base image together with their own templates. The base image already includes default fonts.

1. Create a Dockerfile in your own repository:

```dockerfile
FROM ghcr.io/navikt/pdfgenrs:<release>

COPY templates /app/templates
```

Find the latest `<release>` in [GitHub releases](https://github.com/navikt/pdfgenrs/releases).

2. Create the basic folder structure:

```bash
mkdir -p templates/your_appname
```

3. Add a Typst template (e.g., `templates/your_appname/your_template.typ`), then run a request:

```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/your_appname/your_template \
  -H "Content-Type: application/json" \
  -d '{"key":"value"}' \
  --output output.pdf
```

4. (Optional) Add custom fonts or resources:

If your templates use custom fonts or reference resources (e.g., logos), add them to your Dockerfile:

```dockerfile
FROM ghcr.io/navikt/pdfgenrs:<release>

COPY templates /app/templates
COPY fonts /app/fonts
COPY resources /app/resources
```

Create the corresponding directories locally:

```bash
mkdir -p fonts resources
```

### Example implementation
see https://github.com/navikt/pdfgenrs-test

## Technologies and tools

- [Rust](https://rust-lang.org/)
- [Cargo](https://crates.io/)
- [Axum](https://docs.rs/axum/latest/axum/)
- [Docker](https://www.docker.com/)
- [Typst](https://typst.app/#start)
- [JSON](https://www.json.org/json-en.html)

## Folder structure

- `templates/your_appname/`
  - Add `.typ` Typst templates.
  - Template file names are part of API paths.
  - Templates can read JSON with `#let data = json("/data/{app_name}/{template_name}.json")`.
- `data/your_appname/`
  - Add JSON files matching template names for local testing.
- `fonts/`
  - Add `.ttf`, `.otf`, or `.ttc` fonts used by templates.
- `resources/`
  - Add other assets your templates need.

For template examples, see [templates](templates).

## API

Base URL (local): `http://localhost:8080`

`<your_appname>` maps to a folder under `templates/`, and `<template>` maps to a `.typ` file in that folder.

Example:

- Template file: `templates/pale-2/pale-2.typ`
- Endpoint path: `/api/v1/genpdf/pale-2/pale-2`

### Endpoint overview

| Endpoint                                    | Method | Request Content-Type                                        | Response Content-Type      | Notes                |
|---------------------------------------------|--------|-------------------------------------------------------------|----------------------------|----------------------|
| `/api/v1/genpdf/{your_appname}/{template}`  | `POST` | `application/json`                                          | `application/pdf`          | Typst + JSON to PDF  |
| `/api/v1/genpdf/html/{your_appname}`        | `POST` | `text/html`                                                 | `application/pdf`          | HTML to PDF          |
| `/api/v1/genpdf/image/{your_appname}`       | `POST` | `image/png`, `image/jpeg`, `image/webp`, or `image/svg+xml` | `application/pdf`          | Image to PDF         |
| `/api/v1/genhtml/{your_appname}/{template}` | `POST` | `application/json`                                          | `text/html; charset=utf-8` | Typst + JSON to HTML |
| `/internal/is_alive`                        | `GET`  | -                                                           | -                          | Liveness             |
| `/internal/is_ready`                        | `GET`  | -                                                           | -                          | Readiness            |
| `/internal/metrics`                         | `GET`  | -                                                           | `text/plain`               | Prometheus metrics   |

### Request body size limit

All `POST` endpoints enforce a request body limit of `2097152` bytes (2 MiB), including:

- `POST /api/v1/genpdf/html/{your_appname}`
- `POST /api/v1/genpdf/image/{your_appname}`
- `POST /api/v1/genpdf/{your_appname}/{template}`
- `POST /api/v1/genhtml/{your_appname}/{template}`

Set environment variable `REQUEST_BODY_LIMIT_BYTES` to tune this limit. Example in Dockerfile for 3 MiB:

```dockerfile
FROM ghcr.io/navikt/pdfgenrs:<release>

COPY templates /app/templates
ENV REQUEST_BODY_LIMIT_BYTES=3145728

```

### 1) Generate PDF from Typst + JSON

#### `POST /api/v1/genpdf/{your_appname}/{template}`

Compiles a Typst template using JSON request data and returns a PDF.

- Request Content-Type: `application/json`
- Response Content-Type: `application/pdf`
- Success: `200 OK`
- Common errors:
  - `404 Not Found` (template/app not found)
  - `500 Internal Server Error` (rendering failed)

```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/<your_appname>/<template> \
  -H "Content-Type: application/json" \
  -d '{"key":"value"}' \
  --output output.pdf
```

### 2) Generate PDF from HTML

#### `POST /api/v1/genpdf/html/{your_appname}`

Converts HTML in the request body to a PDF.

- Request Content-Type: typically `text/html`
- Response Content-Type: `application/pdf`
- Success: `200 OK`
- Common errors:
  - `500 Internal Server Error`

```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/html/<your_appname> \
  -H "Content-Type: text/html" \
  --data-binary '<html><body><h1>Hello</h1></body></html>' \
  --output output.pdf
```

### 3) Generate PDF from image

#### `POST /api/v1/genpdf/image/{your_appname}`

Converts an image to PDF.

- Supported Request Content-Type:
  - `image/png`
  - `image/jpeg`
  - `image/webp`
  - `image/svg+xml`
- Response Content-Type: `application/pdf`
- Success: `200 OK`
- Common errors:
  - `415 Unsupported Media Type` (if the image format is not supported)
  - `500 Internal Server Error`

```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/image/<your_appname> \
  -H "Content-Type: image/png" \
  --data-binary @image.png \
  --output output.pdf
```

### 4) Generate HTML from Typst + JSON

#### `POST /api/v1/genhtml/{your_appname}/{template}`

Compiles a Typst template using JSON request data and returns HTML.

- Request Content-Type: `application/json`
- Response Content-Type: `text/html; charset=utf-8`
- Success: `200 OK`
- Common errors:
  - `404 Not Found` (template/app not found)
  - `500 Internal Server Error` (rendering failed)

```bash
curl -s -X POST http://localhost:8080/api/v1/genhtml/<your_appname>/<template> \
  -H "Content-Type: application/json" \
  -d '{"key":"value"}'
```

### Dev mode only endpoints (`DEV_MODE=true`)

When `DEV_MODE=true`, test data from `data/{your_appname}/{template}.json` is loaded and GET endpoints are enabled:

- `GET /api/v1/genpdf/{your_appname}/{template}` → returns `application/pdf`
- `GET /api/v1/genhtml/{your_appname}/{template}` → returns `text/html; charset=utf-8`

These endpoints return:

- `200 OK` on success
- `404 Not Found` if template or test data is missing

When `DEV_MODE=false`, these GET endpoints are not available (`405 Method Not Allowed`).

### Health endpoints

#### `GET /internal/is_alive`

- `200 OK` when alive
- `503 Service Unavailable` otherwise

#### `GET /internal/is_ready`

- `200 OK` when ready
- `503 Service Unavailable` otherwise

### Metrics endpoint

#### `GET /internal/metrics`

Exposes Prometheus metrics for operational monitoring.

- Response Content-Type: `text/plain`
- Success: `200 OK`

**Metrics exposed:**

| Metric                          | Type      | Labels               | Description                    |
|---------------------------------|-----------|----------------------|--------------------------------|
| `http_requests_total`           | Counter   | method, path, status | Total number of HTTP requests  |
| `http_request_duration_seconds` | Histogram | method, path, status | Request latency distribution   |
| `http_request_body_size_bytes`  | Histogram | method, path, status | Request body size distribution |
| `http_response_body_size_bytes` | Histogram | method, path, status | Response body size distribution|
| `typst_compilation_duration_seconds` | Histogram | output           | Typst compilation latency distribution |
| `comemo_evictions_total`        | Counter   | output               | Number of comemo cache eviction runs |
| `comemo_eviction_threshold`     | Gauge     | -                    | Configured `COMEMO_EVICTION_THRESHOLD` value |

By default, pdfgenrs loads all assets (`templates`, `data`) into memory on startup. Changes to files in these folders require an application restart.

Font files are loaded from `FONTS_DIR` (default: `fonts`) on startup.

## Applications that use pdfgenrs

- https://github.com/navikt/pdfgenrs-test
- https://github.com/navikt/pengeflyt-pdfgenrs
- https://github.com/navikt/helse-sprinter
- https://github.com/navikt/aap-pdfgenerator
- https://github.com/navikt/tiltakspenger-pdfgenrs
- https://github.com/navikt/orkivar-pdfgen
- https://github.com/navikt/mulighetsrommet/tree/main/mulighetsrommet-pdfgenrs
- https://github.com/navikt/pia-pdfgen
- https://github.com/navikt/klage-dittnav-pdfgen

## Environment variables

All configuration is done through environment variables. If an environment variable is not set, the default value is used.

| Variable                      | Description                                                                                                                                              | Default           |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------|
| `SERVER_PORT`                 | TCP port the server listens on                                                                                                                           | `8080`            |
| `ROOT_DIR`                    | Root directory used as the Typst filesystem root. Relative directory paths are resolved from this directory.                                             | `.`               |
| `TEMPLATES_DIR`               | Directory containing Typst template files                                                                                                                | `templates`       |
| `RESOURCES_DIR`               | Directory containing static resource files (e.g., logos)                                                                                                 | `resources`       |
| `DATA_DIR`                    | Directory containing test JSON data used in dev mode                                                                                                     | `data`            |
| `FONTS_DIR`                   | Directory containing font files used by Typst                                                                                                            | `fonts`           |
| `DEV_MODE`                    | When `true`, enables GET endpoints and loads test data from `DATA_DIR`                                                                                   | `false`           |
| `REQUEST_BODY_LIMIT_BYTES`    | Maximum accepted request body size in bytes                                                                                                              | `2097152` (2 MiB) |
| `COMPILE_TIMEOUT_SECONDS`     | Maximum time in seconds allowed for a single compilation task. Requests exceeding this timeout are aborted with `408 Request Timeout`.                   | `30`              |
| `SHUTDOWN_DRAIN_SECONDS`      | Duration in seconds to wait between marking the application as not ready and not alive during shutdown, allowing Kubernetes to stop routing new traffic. | `5`               |
| `MAX_CONCURRENT_COMPILATIONS` | Maximum number of concurrent compilation tasks allowed. `0` disables the limit.                                                                          | `4`               |
| `SEMAPHORE_ACQUIRE_TIMEOUT_SECONDS` | Maximum time in seconds to wait for a compilation semaphore permit. Exceeded timeout returns `503 Service Unavailable`.                              | `10`              |
| `COMEMO_EVICTION_THRESHOLD`         | Maximum number of cache entries to evict from the comemo memoization cache after each compilation. Higher values free more memory at the cost of cache hit rate. Set to `0` to evict the entire cache. | `15`              |
| `MAX_IMAGE_DIMENSION_PIXELS`        | Maximum accepted width or height of an uploaded image, in pixels.                                                                                    | `8192`            |
| `MAX_IMAGE_PIXELS`                  | Maximum accepted total pixel count (width × height) of an uploaded image. This is the primary memory guard for the image endpoint.                    | `25000000` (25 MP) |

#### Sizing the image limits

`MAX_IMAGE_PIXELS` — not `REQUEST_BODY_LIMIT_BYTES` — is what protects the service
from running out of memory, because peak memory depends on the format:

| Format | How it is embedded | Peak memory |
| ------ | ------------------ | ----------- |
| JPEG   | Passed through to the PDF as `DCTDecode` without ever being decoded to pixels | Proportional to the **file size** |
| PNG, WebP | Decoded to RGBA (4 bytes per pixel), then deflated | Proportional to the **pixel count** |

A body size limit is therefore not a memory guard on its own. A well-compressed
synthetic PNG (screenshot, map, line drawing) can reach a very high pixel count in
a small file — for example, a 15 MB PNG of flat colours can exceed 100 megapixels,
which allocates a ~400 MB RGBA buffer.

Raising these limits requires matching changes elsewhere:

- **Pod memory.** Budget roughly `MAX_IMAGE_PIXELS × 4 bytes` per in-flight
  compilation, plus the deflate output.
- **`MAX_CONCURRENT_COMPILATIONS`.** Peak memory is approximately
  `MAX_CONCURRENT_COMPILATIONS × peak memory per request`. Lower it when raising
  the image limits, or scale out with more replicas instead.
- **`COMPILE_TIMEOUT_SECONDS`.** Note that this does **not** free resources: a
  compilation running on a blocking thread cannot be cancelled once it has
  started. The client receives `408`, but the work — and its memory — continues
  until it finishes. The `template_compilations_in_flight_after_timeout` gauge
  tracks this. Set the timeout high enough that large images actually complete.

Example for images up to 50 MB, matched to a pod with `memory` limits of `3Gi`:

```yaml
env:
  - name: REQUEST_BODY_LIMIT_BYTES
    value: "52428800"
  - name: MAX_IMAGE_DIMENSION_PIXELS
    value: "16384"
  - name: MAX_IMAGE_PIXELS
    value: "100000000"
  - name: MAX_CONCURRENT_COMPILATIONS
    value: "2"
  - name: COMPILE_TIMEOUT_SECONDS
    value: "60"
```

Note that `REQUEST_BODY_LIMIT_BYTES` applies to **all** endpoints, so raising it
also allows larger JSON and HTML payloads.

### Logging and tracing

`pdfgenrs` uses [`tracing`](https://docs.rs/tracing) with an [`EnvFilter`](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html) that reads the standard `RUST_LOG` environment variable to control log verbosity. The default level is `INFO`.

Examples:

```bash
# Show debug logs for pdfgenrs only
RUST_LOG=pdfgenrs=debug

# Show trace logs for all crates
RUST_LOG=trace

# Combine filters
RUST_LOG=pdfgenrs=debug,tower_http=trace
```

See the [`EnvFilter` documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives) for the full filter syntax.

#### OpenTelemetry (OTEL) variables

When deployed on [NAIS](https://doc.nais.io/), OpenTelemetry tracing is configured automatically via environment variables injected by the platform. These variables are also respected when running locally if you want to export spans to a collector:

| Variable                          | Description                                                                 | Default      |
|-----------------------------------|-----------------------------------------------------------------------------|--------------|
| `OTEL_EXPORTER_OTLP_ENDPOINT`    | gRPC endpoint for the OTLP span exporter. When unset, no spans are exported. | *(unset)*    |
| `OTEL_SERVICE_NAME`              | Logical service name attached to exported spans                              | `pdfgenrs`   |
| `OTEL_RESOURCE_ATTRIBUTES`       | Additional resource attributes (key=value pairs)                             | *(unset)*    |
| `OTEL_EXPORTER_OTLP_INSECURE`   | Use insecure (plaintext) gRPC connection                                     | *(unset)*    |

## Developing pdfgenrs

### Prerequisites

Make sure Rust and Cargo are installed:

```bash
rustc --version
cargo --version
```

### Development commands

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo build
cargo test
cargo bench --bench performance
cargo bench --bench criterion_bench
DEV_MODE=true cargo run
```

## Benchmark reports
https://navikt.github.io/pdfgenrs/dev/criterion-report/report/
https://navikt.github.io/pdfgenrs/dev/bench/

## Release

We use default GitHub releases.

This project follows [semantic versioning](https://semver.org/) and does **not** prefix tags or release titles with `v` (use `1.2.3`, not `v1.2.3`).

For release steps, see [Creating a release on GitHub](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository#creating-a-release)

## Contact

This project is currently maintained by [@navikt](https://github.com/navikt).

If you have questions, please create an issue and tag it with the appropriate label.

For contact requests within the [@navikt](https://github.com/navikt) org, use the Slack channel `#pdfgen`

## Contributing

To get started, fork the repository and create a new branch.

See more info in [CONTRIBUTING.md](CONTRIBUTING.md)
