# pdfgenrs

![pdfgenrs logo](resources/pdfgenrs-logo.svg)

[![Build main](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml)
![GitHub Release](https://img.shields.io/github/v/release/navikt/pdfgenrs)

`pdfgenrs` is a Rust application for generating PDFs through an API.

## Table of contents

- [Quick start](#quick-start)
- [Technologies and tools](#technologies-and-tools)
- [Folder structure](#folder-structure)
- [API](#api)
- [Applications that use pdfgenrs](#applications-that-use-pdfgenrs)
- [Developing pdfgenrs](#developing-pdfgenrs)
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
  - Templates can read JSON with `#let data = json("/data.json")`.
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

| Endpoint | Method | Request Content-Type | Response Content-Type | Notes |
|---|---|---|---|---|
| `/api/v1/genpdf/{your_appname}/{template}` | `POST` | `application/json` | `application/pdf` | Typst + JSON to PDF |
| `/api/v1/genpdf/html/{your_appname}` | `POST` | `text/html` | `application/pdf` | HTML to PDF |
| `/api/v1/genpdf/image/{your_appname}` | `POST` | `image/png` or `image/jpeg` | `application/pdf` | Image to PDF |
| `/api/v1/genhtml/{your_appname}/{template}` | `POST` | `application/json` | `text/html; charset=utf-8` | Typst + JSON to HTML |
| `/internal/is_alive` | `GET` | - | - | Liveness |
| `/internal/is_ready` | `GET` | - | - | Readiness |
| `/internal/metrics` | `GET` | - | `text/plain` | Prometheus metrics |

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
- Response Content-Type: `application/pdf`
- Success: `200 OK`
- Common errors:
  - `415 Unsupported Media Type` (if not PNG/JPEG)
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
- `500 Internal Server Error` otherwise

#### `GET /internal/is_ready`

- `200 OK` when ready
- `500 Internal Server Error` otherwise

### Metrics endpoint

#### `GET /internal/metrics`

Exposes Prometheus metrics for operational monitoring.

- Response Content-Type: `text/plain`
- Success: `200 OK`

**Metrics exposed:**

| Metric | Type | Labels |
|--------|------|--------|
| `http_requests_total` | Counter | method, path, status |
| `http_request_duration_seconds` | Histogram | method, path, status |

By default, pdfgenrs loads all assets (`templates`, `data`) into memory on startup. Changes to files in these folders require an application restart.

Font files are loaded from `FONTS_DIR` (default: `fonts`) on startup.

## Applications that use pdfgenrs

- https://github.com/navikt/pdfgenrs-test
- https://github.com/navikt/pale-2-pdfgenrs

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
cargo run
```

## Release

We use default GitHub releases.

This project follows [semantic versioning](https://semver.org/) and does **not** prefix tags or release titles with `v` (use `1.2.3`, not `v1.2.3`).

For release steps, see [Creating a release on GitHub](https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository#creating-a-release).

## 👥 Contact

This project is currently maintained by [@navikt](https://github.com/navikt).

If you have questions, please create an issue and tag it with the appropriate label.

For contact requests within the [@navikt](https://github.com/navikt) org, use the Slack channel `#pdfgen`.

## ✏️ Contributing

To get started, fork the repository and create a new branch.

```bash
cargo build
```

See more info in [CONTRIBUTING.md](CONTRIBUTING.md).
