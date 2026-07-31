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
antigravity.catalogue
antigravity.headless
alibaba.conversations
anthropic.managed-agent
anthropic.messages
bedrock.catalogue
bedrock.runtime
claude-agent.acp
claude-code.headless
codex.app-server
codex.exec
cursor-agent.acp
cursor-agent.catalogue
cursor-agent.headless
deepseek.continuation
gemini-cli.acp
gemini-cli.headless
gemini.live
grok-build.acp
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

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 32 ]; then
  printf 'provider route matrix must contain exactly 32 route rows\n' >&2
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

if [ "$(wc -l < "$route_lifecycle_actual" | tr -d ' ')" -ne 32 ]; then
  printf 'provider session lifecycle matrix must contain exactly 32 route rows\n' >&2
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
antigravity.catalogue|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
antigravity.headless|unsupported|no|unsupported|unsupported|unsupported|unsupported
alibaba.conversations|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
anthropic.managed-agent|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
anthropic.messages|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
bedrock.catalogue|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
bedrock.runtime|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
claude-agent.acp|supported|yes|unsupported|unsupported|supported|ProviderDataDeleted
claude-code.headless|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
codex.app-server|supported|yes|supported|supported|supported|ProviderHardDeleted
codex.exec|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
cursor-agent.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
cursor-agent.catalogue|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
cursor-agent.headless|unsupported|no|unsupported|unsupported|unsupported|unsupported
deepseek.continuation|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
gemini-cli.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
gemini-cli.headless|supported|yes|unsupported|unsupported|supported|HistoryRemoved
gemini.live|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
grok-build.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
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


python3 "$route_matrix_repo_root/scripts/provider_route_matrix/validate.py" \
  "$feature_matrix_file"

python3 "$route_matrix_repo_root/scripts/check-provider-activity-matrix.py"

printf 'provider route, lifecycle, 25-solution feature, and activity matrices passed\n'
