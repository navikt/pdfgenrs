# Contributing

This project accepts feature requests and contributions from the open source community. Fork the repository and create a branch for your change.

## Development environment

### Rust and Cargo

`pdfgenrs` is pinned to Rust `1.98.1` (`Cargo.toml` and CI workflows). Verify your setup:

```bash
rustc --version
cargo --version
```

Install Rust/Cargo with [rustup](https://rust-lang.org/tools/install/) if needed.

### Docker

Docker is optional for normal development, but useful when validating Docker image behavior:

```bash
docker --version
```

Install Docker if needed: https://docs.docker.com/engine/install/

## Local commands
Run these commands before opening or updating a PR:

```bash
cargo audit
cargo fmt -- --check
cargo clippy --locked --release --all-targets -- -D warnings
cargo test --locked -- --nocapture
cargo build --locked --release
```

Install the dependency audit tool once:
```bash
cargo install cargo-audit --locked
```
Then run the dependency auditcheck
```bash
cargo audit
```

## Additional development commands

### Run k6 performance tests

Install [Grafana k6](https://grafana.com/docs/k6/latest/set-up/install-k6/), then run:

```bash
cargo build --locked --release
bash performance-test/run.sh
```

The runner starts a local server, waits for readiness,
runs k6, and stops the server. See [Performance tests](README.md#performance-tests)
for workloads, thresholds, configuration, and results.

### Run the application in dev mode
```bash
export DEV_MODE=true
cargo run 
```

### Validate the Docker image locally

Run these commands from the repository root. The [Dockerfile](Dockerfile) includes
the binary and default fonts, but not templates, test data, or resources. Mount the
repository fixtures as in the [Docker test workflow](.github/workflows/test-dockerfile.yml):

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

Stop the container with Ctrl+C. Templates and test data are loaded at startup,
so restart the container after editing them. Rebuild the image after code changes.

## Pull requests

When your branch is ready, open a pull request. Maintainers will review the change and merge it when it is ready.
