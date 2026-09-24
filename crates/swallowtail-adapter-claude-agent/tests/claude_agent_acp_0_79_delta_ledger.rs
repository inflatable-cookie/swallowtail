use serde_json::Value;
use std::collections::BTreeSet;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.79.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.79.0/dist-inventory.json");

const UNMAPPED_0_77_0_KEYS: &[&str] = &[
    "agent_config_option_removed",
    "swallowtail_never_mapped_agent_config_option",
    "agent_sdk_pin_0_3_257_to_0_3_270",
    "do_not_flatten_to_claude_agent_sdk_family",
    "allow_dangerously_skip_permissions_host_opt_out",
    "default_to_no_permission_option_order_only",
    "multi_select_other_description_already_mapped_form",
    "tasklist_parser_internal",
    "native_subagent_failed_tool_name_unmapped",
    "air_access_denied_failure_lane_unmapped",
    "acp_sdk_stays_1_4_0",
];

const UNMAPPED_0_78_0_KEYS: &[&str] = &[
    "compaction_update_gated_on_client_session_compaction_capability",
    "swallowtail_does_not_advertise_session_compaction",
    "selected_route_keeps_tool_call_compaction_lifecycle",
    "single_select_other_description_already_mapped_form",
    "custom_answer_annotations_notes_unmapped_after_accept",
    "air_diff_stats_and_file_change_audit_unmapped",
    "structured_patch_diff_helper_extracted",
    "acp_sdk_stays_1_4_0",
    "agent_sdk_stays_0_3_270",
];

const UNMAPPED_0_79_0_KEYS: &[&str] = &[
    "agent_sdk_pin_0_3_270_to_0_3_274",
    "do_not_flatten_to_claude_code_or_sdk_family",
    "shell_permission_title_keeps_command_text",
    "powershell_presented_like_bash",
    "acp_sdk_stays_1_4_0",
];

const SELECTED_COMPATIBLE_BECAUSE_KEYS: &[&str] = &[
    "explicit_set_config_option_model_then_confirm_model",
    "explicit_set_config_option_effort_then_confirm_reasoning",
    "session_new_display_seed_is_not_the_confirmation_path",
    "config_option_ids_and_categories_unchanged",
    "permission_option_kinds_unchanged",
    "usage_invariant_unchanged",
    "stopReason_domain_and_catchall_unchanged",
    "cancel_still_session_cancel",
    "unknown_session_updates_ignored",
    "unknown_underscore_notifications_ignored",
    "acp_sdk_pin_unchanged",
    "agent_sdk_pin_unmapped",
    "compaction_update_not_advertised_so_not_emitted_on_selected_route",
    "elicitation_form_keys_and_other_title_unchanged",
    "already_mapped_elicitation_accepts_new_other_descriptions",
];

const FROM_0_76_TO_0_77_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/clear-context-coordinator.d.ts",
    "dist/clear-context-coordinator.d.ts.map",
    "dist/clear-context-coordinator.js",
    "dist/elicitation.d.ts",
    "dist/elicitation.d.ts.map",
    "dist/elicitation.js",
    "dist/native-subagents.js",
    "dist/permissions/modes.d.ts",
    "dist/permissions/modes.d.ts.map",
    "dist/permissions/modes.js",
    "dist/permissions/options.d.ts.map",
    "dist/permissions/options.js",
    "dist/permissions/options/shared.d.ts",
    "dist/permissions/options/shared.d.ts.map",
    "dist/permissions/presentation.d.ts",
    "dist/permissions/presentation.d.ts.map",
    "dist/permissions/presentation.js",
    "dist/session-config-ids.d.ts",
    "dist/session-config-ids.d.ts.map",
    "dist/session-config-ids.js",
    "dist/session-failure-extension.d.ts",
    "dist/session-failure-extension.d.ts.map",
    "dist/session-failure-extension.js",
    "dist/session-mode.d.ts",
    "dist/session-mode.d.ts.map",
    "dist/session-mode.js",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_77_TO_0_78_ADDED: &[&str] = &[
    "dist/diff.d.ts",
    "dist/diff.d.ts.map",
    "dist/diff.js",
];

const FROM_0_77_TO_0_78_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/air-extension.d.ts",
    "dist/air-extension.d.ts.map",
    "dist/air-extension.js",
    "dist/context-compaction.d.ts",
    "dist/context-compaction.d.ts.map",
    "dist/context-compaction.js",
    "dist/elicitation.d.ts",
    "dist/elicitation.d.ts.map",
    "dist/elicitation.js",
    "dist/file-change-audit.d.ts",
    "dist/file-change-audit.d.ts.map",
    "dist/file-change-audit.js",
    "dist/tools.d.ts",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_78_TO_0_79_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/permissions/presentation.d.ts.map",
    "dist/permissions/presentation.js",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

#[test]
fn identity_delta_ledger_is_mutation_sensitive() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.79.0 protocol corpus is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY)
        .expect("Claude Agent 0.79.0 dist inventory is valid JSON");

    assert_eq!(protocol["mode_option_id_unchanged"], "mode");
    assert_eq!(protocol["mode_option_category_unchanged"], "mode");
    assert_eq!(protocol["plan_and_acceptEdits_still_advertised"], true);
    assert_eq!(protocol["effort_option_id_unchanged"], "effort");
    assert_eq!(
        protocol["effort_option_category_unchanged"],
        "thought_level"
    );
    assert_eq!(
        protocol["permission_kinds_unchanged"],
        serde_json::json!(["allow_once", "allow_always", "reject_once"])
    );
    assert_eq!(
        protocol["prompt_usage_fields_unchanged"],
        serde_json::json!([
            "inputTokens",
            "outputTokens",
            "cachedReadTokens",
            "cachedWriteTokens",
            "totalTokens"
        ])
    );
    assert_eq!(protocol["cancel_still_session_cancel"], true);
    assert_eq!(
        protocol["set_config_option_response_still_config_options"],
        true
    );
    assert_eq!(protocol["acp_sdk_all_hops"], "1.4.0");
    assert_eq!(
        protocol["session_capabilities_required_by_adapter"],
        serde_json::json!(["close", "delete", "resume"])
    );
    assert_eq!(
        protocol["new_session_update_kinds_emitted_without_compaction_capability"],
        serde_json::json!([])
    );
    assert_eq!(
        protocol["legacy_other_description"],
        "Type your own answer instead of choosing an option above (optional)."
    );
    assert_eq!(
        protocol["multi_select_other_description_from_0_77_0"],
        "Type your own answer to add to your selection above (optional)."
    );
    assert_eq!(
        protocol["single_select_other_description_from_0_78_0"],
        "Type your own answer, or add a note to the option you chose above (optional)."
    );
    assert_true_object(&protocol["unmapped_0_77_0"], UNMAPPED_0_77_0_KEYS);
    assert_true_object(&protocol["unmapped_0_78_0"], UNMAPPED_0_78_0_KEYS);
    assert_true_object(&protocol["unmapped_0_79_0"], UNMAPPED_0_79_0_KEYS);
    assert_true_object(
        &protocol["selected_compatible_because"],
        SELECTED_COMPATIBLE_BECAUSE_KEYS,
    );
    assert_eq!(
        protocol["acp_agent_js_byte_identical_0_76_0_through_0_79_0"],
        false
    );
    assert_eq!(
        protocol["elicitation_js_byte_identical_0_76_0_through_0_79_0"],
        false
    );

    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.76.0": 123,
            "0.77.0": 123,
            "0.78.0": 126,
            "0.79.0": 126
        })
    );
    assert_exact_string_set(
        &inventory["from_0_76_0_to_0_77_0"]["changed"],
        FROM_0_76_TO_0_77_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_77_0_to_0_78_0"]["added"],
        FROM_0_77_TO_0_78_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_77_0_to_0_78_0"]["changed"],
        FROM_0_77_TO_0_78_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_78_0_to_0_79_0"]["changed"],
        FROM_0_78_TO_0_79_CHANGED,
    );
    for hop in [
        "from_0_76_0_to_0_77_0",
        "from_0_77_0_to_0_78_0",
        "from_0_78_0_to_0_79_0",
    ] {
        assert_eq!(inventory[hop]["removed"], serde_json::json!([]));
    }
    assert_eq!(inventory["from_0_76_0_to_0_77_0"]["added"], serde_json::json!([]));
    assert_eq!(inventory["from_0_78_0_to_0_79_0"]["added"], serde_json::json!([]));
    assert_eq!(
        string_set(&inventory["identical_through_0_76_0_0_77_0_0_78_0_0_79_0"]).len(),
        81
    );
    assert_eq!(
        string_set(&inventory["from_0_76_0_to_0_77_0"]["identical"]).len(),
        91
    );
    assert_eq!(
        string_set(&inventory["from_0_77_0_to_0_78_0"]["identical"]).len(),
        104
    );
    assert_eq!(
        string_set(&inventory["from_0_78_0_to_0_79_0"]["identical"]).len(),
        119
    );
    for path in ["dist/index.js", "dist/settings.js", "dist/utils.js", "dist/lib.js"] {
        let hashes = inventory["hashes"][path]
            .as_object()
            .expect("per-version hash object");
        assert_eq!(hashes.len(), 4, "{path}");
        assert_eq!(
            hashes
                .values()
                .map(|value| value.as_str().expect("hash string"))
                .collect::<BTreeSet<_>>()
                .len(),
            1,
            "{path} must be byte-identical across every compared hop"
        );
    }
    assert_eq!(
        inventory["hashes"]["dist/index.js"]["0.79.0"],
        "9d73d1f0f121fb96cc8badb28c22d5bff02d8582eb2e40360a81c189e1b9422a"
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.76.0"],
        "c1635f643ceff71c906011b85f33f1b135a4a07aeaa5546d44fb6c5fad11c96d"
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.79.0"],
        "e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c"
    );
    assert_ne!(
        inventory["hashes"]["dist/elicitation.js"]["0.76.0"],
        inventory["hashes"]["dist/elicitation.js"]["0.77.0"]
    );
    assert_ne!(
        inventory["hashes"]["dist/elicitation.js"]["0.77.0"],
        inventory["hashes"]["dist/elicitation.js"]["0.78.0"]
    );
    assert_eq!(
        inventory["hashes"]["dist/elicitation.js"]["0.78.0"],
        inventory["hashes"]["dist/elicitation.js"]["0.79.0"]
    );
    assert_eq!(
        inventory["hashes"]["package.json"]["0.76.0"],
        "ac440f790e4f1cf44a6798249f79296decd668f6c166bf5f6a3892264455a075"
    );
    assert_eq!(
        inventory["named_unmapped_with_reason"]["0.78.0_added_modules"][0]["path"],
        "dist/diff.js"
    );
}

fn assert_true_object(value: &Value, expected: &[&str]) {
    assert_exact_object_keys(value, expected);
    for key in expected {
        assert_eq!(value[key], true, "{key}");
    }
}

fn assert_exact_object_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_string_set(value: &Value, expected: &[&str]) {
    assert_eq!(string_set(value), expected.iter().copied().collect());
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string"))
        .collect()
}
