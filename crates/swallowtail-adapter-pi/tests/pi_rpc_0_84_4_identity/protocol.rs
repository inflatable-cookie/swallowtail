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
        ]
    );
    assert_eq!(protocol["mapped_deltas_from_0_84_3"], serde_json::json!([]));
    assert!(protocol.get("unused_help_deltas").is_none());
    assert_eq!(
        protocol["standing_unused_help_unselected_carried_from"],
        "0.84.3"
    );
    assert_eq!(
        protocol["standing_unused_help_unselected_not_new_0_84_4_delta"],
        true
    );
    assert_eq!(
        strings(&protocol["standing_unused_help_unselected"]),
        ["--use-theme", "defaultTools", "--", "powershell"]
    );
    assert_eq!(
        strings(&protocol["unmapped_changelog"]),
        [
            "0.84.4 RPC clear_queue",
            "0.84.4 terminal capability overrides",
            "0.84.4 extension ui_prompt_start and ui_prompt_end",
            "0.84.4 fullscreen selection copy controls",
            "0.84.4 DeepSeek V4 Flash Vision",
            "0.84.4 JSONL resume trailing-newline fix outside mapped jsonl.ts",
            "0.84.4 large tool results crossing auto-compaction threshold / _compactBeforeNextAssistantResponse",
            "0.84.4 extension messages with triggerTurn:false delaying message_start/message_end",
        ]
    );
    assert_eq!(
        protocol["unmapped_changelog_inert_reasons"]["0.84.4 large tool results crossing auto-compaction threshold / _compactBeforeNextAssistantResponse"],
        "Swallowtail sends set_auto_compaction false, so shouldCompact is false despite mapped auto_compaction events"
    );
    assert_eq!(
        protocol["unmapped_changelog_inert_reasons"]["0.84.4 extension messages with triggerTurn:false delaying message_start/message_end"],
        "both argv shapes use --no-extensions; non-assistant message_end decodes as Progress"
    );
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["map_clear_queue"], false);
    assert_eq!(decision["map_large_tool_result_auto_compaction"], false);
    assert_eq!(decision["map_extension_trigger_turn_false"], false);
    assert_eq!(decision["map_streaming_usage"], false);
    assert_eq!(decision["map_toolcall_start_id_and_tool_name"], false);
    assert_eq!(decision["map_terminal_capability_overrides"], false);
    assert_eq!(decision["map_extension_ui_prompt_events"], false);
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
    assert_eq!(
        protocol["rpc_types_source_changed_only_by_additive_clear_queue"],
        true
    );
    assert_eq!(
        protocol["rpc_mode_source_changed_only_by_additive_clear_queue"],
        true
    );
    assert_eq!(protocol["jsonl_identical_0_83_0_through_0_84_4"], true);
    assert_eq!(protocol["session_cwd_identical"], true);
    assert_eq!(protocol["json_event_identical_to_0_84_3"], true);
    assert_eq!(protocol["args_identical_to_0_84_3"], true);
    assert_eq!(
        protocol["message_update_drops_cumulative_snapshot_from"],
        "0.84.0"
    );
    assert_eq!(protocol["clear_queue_from"], "0.84.4");
    assert_eq!(protocol["toolcall_start_classifies_as_progress"], true);
    assert_eq!(protocol["decoder_corpus"], "pi-rpc-0.80.10");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_rpc_session"], false);
    assert_eq!(protocol["official_binary_executed"], false);
}
