use super::support::{IDENTITY, PROTOCOL, RESPONSE_ONLY, json, strings};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_RESPONSE_ONLY_AXIS;

#[test]
fn mapped_and_unmapped_ledgers_are_exact() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        protocol["mapped_deltas_from_2_1_252"],
        serde_json::json!([])
    );
    assert_eq!(
        strings(&protocol["help_deltas_from_2_1_252"]),
        ["--bg resume wording", "--system-prompt-snapshot"]
    );
    assert_eq!(
        strings(&protocol["unused_help_deltas"]),
        [
            "--bare",
            "--brief",
            "--cloud",
            "--include-hook-events",
            "--include-partial-messages",
            "--forward-subagent-text",
            "--json-schema",
            "--max-budget-usd",
            "--restricted",
            "--all",
            "--system-prompt-snapshot",
        ]
    );
    assert_eq!(
        strings(&protocol["unused_command_deltas"]),
        ["attach", "logs", "stop", "kill", "respawn", "rm"]
    );
    assert_eq!(
        strings(&protocol["unmapped_changelog"]),
        [
            "2.1.257 Claude Fable 5.1 default Fable model alias",
            "2.1.257 --effort lifts a new-model default-effort hold for that session only",
            "2.1.257 claude -p waits if a Monitor the model armed is still running",
            "2.1.257 defaultMode bypassPermissions in project settings is ignored",
            "2.1.257 --disallowedTools dropped after settings reload under allowManagedPermissionRulesOnly",
            "2.1.257 unbounded memory when non-JSONL is piped to -p --input-format stream-json",
            "2.1.257 remaining interactive, VSCode, cloud, Remote Control, MCP, sandbox, telemetry, subagent, and background-session extras",
        ]
    );
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["map_restricted"], false);
    assert_eq!(decision["map_watcher_flags"], false);
    assert_eq!(decision["map_background_session_commands"], false);
    assert_eq!(decision["map_system_prompt_snapshot"], false);
    assert_eq!(decision["widen_maximum_turns"], false);
}

#[test]
fn selected_headless_flags_and_domains_are_exact() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        strings(&protocol["help_selected_flags_present"]),
        [
            "-p",
            "--print",
            "--input-format",
            "--output-format",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "--effort",
            "--permission-mode",
            "--tools",
            "--setting-sources",
            "--mcp-config",
            "--strict-mcp-config",
        ]
    );
    assert_eq!(
        strings(&protocol["input_format_choices"]),
        ["text", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["output_format_choices"]),
        ["text", "json", "stream-json"]
    );
    assert_eq!(
        strings(&protocol["effort_choices"]),
        ["low", "medium", "high", "xhigh", "max"]
    );
    assert_eq!(
        strings(&protocol["permission_mode_choices"]),
        [
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "manual",
            "dontAsk",
            "plan",
        ]
    );
    assert_eq!(protocol["selected_permission_mode"], "plan");
    assert_eq!(protocol["selected_tools"], "Read,Glob,Grep");
    assert_eq!(protocol["selected_setting_sources"], "user,project,local");
    assert_eq!(protocol["include_partial_messages_selected"], false);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
}

#[test]
fn response_only_selected_flags_are_exact() {
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(response_only["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(response_only["version"], "2.1.257");
    assert_eq!(
        strings(&response_only["help_selected_flags_present"]),
        [
            "-p",
            "--input-format",
            "--output-format",
            "--verbose",
            "--no-session-persistence",
            "--model",
            "--effort",
            "--tools",
            "--safe-mode",
            "--disable-slash-commands",
            "--no-chrome",
            "--prompt-suggestions",
            "--mcp-config",
            "--strict-mcp-config",
        ]
    );
    assert_eq!(response_only["selected_tools"], "");
    assert_eq!(response_only["selected_prompt_suggestions"], "false");
    assert_eq!(response_only["include_partial_messages_selected"], false);
    assert_eq!(
        response_only["qualified_decoder_corpus"],
        serde_json::json!(["claude-code-2.1.227", "claude-code-2.1.228"])
    );
}
