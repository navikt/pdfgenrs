# Rust Best Practices Audit

A full read of the source tree (`src/`) produced the findings below. They are
grouped by priority. Each item describes the problem, its location, and a
suggested fix.

---

## 🔴 High Priority

### 1. `std::env::set_var` / `remove_var` used in tests (deprecated in Rust 1.81)

**Location:** `src/tracing_setup.rs` — `resolve_service_name_uses_env_value`,
`resolve_service_name_falls_back_when_missing_or_empty`,
`nais_otlp_exporter_is_none_without_endpoint`

`std::env::set_var` is deprecated in Rust 1.81 because it is unsound in
multi-threaded programs (even with a mutex the OS-level `setenv(3)` is still
called without holding a process-wide lock that the C runtime respects). The
tests already use a `Mutex` to serialise access within Rust, which is the
currently accepted mitigation, but the deprecation warning is loud and will
become a hard error in a future edition.

**Recommendation:** Call the deprecated functions through a thin `#[allow]` scope
comment, or switch to a crate such as `temp-env` that wraps `set_var` correctly,
or refactor the code under test to accept the value as a parameter so
`set_var` is unnecessary.

---

### 2. `ContextGuard`-like return value silently discarded with `let _ =`

**Location:** `src/main.rs:70`

```rust
let _ = span.set_parent(parent_cx);
```

`OpenTelemetrySpanExt::set_parent` currently returns `()`, so the `let _ =`
is harmless—but it implies the author expected a value that needs to be
discarded. If a future upgrade of `tracing-opentelemetry` changes the return
type to a guard (as some OTel APIs do), silently dropping it would break
context propagation without any compile-time warning.

**Recommendation:** Remove the `let _ =` and call `span.set_parent(parent_cx)`
directly. If a future API returns a guard, the compiler will force you to handle
it.

---

## 🟡 Medium Priority

### 3. Redundant `#[cfg(test)]` in `performance_test.rs`

**Location:** `src/performance_test.rs:1-3`

```rust
#[cfg(test)]
mod tests {
```

`performance_test` is already conditionally compiled via the `#[cfg(test)] mod
performance_test;` declaration in `main.rs`. The inner `#[cfg(test)]` attribute
on `mod tests` is redundant and misleading.

**Recommendation:** Remove the inner `#[cfg(test)]` attribute (keep `mod tests`
or move the items to the module root).

---

### 4. `get_html` handler is missing timing instrumentation

**Location:** `src/routes/html.rs` — `get_html`

`post_html` records `start = std::time::Instant::now()` and logs
`duration_ms` on success. The `get_html` handler has neither the timer nor the
`info!` log, making dev-mode GET requests invisible in traces and dashboards.

**Recommendation:** Add the same timing/logging pattern to `get_html` that
`post_html` already uses, keeping both handlers consistent.

---

### 5. `collect_font_files` re-implements directory walking already provided by `walkdir`

**Location:** `src/typst_world.rs:65-81`

```rust
fn collect_font_files(dir: &Path, files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
```

`walkdir` is already a declared dependency and is used in `template.rs`. The
hand-rolled recursive `read_dir` loop does not follow symlinks by default and
adds code that must be maintained separately.

**Recommendation:** Replace `collect_font_files` with a `WalkDir::new(dir).follow_links(true)` iterator, consistent with `load_templates_from_dir`.

---

### 6. Magic number `15` in `comemo::evict` calls

**Location:** `src/typst_world.rs:225, 285`

```rust
comemo::evict(15);
```

The integer `15` has no in-line explanation. Readers have no way to know what
cache generation threshold was chosen or why.

**Recommendation:** Introduce a named constant:

```rust
const COMEMO_EVICT_AGE: usize = 15;
comemo::evict(COMEMO_EVICT_AGE);
```

and add a brief comment explaining the trade-off (cache hit-rate vs. memory).

---

### 7. Unnecessary `image_bytes.to_vec()` copy

**Location:** `src/routes/pdf.rs:139`

```rust
gen_pdf::image_to_pdf(image_bytes.to_vec(), image_path, fonts, &root)
```

`image_bytes` is `axum::body::Bytes` (cheaply cloneable, reference-counted).
`image_to_pdf` accepts `Vec<u8>`, so a full copy is forced. For large images
this is an avoidable allocation.

**Recommendation:** Change `image_to_pdf`'s signature to accept `impl Into<Vec<u8>>`
or `bytes::Bytes` (which `ironpress` / `typst` may already accept).

---

### 8. Duplicated `make_state` test helper across multiple test modules

**Location:** `src/routes/pdf.rs`, `src/routes/html.rs`, `src/main.rs` (tests)

Three separate test modules define nearly identical `make_state()` functions
that construct an `AppState` with hardcoded test values. If `AppState` gains
a new field, all three copies must be updated.

**Recommendation:** Extract a shared `test_utils` module (inside `#[cfg(test)]`)
or a common helper crate that all test modules import.

---

### 9. `PdfgenWorld` is `pub` but is only used within the crate

**Location:** `src/typst_world.rs:95`

```rust
pub struct PdfgenWorld {
```

For a binary crate this makes no functional difference, but `pub` is misleading
— it signals that external consumers are expected. Consistency with the rest of
the crate (most internal items are `pub(crate)` or private) would improve
readability.

**Recommendation:** Change to `pub(crate) struct PdfgenWorld`.

---

## 🟢 Low Priority / Style

### 10. Missing `rust-version` (MSRV) in `Cargo.toml`

There is no `rust-version` key in `[package]`. Without it CI tooling cannot
enforce a minimum supported Rust version, and `cargo` cannot warn when a
dependency requires a newer compiler.

**Recommendation:** Add `rust-version = "1.81"` (or whatever the minimum
version tested in CI) to `Cargo.toml`.

---

### 11. No `[lints]` table or `.cargo/config.toml` clippy configuration

There is no project-wide clippy configuration (e.g. `[lints.clippy]` in
`Cargo.toml` or a `clippy.toml`). Without it, useful lints such as
`clippy::unwrap_used`, `clippy::expect_used` (in non-test code), and
`clippy::todo` are not enforced.

**Recommendation:** Add a `[lints.clippy]` table to `Cargo.toml` and configure
at minimum:

```toml
[lints.clippy]
unwrap_used = "warn"
expect_used = "warn"   # allow in tests via #[allow(clippy::expect_used)]
```

---

### 12. `NaisJsonFormat` could derive standard traits

**Location:** `src/tracing_setup.rs:21`

```rust
struct NaisJsonFormat;
```

The unit struct derives nothing. Adding `#[derive(Debug, Clone, Copy, Default)]`
costs nothing and aligns with Rust idioms for zero-sized types.

---

### 13. Stray blank line in `html.rs`

**Location:** `src/html.rs:9`

A blank line between `use crate::typst_world::{self, Fonts};` and the
function doc-comment is harmless but inconsistent with the other source files
and would be flagged by `rustfmt`.

**Recommendation:** Run `cargo fmt` (also consider adding a `rustfmt.toml` and
enforcing it in CI).

---

### 14. `FieldVisitor` defined inside a method body

**Location:** `src/tracing_setup.rs:86-106`

```rust
struct FieldVisitor<'a> { ... }
impl tracing::field::Visit for FieldVisitor<'_> { ... }
```

These are declared inside `format_event`. Rust supports this, but it hurts
readability and prevents the type from being reused or tested independently.

**Recommendation:** Hoist `FieldVisitor` to module scope as a private item.

---

## Summary

| # | Severity | Location | Issue |
|---|----------|----------|-------|
| 1 | 🔴 High | `tracing_setup.rs` tests | `set_var`/`remove_var` deprecated in Rust 1.81 |
| 2 | 🔴 High | `main.rs:70` | Unnecessary `let _ =` may silently drop future guard |
| 3 | 🟡 Medium | `performance_test.rs` | Redundant inner `#[cfg(test)]` |
| 4 | 🟡 Medium | `routes/html.rs` | `get_html` missing timing/logging |
| 5 | 🟡 Medium | `typst_world.rs` | `collect_font_files` re-implements `walkdir` |
| 6 | 🟡 Medium | `typst_world.rs` | Magic number `15` in `comemo::evict` |
| 7 | 🟡 Medium | `routes/pdf.rs:139` | Unnecessary `to_vec()` copy for image bytes |
| 8 | 🟡 Medium | multiple test modules | Duplicated `make_state` helper |
| 9 | 🟡 Medium | `typst_world.rs:95` | `PdfgenWorld` should be `pub(crate)` |
| 10 | 🟢 Low | `Cargo.toml` | Missing `rust-version` (MSRV) |
| 11 | 🟢 Low | `Cargo.toml` | No clippy lint configuration |
| 12 | 🟢 Low | `tracing_setup.rs:21` | `NaisJsonFormat` missing standard derives |
| 13 | 🟢 Low | `html.rs:9` | Stray blank line (rustfmt) |
| 14 | 🟢 Low | `tracing_setup.rs:86` | `FieldVisitor` defined inside method body |
