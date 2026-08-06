#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.1.0.env

for release_toolchain in \
  "$SWALLOWTAIL_GENERAL_MSRV" \
  "$SWALLOWTAIL_BEDROCK_MSRV"
do
  if ! rustup toolchain list | rg -q "^${release_toolchain}(-|$)"; then
    printf 'missing required Rust toolchain: %s\n' "$release_toolchain" >&2
    exit 1
  fi
done

nice -n 5 rustup run "$SWALLOWTAIL_GENERAL_MSRV" cargo clippy \
  --workspace --exclude swallowtail-adapter-bedrock \
  --all-targets --all-features --locked -- -D warnings
nice -n 5 rustup run "$SWALLOWTAIL_GENERAL_MSRV" cargo test \
  --workspace --exclude swallowtail-adapter-bedrock \
  --all-features --locked

nice -n 5 rustup run "$SWALLOWTAIL_BEDROCK_MSRV" cargo clippy \
  --package swallowtail-adapter-bedrock \
  --all-targets --all-features --locked -- -D warnings
nice -n 5 rustup run "$SWALLOWTAIL_BEDROCK_MSRV" cargo test \
  --package swallowtail-adapter-bedrock \
  --all-features --locked

printf 'floor-toolchain Clippy and full tests passed\n'
