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
claude-code.headless
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

if [ "$(wc -l < "$route_matrix_actual" | tr -d ' ')" -ne 26 ]; then
  printf 'provider route matrix must contain exactly 26 route rows\n' >&2
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

if [ "$(wc -l < "$route_lifecycle_actual" | tr -d ' ')" -ne 26 ]; then
  printf 'provider session lifecycle matrix must contain exactly 26 route rows\n' >&2
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
claude-code.headless|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
codex.app-server|supported|yes|supported|supported|supported|ProviderHardDeleted
codex.exec|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
deepseek.continuation|not-applicable|no|not-applicable|not-applicable|not-applicable|not-applicable
gemini-cli.acp|unsupported|no|unsupported|unsupported|unsupported|unsupported
gemini-cli.headless|supported|yes|unsupported|unsupported|supported|HistoryRemoved
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

if len(rows) != 22:
    raise SystemExit("provider solution feature matrix must contain exactly 22 rows")

providers = [row["provider"] for row in rows]
if providers != sorted(providers, key=str.casefold):
    raise SystemExit("provider solution feature matrix must be sorted by provider")

expected = Counter(
    {
        "Yes": 16,
        "Session-negotiated": 2,
        "Not applicable": 2,
        "Caller-supplied": 2,
    }
)
actual = Counter(row["model_catalog"] for row in rows)
if actual != expected:
    raise SystemExit(
        f"provider solution model_catalog dispositions changed: {dict(actual)}"
    )

structured = Counter(row["structured_run"] for row in rows)
if structured != Counter({"Yes": 19, "No": 2, "Not applicable": 1}):
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

approval_by_route = {
    row["route_id"]: row["approval_question_exchange"]
    for row in rows
}
if approval_by_route.get("claude-agent.acp") != "Yes":
    raise SystemExit("Claude Agent consumer-mediated permission exchange is not realized")

attachments_by_route = {
    row["route_id"]: row["attachments"]
    for row in rows
}
if attachments_by_route.get("pi.rpc") != "Yes":
    raise SystemExit("Pi RPC attachment input is not realized")

serving_not_applicable = {
    "interactive_session",
    "realtime_media_session",
    "streaming_events",
    "usage_evidence",
    "billed_cost_evidence",
    "output_token_limit",
    "reasoning_selection",
    "structured_output",
    "attachments",
    "consumer_tool_exchange",
    "approval_question_exchange",
    "cancellation_or_interruption",
    "load_session",
    "resume_session",
    "working_resource",
    "bounded_workspace_text_write",
    "external_search",
    "retained_background_execution",
    "stream_reattachment",
    "provider_managed_recovery",
    "provider_session_archive",
    "provider_session_restore",
    "provider_session_delete",
    "native_session_close",
    "owned_remote_resource_cleanup",
    "planned_connection_rollover",
}
serving = next(row for row in rows if row["route_id"] == "llama-cpp.owned")
for feature in serving_not_applicable:
    if serving[feature] != "Not applicable":
        raise SystemExit(
            f"llama.cpp owned serving feature must remain Not applicable: {feature}"
        )

expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 2,
        "structured_run": 2,
        "interactive_session": 5,
        "realtime_media_session": 3,
        "usage_evidence": 2,
        "billed_cost_evidence": 15,
        "output_token_limit": 13,
        "reasoning_selection": 11,
        "structured_output": 17,
        "attachments": 17,
        "consumer_tool_exchange": 17,
        "approval_question_exchange": 15,
        "load_session": 17,
        "resume_session": 16,
        "bounded_workspace_text_write": 6,
        "external_search": 19,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 16,
        "provider_session_archive": 4,
        "provider_session_restore": 4,
        "provider_session_delete": 2,
        "native_session_close": 20,
        "owned_remote_resource_cleanup": 2,
        "planned_connection_rollover": 1,
    }
)
actual_no_counts = Counter()
no_cells = []
matrix_columns = list(rows[0])
audited_columns = matrix_columns[
    matrix_columns.index("unverified_newer_allowed")
    : matrix_columns.index("planned_connection_rollover") + 1
]
for row in rows:
    for feature in audited_columns:
        if row[feature] == "No":
            actual_no_counts[feature] += 1
            no_cells.append((row["provider"], row["solution"], feature))
audited_value_counts = Counter(
    row[feature] for row in rows for feature in audited_columns
)
if audited_value_counts != Counter(
    {
        "Yes": 202,
        "No": 234,
        "Not applicable": 216,
        "Partial": 4,
        "Caller-supplied": 2,
        "Session-negotiated": 2,
    }
):
    raise SystemExit(
        f"provider solution disposition counts changed: {dict(audited_value_counts)}"
    )
if actual_no_counts != expected_no_counts:
    raise SystemExit(
        f"provider solution No inventory changed: {dict(actual_no_counts)}"
    )
if len(no_cells) != 234 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 234 unique cells")

no_classification_overrides = {
    (
        "usage_evidence",
        "kimi-code.acp + kimi-code.headless",
    ): "upstream_unsupported",
    ("usage_evidence", "kimi-code.local-server"): "upstream_unsupported",
    ("structured_run", "gemini.live"): "operation_shape_not_applicable",
    ("structured_run", "openai.realtime"): "operation_shape_not_applicable",
}
generation_control_classifications = {
    ("output_token_limit", "qwen.headless"): "contract_or_corpus_required",
    ("reasoning_selection", "qwen.headless"): "contract_or_corpus_required",
    ("structured_output", "qwen.headless"): "contract_or_corpus_required",
    ("output_token_limit", "alibaba.conversations"): "upstream_unsupported",
    ("reasoning_selection", "alibaba.conversations"): "contract_or_corpus_required",
    ("structured_output", "alibaba.conversations"): "upstream_unsupported",
    ("reasoning_selection", "bedrock.catalogue; bedrock.runtime"): "contract_or_corpus_required",
    ("structured_output", "bedrock.catalogue; bedrock.runtime"): "contract_or_corpus_required",
    ("output_token_limit", "claude-agent.acp"): "upstream_unsupported",
    ("structured_output", "claude-agent.acp"): "upstream_unsupported",
    ("output_token_limit", "claude-code.headless"): "upstream_unsupported",
    ("structured_output", "claude-code.headless"): "upstream_unsupported",
    ("output_token_limit", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("reasoning_selection", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("structured_output", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("reasoning_selection", "anthropic.messages"): "contract_or_corpus_required",
    ("structured_output", "anthropic.messages"): "contract_or_corpus_required",
    ("output_token_limit", "pi.rpc"): "upstream_unsupported",
    ("reasoning_selection", "pi.rpc"): "contract_or_corpus_required",
    ("structured_output", "pi.rpc"): "upstream_unsupported",
    ("structured_output", "deepseek.continuation"): "upstream_unsupported",
    ("output_token_limit", "gemini-cli.acp + gemini-cli.headless"): "contract_or_corpus_required",
    ("reasoning_selection", "gemini-cli.acp + gemini-cli.headless"): "contract_or_corpus_required",
    ("structured_output", "gemini-cli.acp + gemini-cli.headless"): "upstream_unsupported",
    ("output_token_limit", "gemini.live"): "ready_existing_contract",
    ("reasoning_selection", "gemini.live"): "contract_or_corpus_required",
    ("structured_output", "gemini.live"): "upstream_unsupported",
    ("reasoning_selection", "llama-cpp.attached"): "contract_or_corpus_required",
    ("structured_output", "llama-cpp.attached"): "contract_or_corpus_required",
    ("output_token_limit", "kimi-code.acp + kimi-code.headless"): "upstream_unsupported",
    ("structured_output", "kimi-code.acp + kimi-code.headless"): "upstream_unsupported",
    ("output_token_limit", "kimi-code.local-server"): "upstream_unsupported",
    ("structured_output", "kimi-code.local-server"): "upstream_unsupported",
    ("structured_output", "kimi-platform.chat"): "upstream_unsupported",
    ("output_token_limit", "codex.app-server; codex.exec"): "upstream_unsupported",
    ("reasoning_selection", "openai.realtime"): "upstream_unsupported",
    ("structured_output", "openai.realtime"): "upstream_unsupported",
    ("output_token_limit", "opencode.http"): "upstream_unsupported",
    ("output_token_limit", "xai.responses-websocket"): "ready_operator_hold",
    ("reasoning_selection", "xai.responses-websocket"): "ready_operator_hold",
    ("structured_output", "xai.responses-websocket"): "ready_operator_hold",
}
input_callback_classifications = {
    ("attachments", "qwen.headless"): "upstream_unsupported",
    ("consumer_tool_exchange", "qwen.headless"): "upstream_unsupported",
    ("approval_question_exchange", "qwen.headless"): "upstream_unsupported",
    ("external_search", "qwen.headless"): "upstream_unsupported",
    ("attachments", "alibaba.conversations"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "alibaba.conversations"): "contract_or_corpus_required",
    ("approval_question_exchange", "alibaba.conversations"): "upstream_unsupported",
    ("external_search", "alibaba.conversations"): "contract_or_corpus_required",
    ("attachments", "bedrock.catalogue; bedrock.runtime"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "bedrock.catalogue; bedrock.runtime"): "contract_or_corpus_required",
    ("approval_question_exchange", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("external_search", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("attachments", "claude-agent.acp"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "claude-agent.acp"): "contract_or_corpus_required",
    ("external_search", "claude-agent.acp"): "contract_or_corpus_required",
    ("attachments", "claude-code.headless"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "claude-code.headless"): "contract_or_corpus_required",
    ("approval_question_exchange", "claude-code.headless"): "contract_or_corpus_required",
    ("external_search", "claude-code.headless"): "contract_or_corpus_required",
    ("attachments", "anthropic.managed-agent"): "contract_or_corpus_required",
    ("approval_question_exchange", "anthropic.managed-agent"): "contract_or_corpus_required",
    ("external_search", "anthropic.managed-agent"): "contract_or_corpus_required",
    ("approval_question_exchange", "anthropic.messages"): "upstream_unsupported",
    ("consumer_tool_exchange", "pi.rpc"): "upstream_unsupported",
    ("external_search", "pi.rpc"): "upstream_unsupported",
    ("attachments", "deepseek.continuation"): "upstream_unsupported",
    ("approval_question_exchange", "deepseek.continuation"): "upstream_unsupported",
    ("external_search", "deepseek.continuation"): "upstream_unsupported",
    ("attachments", "gemini-cli.acp + gemini-cli.headless"): "composite_partial_only",
    ("consumer_tool_exchange", "gemini-cli.acp + gemini-cli.headless"): "composite_partial_only",
    ("external_search", "gemini-cli.acp + gemini-cli.headless"): "contract_or_corpus_required",
    ("attachments", "gemini.live"): "operation_shape_not_applicable",
    ("consumer_tool_exchange", "gemini.live"): "contract_or_corpus_required",
    ("approval_question_exchange", "gemini.live"): "upstream_unsupported",
    ("external_search", "gemini.live"): "contract_or_corpus_required",
    ("attachments", "llama-cpp.attached"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "llama-cpp.attached"): "contract_or_corpus_required",
    ("approval_question_exchange", "llama-cpp.attached"): "upstream_unsupported",
    ("external_search", "llama-cpp.attached"): "upstream_unsupported",
    ("attachments", "kimi-code.acp + kimi-code.headless"): "composite_partial_only",
    ("consumer_tool_exchange", "kimi-code.acp + kimi-code.headless"): "composite_partial_only",
    ("approval_question_exchange", "kimi-code.acp + kimi-code.headless"): "composite_partial_only",
    ("external_search", "kimi-code.acp + kimi-code.headless"): "contract_or_corpus_required",
    ("attachments", "kimi-code.local-server"): "ready_existing_contract",
    ("consumer_tool_exchange", "kimi-code.local-server"): "upstream_unsupported",
    ("external_search", "kimi-code.local-server"): "contract_or_corpus_required",
    ("attachments", "kimi-platform.chat"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "kimi-platform.chat"): "contract_or_corpus_required",
    ("approval_question_exchange", "kimi-platform.chat"): "upstream_unsupported",
    ("external_search", "kimi-platform.chat"): "contract_or_corpus_required",
    ("attachments", "ollama.attached"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "ollama.attached"): "contract_or_corpus_required",
    ("approval_question_exchange", "ollama.attached"): "upstream_unsupported",
    ("external_search", "ollama.attached"): "upstream_unsupported",
    ("attachments", "openai.realtime"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "openai.realtime"): "contract_or_corpus_required",
    ("approval_question_exchange", "openai.realtime"): "upstream_unsupported",
    ("external_search", "openai.realtime"): "upstream_unsupported",
    ("attachments", "openai.background"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "openai.background"): "upstream_unsupported",
    ("approval_question_exchange", "openai.background"): "upstream_unsupported",
    ("external_search", "openai.background"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "opencode.http"): "contract_or_corpus_required",
    ("external_search", "opencode.http"): "contract_or_corpus_required",
    ("attachments", "xai.responses-websocket"): "ready_operator_hold",
    ("consumer_tool_exchange", "xai.responses-websocket"): "ready_operator_hold",
    ("approval_question_exchange", "xai.responses-websocket"): "upstream_unsupported",
    ("external_search", "xai.responses-websocket"): "ready_operator_hold",
}
session_continuity_classifications = {
    ("load_session", "qwen.headless"): "operation_shape_not_applicable",
    ("resume_session", "qwen.headless"): "operation_shape_not_applicable",
    ("native_session_close", "qwen.headless"): "operation_shape_not_applicable",
    ("load_session", "alibaba.conversations"): "shared_contract_expansion_required",
    ("resume_session", "alibaba.conversations"): "shared_contract_expansion_required",
    ("native_session_close", "alibaba.conversations"): "upstream_unsupported",
    ("load_session", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("resume_session", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("native_session_close", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("load_session", "claude-code.headless"): "operation_shape_not_applicable",
    ("resume_session", "claude-code.headless"): "operation_shape_not_applicable",
    ("native_session_close", "claude-code.headless"): "operation_shape_not_applicable",
    ("load_session", "anthropic.managed-agent"): "shared_contract_expansion_required",
    ("resume_session", "anthropic.managed-agent"): "shared_contract_expansion_required",
    ("native_session_close", "anthropic.managed-agent"): "upstream_unsupported",
    ("load_session", "anthropic.messages"): "operation_shape_not_applicable",
    ("resume_session", "anthropic.messages"): "operation_shape_not_applicable",
    ("native_session_close", "anthropic.messages"): "operation_shape_not_applicable",
    ("load_session", "pi.rpc"): "ready_existing_contract",
    ("resume_session", "pi.rpc"): "ready_existing_contract",
    ("native_session_close", "pi.rpc"): "upstream_unsupported",
    ("load_session", "deepseek.continuation"): "operation_shape_not_applicable",
    ("resume_session", "deepseek.continuation"): "operation_shape_not_applicable",
    ("native_session_close", "deepseek.continuation"): "operation_shape_not_applicable",
    ("load_session", "gemini-cli.acp + gemini-cli.headless"): "upstream_ordering_blocked",
    ("resume_session", "gemini-cli.acp + gemini-cli.headless"): "upstream_unsupported",
    ("native_session_close", "gemini-cli.acp + gemini-cli.headless"): "upstream_unsupported",
    ("load_session", "gemini.live"): "operation_shape_not_applicable",
    ("resume_session", "gemini.live"): "operation_shape_not_applicable",
    ("native_session_close", "gemini.live"): "operation_shape_not_applicable",
    ("load_session", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("resume_session", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("native_session_close", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("native_session_close", "kimi-code.acp + kimi-code.headless"): "upstream_unsupported",
    ("load_session", "kimi-code.local-server"): "upstream_unsupported",
    ("native_session_close", "kimi-code.local-server"): "upstream_unsupported",
    ("load_session", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("resume_session", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("native_session_close", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("load_session", "ollama.attached"): "operation_shape_not_applicable",
    ("resume_session", "ollama.attached"): "operation_shape_not_applicable",
    ("native_session_close", "ollama.attached"): "operation_shape_not_applicable",
    ("native_session_close", "codex.app-server; codex.exec"): "upstream_unsupported",
    ("load_session", "openai.realtime"): "operation_shape_not_applicable",
    ("resume_session", "openai.realtime"): "operation_shape_not_applicable",
    ("native_session_close", "openai.realtime"): "operation_shape_not_applicable",
    ("load_session", "openai.background"): "operation_shape_not_applicable",
    ("resume_session", "openai.background"): "operation_shape_not_applicable",
    ("native_session_close", "openai.background"): "operation_shape_not_applicable",
    ("native_session_close", "opencode.http"): "upstream_unsupported",
    ("load_session", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("resume_session", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("native_session_close", "xai.responses-websocket"): "operation_shape_not_applicable",
}
provider_retention_not_applicable = {
    "qwen.headless",
    "alibaba.conversations",
    "bedrock.catalogue; bedrock.runtime",
    "claude-code.headless",
    "anthropic.managed-agent",
    "anthropic.messages",
    "pi.rpc",
    "deepseek.continuation",
    "gemini.live",
    "llama-cpp.attached",
    "kimi-platform.chat",
    "ollama.attached",
    "openai.realtime",
    "xai.responses-websocket",
}
provider_session_not_applicable = provider_retention_not_applicable | {
    "openai.background",
}
owned_cleanup_not_applicable = (
    provider_retention_not_applicable
    - {"alibaba.conversations", "anthropic.managed-agent"}
) | {"codex.app-server; codex.exec"}
provider_retention_classifications = {}
for feature in ["provider_session_archive", "provider_session_restore"]:
    for route in provider_session_not_applicable:
        provider_retention_classifications[(feature, route)] = (
            "operation_shape_not_applicable"
        )
    for route in [
        "claude-agent.acp",
        "gemini-cli.acp + gemini-cli.headless",
        "kimi-code.acp + kimi-code.headless",
        "opencode.http",
    ]:
        provider_retention_classifications[(feature, route)] = "upstream_unsupported"
for route in provider_session_not_applicable:
    provider_retention_classifications[("provider_session_delete", route)] = (
        "operation_shape_not_applicable"
    )
for route in [
    "kimi-code.acp + kimi-code.headless",
    "kimi-code.local-server",
]:
    provider_retention_classifications[("provider_session_delete", route)] = (
        "upstream_unsupported"
    )
provider_retention_classifications[
    ("provider_session_delete", "gemini-cli.acp + gemini-cli.headless")
] = "separate_transport_and_corpus_required"
for route in owned_cleanup_not_applicable:
    provider_retention_classifications[("owned_remote_resource_cleanup", route)] = (
        "operation_shape_not_applicable"
    )
for route in [
    "kimi-code.acp + kimi-code.headless",
    "kimi-code.local-server",
]:
    provider_retention_classifications[("owned_remote_resource_cleanup", route)] = (
        "upstream_unsupported"
    )
provider_retention_classifications[
    ("owned_remote_resource_cleanup", "gemini-cli.acp + gemini-cli.headless")
] = "separate_transport_and_corpus_required"
provider_retention_classifications[
    ("owned_remote_resource_cleanup", "claude-agent.acp")
] = "ready_existing_contract"
provider_retention_classifications[
    ("owned_remote_resource_cleanup", "openai.background")
] = "shared_contract_and_corpus_required"
provider_retention_classifications[
    ("owned_remote_resource_cleanup", "opencode.http")
] = "realized_matrix_false_negative"

retained_execution_classifications = {
    ("retained_background_execution", "qwen.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "qwen.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "qwen.headless"): "upstream_unsupported",
    ("retained_background_execution", "alibaba.conversations"): "upstream_unsupported",
    ("stream_reattachment", "alibaba.conversations"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "alibaba.conversations"): "upstream_unsupported",
    ("retained_background_execution", "bedrock.catalogue; bedrock.runtime"): "separate_route_and_contract_required",
    ("stream_reattachment", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("provider_managed_recovery", "bedrock.catalogue; bedrock.runtime"): "upstream_unsupported",
    ("retained_background_execution", "claude-agent.acp"): "operation_shape_not_applicable",
    ("stream_reattachment", "claude-agent.acp"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "claude-agent.acp"): "upstream_unsupported",
    ("retained_background_execution", "claude-code.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "claude-code.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "claude-code.headless"): "upstream_unsupported",
    ("retained_background_execution", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("retained_background_execution", "anthropic.messages"): "separate_route_and_contract_required",
    ("stream_reattachment", "anthropic.messages"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "anthropic.messages"): "upstream_unsupported",
    ("retained_background_execution", "pi.rpc"): "operation_shape_not_applicable",
    ("stream_reattachment", "pi.rpc"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "pi.rpc"): "upstream_unsupported",
    ("retained_background_execution", "deepseek.continuation"): "upstream_unsupported",
    ("stream_reattachment", "deepseek.continuation"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "deepseek.continuation"): "upstream_unsupported",
    ("retained_background_execution", "gemini-cli.acp + gemini-cli.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "gemini-cli.acp + gemini-cli.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "gemini-cli.acp + gemini-cli.headless"): "upstream_unsupported",
    ("retained_background_execution", "gemini.live"): "operation_shape_not_applicable",
    ("stream_reattachment", "gemini.live"): "upstream_unsupported",
    ("provider_managed_recovery", "gemini.live"): "upstream_unsupported",
    ("retained_background_execution", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("stream_reattachment", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("retained_background_execution", "kimi-code.acp + kimi-code.headless"): "operation_shape_not_applicable",
    ("stream_reattachment", "kimi-code.acp + kimi-code.headless"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "kimi-code.acp + kimi-code.headless"): "shared_contract_and_corpus_required",
    ("retained_background_execution", "kimi-code.local-server"): "operation_shape_not_applicable",
    ("stream_reattachment", "kimi-code.local-server"): "shared_contract_and_corpus_required",
    ("provider_managed_recovery", "kimi-code.local-server"): "shared_contract_and_corpus_required",
    ("retained_background_execution", "kimi-platform.chat"): "upstream_unsupported",
    ("stream_reattachment", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "kimi-platform.chat"): "upstream_unsupported",
    ("retained_background_execution", "ollama.attached"): "operation_shape_not_applicable",
    ("stream_reattachment", "ollama.attached"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "ollama.attached"): "operation_shape_not_applicable",
    ("retained_background_execution", "codex.app-server; codex.exec"): "operation_shape_not_applicable",
    ("stream_reattachment", "codex.app-server; codex.exec"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "codex.app-server; codex.exec"): "upstream_unsupported",
    ("retained_background_execution", "openai.realtime"): "operation_shape_not_applicable",
    ("stream_reattachment", "openai.realtime"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "openai.realtime"): "upstream_unsupported",
    ("provider_managed_recovery", "openai.background"): "upstream_unsupported",
    ("retained_background_execution", "opencode.http"): "operation_shape_not_applicable",
    ("stream_reattachment", "opencode.http"): "upstream_unsupported",
    ("provider_managed_recovery", "opencode.http"): "upstream_unsupported",
    ("retained_background_execution", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("stream_reattachment", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("provider_managed_recovery", "xai.responses-websocket"): "upstream_unsupported",
}

working_resource_write_classifications = {
    ("bounded_workspace_text_write", "qwen.headless"): "upstream_unsupported",
    ("working_resource", "alibaba.conversations"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "alibaba.conversations"): "operation_shape_not_applicable",
    ("working_resource", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "bedrock.catalogue; bedrock.runtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "claude-agent.acp"): "upstream_unsupported",
    ("bounded_workspace_text_write", "claude-code.headless"): "upstream_unsupported",
    ("working_resource", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "anthropic.managed-agent"): "operation_shape_not_applicable",
    ("working_resource", "anthropic.messages"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "anthropic.messages"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "pi.rpc"): "upstream_unsupported",
    ("working_resource", "deepseek.continuation"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "deepseek.continuation"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "gemini-cli.acp + gemini-cli.headless"): "contract_or_corpus_required",
    ("working_resource", "gemini.live"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "gemini.live"): "operation_shape_not_applicable",
    ("working_resource", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "llama-cpp.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "kimi-code.local-server"): "upstream_unsupported",
    ("working_resource", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "kimi-platform.chat"): "operation_shape_not_applicable",
    ("working_resource", "ollama.attached"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "ollama.attached"): "operation_shape_not_applicable",
    ("working_resource", "openai.realtime"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "openai.realtime"): "operation_shape_not_applicable",
    ("working_resource", "openai.background"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "openai.background"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "opencode.http"): "upstream_unsupported",
    ("working_resource", "xai.responses-websocket"): "operation_shape_not_applicable",
    ("bounded_workspace_text_write", "xai.responses-websocket"): "operation_shape_not_applicable",
}

owned_runtime_not_applicable = {
    "qwen.headless",
    "alibaba.conversations",
    "bedrock.catalogue; bedrock.runtime",
    "claude-agent.acp",
    "claude-code.headless",
    "anthropic.managed-agent",
    "anthropic.messages",
    "pi.rpc",
    "deepseek.continuation",
    "gemini-cli.acp + gemini-cli.headless",
    "gemini.live",
    "llama-cpp.attached",
    "kimi-code.acp + kimi-code.headless",
    "kimi-platform.chat",
    "ollama.attached",
    "codex.app-server; codex.exec",
    "openai.realtime",
    "openai.background",
    "opencode.http",
    "xai.responses-websocket",
}
rollover_not_applicable = {
    "qwen.headless",
    "alibaba.conversations",
    "bedrock.catalogue; bedrock.runtime",
    "claude-agent.acp",
    "claude-code.headless",
    "anthropic.managed-agent",
    "anthropic.messages",
    "pi.rpc",
    "deepseek.continuation",
    "gemini-cli.acp + gemini-cli.headless",
    "llama-cpp.attached",
    "kimi-code.acp + kimi-code.headless",
    "kimi-code.local-server",
    "kimi-platform.chat",
    "ollama.attached",
    "codex.app-server; codex.exec",
    "openai.background",
    "opencode.http",
    "xai.responses-websocket",
}
runtime_rollover_classifications = {
    **{
        ("owned_runtime_lifecycle", route): "operation_shape_not_applicable"
        for route in owned_runtime_not_applicable
    },
    **{
        ("planned_connection_rollover", route): "operation_shape_not_applicable"
        for route in rollover_not_applicable
    },
    (
        "planned_connection_rollover",
        "openai.realtime",
    ): "selected_surface_absence",
}

residual_interface_not_runtime_ordered = {
    ("unverified_newer_allowed", route)
    for route in {
        "alibaba.conversations",
        "bedrock.catalogue; bedrock.runtime",
        "anthropic.managed-agent",
        "anthropic.messages",
        "deepseek.continuation",
        "gemini.live",
        "kimi-platform.chat",
        "openai.realtime",
        "openai.background",
        "xai.responses-websocket",
    }
}
residual_contract_or_corpus = {
    ("interactive_session", "qwen.headless"),
    ("interactive_session", "bedrock.catalogue; bedrock.runtime"),
    ("interactive_session", "claude-code.headless"),
    ("interactive_session", "anthropic.managed-agent"),
    ("unverified_newer_allowed", "llama-cpp.attached"),
    ("interactive_session", "llama-cpp.attached"),
    ("unverified_newer_allowed", "llama-cpp.owned"),
    ("interactive_session", "kimi-platform.chat"),
    ("interactive_session", "ollama.attached"),
}
residual_operation_not_applicable = {
    ("interactive_session", route)
    for route in {
        "gemini.live",
        "openai.realtime",
        "openai.background",
    }
} | {
    ("realtime_media_session", route)
    for route in {
        "qwen.headless",
        "claude-agent.acp",
        "claude-code.headless",
        "anthropic.managed-agent",
        "anthropic.messages",
        "pi.rpc",
        "deepseek.continuation",
        "gemini-cli.acp + gemini-cli.headless",
        "llama-cpp.attached",
        "kimi-code.acp + kimi-code.headless",
        "kimi-code.local-server",
        "kimi-platform.chat",
        "ollama.attached",
        "codex.app-server; codex.exec",
        "openai.background",
        "opencode.http",
    }
}
residual_separate_route = {
    ("realtime_media_session", "alibaba.conversations"),
    ("realtime_media_session", "bedrock.catalogue; bedrock.runtime"),
    ("realtime_media_session", "xai.responses-websocket"),
}
residual_no_provider_billing = {
    ("billed_cost_evidence", route)
    for route in {
        "claude-code.headless",
        "llama-cpp.attached",
        "kimi-code.acp + kimi-code.headless",
        "kimi-code.local-server",
        "ollama.attached",
    }
}
residual_selected_surface_absence = {
    ("billed_cost_evidence", route)
    for route in {
        "qwen.headless",
        "alibaba.conversations",
        "bedrock.catalogue; bedrock.runtime",
        "anthropic.managed-agent",
        "anthropic.messages",
        "deepseek.continuation",
        "gemini-cli.acp + gemini-cli.headless",
        "gemini.live",
        "kimi-platform.chat",
        "codex.app-server; codex.exec",
        "openai.realtime",
        "openai.background",
    }
}
residual_non_authoritative_cost = {
    ("billed_cost_evidence", route)
    for route in {
        "claude-agent.acp",
        "pi.rpc",
        "opencode.http",
    }
}
residual_feature_classifications = {
    **{
        cell: "interface_axis_not_runtime_ordered"
        for cell in residual_interface_not_runtime_ordered
    },
    **{
        cell: "contract_or_corpus_required"
        for cell in residual_contract_or_corpus
    },
    **{
        cell: "operation_shape_not_applicable"
        for cell in residual_operation_not_applicable
    },
    **{
        cell: "separate_route_and_contract_required"
        for cell in residual_separate_route
    },
    **{
        cell: "no_provider_billing_boundary"
        for cell in residual_no_provider_billing
    },
    **{
        cell: "selected_surface_absence"
        for cell in residual_selected_surface_absence
    },
    **{
        cell: "non_authoritative_cost_evidence"
        for cell in residual_non_authoritative_cost
    },
}
if len(residual_feature_classifications) != 61:
    raise SystemExit("residual feature starting inventory must contain exactly 61 cells")
if Counter(residual_feature_classifications.values()) != Counter(
    {
        "interface_axis_not_runtime_ordered": 10,
        "contract_or_corpus_required": 9,
        "operation_shape_not_applicable": 19,
        "separate_route_and_contract_required": 3,
        "no_provider_billing_boundary": 5,
        "selected_surface_absence": 12,
        "non_authoritative_cost_evidence": 3,
    }
):
    raise SystemExit("residual feature classification counts changed")
residual_feature_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in [
        "unverified_newer_allowed",
        "interactive_session",
        "realtime_media_session",
        "billed_cost_evidence",
    ]
}
residual_not_applicable = {
    "interface_axis_not_runtime_ordered",
    "operation_shape_not_applicable",
    "no_provider_billing_boundary",
}
for cell, classification in residual_feature_classifications.items():
    expected_value = (
        "Yes"
        if cell
        in {
            ("interactive_session", "qwen.headless"),
            ("interactive_session", "ollama.attached"),
        }
        else "Not applicable"
        if classification in residual_not_applicable
        else "No"
    )
    if residual_feature_values.get(cell) != expected_value:
        raise SystemExit(
            f"residual feature final disposition changed: {cell} expected {expected_value}"
        )
if Counter(
    residual_feature_values[cell] for cell in residual_feature_classifications
) != Counter({"Not applicable": 34, "No": 25, "Yes": 2}):
    raise SystemExit("residual feature final counts changed")

provider_retention_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in [
        "provider_session_archive",
        "provider_session_restore",
        "provider_session_delete",
        "owned_remote_resource_cleanup",
    ]
}
if len(provider_retention_classifications) != 75:
    raise SystemExit("provider-retention starting inventory must contain 75 cells")
if Counter(provider_retention_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 58,
        "upstream_unsupported": 12,
        "separate_transport_and_corpus_required": 2,
        "realized_matrix_false_negative": 1,
        "ready_existing_contract": 1,
        "shared_contract_and_corpus_required": 1,
    }
):
    raise SystemExit("provider-retention classification counts changed")
provider_retention_expected_values = {
    "operation_shape_not_applicable": "Not applicable",
    "upstream_unsupported": "No",
    "separate_transport_and_corpus_required": "Yes",
    "realized_matrix_false_negative": "Yes",
    "ready_existing_contract": "Yes",
    "shared_contract_and_corpus_required": "Yes",
}
for cell, classification in provider_retention_classifications.items():
    expected_value = provider_retention_expected_values[classification]
    if provider_retention_values.get(cell) != expected_value:
        raise SystemExit(
            "provider-retention final disposition changed: "
            f"{cell} expected {expected_value}"
        )
provider_retention_final_counts = Counter(
    provider_retention_values[cell] for cell in provider_retention_classifications
)
if provider_retention_final_counts != Counter(
    {"Not applicable": 58, "No": 12, "Yes": 5}
):
    raise SystemExit(
        "provider-retention final counts changed: "
        f"{dict(provider_retention_final_counts)}"
    )

retained_execution_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in [
        "retained_background_execution",
        "stream_reattachment",
        "provider_managed_recovery",
    ]
}
if len(retained_execution_classifications) != 59:
    raise SystemExit("retained-execution starting inventory must contain exactly 59 cells")
if Counter(retained_execution_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 32,
        "upstream_unsupported": 22,
        "separate_route_and_contract_required": 2,
        "shared_contract_and_corpus_required": 3,
    }
):
    raise SystemExit("retained-execution classification counts changed")
retained_execution_realized = {
    ("provider_managed_recovery", "kimi-code.acp + kimi-code.headless"): "Partial",
    ("stream_reattachment", "kimi-code.local-server"): "Yes",
    ("provider_managed_recovery", "kimi-code.local-server"): "Yes",
}
for cell, classification in retained_execution_classifications.items():
    expected = retained_execution_realized.get(
        cell,
        "Not applicable"
        if classification == "operation_shape_not_applicable"
        else "No",
    )
    if retained_execution_values.get(cell) != expected:
        raise SystemExit(
            f"retained-execution final disposition changed: {cell} expected {expected}"
        )
retained_execution_final_counts = Counter(
    retained_execution_values[cell] for cell in retained_execution_classifications
)
if retained_execution_final_counts != Counter(
    {"Not applicable": 32, "No": 24, "Yes": 2, "Partial": 1}
):
    raise SystemExit(
        "retained-execution final counts changed: "
        f"{dict(retained_execution_final_counts)}"
    )

working_resource_write_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in ["working_resource", "bounded_workspace_text_write"]
}
if len(working_resource_write_classifications) != 31:
    raise SystemExit("working-resource/write starting inventory must contain exactly 31 cells")
if Counter(working_resource_write_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 24,
        "upstream_unsupported": 6,
        "contract_or_corpus_required": 1,
    }
):
    raise SystemExit("working-resource/write classification counts changed")
working_resource_write_expected = {
    "operation_shape_not_applicable": "Not applicable",
    "upstream_unsupported": "No",
    "contract_or_corpus_required": "Yes",
}
for cell, classification in working_resource_write_classifications.items():
    expected = working_resource_write_expected[classification]
    if working_resource_write_values.get(cell) != expected:
        raise SystemExit(
            f"working-resource/write final disposition changed: {cell} expected {expected}"
        )
if Counter(
    working_resource_write_values[cell]
    for cell in working_resource_write_classifications
) != Counter({"Not applicable": 24, "No": 6, "Yes": 1}):
    raise SystemExit("working-resource/write final counts changed")

runtime_rollover_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in [
        "owned_runtime_lifecycle",
        "planned_connection_rollover",
    ]
}
if len(runtime_rollover_classifications) != 40:
    raise SystemExit("runtime-ownership/rollover inventory must contain exactly 40 cells")
if Counter(runtime_rollover_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 39,
        "selected_surface_absence": 1,
    }
):
    raise SystemExit("runtime-ownership/rollover classification counts changed")
for cell, classification in runtime_rollover_classifications.items():
    expected = (
        "Not applicable"
        if classification == "operation_shape_not_applicable"
        else "No"
    )
    if runtime_rollover_values.get(cell) != expected:
        raise SystemExit(
            "runtime-ownership/rollover final disposition changed: "
            f"{cell} expected {expected}"
        )
if Counter(
    runtime_rollover_values[cell]
    for cell in runtime_rollover_classifications
) != Counter({"Not applicable": 39, "No": 1}):
    raise SystemExit("runtime-ownership/rollover final counts changed")

generation_control_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in ["output_token_limit", "reasoning_selection", "structured_output"]
    if row[feature] == "No"
}
if len(generation_control_no_cells) != 41:
    raise SystemExit("generation-control inventory must contain exactly 41 No cells")
if generation_control_no_cells != set(generation_control_classifications):
    raise SystemExit("generation-control No classifications changed")

input_callback_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in [
        "attachments",
        "consumer_tool_exchange",
        "approval_question_exchange",
        "external_search",
    ]
    if row[feature] == "No"
}
if len(input_callback_no_cells) != 68:
    raise SystemExit("input/callback inventory must contain exactly 68 No cells")
if input_callback_no_cells != set(input_callback_classifications):
    raise SystemExit("input/callback No classifications changed")

session_continuity_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in ["load_session", "resume_session", "native_session_close"]
    if row[feature] == "No"
}
if len(session_continuity_no_cells) != 53:
    raise SystemExit("session-continuity inventory must contain exactly 53 No cells")
if session_continuity_no_cells != set(session_continuity_classifications):
    raise SystemExit("session-continuity No classifications changed")

classification_counts = Counter()
for row in rows:
    for feature in audited_columns:
        if row[feature] != "No":
            continue
        cell = (feature, row["route_id"])
        classification = no_classification_overrides.get(
            cell,
            generation_control_classifications.get(cell)
            or input_callback_classifications.get(cell)
            or session_continuity_classifications.get(cell)
            or provider_retention_classifications.get(cell)
            or retained_execution_classifications.get(cell)
            or working_resource_write_classifications.get(cell)
            or runtime_rollover_classifications.get(cell)
            or residual_feature_classifications.get(cell)
            or "missing_shared_contract_or_currentness_evidence",
        )
        classification_counts[classification] += 1
if classification_counts != Counter(
    {
        "contract_or_corpus_required": 54,
        "upstream_unsupported": 97,
        "operation_shape_not_applicable": 42,
        "ready_existing_contract": 4,
        "ready_operator_hold": 6,
        "composite_partial_only": 5,
        "shared_contract_expansion_required": 4,
        "upstream_ordering_blocked": 1,
        "separate_route_and_contract_required": 5,
        "selected_surface_absence": 13,
        "non_authoritative_cost_evidence": 3,
    }
):
    raise SystemExit(
        f"provider solution No classifications changed: {dict(classification_counts)}"
    )

route_ids = [
    route
    for row in rows
    for route in re.split(r"\s*(?:;|\+)\s*", row["route_id"])
]
if len(route_ids) != 26 or len(set(route_ids)) != 26:
    raise SystemExit("provider solution matrix must cover 26 unique route identities")
PY

printf 'provider route, lifecycle, and 22-solution feature matrices passed\n'
