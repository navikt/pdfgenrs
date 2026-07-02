# Contributing to ironpress

Thanks for your interest in contributing!

## Getting Started

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Make your changes
4. Run the checks:
   ```bash
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo test
   ```
5. Commit and push your branch
6. Open a pull request against `main`

## Guidelines

- All code must pass `cargo fmt`, `cargo clippy -- -D warnings`, and `cargo test`
- Add tests for new functionality
- Keep PRs focused on a single change
- Follow existing code style and conventions

## Reporting Issues

Open an issue on GitHub with a clear description and, if possible, a minimal reproduction.
