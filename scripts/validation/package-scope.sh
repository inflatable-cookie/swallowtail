#!/usr/bin/env bash

# Shared package-scope precheck for the validation selectors. Its metadata read
# is a non-building Cargo query, so it takes the same Oxide route as every other
# ordinary local Rust command: `oxide metadata` is exact passthrough and leases
# no slot. These selectors all route their builds through the shared helper.
validation_scope_repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)

validation_parse_package_scope() {
  validation_plan=false
  if [[ "${1:-}" == "--plan" ]]; then
    validation_plan=true
    shift
  fi

  if (( $# < 1 || $# > 4 )); then
    printf 'validation package scope requires one to four package names\n' >&2
    return 2
  fi

  validation_workspace_metadata=$(
    bash "$validation_scope_repo_root/scripts/oxide-rust.sh" \
      metadata --no-deps --format-version 1
  )
  validation_packages=()
  validation_package_count=0

  local validation_candidate
  local validation_existing
  local validation_index
  for validation_candidate in "$@"; do
    if [[ "$validation_candidate" == -* ]]; then
      printf 'validation package scope contains an unsupported option: %s\n' \
        "$validation_candidate" >&2
      return 2
    fi
    if ! jq -e \
      --arg package "$validation_candidate" \
      'any(.packages[]; .name == $package)' \
      <<< "$validation_workspace_metadata" > /dev/null
    then
      printf 'validation package is not an exact workspace package: %s\n' \
        "$validation_candidate" >&2
      return 2
    fi
    for ((validation_index = 0; validation_index < validation_package_count; validation_index++)); do
      validation_existing=${validation_packages[$validation_index]}
      if [[ "$validation_existing" == "$validation_candidate" ]]; then
        printf 'validation package scope contains a duplicate: %s\n' \
          "$validation_candidate" >&2
        return 2
      fi
    done
    validation_packages+=("$validation_candidate")
    validation_package_count=$((validation_package_count + 1))
  done

  validation_cargo_package_args=()
  for validation_candidate in "${validation_packages[@]}"; do
    validation_cargo_package_args+=(--package "$validation_candidate")
  done
}

validation_package_version() {
  local validation_package=$1
  jq -r \
    --arg package "$validation_package" \
    '.packages[] | select(.name == $package) | .version' \
    <<< "$validation_workspace_metadata"
}

validation_package_selected() {
  local validation_package=$1
  local validation_candidate
  for validation_candidate in "${validation_packages[@]}"; do
    if [[ "$validation_candidate" == "$validation_package" ]]; then
      return 0
    fi
  done
  return 1
}

validation_print_packages() {
  local validation_package
  for validation_package in "${validation_packages[@]}"; do
    printf 'package=%s\n' "$validation_package"
  done
}
