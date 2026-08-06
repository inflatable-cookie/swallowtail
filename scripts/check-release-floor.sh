#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source release-baselines/rust-toolchains-0.2.0.env

if ! rustup toolchain list | rg -q "^${SWALLOWTAIL_MSRV}(-|$)"; then
  printf 'missing required Rust toolchain: %s\n' "$SWALLOWTAIL_MSRV" >&2
  exit 1
fi

# Sync the lockfile to the workspace version before gating.
#
# `effigy release prepare` bumps the workspace version and *then* runs the
# gates, so at gate time Cargo.lock still records the previous version and every
# `--locked` gate refuses outright:
#
#   error: cannot update the lock file ... because --locked was passed
#
# Effigy requires `--check-gates` when gates are configured, so there is no way
# round it: the gates cannot run in the one flow they exist for.
#
# `cargo update -w` touches workspace members only. It is emphatically not
# `cargo generate-lockfile`, which re-resolves the whole graph -- that is the
# `sync-files` behaviour that silently bumped ~40 third-party crates *after* the
# gates had run on signal's 0.1.0 prepare.
#
# ORDERING: effigy runs gates alphabetically and `floor` sorts before the other
# `--locked` gates in this repo (api < docs < floor < lint < test), so doing the
# sync here covers them too. Renaming a gate can break that; move the sync
# earlier if it does.
#
# Verified rather than trusted: every changed line must be a `version` line and
# every added one must equal the workspace version, so a third-party bump --
# also a version line -- is still refused. Tested on signal in all three
# directions: no-op silent, legitimate bump accepted, a fabricated
# crossbeam-epoch 0.9.18 -> 0.9.99 refused with the offending diff printed.
release_workspace_version=$(
  rg --max-count 1 '^version = "' Cargo.toml | sed 's/.*"\(.*\)"/\1/'
)
cargo update --workspace --quiet

release_lock_changes=$(
  git diff -- Cargo.lock | rg '^[-+]' | rg -v '^(\+\+\+|---)' || true
)
if [[ -n "$release_lock_changes" ]]; then
  release_lock_non_version=$(rg -v '^[-+]version = "' <<<"$release_lock_changes" || true)
  release_lock_foreign=$(
    rg '^\+version = ' <<<"$release_lock_changes" |
      rg -v "^\\+version = \"${release_workspace_version}\"\$" || true
  )
  if [[ -n "$release_lock_non_version" || -n "$release_lock_foreign" ]]; then
    printf 'cargo update -w moved more than the workspace version to %s:\n' \
      "$release_workspace_version" >&2
    printf '%s\n' "$release_lock_changes" >&2
    exit 1
  fi
  printf 'Cargo.lock synced to workspace version %s\n' \
    "$release_workspace_version"
fi

nice -n 5 rustup run "$SWALLOWTAIL_MSRV" cargo clippy \
  --workspace \
  --all-targets --all-features --locked -- -D warnings
nice -n 5 rustup run "$SWALLOWTAIL_MSRV" cargo test \
  --workspace \
  --all-features --locked

printf 'unified Rust %s floor Clippy and full tests passed\n' "$SWALLOWTAIL_MSRV"
