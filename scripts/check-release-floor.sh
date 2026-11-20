#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.2.0.env

release_installed_toolchains=$(rustup toolchain list)
# Escape the dots so `1.95.0` cannot match `1x95y0`. The list is captured
# first, so grep -q closing early cannot race rustup under pipefail.
release_pattern=${SWALLOWTAIL_MSRV//./\\.}
if ! grep -Eq "^${release_pattern}(-|\$)" <<< "$release_installed_toolchains"; then
  printf 'missing required Rust toolchain: %s\n' "$SWALLOWTAIL_MSRV" >&2
  exit 1
fi

# Both commands run through the shared helper, so the floor is one ordinary
# local Rust route; the pin travels as RUSTUP_TOOLCHAIN, the supported mapping
# for `cargo +<name>`. Reduced priority is kept: `nice` only lowers this run.
nice -n 5 env RUSTUP_TOOLCHAIN="$SWALLOWTAIL_MSRV" \
  bash "$release_repo_root/scripts/oxide-rust.sh" clippy \
  --workspace \
  --all-targets --all-features --locked -- -D warnings
nice -n 5 env RUSTUP_TOOLCHAIN="$SWALLOWTAIL_MSRV" \
  bash "$release_repo_root/scripts/oxide-rust.sh" test \
  --workspace \
  --all-features --locked

printf 'unified Rust %s floor Clippy and full tests passed\n' "$SWALLOWTAIL_MSRV"
