#!/usr/bin/env bash
set -euo pipefail

# Run one command under an isolated HOME and named provider-home variables,
# then restore the host environment before exit.
#
# Usage:
#   run-with-isolated-home.sh [--home-var NAME]... -- command [args...]
#
# Example:
#   run-with-isolated-home.sh --home-var GROK_HOME -- ./grok --version

validation_host_home=${HOME:?HOST HOME must be set}
validation_home_vars=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --home-var)
      shift
      if [[ $# -lt 1 ]]; then
        printf '%s: missing name after --home-var\n' "$0" >&2
        exit 2
      fi
      validation_home_vars+=("$1")
      shift
      ;;
    --)
      shift
      break
      ;;
    *)
      printf 'usage: %s [--home-var NAME]... -- command [args...]\n' "$0" >&2
      exit 2
      ;;
  esac
done

if [[ $# -lt 1 ]]; then
  printf '%s: missing command after --\n' "$0" >&2
  exit 2
fi

validation_isolated_home=$(mktemp -d)

validation_restore_host_home() {
  export HOME="$validation_host_home"
  for validation_home_var in "${validation_home_vars[@]}"; do
    unset "$validation_home_var"
  done
  rm -rf "$validation_isolated_home"
}

trap validation_restore_host_home EXIT

export HOME="$validation_isolated_home"
for validation_home_var in "${validation_home_vars[@]}"; do
  export "$validation_home_var=$validation_isolated_home"
done

"$@"
