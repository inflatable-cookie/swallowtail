#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

release_actual=$(mktemp)
trap 'rm -f "$release_actual"' EXIT

for release_crate_dir in crates/*; do
  release_crate_name=${release_crate_dir##*/}
  release_api_hash=$(
    rg --no-heading --no-line-number --with-filename -g '*.rs' \
      '^\s*pub\s+((async|const|unsafe)\s+)*(mod|use|struct|enum|trait|type|fn|const|static|extern|macro)' \
      "$release_crate_dir/src" |
      sed "s|$release_crate_dir/src/||" |
      sed -E 's/[[:space:]]+/ /g' |
      LC_ALL=C sort |
      shasum -a 256 |
      awk '{print $1}'
  )
  printf '%s\t%s\n' "$release_crate_name" "$release_api_hash" >> "$release_actual"
done

diff -u release-baselines/public-api-0.1.0.sha256 "$release_actual"
printf 'public API declaration baseline passed for 26 crates\n'
