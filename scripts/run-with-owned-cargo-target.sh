#!/usr/bin/env bash
set -euo pipefail

# Run one or more cargo command groups under one fresh, run-owned
# CARGO_TARGET_DIR, then remove that directory before exit.
#
# Usage:
#   run-with-owned-cargo-target.sh command [args...] [--then command [args...]]...
#
# Each validation selector that builds test fixtures calls this once. The
# outer test build and the nested courier build share one warm target that
# belongs to this invocation alone: never the worktree target, never a
# directory another run owns. The directory is created directly under the
# system temp root and marked, so the test helper can refuse anything else.
#
# Routing: this is a private-target driver, so it enters once through the
# shared helper's host-tool mode, one `oxide exec --` lease. The commands below
# stay plain `cargo` on purpose: they are the tool's own calls, and routing them
# again would take a second lease. Oxide's storage environment reaches them, so
# the private directory keeps every final output for this run while the
# intermediates go to the held slot; cleanup below removes only the directory
# this invocation created, never a slot, the worktree target or another run's
# directory.

if [[ $# -eq 0 ]]; then
  echo "usage: $0 command [args...] [--then command [args...]]..." >&2
  exit 2
fi

route_repo_root=$(cd "$(dirname "$0")/.." && pwd)
# One supported lease for this Cargo-running driver. The guard stops a re-exec
# loop; the child inherits the lease's storage environment.
if [[ ${SWALLOWTAIL_OXIDE_EXEC_CHILD:-} != "1" ]]; then
  SWALLOWTAIL_OXIDE_EXEC_CHILD=1 exec bash "$route_repo_root/scripts/oxide-rust.sh" \
    --host-tool bash "$route_repo_root/scripts/run-with-owned-cargo-target.sh" "$@"
fi

target="$(mktemp -d "${TMPDIR:-/tmp}/swallowtail-sdk-courier.XXXXXXXX")"

cleanup() {
  # Remove only the directory this invocation created.
  if [[ -n "${target:-}" && -d "$target" ]]; then
    rm -rf -- "$target"
  fi
}
trap cleanup EXIT

printf '%s\n' "$$" > "$target/.swallowtail-run-owned"
export CARGO_TARGET_DIR="$target"
# A per-run private target cannot be honoured from a shared build directory:
# Cargo keeps the compiled test binary in the build directory, so a
# `CARGO_BIN_EXE_*` value baked by an earlier run's target path survives the
# rebuild check and the courier spawn fails with `executable_not_found`
# (measured 2026-10-25: three of seven registered_tool_proxy tests, with the
# slot's `.d` file naming the previous run's directory). This proof therefore
# builds without any shared intermediate state: its build directory lives
# inside the run-owned target and leaves with it. Named proof exception in
# docs/guides/validation-tiers.md.
export CARGO_BUILD_BUILD_DIR="$target/build"

group=()
run_group() {
  if [[ ${#group[@]} -eq 0 ]]; then
    return
  fi
  "${group[@]}"
  group=()
}

for arg in "$@"; do
  if [[ "$arg" == "--then" ]]; then
    run_group
  else
    group+=("$arg")
  fi
done
run_group
