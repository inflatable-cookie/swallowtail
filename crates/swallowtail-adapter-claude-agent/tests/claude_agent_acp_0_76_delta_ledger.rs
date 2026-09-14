use serde_json::Value;
use std::collections::BTreeSet;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.76.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.76.0/dist-inventory.json");

const UNMAPPED_0_74_0_KEYS: &[&str] = &[
    "hide_claude_auth_argv_guard_only",
    "hide_claude_auth_flag_never_passed_by_swallowtail",
    "session_failure_extension_payload_unmapped",
    "provider_update_error_containment_internal",
    "allowlist_and_model_info_plumbing_internal",
    "session_new_effort_seed_not_applied_to_sdk",
    "resumed_context_window_scheduling_internal",
    "package_json_version_bump_only",
    "acp_sdk_stays_1_4_0",
    "agent_sdk_stays_0_3_257",
];

const UNMAPPED_0_75_0_KEYS: &[&str] = &[
    "auth_status_extension_push_only",
    "auth_status_capability_marker_additive",
    "auth_status_notifications_underscore_prefixed_and_ignored",
    "context_compaction_as_mapped_tool_call_lifecycle",
    "context_compaction_meta_key_unmapped",
    "usage_markdown_display_only",
    "lib_reexports_auth_status_types_only",
    "package_json_version_bump_only",
    "acp_sdk_stays_1_4_0",
    "agent_sdk_stays_0_3_257",
];

const UNMAPPED_0_75_1_KEYS: &[&str] = &[
    "resumed_session_transcript_read_replaces_get_context_usage",
    "session_load_seed_only_not_wire_shape",
    "compaction_usage_update_used_from_boundary_post_tokens",
    "usage_update_shape_unchanged",
    "fork_meta_fidelity_unmapped",
    "package_json_version_bump_only",
    "acp_sdk_stays_1_4_0",
    "agent_sdk_stays_0_3_257",
];

const UNMAPPED_0_76_0_KEYS: &[&str] = &[
    "recommended_config_values_gated_on_client_capability",
    "swallowtail_does_not_advertise_recommended_value_capability",
    "legacy_config_option_shape_preserved_without_capability",
    "boolean_config_option_capability_not_advertised",
    "clear_context_coordinator_unmapped",
    "air_recommended_value_constant_additive",
    "dev_dependency_vitest_bump_internal",
    "acp_sdk_stays_1_4_0",
    "agent_sdk_stays_0_3_257",
];

const SELECTED_COMPATIBLE_BECAUSE_KEYS: &[&str] = &[
    "explicit_set_config_option_model_then_confirm_model",
    "explicit_set_config_option_effort_then_confirm_reasoning",
    "session_new_display_seed_is_not_the_confirmation_path",
    "config_option_ids_and_categories_byte_identical",
    "permission_option_kinds_byte_identical",
    "usage_invariant_unchanged",
    "stopReason_domain_and_catchall_unchanged",
    "cancel_still_session_cancel",
    "unknown_session_updates_ignored",
    "unknown_underscore_notifications_ignored",
    "acp_sdk_pin_unchanged",
    "agent_sdk_pin_unchanged",
    "compaction_tool_call_uses_mapped_kind_and_status",
];

const FROM_0_73_TO_0_74_ADDED: &[&str] = &[
    "dist/hide-claude-auth.d.ts",
    "dist/hide-claude-auth.d.ts.map",
    "dist/hide-claude-auth.js",
];

const FROM_0_73_TO_0_74_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/session-failure-extension.d.ts",
    "dist/session-failure-extension.d.ts.map",
    "dist/session-failure-extension.js",
    "package.json",
];

const FROM_0_74_TO_0_75_ADDED: &[&str] = &[
    "dist/auth-status.d.ts",
    "dist/auth-status.d.ts.map",
    "dist/auth-status.js",
    "dist/context-compaction-meta.d.ts",
    "dist/context-compaction-meta.d.ts.map",
    "dist/context-compaction-meta.js",
    "dist/context-compaction.d.ts",
    "dist/context-compaction.d.ts.map",
    "dist/context-compaction.js",
    "dist/usage-markdown.d.ts",
    "dist/usage-markdown.d.ts.map",
    "dist/usage-markdown.js",
];

const FROM_0_74_TO_0_75_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/hide-claude-auth.d.ts",
    "dist/hide-claude-auth.d.ts.map",
    "dist/hide-claude-auth.js",
    "dist/lib.d.ts",
    "dist/lib.d.ts.map",
    "dist/lib.js",
    "package.json",
];

const FROM_0_75_TO_0_75_1_ADDED: &[&str] = &[
    "dist/resumed-session.d.ts",
    "dist/resumed-session.d.ts.map",
    "dist/resumed-session.js",
    "dist/session-timing.d.ts",
    "dist/session-timing.d.ts.map",
    "dist/session-timing.js",
];

const FROM_0_75_TO_0_75_1_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/fork-session.d.ts",
    "dist/fork-session.d.ts.map",
    "dist/fork-session.js",
    "package.json",
];

const FROM_0_75_1_TO_0_76_0_ADDED: &[&str] = &[
    "dist/session-effort.d.ts",
    "dist/session-effort.d.ts.map",
    "dist/session-effort.js",
    "dist/session-model.d.ts",
    "dist/session-model.d.ts.map",
    "dist/session-model.js",
];

const FROM_0_75_1_TO_0_76_0_CHANGED: &[&str] = &[
    "README.md",
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/air-extension.d.ts",
    "dist/air-extension.d.ts.map",
    "dist/air-extension.js",
    "dist/clear-context-coordinator.d.ts",
    "dist/clear-context-coordinator.d.ts.map",
    "dist/clear-context-coordinator.js",
    "package.json",
];

#[test]
fn identity_delta_ledger_is_mutation_sensitive() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.76.0 protocol corpus is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY)
        .expect("Claude Agent 0.76.0 dist inventory is valid JSON");

    assert_eq!(protocol["mode_option_id_unchanged"], "mode");
    assert_eq!(protocol["mode_option_category_unchanged"], "mode");
    assert_eq!(protocol["plan_and_acceptEdits_still_advertised"], true);
    assert_eq!(protocol["effort_option_id_unchanged"], "effort");
    assert_eq!(
        protocol["effort_option_category_unchanged"],
        "thought_level"
    );
    assert_eq!(
        protocol["effort_default_option_still_present_without_recommended_value_capability"],
        true
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
    assert_eq!(
        protocol["prompt_usage_total_equals_sum_invariant_unchanged"],
        true
    );
    assert_eq!(protocol["cancel_still_session_cancel"], true);
    assert_eq!(
        protocol["set_config_option_response_still_config_options"],
        true
    );
    assert_eq!(protocol["acp_sdk_all_hops"], "1.4.0");
    assert_eq!(protocol["agent_sdk_all_hops"], "0.3.257");
    assert_eq!(protocol["load_session_advertised_true"], true);
    assert_eq!(
        protocol["session_capabilities_required_by_adapter"],
        serde_json::json!(["close", "delete", "resume"])
    );
    assert_eq!(
        protocol["new_session_update_kinds_0_73_0_through_0_76_0"],
        serde_json::json!([])
    );
    assert_true_object(&protocol["unmapped_0_74_0"], UNMAPPED_0_74_0_KEYS);
    assert_true_object(&protocol["unmapped_0_75_0"], UNMAPPED_0_75_0_KEYS);
    assert_true_object(&protocol["unmapped_0_75_1"], UNMAPPED_0_75_1_KEYS);
    assert_true_object(&protocol["unmapped_0_76_0"], UNMAPPED_0_76_0_KEYS);
    assert_true_object(
        &protocol["selected_compatible_because"],
        SELECTED_COMPATIBLE_BECAUSE_KEYS,
    );
    assert_eq!(
        protocol["changed_failure_point_0_72_0"]["mapped_fail_closed_is_post_new_set_config_option_model_confirm_exact_match"],
        true
    );
    assert_eq!(
        protocol["acp_agent_js_byte_identical_0_73_0_through_0_76_0"],
        false
    );
    assert_eq!(protocol["lib_js_not_byte_identical_to_0_73_0"], true);

    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.73.0": 96,
            "0.74.0": 99,
            "0.75.0": 111,
            "0.75.1": 117,
            "0.76.0": 123
        })
    );
    assert_exact_string_set(
        &inventory["from_0_73_0_to_0_74_0"]["added"],
        FROM_0_73_TO_0_74_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_73_0_to_0_74_0"]["changed"],
        FROM_0_73_TO_0_74_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_74_0_to_0_75_0"]["added"],
        FROM_0_74_TO_0_75_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_74_0_to_0_75_0"]["changed"],
        FROM_0_74_TO_0_75_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_75_0_to_0_75_1"]["added"],
        FROM_0_75_TO_0_75_1_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_75_0_to_0_75_1"]["changed"],
        FROM_0_75_TO_0_75_1_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_75_1_to_0_76_0"]["added"],
        FROM_0_75_1_TO_0_76_0_ADDED,
    );
    assert_exact_string_set(
        &inventory["from_0_75_1_to_0_76_0"]["changed"],
        FROM_0_75_1_TO_0_76_0_CHANGED,
    );
    for hop in [
        "from_0_73_0_to_0_74_0",
        "from_0_74_0_to_0_75_0",
        "from_0_75_0_to_0_75_1",
        "from_0_75_1_to_0_76_0",
    ] {
        assert_eq!(inventory[hop]["removed"], serde_json::json!([]));
    }
    assert_eq!(
        string_set(&inventory["identical_through_0_73_0_0_74_0_0_75_0_0_75_1_0_76_0"]).len(),
        76
    );
    assert_eq!(
        string_set(&inventory["from_0_73_0_to_0_74_0"]["identical"]).len(),
        89
    );
    assert_eq!(
        string_set(&inventory["from_0_74_0_to_0_75_0"]["identical"]).len(),
        89
    );
    assert_eq!(
        string_set(&inventory["from_0_75_0_to_0_75_1"]["identical"]).len(),
        104
    );
    assert_eq!(
        string_set(&inventory["from_0_75_1_to_0_76_0"]["identical"]).len(),
        106
    );
    for path in [
        "dist/index.js",
        "dist/elicitation.js",
        "dist/settings.js",
        "dist/utils.js",
        "dist/tools.js",
        "dist/session-mode.js",
        "dist/session-config-ids.js",
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
        inventory["hashes"]["dist/acp-agent.js"]["0.73.0"],
        "e41014b49c5ac096b5e18a89f990ee0ec64452e440666b59dcf4e087f632e370"
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.76.0"],
        "c1635f643ceff71c906011b85f33f1b135a4a07aeaa5546d44fb6c5fad11c96d"
    );
    assert_eq!(
        inventory["hashes"]["dist/lib.js"]["0.73.0"],
        inventory["hashes"]["dist/lib.js"]["0.74.0"]
    );
    assert_ne!(
        inventory["hashes"]["dist/lib.js"]["0.74.0"],
        inventory["hashes"]["dist/lib.js"]["0.75.0"]
    );
    assert_ne!(
        inventory["hashes"]["package.json"]["0.75.1"],
        inventory["hashes"]["package.json"]["0.76.0"]
    );
    assert!(
        inventory["named_unmapped_with_reason"]["0.76.0_added_modules"]
            .as_array()
            .expect("named 0.76 added modules")
            .iter()
            .any(|entry| entry["path"] == "dist/session-effort.js")
    );
    assert!(
        inventory["named_unmapped_with_reason"]["0.75.0_added_modules"]
            .as_array()
            .expect("named 0.75 added modules")
            .iter()
            .any(|entry| entry["path"] == "dist/context-compaction.js")
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
