#!/usr/bin/env bash
set -euo pipefail

release_repo_root=$(cd "$(dirname "$0")/.." && pwd)
cd "$release_repo_root"

source "$release_repo_root/scripts/release-package-set.sh"

release_candidate=${1:-.effigy/release-candidates/$release_version}
if [[ "$release_candidate" != /* ]]; then
  release_candidate="$release_repo_root/$release_candidate"
fi

test -d "$release_candidate"
test -f "$release_candidate/candidate.env"
test -f "$release_candidate/swallowtail-$release_version-source.bundle"

(
  cd "$release_candidate"
  shasum -a 256 -c evidence.sha256
  shasum -a 256 -c packages.sha256
  shasum -a 256 -c package-files.sha256
)
git -C "$release_repo_root" bundle verify \
  "$release_candidate/swallowtail-$release_version-source.bundle"

release_tmp=$(mktemp -d)
trap 'rm -rf "$release_tmp"' EXIT

git clone -q \
  "$release_candidate/swallowtail-$release_version-source.bundle" \
  "$release_tmp/source"

release_expected_commit=$(
  sed -n 's/^source_commit=//p' "$release_candidate/candidate.env"
)
release_expected_parent=$(
  sed -n 's/^source_parent_commit=//p' "$release_candidate/candidate.env"
)
test "$(
  sed -n 's/^format=//p' "$release_candidate/candidate.env"
)" = "swallowtail.release-candidate.v2"
test "$(
  sed -n 's/^source_scope=//p' "$release_candidate/candidate.env"
)" = "clean-head-excluding-generated-candidate-evidence"
test "$(git -C "$release_tmp/source" rev-parse HEAD)" = "$release_expected_commit"
test "$(git -C "$release_tmp/source" rev-parse HEAD^)" = \
  "$release_expected_parent"
test -z "$(git -C "$release_tmp/source" status --porcelain)"

SWALLOWTAIL_CANDIDATE_OUTPUT="$release_tmp/regenerated" \
  bash "$release_tmp/source/scripts/verify-packages-local.sh"

diff -u \
  "$release_candidate/packages.sha256" \
  "$release_tmp/regenerated/packages.sha256"
diff -u \
  "$release_candidate/package-files.sha256" \
  "$release_tmp/regenerated/package-files.sha256"

release_regenerated_commit=$(
  sed -n 's/^source_commit=//p' "$release_tmp/regenerated/candidate.env"
)
release_regenerated_parent=$(
  sed -n 's/^source_parent_commit=//p' \
    "$release_tmp/regenerated/candidate.env"
)
test "$release_regenerated_commit" = "$release_expected_commit"
test "$release_regenerated_parent" = "$release_expected_parent"

printf 'release candidate is intact and reproducible from source commit %s\n' \
  "$release_expected_commit"
