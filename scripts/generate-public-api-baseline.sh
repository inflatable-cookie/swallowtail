#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

if [[ $# -ne 1 || -z $1 ]]; then
  printf 'usage: %s OUTPUT_DIRECTORY\n' "$0" >&2
  exit 2
fi

release_output_dir=$1
release_toolchain=nightly-2026-08-05
release_tool_version='cargo-public-api 0.52.0'
release_expected_packages=30

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

mkdir -p "$release_output_dir"

release_packages=$(for release_manifest in crates/*/Cargo.toml; do
  sed -n 's/^name = "\([^"]*\)"$/\1/p' "$release_manifest" | head -n 1
done | LC_ALL=C sort)

release_package_count=$(printf '%s\n' "$release_packages" | wc -l | tr -d ' ')
if [[ $release_package_count -ne $release_expected_packages ]]; then
  printf 'expected %s public workspace packages, found %s\n' \
    "$release_expected_packages" "$release_package_count" >&2
  exit 1
fi

printf '%s\n' "$release_packages" > "$release_output_dir/packages.txt"

while IFS= read -r release_package; do
  cargo +"$release_toolchain" public-api \
    --package "$release_package" \
    --all-features \
    --simplified --simplified --simplified \
    --color never \
    > "$release_output_dir/$release_package.txt"
done <<< "$release_packages"
