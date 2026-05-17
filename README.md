# pdfgenrs

[![Build main](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml)

![GitHub Release](https://img.shields.io/github/v/release/navikt/pdfgenrs)

Repository for `pdfgenrs`, an application written in Rust used to create PDFs through an API

## Technologies & Tools

* [Rust](https://rust-lang.org/)
* [Cargo](https://crates.io/)
* [Axum](https://docs.rs/axum/latest/axum/)
* [Docker](https://www.docker.com/)
* [Typst](https://typst.app/#start)
* [Json](https://www.json.org/json-en.html)


## Getting started

Most commonly, pdfgenrs is used as a base image alongside templates, fonts, additional resources, and potential test data to verify that valid PDFs get produced by the aforementioned templates.

In your own repository, create a Dockerfile with the following contents

```dockerfile
# Dockerfile
FROM ghcr.io/navikt/pdfgenrs:<release>

COPY templates /app/templates # typst templates

COPY fonts /app/fonts         # fonts to be embedded

COPY resources /app/resources # additional resources
```

Check [GitHub releases](https://github.com/navikt/pdfgenrs/releases) to find the latest `release` version

Set up the basic folder structure
```bash
mkdir {templates,resources,data,fonts}
```

Create subfolders in `templates` and `data`
```bash
mkdir {templates,data}/your_appname # your_appname can be anything, but it'll be a necessary part of the API later
```

* `templates/your_appname/` should then be populated with your `.typ` Typst templates. the names of these templates will also decide parts of the API paths. Templates receive JSON data via `#let data = json("/data.json")`.
* `data/your_appname/` should be populated with json files with names corresponding to a target `.typ` template, this can be used to test your PDFs during development of templates.
* `fonts/` should contain the `.ttf`, `.otf`, or `.ttc` files used by your templates.

* For example typ templates see: [templates](templates)
  

### Applications that uses pdfgenrs
- https://github.com/navikt/pdfgenrs-test
- https://github.com/navikt/pale-2-pdfgenrs

## API

Base URL (local): `http://localhost:8080`

`<your_appname>` maps to a folder under `templates/`, and `<template>` maps to a `.typ` file name in that folder.

Example:

- Template file: `templates/pale-2/pale-2.typ`
- Endpoint path: `/api/v1/genpdf/pale-2/pale-2`

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

### 3) Generate PDF from Image

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

### Dev Mode only endpoints (`DEV_MODE=true`)

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

By default, pdfgenrs will load all assets (`templates`, `data`) to memory on startup. Any change on files inside these folders will not be loaded before a restart of the application.
Font files are loaded from `FONTS_DIR` (default: `fonts`) on startup.

## Developing pdfgenrs

### Prerequisites
Make sure you have the rust installed using this command:
#### Rust
```bash script
rustc --version
```

#### Cargo
Make sure you have cargo installed using this command:
```bash script
cargo --version
```

### Formating
Format the code
```bash script
cargo fmt
```

### Linting
Run the linter
```bash script
cargo clippy --all-targets -- -D warnings
```

### Build
Build the code without running it
```bash script
cargo build
```

### Tests
To run the tests
```bash script
cargo test
```

### Run
Run the code
```bash script
cargo run
```

### Release
We use default GitHub release. 
This project uses [semantic versioning](https://semver.org/) and does NOT prefix tags or release titles with `v` i.e. use `1.2.3` instead of `v1.2.3` 

See guide about how to release: [creating release github](
https://docs.github.com/en/repositories/releasing-projects-on-github/managing-releases-in-a-repository#creating-a-release)


## 👥 Contact

This project is currently maintained by the organisation [@navikt](https://github.com/navikt).

If you need to raise an issue or question about this library, please create an issue here and tag it with the appropriate label.

For contact requests within the [@navikt](https://github.com/navikt) org, you can use the Slack channel #pdfgen

If you need to contact anyone directly, please see [CODEOWNERS](CODEOWNERS)

## ✏️ Contributing

To get started, please fork the repo and checkout a new branch. You can then build the library

```shell script
cargo build
```

See more info in [CONTRIBUTING.md](CONTRIBUTING.md)
