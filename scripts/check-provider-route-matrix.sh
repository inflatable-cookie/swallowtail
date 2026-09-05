#!/usr/bin/env bash
set -euo pipefail

# Keep route-matrix Python entry points from writing __pycache__ under scripts/.
export PYTHONDONTWRITEBYTECODE=1

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

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 49 ]; then
  printf 'provider route matrix must contain exactly 49 route rows\n' >&2
  exit 1
fi

if [ -n "$(uniq -d "$route_matrix_actual")" ]; then
  printf 'provider route matrix contains duplicate route rows\n' >&2
  uniq -d "$route_matrix_actual" >&2
  exit 1
fi

diff -u "$route_matrix_expected" "$route_matrix_actual"

python3 - "$route_matrix_repo_root" "$route_matrix_expected" <<'PY'
import csv
import sys
from pathlib import Path

root = Path(sys.argv[1])
current_route_file = Path(sys.argv[2])
historical_route_file = root / "release-baselines/production-routes-0.3.3.txt"
immutable_route_file = root / "release-baselines/production-routes-0.4.0.txt"
candidate_route_file = root / "release-baselines/production-routes-0.4.1.txt"
ledger_file = root / (
    "docs/research/281-v0-4-0-compatibility-and-freeze-audit/"
    "route-behavior-ledger.tsv"
)
additions = {"pi.sdk-sidecar", "claude-agent.sdk"}


def fail(message: str) -> None:
    raise SystemExit(message)


immutable_routes = {
    line.strip()
    for line in immutable_route_file.read_text().splitlines()
    if line.strip()
}
current_routes = {
    line.strip() for line in current_route_file.read_text().splitlines() if line.strip()
}

historical_routes = {
    line.strip()
    for line in historical_route_file.read_text().splitlines()
    if line.strip()
}
if len(immutable_routes) != 49:
    fail(
        "immutable v0.4.0 route inventory must contain exactly 49 rows: "
        f"{len(immutable_routes)}"
    )
if len(current_routes) != 49:
    fail(f"current route inventory must contain exactly 49 rows: {len(current_routes)}")
if current_routes != immutable_routes:
    fail(
        "current route inventory must equal immutable v0.4.0 exactly: "
        f"added={sorted(current_routes - immutable_routes)}, "
        f"missing={sorted(immutable_routes - current_routes)}"
    )

candidate_routes = {
    line.strip()
    for line in candidate_route_file.read_text().splitlines()
    if line.strip()
}
if candidate_routes != current_routes:
    fail(
        "v0.4.1 candidate route baseline must equal the current 49-route set: "
        f"added={sorted(candidate_routes - current_routes)}, "
        f"missing={sorted(current_routes - candidate_routes)}"
    )

with ledger_file.open(newline="") as stream:
    reader = csv.DictReader(stream, delimiter="\t")
    rows = list(reader)

required_fields = {
    "route",
    "release_inventory_v0.3.3",
    "compatibility_class",
    "changelog_release_note_coverage",
    "upgrade_rollback",
}
if reader.fieldnames is None or not required_fields <= set(reader.fieldnames):
    fail(f"route behavior ledger lacks required fields: {sorted(required_fields)}")
if len(rows) != len(current_routes):
    fail(f"route behavior ledger must contain exactly 49 rows: {len(rows)}")

ledger_by_route: dict[str, dict[str, str]] = {}
for row in rows:
    route = row["route"].strip().strip(chr(96))
    if route in ledger_by_route:
        fail(f"route behavior ledger contains duplicate route: {route}")
    ledger_by_route[route] = row

if set(ledger_by_route) != current_routes:
    fail(
        "route behavior ledger must cover the current 49-route set exactly: "
        f"extra={sorted(set(ledger_by_route) - current_routes)}, "
        f"missing={sorted(current_routes - set(ledger_by_route))}"
    )

expected_membership = {
    route: "yes" if route in historical_routes else "no" for route in current_routes
}
actual_membership = {
    route: row["release_inventory_v0.3.3"].strip()
    for route, row in ledger_by_route.items()
}
if actual_membership != expected_membership:
    fail(
        "route behavior ledger release_inventory_v0.3.3 must match the immutable "
        "47-route set: mismatches="
        f"{sorted(route for route in current_routes if actual_membership.get(route) != expected_membership[route])}"
    )

actual_no = {route for route, value in actual_membership.items() if value == "no"}
if actual_no != additions:
    fail(
        "route behavior ledger must have exactly two historical non-members: "
        f"{sorted(additions)}; actual={sorted(actual_no)}"
    )

candidate_phrase = "candidate inclusion is frozen by Card051's explicit 49-route boundary"
for route in sorted(additions):
    row = ledger_by_route[route]
    if candidate_phrase not in row["compatibility_class"]:
        fail(f"{route} lacks the frozen 49-route candidate inclusion evidence")

pi_row = ledger_by_route["pi.sdk-sidecar"]
if "Required Card051" not in pi_row["changelog_release_note_coverage"]:
    fail("pi.sdk-sidecar lacks explicit Card051 release-note treatment")
if "exact SDK/runtime/sidecar/wire/session-directory axes" not in pi_row["upgrade_rollback"]:
    fail("pi.sdk-sidecar lacks explicit consumer provisioning treatment")
if (
    "omits the route and sidecar calls" not in pi_row["upgrade_rollback"]
    or "pi.rpc" not in pi_row["upgrade_rollback"]
):
    fail("pi.sdk-sidecar lacks explicit v0.3.3 rollback and pi.rpc separation treatment")
PY

sed -n \
  '/<!-- provider-session-lifecycle-matrix:start -->/,/<!-- provider-session-lifecycle-matrix:end -->/p' \
  "$route_matrix_file" |
  sed -n '/^| `/p' > "$route_lifecycle_rows"

sed -n 's/^| `\([^`]*\)` |.*$/\1/p' "$route_lifecycle_rows" |
  LC_ALL=C sort > "$route_lifecycle_actual"

if [ "$(wc -l < "$route_lifecycle_actual" | tr -d ' ')" -ne 49 ]; then
  printf 'provider session lifecycle matrix must contain exactly 49 route rows\n' >&2
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

printf 'provider route, lifecycle, 41-solution/49-route feature, activity, immutable v0.4.0 candidate, and Card050 historical ledger boundary checks passed (49 immutable/current; 47 historical yes; exact no set pi.sdk-sidecar, claude-agent.sdk)\n'
