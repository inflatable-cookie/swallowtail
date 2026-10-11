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

if [[ $# -eq 0 ]]; then
  echo "usage: $0 command [args...] [--then command [args...]]..." >&2
  exit 2
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
