#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_immutable_baseline_dir=release-baselines/public-api-0.3.0
release_baseline_dir=release-baselines/public-api-0.4.0
release_approved_breaking_dir=release-baselines/public-api-unreleased/approved-v0.4.0-removals
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
while IFS= read -r release_approved_file; do
  release_approved_package=$(basename "$release_approved_file" .txt)
  if ! grep -Fxq "$release_approved_package" "$release_expected_packages"; then
    printf 'approved v0.4.0 removal names an unknown current package: %s\n' \
      "$release_approved_package" >&2
    exit 1
  fi
  if [[ ! -f "$release_immutable_baseline_dir/$release_approved_package.txt" ]]; then
    printf 'approved v0.4.0 removal names a package without an immutable baseline: %s\n' \
      "$release_approved_package" >&2
    exit 1
  fi
done < <(find "$release_approved_breaking_dir" -maxdepth 1 -type f -name '*.txt' | LC_ALL=C sort)
while IFS= read -r release_package; do
  release_api="$release_baseline_dir/$release_package.txt"
  if [[ -f "$release_immutable_baseline_dir/$release_package.txt" ]]; then
    release_removed_api="$release_actual_dir/$release_package.removed.txt"
    grep -Fvx -f "$release_api" \
      "$release_immutable_baseline_dir/$release_package.txt" \
      > "$release_removed_api" || true
    release_approved_removals="$release_approved_breaking_dir/$release_package.txt"
    if [[ -s "$release_removed_api" ]]; then
      if [[ ! -f "$release_approved_removals" ]]; then
        printf 'v0.4.0 release API removes an unapproved immutable v0.3.0 item: %s\n' \
          "$release_package" >&2
        exit 1
      fi
      diff -u "$release_approved_removals" "$release_removed_api"
    elif [[ -f "$release_approved_removals" ]]; then
      printf 'approved v0.4.0 removal is not removed from current API: %s\n' \
        "$release_package" >&2
      exit 1
    fi
  fi
  diff -u \
    "$release_api" \
    "$release_actual_dir/$release_package.txt"
done < "$release_expected_packages"
printf 'semantic API passed: 40 packages at v0.4.0; immutable v0.3.3 remains 40; v0.3.0 removals require exact approved v0.4.0 evidence\n'
