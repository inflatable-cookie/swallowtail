use serde_json::Value;
use std::collections::BTreeSet;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.81.2/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.81.2/dist-inventory.json");

const UNMAPPED_0_80_0_KEYS: &[&str] = &[
    "acp_sdk_pin_1_4_0_to_1_5_0_runtime_byte_identical",
    "protocol_version_stays_1",
    "agent_sdk_pin_0_3_274_to_0_3_278",
    "do_not_flatten_to_claude_agent_sdk_family",
    "compaction_interrupt_only_when_compaction_update_presentation",
    "swallowtail_does_not_advertise_session_compaction",
    "handback_frame_unwrap_is_tool_text_cleanup",
    "terminal_output_delta_unselected",
];

const UNMAPPED_0_81_0_KEYS: &[&str] = &[
    "notice_gated_on_client_session_notices",
    "swallowtail_does_not_advertise_session_notices",
    "auto_mode_fallback_transcript_text_unchanged_without_notices",
    "task_stopped_transcript_text_unchanged_without_notices",
    "agent_sdk_pin_0_3_278_to_0_3_280",
    "acp_sdk_stays_1_5_0",
];

const UNMAPPED_0_81_1_KEYS: &[&str] = &[
    "usage_model_in_claude_meta",
    "usage_update_used_size_cost_unchanged",
    "informational_chunk_meta_ignored",
    "managed_policy_env_before_acp_traffic",
    "exit_plan_extra_options_are_allow_always",
    "write_tool_input_aliases_normalized_before_projection",
    "disable_bypass_permissions_mode_is_settings_gate",
    "acp_sdk_stays_1_5_0",
    "agent_sdk_stays_0_3_280",
];

const UNMAPPED_0_81_2_KEYS: &[&str] = &[
    "exit_plan_interruption_diagnostic_internal",
    "resumed_subagent_keeps_live_parent",
    "subagent_update_kinds_unchanged",
    "acp_sdk_stays_1_5_0",
    "agent_sdk_stays_0_3_280",
    "do_not_flatten_to_claude_code_or_sdk_family",
];

const SELECTED_COMPATIBLE_BECAUSE_KEYS: &[&str] = &[
    "explicit_set_config_option_model_then_confirm_model",
    "explicit_set_config_option_effort_then_confirm_reasoning",
    "config_option_ids_and_categories_unchanged",
    "permission_option_kinds_unchanged",
    "usage_invariant_unchanged",
    "stopReason_domain_and_catchall_unchanged",
    "cancel_still_session_cancel",
    "unknown_session_updates_ignored",
    "unknown_underscore_notifications_ignored",
    "acp_sdk_runtime_byte_identical",
    "agent_sdk_pin_unmapped",
    "notice_not_advertised_so_not_emitted_on_selected_route",
    "elicitation_byte_identical",
    "no_new_public_mapped_operation",
];

const FROM_0_79_TO_0_80_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/context-compaction.d.ts",
    "dist/context-compaction.d.ts.map",
    "dist/context-compaction.js",
    "dist/tools.d.ts",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_80_TO_0_81_ADDED: &[&str] = &[
    "dist/session-notices.d.ts",
    "dist/session-notices.d.ts.map",
    "dist/session-notices.js",
];

const FROM_0_80_TO_0_81_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/async-tasks.d.ts",
    "dist/async-tasks.d.ts.map",
    "dist/async-tasks.js",
    "dist/session-mode.d.ts",
    "dist/session-mode.d.ts.map",
    "dist/session-mode.js",
    "package.json",
];

const FROM_0_81_0_TO_0_81_1_ADDED: &[&str] = &[
    "dist/managed-policy.d.ts",
    "dist/managed-policy.d.ts.map",
    "dist/managed-policy.js",
];

const FROM_0_81_0_TO_0_81_1_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/index.js",
    "dist/permissions/options/shared.d.ts",
    "dist/permissions/options/shared.d.ts.map",
    "dist/permissions/options/tools.d.ts.map",
    "dist/permissions/options/tools.js",
    "dist/session-mode.d.ts",
    "dist/session-mode.d.ts.map",
    "dist/session-mode.js",
    "dist/session-notices.d.ts",
    "dist/session-notices.d.ts.map",
    "dist/session-notices.js",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_81_1_TO_0_81_2_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/exit-plan.d.ts",
    "dist/exit-plan.d.ts.map",
    "dist/exit-plan.js",
    "dist/native-subagents.d.ts",
    "dist/native-subagents.d.ts.map",
    "dist/native-subagents.js",
    "package.json",
];

#[test]
fn identity_delta_ledger_is_mutation_sensitive() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.81.2 protocol corpus is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY)
        .expect("Claude Agent 0.81.2 dist inventory is valid JSON");

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
    assert_eq!(protocol["exit_plan_added_options_remain_allow_always"], true);
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
    assert_eq!(
        protocol["usage_update_fields_unchanged"],
        serde_json::json!(["used", "size", "cost"])
    );
    assert_eq!(protocol["usage_model_rides_in_meta_not_a_usage_field"], true);
    assert_eq!(protocol["cancel_still_session_cancel"], true);
    assert_eq!(
        protocol["session_capabilities_required_by_adapter"],
        serde_json::json!(["close", "delete", "resume"])
    );
    assert_eq!(
        protocol["new_session_update_kinds_emitted_on_selected_route"],
        serde_json::json!([])
    );
    assert_eq!(
        protocol["acp_sdk_schema_added_defs"],
        serde_json::json!(["Notice", "NoticeCapabilities", "NoticeSeverity"])
    );
    assert_true_object(&protocol["unmapped_0_80_0"], UNMAPPED_0_80_0_KEYS);
    assert_true_object(&protocol["unmapped_0_81_0"], UNMAPPED_0_81_0_KEYS);
    assert_true_object(&protocol["unmapped_0_81_1"], UNMAPPED_0_81_1_KEYS);
    assert_true_object(&protocol["unmapped_0_81_2"], UNMAPPED_0_81_2_KEYS);
    assert_true_object(
        &protocol["selected_compatible_because"],
        SELECTED_COMPATIBLE_BECAUSE_KEYS,
    );
    assert_eq!(
        protocol["elicitation_js_byte_identical_0_79_0_through_0_81_2"],
        true
    );
    assert_eq!(protocol["index_js_byte_identical_0_79_0_through_0_81_0"], true);
    assert_eq!(protocol["index_js_changes_only_at_0_81_1"], true);
    assert_eq!(
        protocol["acp_agent_js_byte_identical_0_79_0_through_0_81_2"],
        false
    );

    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["0.79.0", "0.80.0", "0.81.0", "0.81.1", "0.81.2"])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.79.0": 126,
            "0.80.0": 126,
            "0.81.0": 129,
            "0.81.1": 132,
            "0.81.2": 132
        })
    );
    assert_exact_string_set(
        &inventory["from_0_79_0_to_0_80_0"]["changed"],
        FROM_0_79_TO_0_80_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_80_0_to_0_81_0"]["added"],
        FROM_0_80_TO_0_81_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_80_0_to_0_81_0"]["changed"],
        FROM_0_80_TO_0_81_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_81_0_to_0_81_1"]["added"],
        FROM_0_81_0_TO_0_81_1_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_81_0_to_0_81_1"]["changed"],
        FROM_0_81_0_TO_0_81_1_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_81_1_to_0_81_2"]["changed"],
        FROM_0_81_1_TO_0_81_2_CHANGED,
    );
    for hop in [
        "from_0_79_0_to_0_80_0",
        "from_0_80_0_to_0_81_0",
        "from_0_81_0_to_0_81_1",
        "from_0_81_1_to_0_81_2",
    ] {
        assert_eq!(inventory[hop]["removed"], serde_json::json!([]));
    }
    assert_eq!(
        inventory["from_0_79_0_to_0_80_0"]["added"],
        serde_json::json!([])
    );
    assert_eq!(
        inventory["from_0_81_1_to_0_81_2"]["added"],
        serde_json::json!([])
    );
    assert_eq!(
        string_set(&inventory["identical_through_0_79_0_0_80_0_0_81_0_0_81_1_0_81_2"]).len(),
        99
    );
    assert_eq!(
        string_set(&inventory["from_0_79_0_to_0_80_0"]["identical"]).len(),
        116
    );
    assert_eq!(
        string_set(&inventory["from_0_80_0_to_0_81_0"]["identical"]).len(),
        116
    );
    assert_eq!(
        string_set(&inventory["from_0_81_0_to_0_81_1"]["identical"]).len(),
        112
    );
    assert_eq!(
        string_set(&inventory["from_0_81_1_to_0_81_2"]["identical"]).len(),
        123
    );
    for path in [
        "dist/elicitation.js",
        "dist/lib.js",
        "dist/settings.js",
        "dist/utils.js",
        "dist/session-config-ids.js",
        "dist/permissions/options.js",
        "dist/permissions/presentation.js",
    ] {
        let hashes = inventory["hashes"][path]
            .as_object()
            .expect("per-version hash object");
        assert_eq!(hashes.len(), 5, "{path}");
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
        inventory["hashes"]["dist/index.js"]["0.81.0"]
    );
    assert_ne!(
        inventory["hashes"]["dist/index.js"]["0.81.0"],
        inventory["hashes"]["dist/index.js"]["0.81.1"]
    );
    assert_eq!(
        inventory["hashes"]["dist/index.js"]["0.81.1"],
        inventory["hashes"]["dist/index.js"]["0.81.2"]
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.79.0"],
        "e9711af5c5dd150718c4a22b1760f913ce8a871b7355d1f0f63aa845d282e37c"
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.81.2"],
        "36a33a984624541ea013c085f5bd1d982294cfe70beb42826752f62bdc538d64"
    );
    assert_eq!(
        inventory["hashes"]["package.json"]["0.79.0"],
        "9f6cefab78ff1a788f155e857509ac57d11564aaab6183c9d976a77b858f2558"
    );
    assert_eq!(
        inventory["named_unmapped_with_reason"]["0.81.0_added_modules"][0]["path"],
        "dist/session-notices.js"
    );
    assert_eq!(
        inventory["named_unmapped_with_reason"]["0.81.1_added_modules"][0]["path"],
        "dist/managed-policy.js"
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
