#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.2.0.env

release_installed_toolchains=$(rustup toolchain list)
for release_toolchain in \
  "$SWALLOWTAIL_MSRV" \
  "$SWALLOWTAIL_CURRENT_STABLE"
do
  # Escape the dots so `1.95.0` cannot match `1x95y0`. The list is captured
  # first, so grep -q closing early cannot race rustup under pipefail.
  release_pattern=${release_toolchain//./\\.}
  if ! grep -Eq "^${release_pattern}(-|\$)" <<< "$release_installed_toolchains"; then
    printf 'missing required Rust toolchain: %s\n' "$release_toolchain" >&2
    exit 1
  fi
done

# The feature set matches the rust-floor clippy gate in .github/workflows/ci.yml.
RUSTUP_TOOLCHAIN=$SWALLOWTAIL_MSRV \
  bash "$release_repo_root/scripts/oxide-rust.sh" check \
  --workspace --all-targets --all-features --locked

RUSTUP_TOOLCHAIN=$SWALLOWTAIL_CURRENT_STABLE \
  bash "$release_repo_root/scripts/oxide-rust.sh" check \
  --workspace --all-targets --all-features --locked

printf 'unified MSRV and current-stable checks passed\n'
