#!/usr/bin/env bash
set -euo pipefail

validation_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$validation_repo_root"

source "$validation_repo_root/scripts/validation/archive.sh"

validation_expect_failure() {
  local validation_expected=$1
  shift
  local validation_output
  if validation_output=$("$@" 2>&1); then
    printf 'expected validation selector failure\n' >&2
    exit 1
  fi
  if [[ "$validation_output" != *"$validation_expected"* ]]; then
    printf 'validation selector failure changed: %s\n' "$validation_output" >&2
    exit 1
  fi
}

validation_focused_plan=$(
  bash scripts/validate-focused-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-xai
)
[[ "$validation_focused_plan" == *"selector=validate:focused"* ]]
[[ "$validation_focused_plan" == *"package=swallowtail-adapter-pi"* ]]
[[ "$validation_focused_plan" == *"proof=warnings-denied-all-target-clippy"* ]]

validation_affected_plan=$(
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-xai
)
[[ "$validation_affected_plan" == *"selector=package:verify-affected"* ]]
[[ "$validation_affected_plan" == *"archive_isolation=independent"* ]]
[[ "$validation_affected_plan" == *"compile_target=shared"* ]]

validation_expect_failure \
  "requires one to four package names" \
  bash scripts/validate-focused-packages.sh --plan
validation_expect_failure \
  "contains a duplicate" \
  bash scripts/validate-focused-packages.sh \
    --plan \
    swallowtail-adapter-pi \
    swallowtail-adapter-pi
validation_expect_failure \
  "is not an exact workspace package" \
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-adapter-not-real
validation_expect_failure \
  "requires one to four package names" \
  bash scripts/verify-affected-packages.sh \
    --plan \
    swallowtail-core \
    swallowtail-runtime \
    swallowtail-testkit \
    swallowtail-host-local \
    swallowtail-adapter-pi

printf '%s\n' "package-0.1.0/src/lib.rs" |
  validation_archive_member_list_is_safe
if printf '%s\n' "package-0.1.0/.env" |
  validation_archive_member_list_is_safe
then
  printf 'unsafe archive member was accepted\n' >&2
  exit 1
fi

printf 'validation selector argument and archive-scope tests passed\n'
