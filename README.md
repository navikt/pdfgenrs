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
- https://github.com/navikt/pale-2-pdfgenrs
- https://github.com/navikt/smarbeidsgiver-pdfgenrs
- https://github.com/navikt/pdfgenrs-test

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

### Build
Build the code without running it
```bash script
cargo build
```

### Run
Run the code
```bash script
cargo run
```

Running the application with environment DEV_MODE = true will exposes a GET endpoint at `/api/v1/genpdf/<your_appname>/<template>`
which looks for test data at `data/<your_appname>/<template>.json` and outputs a PDF to your browser.
The template and data directory structure both follow the `<your_appname>/<template>` structure.

To generate a PDF by posting JSON data directly, use the POST endpoint:
```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/<your_appname>/<template> \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}' \
  --output output.pdf
```

pdfgenrs also exposes a endpoint HTML-to-PDF route:
```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/html/<your_appname> \
  -H "Content-Type: text/html" \
  --data-binary '<html><body>Hello</body></html>'
```
This endpoint converts the posted HTML into a PDF and returns it as `application/pdf`.

pdfgenrs also exposes a endpoint image-to-PDF route:
```bash
curl -s -X POST http://localhost:8080/api/v1/genpdf/image/<your_appname> \
  -H "Content-Type: image/png" \
  --data-binary @image.png \
  --output output.pdf
```
This endpoint accepts `image/png` and `image/jpeg` request bodies and returns `application/pdf`.

Similarly, pdfgenrs exposes a `POST /api/v1/genhtml/<your_appname>/<template>` endpoint that compiles the Typst template with the provided JSON data and returns the result as HTML:
```bash
curl -s -X POST http://localhost:8080/api/v1/genhtml/<your_appname>/<template> \
  -H "Content-Type: application/json" \
  -d '{"key": "value"}'
```

Running with DEV_MODE = true also exposes a GET endpoint at `/api/v1/genhtml/<your_appname>/<template>`
which looks for test data at `data/<your_appname>/<template>.json` and returns the rendered HTML in your browser.

By default, pdfgenrs will load all assets (`templates`, `data`) to memory on startup. Any change on files inside these folders will not be loaded before a restart of the application.
Font files are loaded from `FONTS_DIR` (default: `fonts`) on startup.

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
