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
