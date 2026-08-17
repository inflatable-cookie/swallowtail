#!/usr/bin/env bash
set -euo pipefail

route_matrix_repo_root=$(cd "$(dirname "$0")/.." && pwd)
route_matrix_file="$route_matrix_repo_root/docs/guides/provider-route-matrix.md"
feature_matrix_file="$route_matrix_repo_root/docs/guides/provider-solution-feature-matrix.csv"
route_matrix_actual=$(mktemp)
route_matrix_expected=$(mktemp)
route_lifecycle_rows=$(mktemp)
route_lifecycle_actual=$(mktemp)
route_lifecycle_posture_actual=$(mktemp)
route_lifecycle_posture_expected=$(mktemp)
trap 'rm -f "$route_matrix_actual" "$route_matrix_expected" "$route_lifecycle_rows" "$route_lifecycle_actual" "$route_lifecycle_posture_actual" "$route_lifecycle_posture_expected"' EXIT

sed '/<!-- provider-session-lifecycle-matrix:start -->/,$d' "$route_matrix_file" |
  sed -n 's/^| `\([^`]*\)` |.*$/\1/p' |
  LC_ALL=C sort > "$route_matrix_actual"

python3 "$route_matrix_repo_root/scripts/provider_route_matrix/route_inventory.py" |
  LC_ALL=C sort > "$route_matrix_expected"

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 39 ]; then
  printf 'provider route matrix must contain exactly 39 route rows\n' >&2
  exit 1
fi

if [ -n "$(uniq -d "$route_matrix_actual")" ]; then
  printf 'provider route matrix contains duplicate route rows\n' >&2
  uniq -d "$route_matrix_actual" >&2
  exit 1
fi

diff -u "$route_matrix_expected" "$route_matrix_actual"

sed -n \
  '/<!-- provider-session-lifecycle-matrix:start -->/,/<!-- provider-session-lifecycle-matrix:end -->/p' \
  "$route_matrix_file" |
  sed -n '/^| `/p' > "$route_lifecycle_rows"

sed -n 's/^| `\([^`]*\)` |.*$/\1/p' "$route_lifecycle_rows" |
  LC_ALL=C sort > "$route_lifecycle_actual"

if [ "$(wc -l < "$route_lifecycle_actual" | tr -d ' ')" -ne 39 ]; then
  printf 'provider session lifecycle matrix must contain exactly 39 route rows\n' >&2
  exit 1
fi

if [ -n "$(uniq -d "$route_lifecycle_actual")" ]; then
  printf 'provider session lifecycle matrix contains duplicate route rows\n' >&2
  uniq -d "$route_lifecycle_actual" >&2
  exit 1
fi

diff -u "$route_matrix_expected" "$route_lifecycle_actual"

awk -F '|' '
  function trim(value) {
    gsub(/^[[:space:]]+|[[:space:]]+$/, "", value)
    gsub(/`/, "", value)
    return value
  }

  /^\| `/ {
    route = trim($2)
    posture = trim($3)
    binding = trim($4)
    archive = trim($5)
    restore = trim($6)
    delete_action = trim($7)
    strength = trim($8)
    version = trim($9)
    cleanup = trim($10)

    if (version == "" || cleanup == "") {
      printf "provider session lifecycle row lacks version or cleanup evidence: %s\n", route > "/dev/stderr"
      exit 1
    }

    printf "%s|%s|%s|%s|%s|%s|%s\n",
      route, posture, binding, archive, restore, delete_action, strength
  }
' "$route_lifecycle_rows" | LC_ALL=C sort > "$route_lifecycle_posture_actual"

python3 "$route_matrix_repo_root/scripts/provider_route_matrix/route_inventory.py" \
  --lifecycle-postures | LC_ALL=C sort > "$route_lifecycle_posture_expected"

diff -u "$route_lifecycle_posture_expected" "$route_lifecycle_posture_actual"


python3 "$route_matrix_repo_root/scripts/provider_route_matrix/validate.py" \
  "$feature_matrix_file"

python3 "$route_matrix_repo_root/scripts/check-provider-activity-matrix.py"

printf 'provider route, lifecycle, 31-solution/39-route feature, and activity matrices passed\n'
