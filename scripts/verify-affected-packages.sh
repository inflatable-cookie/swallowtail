#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/validation/path.sh"
validation_repo_root=$(validation_canonical_path "$(dirname "$0")/..")
cd "$validation_repo_root"

source "$validation_repo_root/scripts/release-package-set.sh"
source "$validation_repo_root/scripts/validation/archive.sh"
source "$validation_repo_root/scripts/validation/package-scope.sh"

validation_parse_package_scope "$@"

if [[ "$validation_plan" == true ]]; then
  printf 'selector=package:verify-affected\n'
  printf 'mode=plan\n'
  validation_print_packages
  printf 'archive_isolation=independent\n'
  printf 'compile_target=shared\n'
  printf 'provider_calls=none\n'
  exit 0
fi

validation_started=$SECONDS
validation_tmp=$(mktemp -d)
trap 'rm -rf "$validation_tmp"' EXIT

validation_patch_args=()
for validation_internal_package in \
  "${release_internal_patch_packages[@]}"
do
  validation_patch_args+=(
    --config
    "patch.crates-io.$validation_internal_package.path=\"$validation_repo_root/crates/$validation_internal_package\""
  )
done

for validation_package in "${validation_packages[@]}"; do
  cargo package \
    --package "$validation_package" \
    --allow-dirty \
    --no-verify \
    --locked \
    --offline \
    --target-dir "$validation_tmp/package-target" \
    "${validation_patch_args[@]}"

  validation_version=$(validation_package_version "$validation_package")
  validation_archive="$validation_tmp/package-target/package/$validation_package-$validation_version.crate"
  test -f "$validation_archive"

  validation_archive_size=$(wc -c < "$validation_archive" | tr -d ' ')
  if (( validation_archive_size >= 10000000 )); then
    printf 'affected package exceeds crates.io 10 MB limit: %s\n' \
      "$validation_package" >&2
    exit 1
  fi
  if ! validation_archive_is_safe "$validation_archive"; then
    printf 'affected package contains a forbidden archive path: %s\n' \
      "$validation_package" >&2
    exit 1
  fi

  mkdir -p "$validation_tmp/extracted"
  tar -xzf "$validation_archive" -C "$validation_tmp/extracted"
  mv \
    "$validation_tmp/extracted/$validation_package-$validation_version" \
    "$validation_tmp/extracted/$validation_package"

  validation_manifest="$validation_tmp/extracted/$validation_package/Cargo.toml"
  if ! validation_manifest_has_no_path_or_git "$validation_manifest"; then
    printf 'affected package retained a path or git dependency: %s\n' \
      "$validation_package" >&2
    exit 1
  fi
done

if ! validation_extracted_tree_is_safe \
  "$validation_tmp/extracted" \
  "$validation_repo_root"
then
  printf 'affected package content audit failed\n' >&2
  exit 1
fi

for validation_package in "${validation_packages[@]}"; do
  validation_package_root="$validation_tmp/extracted/$validation_package"
  validation_manifest="$validation_package_root/Cargo.toml"
  cp "$validation_repo_root/Cargo.lock" "$validation_package_root/Cargo.lock"

  cargo generate-lockfile \
    --manifest-path "$validation_manifest" \
    --offline \
    "${validation_patch_args[@]}"

  CARGO_TARGET_DIR="$validation_tmp/compile-target" \
    cargo check \
      --manifest-path "$validation_manifest" \
      --all-targets \
      --locked \
      --offline \
      "${validation_patch_args[@]}"
done

printf 'affected package proof passed for %s package(s) in %s seconds\n' \
  "${#validation_packages[@]}" "$((SECONDS - validation_started))"
