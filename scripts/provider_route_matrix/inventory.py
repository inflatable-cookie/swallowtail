expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 5,
        "structured_run": 2,
        "interactive_session": 6,
        "realtime_media_session": 3,
        "usage_evidence": 4,
        "billed_cost_evidence": 18,
        "output_token_limit": 20,
        "reasoning_selection": 15,
        "structured_output": 23,
        "attachments": 23,
        "consumer_tool_exchange": 24,
        "permission_exchange": 25,
        "question_exchange": 22,
        "load_session": 22,
        "resume_session": 22,
        "provider_session_catalogue": 6,
        "provider_session_import": 6,
        "bounded_workspace_text_write": 11,
        "external_search": 26,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 22,
        "provider_session_archive": 8,
        "provider_session_restore": 8,
        "provider_session_delete": 6,
        "native_session_close": 26,
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
        "Yes": 268,
        "No": 365,
        "Not applicable": 314,
        "Partial": 2,
        "Caller-supplied": 5,
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
if len(no_cells) != 365 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 365 unique cells")
