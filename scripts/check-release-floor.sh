#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.2.0.env

if ! rustup toolchain list | rg -q "^${SWALLOWTAIL_MSRV}(-|$)"; then
  printf 'missing required Rust toolchain: %s\n' "$SWALLOWTAIL_MSRV" >&2
  exit 1
fi

RUSTUP_TOOLCHAIN=$SWALLOWTAIL_MSRV nice -n 5 \
  bash "$release_repo_root/scripts/oxide-rust.sh" clippy \
  --workspace \
  --all-targets --all-features --locked -- -D warnings
RUSTUP_TOOLCHAIN=$SWALLOWTAIL_MSRV nice -n 5 \
  bash "$release_repo_root/scripts/oxide-rust.sh" test \
  --workspace \
  --all-features --locked

printf 'unified Rust %s floor Clippy and full tests passed\n' "$SWALLOWTAIL_MSRV"
