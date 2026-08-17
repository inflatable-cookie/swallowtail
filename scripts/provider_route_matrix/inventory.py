expected_no_counts = Counter(
    {
        "unverified_newer_allowed": 7,
        "structured_run": 2,
        "interactive_session": 6,
        "realtime_media_session": 3,
        "usage_evidence": 4,
        "billed_cost_evidence": 18,
        "output_token_limit": 22,
        "reasoning_selection": 17,
        "structured_output": 25,
        "attachments": 25,
        "consumer_tool_exchange": 26,
        "permission_exchange": 27,
        "question_exchange": 24,
        "load_session": 23,
        "resume_session": 23,
        "provider_session_catalogue": 6,
        "provider_session_import": 7,
        "bounded_workspace_text_write": 13,
        "external_search": 28,
        "retained_background_execution": 5,
        "stream_reattachment": 3,
        "provider_managed_recovery": 24,
        "provider_session_archive": 8,
        "provider_session_restore": 9,
        "provider_session_delete": 7,
        "native_session_close": 27,
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
        "Yes": 282,
        "No": 393,
        "Not applicable": 336,
        "Partial": 2,
        "Caller-supplied": 7,
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
if len(no_cells) != 393 or len(no_cells) != len(set(no_cells)):
    raise SystemExit("provider solution No inventory must contain 393 unique cells")
