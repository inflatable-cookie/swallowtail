#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."
source scripts/release-version-identity.sh
release_load_version_identity

guard_api_mode=${1:-check}
case "$guard_api_mode" in check|update) ;; *) exit 2 ;; esac
guard_api_root=$(mktemp -d)
trap 'rm -rf "$guard_api_root"' EXIT
for guard_api_package in swallowtail-runtime swallowtail-host-local swallowtail-adapter-claude-agent; do
  RUSTUP_TOOLCHAIN=nightly-2026-08-05 bash scripts/oxide-rust.sh public-api \
    --package "$guard_api_package" --all-features \
    --simplified --simplified --simplified --color never \
    > "$guard_api_root/$guard_api_package.txt"
  guard_api_baseline="release-baselines/public-api-$release_current_version/$guard_api_package.txt"
  # Existing released items remain immutable even when refreshing the working baseline.
  guard_api_previous="release-baselines/public-api-$release_previous_version/$guard_api_package.txt"
  if [[ -f "$guard_api_previous" ]]; then
    grep -Fvx -f "$guard_api_root/$guard_api_package.txt" "$guard_api_previous" \
      > "$guard_api_root/removed.txt" || true
    if [[ -s "$guard_api_root/removed.txt" ]]; then
      cat "$guard_api_root/removed.txt" >&2
      exit 1
    fi
  fi
  if [[ "$guard_api_mode" == update ]]; then
    cp "$guard_api_root/$guard_api_package.txt" "$guard_api_baseline"
  else
    diff -u "$guard_api_baseline" "$guard_api_root/$guard_api_package.txt"
  fi
done
