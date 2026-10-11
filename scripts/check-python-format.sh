#!/usr/bin/env bash
set -euo pipefail

# Check-only: never rewrites files, so frozen proof runners keep their bytes.
if (( $# < 1 )); then
  printf 'python format check requires one or more file paths\n' >&2
  exit 2
fi

for format_candidate in "$@"; do
  if [[ "$format_candidate" == -* ]]; then
    printf 'python format check contains an unsupported option: %s\n' \
      "$format_candidate" >&2
    exit 2
  fi
  if [[ ! -f "$format_candidate" ]]; then
    printf 'python format check path is not a regular file: %s\n' \
      "$format_candidate" >&2
    exit 2
  fi
done

if ! command -v ruff > /dev/null 2>&1; then
  printf 'python format check needs ruff on PATH; none was found.\n' >&2
  printf 'Adding ruff is a dependency change and needs operator approval.\n' >&2
  exit 127
fi

ruff format --check --no-cache -- "$@"
