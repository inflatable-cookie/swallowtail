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

cat <<'EOF' | LC_ALL=C sort > "$route_matrix_expected"
alibaba.conversations
anthropic.managed-agent
anthropic.messages
bedrock.catalogue
bedrock.runtime
claude-agent.acp
codex.app-server
codex.exec
deepseek.continuation
gemini-cli.acp
gemini-cli.headless
gemini.live
kimi-code.acp
kimi-code.headless
kimi-code.local-server
kimi-platform.chat
llama-cpp.attached
llama-cpp.owned
ollama.attached
openai.background
openai.realtime
opencode.http
pi.rpc
qwen.headless
xai.responses-websocket
EOF

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 25 ]; then
  printf 'provider route matrix must contain exactly 25 route rows\n' >&2
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

if [ "$(wc -l < "$route_lifecycle_actual" | tr -d ' ')" -ne 25 ]; then
  printf 'provider session lifecycle matrix must contain exactly 25 route rows\n' >&2
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

cat <<'EOF' | LC_ALL=C sort > "$route_lifecycle_posture_expected"
alibaba.conversations|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
anthropic.managed-agent|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
anthropic.messages|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
bedrock.catalogue|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
bedrock.runtime|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
claude-agent.acp|supported|yes|unsupported|unsupported|supported|ProviderDataDeleted
codex.app-server|supported|yes|supported|supported|supported|ProviderHardDeleted
codex.exec|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
deepseek.continuation|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
gemini-cli.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
gemini-cli.headless|unsupported|no|unsupported|unsupported|unsupported|unsupported
gemini.live|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
kimi-code.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
kimi-code.headless|unsupported|no|unsupported|unsupported|unsupported|unsupported
kimi-code.local-server|supported|yes|supported|supported|unsupported|unsupported
kimi-platform.chat|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
llama-cpp.attached|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
llama-cpp.owned|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
ollama.attached|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
openai.background|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
openai.realtime|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
opencode.http|supported|yes|unsupported|unsupported|supported|ProviderDataDeleted
pi.rpc|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
qwen.headless|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
xai.responses-websocket|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
EOF

diff -u "$route_lifecycle_posture_expected" "$route_lifecycle_posture_actual"

python3 - "$feature_matrix_file" <<'PY'
import csv
import re
import sys
from collections import Counter

with open(sys.argv[1], newline="", encoding="utf-8") as feature_file:
    rows = list(csv.DictReader(feature_file))

if len(rows) != 21:
    raise SystemExit("provider solution feature matrix must contain exactly 21 rows")

providers = [row["provider"] for row in rows]
if providers != sorted(providers, key=str.casefold):
    raise SystemExit("provider solution feature matrix must be sorted by provider")

expected = Counter(
    {
        "Yes": 16,
        "Session-negotiated": 2,
        "Not applicable": 2,
        "Caller-supplied": 1,
    }
)
actual = Counter(row["model_catalog"] for row in rows)
if actual != expected:
    raise SystemExit(
        f"provider solution model_catalog dispositions changed: {dict(actual)}"
    )

structured = Counter(row["structured_run"] for row in rows)
if structured != Counter({"Yes": 18, "No": 2, "Not applicable": 1}):
    raise SystemExit(
        f"provider solution structured_run dispositions changed: {dict(structured)}"
    )
for row in rows:
    if row["structured_run"] == "Yes" and row["prepared_facade"] != "Yes":
        raise SystemExit(
            f"structured solution lacks a prepared facade: {row['solution']}"
        )

structured_by_route = {
    row["route_id"]: row["structured_run"]
    for row in rows
}
for route in ["gemini.live", "openai.realtime"]:
    if structured_by_route.get(route) != "No":
        raise SystemExit(f"realtime route must remain structured No: {route}")
if structured_by_route.get("llama-cpp.owned") != "Not applicable":
    raise SystemExit("llama.cpp owned serving facade must remain structured Not applicable")
for route in [
    "kimi-code.acp + kimi-code.headless",
    "kimi-code.local-server",
]:
    if structured_by_route.get(route) != "Yes":
        raise SystemExit(f"Kimi structured solution is not realized: {route}")

route_ids = [
    route
    for row in rows
    for route in re.split(r"\s*(?:;|\+)\s*", row["route_id"])
]
if len(route_ids) != 25 or len(set(route_ids)) != 25:
    raise SystemExit("provider solution matrix must cover 25 unique route identities")
PY

printf 'provider route, lifecycle, and 21-solution feature matrices passed\n'
