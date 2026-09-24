expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 17,
        "structured_run": 10,
        "interactive_session": 9,
        "realtime_media_session": 3,
        "usage_evidence": 14,
        "billed_cost_evidence": 19,
        "output_token_limit": 31,
        "reasoning_selection": 21,
        "structured_output": 36,
        "attachments": 35,
        "consumer_tool_exchange": 36,
        "permission_exchange": 37,
        "question_exchange": 35,
        "load_session": 33,
        "resume_session": 33,
        "provider_session_catalogue": 8,
        "provider_session_import": 9,
        "bounded_workspace_text_write": 24,
        "external_search": 39,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 35,
        "provider_session_archive": 14,
        "provider_session_restore": 15,
        "provider_session_delete": 13,
        "native_session_close": 37,
        "owned_remote_resource_cleanup": 9,
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
        "Yes": 345,
        "No": 581,
        "Not applicable": 439,
        "Partial": 2,
        "Caller-supplied": 17,
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
if len(no_cells) != 581 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 581 unique cells")
