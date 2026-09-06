#!/usr/bin/env bash
# Current and previous release identity for the five gate scripts.
# Current: Cargo.toml workspace.package.version.
# Previous: newest tagged public-api-* directory older than current.
set -euo pipefail

release_load_version_identity() {
  local cargo_toml=${1:-Cargo.toml}

  release_current_version=
  release_previous_version=

  release_current_version=$(awk '
    $0 == "[workspace.package]" { found = 1; next }
    found && /^\[/ { exit }
    found && $0 ~ /^version[ \t]*=/ {
      split($0, parts, "\"")
      print parts[2]
      exit
    }
  ' "$cargo_toml")
  if [[ -z ${release_current_version} ]]; then
    printf 'missing workspace.package.version in %s\n' "$cargo_toml" >&2
    return 1
  fi

  release_previous_version=$(
    for baseline_dir in release-baselines/public-api-[0-9]*; do
      [[ -d $baseline_dir ]] || continue
      name=${baseline_dir##*/public-api-}
      [[ $name != "$release_current_version" ]] || continue
      printf '%s\n' "$name"
    done | sort -t. -k1,1n -k2,2n -k3,3n | tail -n 1
  )

  if [[ -z ${release_previous_version} ]]; then
    printf 'no tagged public-api baseline older than %s\n' "$release_current_version" >&2
    return 1
  fi
}

if [[ ${BASH_SOURCE[0]} == "$0" ]]; then
  release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
  cd "$release_repo_root"
  release_load_version_identity
  printf 'current=%s\nprevious=%s\n' \
    "$release_current_version" "$release_previous_version"
fi
