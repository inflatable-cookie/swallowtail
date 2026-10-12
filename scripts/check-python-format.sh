#!/usr/bin/env bash
set -euo pipefail

# Check-only: never rewrites files, so frozen proof runners keep their bytes.
# The formatter is pinned so every checkout judges the same bytes the same way.
format_ruff_version="0.14.1"

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

format_ruff_found=$(ruff --version 2> /dev/null || true)
if [[ "$format_ruff_found" != "ruff $format_ruff_version" ]]; then
  printf 'python format check needs ruff %s on PATH; found: %s\n' \
    "$format_ruff_version" "${format_ruff_found:-none}" >&2
  printf 'Install the pinned formatter in a task-owned venv and prefix PATH.\n' >&2
  printf 'See docs/guides/validation-tiers.md (Python format prerequisite).\n' >&2
  exit 127
fi

ruff format --check --no-cache -- "$@"
