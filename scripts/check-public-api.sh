#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_immutable_baseline_dir=release-baselines/public-api-0.4.0
release_baseline_dir=release-baselines/public-api-0.4.1
release_toolchain=nightly-2026-08-05
release_tool_version='cargo-public-api 0.52.0'

if ! command -v cargo-public-api >/dev/null 2>&1; then
  printf 'cargo-public-api 0.52.0 is required; install it with cargo install cargo-public-api --version 0.52.0 --locked\n' >&2
  exit 1
fi

if [[ $(cargo-public-api --version) != "$release_tool_version" ]]; then
  printf 'expected %s, found %s\n' "$release_tool_version" "$(cargo-public-api --version)" >&2
  exit 1
fi

if ! rustup run "$release_toolchain" rustc --version >/dev/null 2>&1; then
  printf '%s is required; install it with rustup toolchain install %s --profile minimal\n' \
    "$release_toolchain" "$release_toolchain" >&2
  exit 1
fi

release_actual_dir=$(mktemp -d)
trap 'rm -rf "$release_actual_dir"' EXIT

bash scripts/generate-public-api-baseline.sh "$release_actual_dir"
release_expected_packages="$release_baseline_dir/packages.txt"
diff -u "$release_expected_packages" "$release_actual_dir/packages.txt"
while IFS= read -r release_package; do
  release_api="$release_baseline_dir/$release_package.txt"
  if [[ -f "$release_immutable_baseline_dir/$release_package.txt" ]]; then
    release_removed_api="$release_actual_dir/$release_package.removed.txt"
    grep -Fvx -f "$release_api" \
      "$release_immutable_baseline_dir/$release_package.txt" \
      > "$release_removed_api" || true
    if [[ -s "$release_removed_api" ]]; then
      printf 'v0.4.1 release API removes an immutable v0.4.0 item: %s\n' \
        "$release_package" >&2
      cat "$release_removed_api" >&2
      exit 1
    fi
  fi
  diff -u \
    "$release_api" \
    "$release_actual_dir/$release_package.txt"
done < "$release_expected_packages"
printf 'semantic API passed: 40 packages at v0.4.1; immutable v0.4.0 remains 40; no API removals are permitted in this patch\n'
