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
release_expected_packages=41

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

# Packages run concurrently, each through its own Oxide route. Every invocation
# writes only its own file, and packages.txt fixes the order, so the baseline
# bytes match a sequential run. xargs exits non-zero when any invocation fails,
# which fails this script. SWALLOWTAIL_PUBLIC_API_JOBS overrides the default of
# the CPU count, capped at eight.
release_jobs=${SWALLOWTAIL_PUBLIC_API_JOBS:-}
if [[ -z $release_jobs ]]; then
  release_jobs=$(getconf _NPROCESSORS_ONLN 2>/dev/null || printf '4')
  if (( release_jobs > 8 )); then
    release_jobs=8
  fi
fi
if [[ ! $release_jobs =~ ^[1-9][0-9]*$ ]]; then
  printf 'SWALLOWTAIL_PUBLIC_API_JOBS must be a positive integer, found %s\n' \
    "$release_jobs" >&2
  exit 2
fi

printf '%s\n' "$release_packages" | xargs -P "$release_jobs" -I {} \
  bash -c '
    release_status=0
    RUSTUP_TOOLCHAIN=$1 bash "$2/scripts/oxide-rust.sh" public-api \
      --package "$3" \
      --all-features \
      --simplified --simplified --simplified \
      --color never \
      > "$4/$3.txt" < /dev/null || release_status=$?
    if (( release_status != 0 )); then
      printf "public-api extraction failed for %s (exit %s)\n" "$3" "$release_status" >&2
      exit "$release_status"
    fi
  ' _ "$release_toolchain" "$release_repo_root" {} "$release_output_dir"
