#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_candidate=${1:-.effigy/release-candidates/$release_version}
if [[ "$release_candidate" != /* ]]; then
  release_candidate="$release_repo_root/$release_candidate"
fi

test -f "$release_candidate/candidate.env"
test -f "$release_candidate/packages.sha256"
test -f "$release_candidate/swallowtail-$release_version-source.bundle"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

release_packages_root="$release_tmp/packages"
mkdir -p "$release_packages_root"
for release_package in "${release_packages[@]}"; do
  tar -xzf \
    "$release_candidate/packages/$release_package-$release_version.crate" \
    -C "$release_packages_root"
  mv \
    "$release_packages_root/$release_package-$release_version" \
    "$release_packages_root/$release_package"
done

release_candidate_source="$release_tmp/candidate-source"
git clone -q \
  "$release_candidate/swallowtail-$release_version-source.bundle" \
  "$release_candidate_source"

bash "$release_candidate_source/scripts/check-provider-route-matrix.sh"

{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for release_package in "${release_packages[@]}"; do
    printf '  "%s",\n' "$release_package"
  done
  printf ']\n\n[patch.crates-io]\n'
  for release_package in "${release_internal_patch_packages[@]}"; do
    printf '%s = { path = "%s" }\n' \
      "$release_package" "$release_package"
  done
} > "$release_packages_root/Cargo.toml"

cp "$release_candidate_source/Cargo.lock" "$release_packages_root/Cargo.lock"

release_management_adapters_actual="$release_tmp/management-adapters-actual.txt"
release_management_adapters_expected="$release_tmp/management-adapters-expected.txt"

for release_adapter_root in "$release_packages_root"/swallowtail-adapter-*; do
  if rg -q \
    'DriverRole::ProviderSessionManagement' \
    "$release_adapter_root/src"
  then
    basename "$release_adapter_root"
  fi
done | LC_ALL=C sort > "$release_management_adapters_actual"

cat <<'EOF' > "$release_management_adapters_expected"
swallowtail-adapter-claude-agent
swallowtail-adapter-codex
swallowtail-adapter-gemini
swallowtail-adapter-kimi
swallowtail-adapter-opencode
EOF

diff -u \
  "$release_management_adapters_expected" \
  "$release_management_adapters_actual"

release_lifecycle_suites=(
  "swallowtail-testkit|provider_session_management|"
  "swallowtail-adapter-codex|lifecycle_compatibility_corpus|"
  "swallowtail-adapter-codex|prepared_profiles|session_management"
  "swallowtail-adapter-claude-agent|acp_driver|missing_delete_capability_stops_before_session_or_management_effects"
  "swallowtail-adapter-claude-agent|prepared_facade|session_management"
  "swallowtail-adapter-claude-agent|lifecycle_portability|"
  "swallowtail-adapter-gemini|headless_structured_run|"
  "swallowtail-adapter-opencode|deletion_range|"
  "swallowtail-adapter-opencode|conformance|provider_neutral_management_contract_covers_opencode_delete_boundaries"
  "swallowtail-adapter-opencode|prepared_facade|deletion"
  "swallowtail-adapter-kimi|local_server_corpus|"
  "swallowtail-adapter-kimi|local_server_lifecycle|"
  "swallowtail-adapter-kimi|local_server_binding_import|"
  "swallowtail-adapter-kimi|local_server_interactive|"
)

for release_lifecycle_suite in "${release_lifecycle_suites[@]}"; do
  IFS='|' read -r release_package release_test release_filter \
    <<< "$release_lifecycle_suite"
  release_test_command=(
    cargo test
    --manifest-path "$release_packages_root/Cargo.toml"
    --package "$release_package"
    --test "$release_test"
    --locked
  )
  if [[ -n "$release_filter" ]]; then
    release_test_command+=("$release_filter")
  fi
  CARGO_TARGET_DIR="$release_tmp/target/provider-lifecycle" \
    "${release_test_command[@]}"
done

release_lifecycle_rows="$release_tmp/lifecycle-rows.txt"
sed -n \
  '/<!-- provider-session-lifecycle-matrix:start -->/,/<!-- provider-session-lifecycle-matrix:end -->/p' \
  "$release_candidate_source/docs/guides/provider-route-matrix.md" |
  awk -F '|' '
    function trim(value) {
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
      gsub(/`/, "", value)
      return value
    }

    /^\| `/ {
      printf "route=%s posture=%s binding=%s archive=%s restore=%s delete=%s strength=%s\n",
        trim($2), trim($3), trim($4), trim($5), trim($6), trim($7), trim($8)
    }
  ' > "$release_lifecycle_rows"

test "$(wc -l < "$release_lifecycle_rows" | tr -d ' ')" -eq 26
test "$(rg -c ' posture=supported ' "$release_lifecycle_rows")" -eq 5
test "$(rg -c ' posture=unsupported ' "$release_lifecycle_rows")" -eq 3
test "$(rg -c ' posture=not-applicable ' "$release_lifecycle_rows")" -eq 18

release_lifecycle_evidence="$release_tmp/lifecycle-validation.env"
{
  printf 'format=swallowtail.provider-lifecycle-candidate-validation.v1\n'
  printf 'version=%s\n' "$release_version"
  printf 'candidate_source_commit=%s\n' \
    "$(sed -n 's/^source_commit=//p' "$release_candidate/candidate.env")"
  printf 'candidate_packages_sha256=%s\n' \
    "$(shasum -a 256 "$release_candidate/packages.sha256" | awk '{ print $1 }')"
  printf 'suite_count=%s\n' "${#release_lifecycle_suites[@]}"
  printf 'management_adapter_count=5\n'
  printf 'supported_route_count=5\n'
  printf 'unsupported_route_count=3\n'
  printf 'not_applicable_route_count=18\n'
  cat "$release_lifecycle_rows"
  printf 'live_credentials=absent\n'
  printf 'provider_calls=none\n'
} > "$release_lifecycle_evidence"

if [[ -f "$release_candidate/lifecycle-validation.env" ]]; then
  diff -u \
    "$release_candidate/lifecycle-validation.env" \
    "$release_lifecycle_evidence"
else
  cp "$release_lifecycle_evidence" \
    "$release_candidate/lifecycle-validation.env"
  (
    cd "$release_candidate"
    shasum -a 256 lifecycle-validation.env > lifecycle-validation.sha256
  )
fi

cat "$release_lifecycle_evidence"
printf 'packaged provider lifecycle proof passed for 26 production routes\n'
