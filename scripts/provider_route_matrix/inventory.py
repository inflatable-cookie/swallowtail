expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 2,
        "structured_run": 2,
        "interactive_session": 5,
        "realtime_media_session": 3,
        "usage_evidence": 3,
        "billed_cost_evidence": 16,
        "output_token_limit": 14,
        "reasoning_selection": 12,
        "structured_output": 18,
        "attachments": 18,
        "consumer_tool_exchange": 18,
        "permission_exchange": 19,
        "question_exchange": 17,
        "load_session": 18,
        "resume_session": 17,
        "bounded_workspace_text_write": 7,
        "external_search": 20,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 17,
        "provider_session_archive": 5,
        "provider_session_restore": 5,
        "provider_session_delete": 3,
        "native_session_close": 21,
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
        "Yes": 212,
        "No": 272,
        "Not applicable": 222,
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
if len(no_cells) != 272 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 272 unique cells")
