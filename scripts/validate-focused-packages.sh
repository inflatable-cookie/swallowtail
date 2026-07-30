#!/usr/bin/env bash
set -euo pipefail

validation_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$validation_repo_root"

source "$validation_repo_root/scripts/validation/package-scope.sh"

validation_parse_package_scope "$@"

if [[ "$validation_plan" == true ]]; then
  printf 'selector=validate:focused\n'
  printf 'mode=plan\n'
  validation_print_packages
  printf 'proof=nextest\n'
  printf 'proof=warnings-denied-all-target-clippy\n'
  exit 0
fi

validation_started=$SECONDS
cargo nextest run \
  --locked \
  "${validation_cargo_package_args[@]}"
cargo clippy \
  --locked \
  "${validation_cargo_package_args[@]}" \
  --all-targets \
  -- \
  -D warnings

printf 'focused package validation passed for %s package(s) in %s seconds\n' \
  "${#validation_packages[@]}" "$((SECONDS - validation_started))"
