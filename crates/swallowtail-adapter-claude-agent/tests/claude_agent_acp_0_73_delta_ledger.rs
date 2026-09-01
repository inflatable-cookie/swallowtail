use serde_json::Value;
use std::collections::BTreeSet;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-acp-0.73.0/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/claude-agent-acp-0.73.0/dist-inventory.json");

const UNMAPPED_0_71_0_KEYS: &[&str] = &[
    "session_capabilities_subagents",
    "native_subagent_session_updates",
    "async_task_session_updates",
    "session_titles_already_emitted_at_0_70_idle",
    "github_1004_align_modes_clear_context_planning",
    "github_1045_defer_steering_while_user_input_pending",
    "dontAsk_dropped_from_advertised_available_modes",
    "clear_context_and_exit_plan_unmapped",
    "steering_meta_still_unmapped",
    "permission_callback_observable_contract_unchanged",
];

const UNMAPPED_0_72_0_KEYS: &[&str] = &[
    "effort_removes_session_new_applyFlagSettings",
    "effort_per_model_currentValue",
    "effort_pinned_by_user_gating",
    "user_message_uuid_ensureActiveTurn",
    "queued_cancelled_turn_ownership",
    "stamped_unmatched_empty_interruption_fallback",
    "post_model_switch_mirrors_cli_into_config_option_and_current_mode_updates",
    "pre_model_switch_session_new_veto_logs_and_falls_back_to_models_0",
    "fail_closed_moved_to_post_new_set_config_option_model_confirm",
    "acp_sdk_createElicitation_rename",
];

const UNMAPPED_0_73_0_KEYS: &[&str] = &[
    "dist_byte_identical_to_0_72_0",
    "package_json_version_bump",
    "agent_sdk_pin_0_3_252_to_0_3_257",
    "acp_sdk_stays_1_4_0",
    "github_1066_claude_sdk_dependency_only",
];

const SELECTED_COMPATIBLE_BECAUSE_KEYS: &[&str] = &[
    "explicit_set_config_option_model_then_confirm_model",
    "explicit_set_config_option_effort_then_confirm_reasoning",
    "session_new_display_seed_is_not_the_confirmation_path",
    "usage_invariant_unchanged",
    "stopReason_domain_and_catchall_unchanged",
    "cancel_still_session_cancel",
    "unknown_session_updates_ignored",
    "dist_mapped_surface_byte_identical_0_72_0_to_0_73_0",
];

const FROM_0_70_TO_0_71_CHANGED: &[&str] = &[
    "README.md",
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/file-change-audit.d.ts.map",
    "dist/file-change-audit.js",
    "dist/session-failure-extension.d.ts",
    "dist/session-failure-extension.d.ts.map",
    "dist/session-failure-extension.js",
    "dist/tools.d.ts",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_71_TO_0_72_CHANGED: &[&str] = &[
    "dist/acp-agent.d.ts",
    "dist/acp-agent.d.ts.map",
    "dist/acp-agent.js",
    "dist/tools.d.ts.map",
    "dist/tools.js",
    "package.json",
];

const FROM_0_72_TO_0_73_CHANGED: &[&str] = &["package.json"];

const EMITTED_UPDATES_ADDED_0_71_0: &[&str] = &[
    "async_task_progress",
    "async_task_spawned",
    "async_task_state_update",
    "subagent_spawned",
    "subagent_state_update",
];

#[test]
fn identity_delta_ledger_is_mutation_sensitive() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Claude Agent 0.73.0 protocol corpus is valid JSON");
    let inventory: Value = serde_json::from_str(DIST_INVENTORY)
        .expect("Claude Agent 0.73.0 dist inventory is valid JSON");

    assert_eq!(protocol["mode_option_id_unchanged"], "mode");
    assert_eq!(protocol["plan_and_acceptEdits_still_advertised"], true);
    assert_eq!(protocol["effort_option_id_unchanged"], "effort");
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
    assert_true_object(&protocol["unmapped_0_71_0"], UNMAPPED_0_71_0_KEYS);
    assert_true_object(&protocol["unmapped_0_72_0"], UNMAPPED_0_72_0_KEYS);
    assert_true_object(&protocol["unmapped_0_73_0"], UNMAPPED_0_73_0_KEYS);
    assert_true_object(
        &protocol["selected_compatible_because"],
        SELECTED_COMPATIBLE_BECAUSE_KEYS,
    );
    assert!(
        protocol["unmapped_0_72_0"]
            .get("settings_effort_current_value_seed_only")
            .is_none()
    );
    assert_eq!(
        protocol["changed_failure_point_0_72_0"]["session_new_no_longer_fails_closed_on_pre_model_switch_veto"],
        true
    );
    assert_eq!(
        protocol["changed_failure_point_0_72_0"]["mapped_fail_closed_is_post_new_set_config_option_model_confirm_exact_match"],
        true
    );
    assert_eq!(
        protocol["emitted_updates_added_0_71_0"],
        serde_json::json!(EMITTED_UPDATES_ADDED_0_71_0)
    );
    assert_eq!(
        protocol["emitted_updates_added_0_73_0"],
        serde_json::json!([])
    );
    assert_eq!(protocol["dist_byte_identical_0_72_0_through_0_73_0"], true);

    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert_eq!(inventory["operator_restart_to_0_73_0"], true);
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({"0.70.0": 33, "0.71.0": 96, "0.72.0": 96, "0.73.0": 96})
    );
    assert_eq!(
        string_set(&inventory["from_0_70_0_to_0_71_0"]["added"]).len(),
        63
    );
    assert_exact_string_set(
        &inventory["from_0_70_0_to_0_71_0"]["changed"],
        FROM_0_70_TO_0_71_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_71_0_to_0_72_0"]["changed"],
        FROM_0_71_TO_0_72_CHANGED,
    );
    assert_exact_string_set(
        &inventory["from_0_72_0_to_0_73_0"]["changed"],
        FROM_0_72_TO_0_73_CHANGED,
    );
    for hop in ["from_0_71_0_to_0_72_0", "from_0_72_0_to_0_73_0"] {
        assert_eq!(inventory[hop]["added"], serde_json::json!([]));
        assert_eq!(inventory[hop]["removed"], serde_json::json!([]));
    }
    assert_eq!(
        string_set(&inventory["identical_through_0_70_0_0_71_0_0_72_0_0_73_0"]).len(),
        20
    );
    assert_eq!(
        string_set(&inventory["from_0_71_0_to_0_72_0"]["identical"]).len(),
        90
    );
    assert_eq!(
        string_set(&inventory["from_0_72_0_to_0_73_0"]["identical"]).len(),
        95
    );
    assert_eq!(
        inventory["hashes"]["dist/index.js"]["0.70.0"],
        inventory["hashes"]["dist/index.js"]["0.73.0"]
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.70.0"],
        "f0cbbe408bb758cc4bacdae9a244bcac6efbdb6413f680195d017648abc6d816"
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.72.0"],
        inventory["hashes"]["dist/acp-agent.js"]["0.73.0"]
    );
    assert_eq!(
        inventory["hashes"]["dist/acp-agent.js"]["0.73.0"],
        "e41014b49c5ac096b5e18a89f990ee0ec64452e440666b59dcf4e087f632e370"
    );
    assert_ne!(
        inventory["hashes"]["package.json"]["0.72.0"],
        inventory["hashes"]["package.json"]["0.73.0"]
    );
    assert!(
        inventory["named_unmapped_with_reason"]["0.72.0_changed_mapped_adjacent"]
            .as_array()
            .expect("named 0.72 mapped-adjacent")
            .iter()
            .any(|entry| entry["path"] == "dist/acp-agent.js")
    );
    assert!(
        inventory["named_unmapped_with_reason"]["0.73.0_changed_provider_internal"]
            .as_array()
            .expect("named 0.73 internal")
            .iter()
            .any(|entry| entry["path"] == "package.json")
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
