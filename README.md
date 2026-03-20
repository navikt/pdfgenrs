# pdfgenrs

[![Build main](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/navikt/pdfgenrs/actions/workflows/build.yml)

![GitHub Release](https://img.shields.io/github/v/release/navikt/pdfgenrs)

⚠️WIP!⚠️

Repository for `pdfgenrs`, an application written in Rust used to create PDFs and HTMLs

## Technologies & Tools

* Rust
* Cargo
* Axum
* Handlebars
* Typst
* Prometheus
* Docker

## Getting started

Most commonly, pdfgenrs is used as a base image alongside templates, fonts, additional resources, and potential test data to verify that valid PDFs get produced by the aforementioned templates.

In your own repository, create a Dockerfile with the following contents

```dockerfile
# Dockerfile
FROM ghcr.io/navikt/pdfgenrs:<release>

COPY templates /app/templates # handlebars templates
COPY fonts /app/fonts         # fonts to be embedded
COPY resources /app/resources # additional resources
```

Check [GitHub releases](https://github.com/navikt/pdfgenrs/releases) to find the latest `release` version

Set up the basic folder structure
```bash
mkdir {templates,fonts,resources,data}
```

Create subfolders in `templates` and `data`
```bash
mkdir {templates,data}/your_teamname # your_teamname can be anything, but it'll be a necessary part of the API later
```

* `templates/your_teamname/` should then be populated with your .hbs-templates. the names of these templates will also decide parts of the API paths
* `data/your_teamname/` should be populated with json files with names corresponding to a target .hbs-template, this can be used to test your PDFs during development of templates.


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

Running the application locally enables a GET endpoint at `/api/v1/genpdf/<application>/<template>`
which looks for test data at `data/<application>/<template>.json` and outputs a PDF to your browser.
The template and data directory structure both follow the `<application>/<template>` structure.

To enable HTML document support, use the environment variable `ENABLE_HTML_ENDPOINT=true`. This will enable the 
HTML endpoints on `/api/v1/genhtml/<application>/<template>`. 

By default, pdfgenrs will load all assets (`templates`, `resources`, `data`) to memory on startup. Any change on files inside these folders will not be loaded before a restart of the application.

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
