#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.2.0.env

for release_toolchain in \
  "$SWALLOWTAIL_MSRV" \
  "$SWALLOWTAIL_CURRENT_STABLE"
do
  if ! rustup toolchain list | rg -q "^${release_toolchain}(-|$)"; then
    printf 'missing required Rust toolchain: %s\n' "$release_toolchain" >&2
    exit 1
  fi
done

RUSTUP_TOOLCHAIN=$SWALLOWTAIL_MSRV \
  bash "$release_repo_root/scripts/oxide-rust.sh" check \
  --workspace --all-targets --locked

RUSTUP_TOOLCHAIN=$SWALLOWTAIL_CURRENT_STABLE \
  bash "$release_repo_root/scripts/oxide-rust.sh" check \
  --workspace --all-targets --locked

printf 'unified MSRV and current-stable checks passed\n'
