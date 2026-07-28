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
        "unverified_newer_allowed": 12,
        "structured_run": 2,
        "interactive_session": 11,
        "realtime_media_session": 19,
        "usage_evidence": 2,
        "billed_cost_evidence": 20,
        "output_token_limit": 13,
        "reasoning_selection": 11,
        "structured_output": 17,
        "attachments": 19,
        "consumer_tool_exchange": 18,
        "approval_question_exchange": 16,
        "load_session": 20,
        "resume_session": 18,
        "working_resource": 12,
        "bounded_workspace_text_write": 19,
        "external_search": 20,
        "retained_background_execution": 20,
        "stream_reattachment": 19,
        "provider_managed_recovery": 20,
        "provider_session_archive": 19,
        "provider_session_restore": 19,
        "provider_session_delete": 18,
        "native_session_close": 20,
        "owned_remote_resource_cleanup": 19,
        "owned_runtime_lifecycle": 20,
        "planned_connection_rollover": 20,
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
if actual_no_counts != expected_no_counts:
    raise SystemExit(
        f"provider solution No inventory changed: {dict(actual_no_counts)}"
    )
if len(no_cells) != 443 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 443 unique cells")

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
    ("attachments", "anthropic.messages"): "contract_or_corpus_required",
    ("consumer_tool_exchange", "anthropic.messages"): "contract_or_corpus_required",
    ("approval_question_exchange", "anthropic.messages"): "upstream_unsupported",
    ("external_search", "anthropic.messages"): "contract_or_corpus_required",
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
    ("attachments", "opencode.http"): "ready_existing_contract",
    ("consumer_tool_exchange", "opencode.http"): "contract_or_corpus_required",
    ("approval_question_exchange", "opencode.http"): "ready_existing_contract",
    ("external_search", "opencode.http"): "contract_or_corpus_required",
    ("attachments", "xai.responses-websocket"): "ready_operator_hold",
    ("consumer_tool_exchange", "xai.responses-websocket"): "ready_operator_hold",
    ("approval_question_exchange", "xai.responses-websocket"): "upstream_unsupported",
    ("external_search", "xai.responses-websocket"): "ready_operator_hold",
}
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
if len(input_callback_no_cells) != 73:
    raise SystemExit("input/callback inventory must contain exactly 73 No cells")
if input_callback_no_cells != set(input_callback_classifications):
    raise SystemExit("input/callback No classifications changed")

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
            or "missing_shared_contract_or_currentness_evidence",
        )
        classification_counts[classification] += 1
if classification_counts != Counter(
    {
        "missing_shared_contract_or_currentness_evidence": 325,
        "contract_or_corpus_required": 50,
        "upstream_unsupported": 47,
        "operation_shape_not_applicable": 6,
        "ready_existing_contract": 4,
        "ready_operator_hold": 6,
        "composite_partial_only": 5,
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
