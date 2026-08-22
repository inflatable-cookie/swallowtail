if len(residual_feature_classifications) != 114:
    raise SystemExit("residual feature starting inventory must contain exactly 114 cells")
if Counter(residual_feature_classifications.values()) != Counter(
    {
        "interface_axis_not_runtime_ordered": 10,
        "exact_release_only": 14,
        "contract_or_corpus_required": 9,
        "operation_shape_not_applicable": 39,
        "separate_route_and_contract_required": 3,
        "no_provider_billing_boundary": 16,
        "selected_surface_absence": 19,
        "non_authoritative_cost_evidence": 4,
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
) != Counter({"Not applicable": 65, "No": 47, "Yes": 2}):
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
if len(provider_retention_classifications) != 145:
    raise SystemExit("provider-retention starting inventory must contain 145 cells")
if Counter(provider_retention_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 93,
        "upstream_unsupported": 44,
        "separate_transport_and_corpus_required": 1,
        "selected_surface_absence": 3,
        "realized_matrix_false_negative": 1,
        "ready_existing_contract": 1,
        "shared_contract_and_corpus_required": 2,
    }
):
    raise SystemExit("provider-retention classification counts changed")
provider_retention_expected_values = {
    "operation_shape_not_applicable": "Not applicable",
    "upstream_unsupported": "No",
    "separate_transport_and_corpus_required": "Yes",
    "selected_surface_absence": "No",
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
    {"Not applicable": 93, "No": 47, "Yes": 5}
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
if len(retained_execution_classifications) != 109:
    raise SystemExit("retained-execution starting inventory must contain exactly 109 cells")
if Counter(retained_execution_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 64,
        "upstream_unsupported": 31,
        "separate_route_and_contract_required": 2,
        "shared_contract_and_corpus_required": 4,
        "selected_surface_absence": 8,
    }
):
    raise SystemExit("retained-execution classification counts changed")
retained_execution_realized = {
    ("provider_managed_recovery", "kimi-code.acp + kimi-code.headless"): "Partial",
    ("stream_reattachment", "kimi-code.local-server"): "Yes",
    ("provider_managed_recovery", "kimi-code.local-server"): "Yes",
    ("provider_managed_recovery", "openai.background"): "Yes",
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
    {"Not applicable": 64, "No": 41, "Yes": 3, "Partial": 1}
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
if len(working_resource_write_classifications) != 47:
    raise SystemExit("working-resource/write starting inventory must contain exactly 47 cells")
if Counter(working_resource_write_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 24,
        "upstream_unsupported": 8,
        "contract_or_corpus_required": 1,
        "selected_surface_absence": 14,
    }
):
    raise SystemExit("working-resource/write classification counts changed")
working_resource_write_expected = {
    "operation_shape_not_applicable": "Not applicable",
    "upstream_unsupported": "No",
    "contract_or_corpus_required": "Yes",
    "selected_surface_absence": "No",
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
) != Counter({"Not applicable": 24, "No": 22, "Yes": 1}):
    raise SystemExit("working-resource/write final counts changed")

runtime_rollover_values = {
    (feature, row["route_id"]): row[feature]
    for row in rows
    for feature in [
        "owned_runtime_lifecycle",
        "planned_connection_rollover",
    ]
}
if len(runtime_rollover_classifications) != 73:
    raise SystemExit("runtime-ownership/rollover inventory must contain exactly 73 cells")
if Counter(runtime_rollover_classifications.values()) != Counter(
    {
        "operation_shape_not_applicable": 72,
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
) != Counter({"Not applicable": 72, "No": 1}):
    raise SystemExit("runtime-ownership/rollover final counts changed")

generation_control_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in ["output_token_limit", "reasoning_selection", "structured_output"]
    if row[feature] == "No"
}
if len(generation_control_no_cells) != 89:
    raise SystemExit("generation-control inventory must contain exactly 89 No cells")
if generation_control_no_cells != set(generation_control_classifications):
    raise SystemExit("generation-control No classifications changed")

input_callback_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in [
        "attachments",
        "consumer_tool_exchange",
        "permission_exchange",
        "question_exchange",
        "external_search",
    ]
    if row[feature] == "No"
}
if len(input_callback_no_cells) != 174:
    raise SystemExit("input/callback inventory must contain exactly 174 No cells")
if input_callback_no_cells != set(input_callback_classifications):
    raise SystemExit("input/callback No classifications changed")

session_continuity_no_cells = {
    (feature, row["route_id"])
    for row in rows
    for feature in ["load_session", "resume_session", "native_session_close"]
    if row[feature] == "No"
}
if len(session_continuity_no_cells) != 98:
    raise SystemExit("session-continuity inventory must contain exactly 98 No cells")
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
        "contract_or_corpus_required": 81,
        "upstream_unsupported": 178,
        "operation_shape_not_applicable": 64,
        "ready_existing_contract": 4,
        "ready_operator_hold": 6,
        "composite_partial_only": 6,
        "shared_contract_expansion_required": 2,
        "upstream_ordering_blocked": 1,
        "separate_route_and_contract_required": 5,
        "selected_surface_absence": 179,
        "non_authoritative_cost_evidence": 4,
        "exact_release_only": 14,
        "missing_shared_contract_or_currentness_evidence": 10,
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
if len(route_ids) != 48 or len(set(route_ids)) != 48:
    raise SystemExit("provider solution matrix must cover 48 unique route identities")
