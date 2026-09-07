#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

# shellcheck source=scripts/release-version-identity.sh
source "$release_repo_root/scripts/release-version-identity.sh"
release_load_version_identity

source release-baselines/rust-toolchains-0.2.0.env
release_msrv_cargo=${SWALLOWTAIL_MSRV%.0}
release_baseline_packages=release-baselines/public-api-$release_current_version/packages.txt
release_baseline_edges=release-baselines/internal-dependencies-$release_current_version.tsv
if [[ ! -f $release_baseline_packages ]]; then
  printf 'missing working package inventory %s\n' "$release_baseline_packages" >&2
  exit 1
fi
if [[ ! -f $release_baseline_edges ]]; then
  printf 'missing working dependency graph %s\n' "$release_baseline_edges" >&2
  exit 1
fi
release_package_count=$(wc -l < "$release_baseline_packages" | tr -d ' ')

release_metadata=$(mktemp)
release_edges=$(mktemp)
release_names=$(mktemp)
release_tag_names=$(mktemp)
release_expected_names=$(mktemp)
release_expected_edges=$(mktemp)
release_order_names=$(mktemp)
trap 'rm -f "$release_metadata" "$release_edges" "$release_names" "$release_tag_names" "$release_expected_names" "$release_expected_edges" "$release_order_names"' EXIT

cargo metadata --no-deps --format-version 1 > "$release_metadata"
release_version=$(jq -r '.packages[0].version' "$release_metadata")
if [[ $release_version != "$release_current_version" ]]; then
  printf 'cargo metadata version %s does not match workspace.package.version %s\n' \
    "$release_version" "$release_current_version" >&2
  exit 1
fi

jq -e --arg version "$release_version" --arg rust_msrv "$release_msrv_cargo" \
  --argjson package_count "$release_package_count" '
  (.packages | length) == $package_count and
  all(.packages[];
    .version == $version and
    .edition == "2024" and
    .license == "MIT" and
    .repository == "https://github.com/inflatable-cookie/swallowtail" and
    .publish == [] and
    .readme == "../../README.md" and
    (.description | type == "string" and length > 0) and
    (
      (.features | length == 0) or
      (
        (.name == "swallowtail-adapter-claude-agent" or
         .name == "swallowtail-adapter-gemini" or
         .name == "swallowtail-adapter-grok" or
         .name == "swallowtail-adapter-kimi" or
         .name == "swallowtail-adapter-opencode" or
         .name == "swallowtail-adapter-ollama" or
         .name == "swallowtail-adapter-oh-my-pi" or
         .name == "swallowtail-adapter-muse" or
         .name == "swallowtail-adapter-command-code" or
         .name == "swallowtail-adapter-deepseek-harness" or
         .name == "swallowtail-adapter-zcode" or
         .name == "swallowtail-adapter-pi" or
         .name == "swallowtail-adapter-qwen") and
        .features == {"live-probes":[]} or
        (.name == "swallowtail-host-local" and
         .features == {"mediated-stdio-proxy":[]})
      )
    ) and
    all(.targets[];
      all(.kind[]; . == "lib" or . == "test" or . == "example") or
      (.name == "swallowtail-registered-tool-courier" and
       .kind == ["bin"] and
       ."required-features" == ["mediated-stdio-proxy"])
    )
  ) and
  all(.packages[]; .rust_version == $rust_msrv) and
  all(.packages[].dependencies[];
    if .path != null then .req == ("^" + $version) else true end
  )
' "$release_metadata" > /dev/null

jq -r '.packages[].name' "$release_metadata" | LC_ALL=C sort > "$release_names"
LC_ALL=C sort "$release_baseline_packages" > "$release_expected_names"
diff -u "$release_expected_names" "$release_names"

source scripts/release-package-set.sh
printf '%s\n' "${release_packages[@]}" | LC_ALL=C sort > "$release_order_names"
diff -u "$release_names" "$release_order_names"
[[ ${#release_packages[@]} -eq $release_package_count ]]
[[ "${release_stage_2[*]}" == "swallowtail-idioms" ]]
[[ "${release_stage_3[*]}" == "swallowtail-runtime" ]]

jq -r '
  .packages[] as $package |
  $package.dependencies[] |
  select(.path != null and .kind == null) |
  [$package.name, .name, .req] |
  @tsv
' "$release_metadata" | LC_ALL=C sort > "$release_edges"

awk -F '\t' -v OFS='\t' -v requirement="^$release_version" \
  '{$3 = requirement; print}' \
  "$release_baseline_edges" \
  | LC_ALL=C sort > "$release_expected_edges"
diff -u "$release_expected_edges" "$release_edges"

printf 'current-source metadata passed for %s crates at %s and Rust %s; immutable v%s remains the removal baseline\n' \
  "$release_package_count" "$release_version" "$release_msrv_cargo" \
  "$release_previous_version"
