#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_metadata=$(mktemp)
release_edges=$(mktemp)
release_names=$(mktemp)
release_expected_names=$(mktemp)
release_expected_edges=$(mktemp)
trap 'rm -f "$release_metadata" "$release_edges" "$release_names" "$release_expected_names" "$release_expected_edges"' EXIT

cargo metadata --no-deps --format-version 1 > "$release_metadata"
release_version=$(jq -r '.packages[0].version' "$release_metadata")

jq -e --arg version "$release_version" '
  (.packages | length) == 28 and
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
        (.name == "swallowtail-adapter-gemini" or
         .name == "swallowtail-adapter-grok" or
         .name == "swallowtail-adapter-kimi" or
         .name == "swallowtail-adapter-opencode" or
         .name == "swallowtail-adapter-ollama" or
         .name == "swallowtail-adapter-oh-my-pi" or
         .name == "swallowtail-adapter-muse" or
         .name == "swallowtail-adapter-pi" or
         .name == "swallowtail-adapter-qwen") and
        .features == {"live-probes":[]}
      )
    ) and
    all(.targets[]; all(.kind[]; . == "lib" or . == "test" or . == "example"))
  ) and
  all(.packages[]; .rust_version == "1.95") and
  all(.packages[].dependencies[];
    if .path != null then .req == ("^" + $version) else true end
  )
' "$release_metadata" > /dev/null

jq -r '.packages[].name' "$release_metadata" | LC_ALL=C sort > "$release_names"
LC_ALL=C sort release-baselines/public-api-0.2.0/packages.txt \
  > "$release_expected_names"
diff -u "$release_expected_names" "$release_names"

jq -r '
  .packages[] as $package |
  $package.dependencies[] |
  select(.path != null and .kind == null) |
  [$package.name, .name, .req] |
  @tsv
' "$release_metadata" | LC_ALL=C sort > "$release_edges"

awk -F '\t' -v OFS='\t' -v requirement="^$release_version" \
  '{$3 = requirement; print}' \
  release-baselines/internal-dependencies-0.2.0.tsv \
  | LC_ALL=C sort > "$release_expected_edges"
diff -u "$release_expected_edges" "$release_edges"

printf 'coordinated metadata passed for 28 crates at %s and Rust 1.95\n' \
  "$release_version"
