#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.7.0.env

release_installed_toolchains=$(rustup toolchain list)
# Escape the dots so `1.97.1` cannot match `1x97y1`. The list is captured
# first, so grep -q closing early cannot race rustup under pipefail.
release_pattern=${SWALLOWTAIL_MSRV//./\\.}
if ! grep -Eq "^${release_pattern}(-|\$)" <<< "$release_installed_toolchains"; then
  printf 'missing required Rust toolchain: %s\n' "$SWALLOWTAIL_MSRV" >&2
  exit 1
fi

# Clippy runs through the shared helper; tests run through the private-target
# wrapper so courier-backed process tests get a run-owned target matching CI.
# The pin travels as RUSTUP_TOOLCHAIN, the supported mapping for `cargo +<name>`.
# Reduced priority is kept: `nice` only lowers this run.
nice -n 5 env RUSTUP_TOOLCHAIN="$SWALLOWTAIL_MSRV" \
  bash "$release_repo_root/scripts/oxide-rust.sh" clippy \
  --workspace \
  --all-targets --all-features --locked -- -D warnings
nice -n 5 env RUSTUP_TOOLCHAIN="$SWALLOWTAIL_MSRV" \
  bash "$release_repo_root/scripts/run-with-owned-cargo-target.sh" cargo test \
  --workspace \
  --all-features --locked

printf 'unified Rust %s floor Clippy and full tests passed\n' "$SWALLOWTAIL_MSRV"
