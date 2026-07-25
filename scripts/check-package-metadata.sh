#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_metadata=$(mktemp)
release_edges=$(mktemp)
release_names=$(mktemp)
trap 'rm -f "$release_metadata" "$release_edges" "$release_names"' EXIT

cargo metadata --no-deps --format-version 1 > "$release_metadata"

jq -e '
  (.packages | length) == 23 and
  all(.packages[];
    .version == "0.1.0" and
    .edition == "2024" and
    .license == "MIT" and
    .repository == "https://github.com/inflatable-cookie/swallowtail" and
    .publish == ["crates-io"] and
    .readme == "../../README.md" and
    (.description | type == "string" and length > 0) and
    (.features | length == 0) and
    all(.targets[]; all(.kind[]; . == "lib" or . == "test" or . == "example"))
  ) and
  all(.packages[];
    if .name == "swallowtail-adapter-bedrock"
    then .rust_version == "1.94.1"
    else .rust_version == "1.93"
    end
  ) and
  all(.packages[].dependencies[];
    if .path != null then .req == "^0.1.0" else true end
  )
' "$release_metadata" > /dev/null

jq -r '.packages[].name' "$release_metadata" | LC_ALL=C sort > "$release_names"
cut -f1 release-baselines/public-api-0.1.0.sha256 | LC_ALL=C sort |
  diff -u - "$release_names"

jq -r '
  .packages[] as $package |
  $package.dependencies[] |
  select(.path != null and .kind == null) |
  [$package.name, .name, .req] |
  @tsv
' "$release_metadata" | LC_ALL=C sort > "$release_edges"

diff -u release-baselines/internal-dependencies-0.1.0.tsv "$release_edges"

printf 'package metadata and dependency topology passed for 23 crates\n'
