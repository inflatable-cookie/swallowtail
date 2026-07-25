#!/usr/bin/env bash
set -euo pipefail

route_matrix_repo_root=$(cd "$(dirname "$0")/.." && pwd)
route_matrix_file="$route_matrix_repo_root/docs/guides/provider-route-matrix.md"
route_matrix_actual=$(mktemp)
route_matrix_expected=$(mktemp)
trap 'rm -f "$route_matrix_actual" "$route_matrix_expected"' EXIT

sed -n 's/^| `\([^`]*\)` |.*$/\1/p' "$route_matrix_file" |
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
gemini.live
kimi-code.acp
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

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 22 ]; then
  printf 'provider route matrix must contain exactly 22 route rows\n' >&2
  exit 1
fi

if [ -n "$(uniq -d "$route_matrix_actual")" ]; then
  printf 'provider route matrix contains duplicate route rows\n' >&2
  uniq -d "$route_matrix_actual" >&2
  exit 1
fi

diff -u "$route_matrix_expected" "$route_matrix_actual"
printf 'provider route matrix passed for 22 production routes\n'
