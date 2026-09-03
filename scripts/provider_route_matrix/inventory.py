expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 17,
        "structured_run": 9,
        "interactive_session": 9,
        "realtime_media_session": 3,
        "usage_evidence": 13,
        "billed_cost_evidence": 19,
        "output_token_limit": 30,
        "reasoning_selection": 20,
        "structured_output": 35,
        "attachments": 34,
        "consumer_tool_exchange": 36,
        "permission_exchange": 36,
        "question_exchange": 34,
        "load_session": 32,
        "resume_session": 32,
        "provider_session_catalogue": 8,
        "provider_session_import": 9,
        "bounded_workspace_text_write": 23,
        "external_search": 38,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 34,
        "provider_session_archive": 13,
        "provider_session_restore": 14,
        "provider_session_delete": 12,
        "native_session_close": 36,
        "owned_remote_resource_cleanup": 8,
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
        "Yes": 338,
        "No": 563,
        "Not applicable": 431,
        "Partial": 2,
        "Caller-supplied": 16,
        "Session-negotiated": 3,
    }
):
    raise SystemExit(
        f"provider solution disposition counts changed: {dict(audited_value_counts)}"
    )
if actual_no_counts != expected_no_counts:
    raise SystemExit(
        f"provider solution No inventory changed: {dict(actual_no_counts)}"
    )
if len(no_cells) != 563 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 563 unique cells")
