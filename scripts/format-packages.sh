#!/usr/bin/env bash
set -euo pipefail

format_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$format_repo_root"

source "$format_repo_root/scripts/validation/package-scope.sh"

validation_parse_package_scope "$@"

if [[ "$validation_plan" == true ]]; then
  printf 'selector=format:package\n'
  printf 'mode=plan\n'
  validation_print_packages
  printf 'proof=cargo-fmt-check\n'
  exit 0
fi

cargo fmt "${validation_cargo_package_args[@]}"
cargo fmt "${validation_cargo_package_args[@]}" -- --check

printf 'package formatting passed for %s package(s)\n' "${#validation_packages[@]}"
