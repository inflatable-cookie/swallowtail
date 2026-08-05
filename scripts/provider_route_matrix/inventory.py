expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 2,
        "structured_run": 2,
        "interactive_session": 5,
        "realtime_media_session": 3,
        "usage_evidence": 3,
        "billed_cost_evidence": 18,
        "output_token_limit": 16,
        "reasoning_selection": 13,
        "structured_output": 19,
        "attachments": 20,
        "consumer_tool_exchange": 20,
        "permission_exchange": 21,
        "question_exchange": 19,
        "load_session": 19,
        "resume_session": 19,
        "provider_session_catalogue": 5,
        "provider_session_import": 5,
        "bounded_workspace_text_write": 7,
        "external_search": 22,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 18,
        "provider_session_archive": 8,
        "provider_session_restore": 8,
        "provider_session_delete": 6,
        "native_session_close": 23,
        "owned_remote_resource_cleanup": 3,
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
        "Yes": 240,
        "No": 313,
        "Not applicable": 265,
        "Partial": 2,
        "Caller-supplied": 2,
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
if len(no_cells) != 313 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 313 unique cells")
