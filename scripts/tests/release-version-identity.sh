#!/usr/bin/env bash
set -euo pipefail

identity_repo_root=$(cd "$(dirname "$0")/../.." && pwd)
# shellcheck source=scripts/release-version-identity.sh
source "$identity_repo_root/scripts/release-version-identity.sh"

identity_expect_previous() {
  local identity_label=$1
  local identity_expected=$2
  if [[ $release_previous_version != "$identity_expected" ]]; then
    printf '%s: expected previous=%s, got %s\n' \
      "$identity_label" "$identity_expected" "$release_previous_version" >&2
    exit 1
  fi
}

identity_fixture=$(mktemp -d)
trap 'rm -rf "$identity_fixture"' EXIT
mkdir -p \
  "$identity_fixture/release-baselines/public-api-0.4.2" \
  "$identity_fixture/release-baselines/public-api-0.4.3" \
  "$identity_fixture/release-baselines/public-api-0.4.3-rc1" \
  "$identity_fixture/release-baselines/public-api-0.5.0" \
  "$identity_fixture/release-baselines/public-api-0.10.0"
printf '%s\n' '[workspace.package]' 'version = "0.4.3"' \
  > "$identity_fixture/Cargo.toml"

(
  cd "$identity_fixture"
  release_load_version_identity
  [[ $release_current_version == 0.4.3 ]]
  identity_expect_previous 'newer 0.5.0 directory present' 0.4.2
)

cd "$identity_repo_root"
release_load_version_identity
identity_expect_previous 'real tree' 0.4.2

printf 'release-version-identity fixtures passed\n'
