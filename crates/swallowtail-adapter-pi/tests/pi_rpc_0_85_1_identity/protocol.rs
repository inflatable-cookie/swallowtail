use super::support::{IDENTITY, PROTOCOL, json, strings};

#[test]
fn mapped_and_unmapped_ledgers_are_exact() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&protocol["selected_commands"]),
        [
            "prompt",
            "steer",
            "follow_up",
            "abort",
            "get_state",
            "get_available_models",
            "set_auto_compaction",
            "set_auto_retry",
            "set_steering_mode",
            "set_follow_up_mode",
        ]
    );
    assert_eq!(
        strings(&protocol["selected_absences"]),
        [
            "bash",
            "switch_session",
            "fork",
            "clone",
            "extensions",
            "clear_queue",
            "compact",
        ]
    );
    assert_eq!(protocol["mapped_deltas_from_0_84_4"], serde_json::json!([]));
    assert_eq!(protocol["mapped_deltas_from_0_85_0"], serde_json::json!([]));
    assert_eq!(
        protocol["standing_unused_help_unselected_carried_from"],
        "0.84.4"
    );
    assert_eq!(
        protocol["standing_unused_help_unselected_not_new_0_85_delta"],
        true
    );
    assert_eq!(
        strings(&protocol["standing_unused_help_unselected"]),
        ["--use-theme", "defaultTools", "--", "powershell"]
    );
    assert_eq!(strings(&protocol["unmapped_changelog"]).len(), 14);
    assert_eq!(
        protocol["unmapped_changelog_inert_reasons"]
            .as_object()
            .expect("inert reasons are an object")
            .len(),
        4
    );
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["map_clear_queue"], false);
    assert_eq!(decision["map_experimental_server"], false);
    assert_eq!(decision["map_in_memory_session_restore"], false);
    assert_eq!(decision["map_persistent_claude_thinking_effort"], false);
    assert_eq!(decision["map_gpt_6_astra"], false);
    assert_eq!(decision["new_public_mapped_operation"], false);
}

#[test]
fn selected_flags_and_mapped_protocol_stay() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&protocol["help_selected_flags_present"]),
        [
            "--mode",
            "--no-session",
            "--offline",
            "--provider",
            "--model",
            "--tools",
            "--no-extensions",
            "--no-skills",
            "--no-prompt-templates",
            "--no-themes",
            "--no-context-files",
            "--no-tools",
        ]
    );
    assert_eq!(protocol["selected_mode"], "rpc");
    assert_eq!(protocol["rpc_command_count"], 33);
    assert_eq!(
        protocol["rpc_types_js_identical_0_84_4_through_0_85_1"],
        true
    );
    assert_eq!(
        protocol["rpc_mode_js_identical_0_84_4_through_0_85_1"],
        true
    );
    assert_eq!(protocol["jsonl_identical_0_83_0_through_0_85_1"], true);
    assert_eq!(protocol["session_cwd_identical"], true);
    assert_eq!(protocol["json_event_identical_to_0_84_4"], true);
    assert_eq!(protocol["args_identical_0_84_4_and_0_85_1"], true);
    assert_eq!(
        protocol["args_0_85_0_only_adds_unmapped_pi_server_env_help"],
        true
    );
    assert_eq!(
        protocol["abort_also_cancels_compaction_and_branch_summary_from"],
        "0.85.0"
    );
    assert_eq!(
        protocol["abort_wait_for_idle_already_present_at_0_84_4"],
        true
    );
    assert_eq!(protocol["clear_queue_from"], "0.84.4");
    assert_eq!(protocol["toolcall_start_classifies_as_progress"], true);
    assert_eq!(protocol["decoder_corpus"], "pi-rpc-0.80.10");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_rpc_session"], false);
    assert_eq!(protocol["official_binary_executed"], false);
}
