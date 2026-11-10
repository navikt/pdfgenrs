# Contributing

This project accepts feature requests and contributions from the open source community. Fork the repository and create a branch for your change.

## Development environment

Install the following tools and verify that they are available:

| Tool | Check installation | Installation guide |
| --- | --- | --- |
| Rust | `rustc --version` | [Rust setup](https://rust-lang.org/tools/install/) |
| Cargo | `cargo --version` | [Rust setup](https://rust-lang.org/tools/install/)  |
| Docker | `docker --version` | [Install Docker](https://docs.docker.com/engine/install/) |
| curl | `curl --version` | [Install curl](https://curl.se/docs/install.html) |

> [!NOTE]
> Rust `1.98.1` or newer is required by `Cargo.toml`


## Development-commands
Run these commands before opening or updating a PR:
[development-commands](README.md#development-commands)


### Run k6 performance tests
See [Performance tests](performance-test/README.md)
for details.

### Run the application in dev mode
```bash
export DEV_MODE=true
cargo run 
```

### Validate the Docker image

Run these commands from the repository root. The [Dockerfile](Dockerfile) includes
the binary and default fonts, but not templates, test data, or resources. Mount the
repository fixtures:

```bash
docker build --tag pdfgenrs:test .
docker run --rm --name pdfgenrs-dev \
  -p 8080:8080 \
  -v "$(pwd)/templates:/app/templates:ro" \
  -v "$(pwd)/data:/app/data:ro" \
  -v "$(pwd)/resources:/app/resources:ro" \
  -e DEV_MODE=true \
  pdfgenrs:test
```

The container runs in the foreground. In another terminal, check readiness:

```bash
curl --fail --silent --show-error http://localhost:8080/internal/is_ready
```

Once readiness returns `200 OK`, generate a PDF using the bundled template and
test data through the dev-mode GET endpoint:

```bash
curl --fail --silent --show-error \
  --output output.pdf \
  http://localhost:8080/api/v1/genpdf/nav-brev/brev
```

## Pull requests

When your branch is ready, open a pull request. Maintainers will review the change and merge it when it is ready.
