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

## CI-equivalent local checks

Run the same commands used by pull request CI before opening or updating a PR:

```bash
cargo fmt -- --check
cargo clippy --locked --release --all-targets -- -D warnings
cargo test --locked -- --nocapture
cargo bench --locked --bench performance
cargo build --locked --release
```

## Additional development commands

```bash
cargo bench --bench criterion_bench
DEV_MODE=true cargo run
```

## Pull requests

When your branch is ready, open a pull request. Maintainers will review the change and merge it when it is ready.
