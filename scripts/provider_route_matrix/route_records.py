structured = Counter(row["structured_run"] for row in rows)
if structured != Counter({"No": 9, "Not applicable": 1, "Yes": 31}):
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

permission_by_route = {
    row["route_id"]: row["permission_exchange"]
    for row in rows
}
if permission_by_route.get("claude-agent.acp") != "Yes":
    raise SystemExit("Claude Agent consumer-mediated permission exchange is not realized")

question_by_route = {
    row["route_id"]: row["question_exchange"]
    for row in rows
}
for route in ["pi.rpc", "kimi-code.local-server", "codex.app-server; codex.exec", "opencode.http"]:
    if question_by_route.get(route) != "Yes":
        raise SystemExit(f"typed question exchange is not realized: {route}")

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
    "permission_exchange",
    "question_exchange",
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
