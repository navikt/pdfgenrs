# Contributing

This project is open to accept feature requests and contributions from the open source community. Please fork the repo
and start a new branch to work on.

## How to set up a development environment

### Rust

Make sure you have the rust installed using this command:

```bash script
rustc --version
```

else install it https://rust-lang.org/tools/install/

### Cargo

Make sure you have cargo installed using this command:

```bash script
cargo --version
```

else install it https://doc.rust-lang.org/cargo/getting-started/installation.html

### Docker

Make sure you have the Docker installed You can check which version you have installed using this command:

```bash script
docker --version
```

else install it https://docs.docker.com/engine/install/

## Building locally

To run a build simply execute the following:

```shell script
cargo build
```

also run check formatting

```shell script
cargo fmt -- --check
```

and also run the linter

```shell script
cargo clippy --all-targets -- -D warnings
```

If this change can affect performance, you have run this command

```shell script
RUST_LOG=info GITHUB_STEP_SUMMARY=/tmp/bench-summary.md cargo bench --bench performance
cat /tmp/bench-summary.md
```

## Testing

If you are adding a new feature or bug fix please ensure there is proper test coverage. execute the following to run
test:

```shell script
cargo test
```

## Pull Request Review

If you have a branch on your fork that is ready to be merged, please create a new pull request. The maintainers will
review to make sure the above guidelines have been followed and if the changes are helpful to all library users, they
will be merged.
