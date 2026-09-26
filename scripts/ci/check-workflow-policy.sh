#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

cargo_toml="$repo_root/Cargo.toml"
build_workflow="$repo_root/.github/workflows/build.yml"
pr_workflow="$repo_root/.github/workflows/pull-requests.yml"
benchmark_workflow="$repo_root/.github/workflows/criterion-benchmark.yml"

rust_version="$(sed -nE 's/^rust-version = "([^"]+)"/\1/p' "$cargo_toml")"
if [[ -z "$rust_version" ]]; then
  echo "Failed to read rust-version from Cargo.toml" >&2
  exit 1
fi

assert_single_toolchain_pin() {
  local workflow="$1"
  local pins unique_pin

  pins="$(grep -Eo 'dtolnay/rust-toolchain@[0-9]+\.[0-9]+\.[0-9]+' "$workflow" | sed 's/.*@//' | sort -u || true)"
  if [[ -z "$pins" ]]; then
    echo "No dtolnay/rust-toolchain pin found in $workflow" >&2
    exit 1
  fi

  if [[ "$(printf '%s\n' "$pins" | wc -l | tr -d ' ')" -ne 1 ]]; then
    echo "Expected a single rust-toolchain pin in $workflow, found:" >&2
    printf '%s\n' "$pins" >&2
    exit 1
  fi

  unique_pin="$pins"
  if [[ "$unique_pin" != "$rust_version" ]]; then
    echo "Toolchain version drift in $workflow: expected $rust_version, found $unique_pin" >&2
    exit 1
  fi
}

assert_cache_key_prefix() {
  local workflow="$1"
  local expected="rust-${rust_version}-cargo-"

  if ! grep -Fq "$expected" "$workflow"; then
    echo "Missing cache key prefix '$expected' in $workflow" >&2
    exit 1
  fi
}

extract_build_commands() {
  local workflow="$1"

  awk '
    /^      - name: (Check formatting|Run Clippy|Run tests|Run Benchmark performance test|Build)$/ {
      step_name = $0
      sub(/^      - name: /, "", step_name)
      capture = 1
      next
    }
    capture && /^        run: / {
      command = $0
      sub(/^        run: /, "", command)
      print step_name "=" command
      capture = 0
    }
  ' "$workflow"
}

assert_single_toolchain_pin "$build_workflow"
assert_single_toolchain_pin "$pr_workflow"
assert_single_toolchain_pin "$benchmark_workflow"

assert_cache_key_prefix "$build_workflow"
assert_cache_key_prefix "$pr_workflow"
assert_cache_key_prefix "$benchmark_workflow"

build_commands="$(extract_build_commands "$build_workflow")"
pr_commands="$(extract_build_commands "$pr_workflow")"

if [[ -z "$build_commands" || -z "$pr_commands" ]]; then
  echo "Failed to extract duplicated build commands from workflows" >&2
  exit 1
fi

if [[ "$build_commands" != "$pr_commands" ]]; then
  echo "Command drift detected between build.yml and pull-requests.yml" >&2
  diff -u <(printf '%s\n' "$build_commands") <(printf '%s\n' "$pr_commands") || true
  exit 1
fi

echo "Workflow policy checks passed."
