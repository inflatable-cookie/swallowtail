use super::support::{
    INIT_RECORD_SHAPE_SHA256, PROTOCOL, RESPONSE_ONLY, SELECTED_FLAGS, assert_exact_string_set,
    assert_sha256, json, strings,
};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_RESPONSE_ONLY_AXIS;

/// Selected flag and the number of distinct definitions naming it.
const SELECTED_FLAG_DEFINITION_COUNTS: &[(&str, usize)] = &[
    ("--input-format", 1),
    ("--output-format", 1),
    ("--verbose", 2),
    ("--no-session-persistence", 1),
    ("--model", 4),
    ("--effort", 2),
    ("--permission-mode", 2),
    ("--tools", 1),
    ("--setting-sources", 1),
    ("--mcp-config", 2),
    ("--strict-mcp-config", 2),
    ("--max-turns", 1),
    ("--safe-mode", 1),
    ("--disable-slash-commands", 1),
    ("--no-chrome", 1),
    ("--prompt-suggestions", 1),
];

const EXACT_OPTION_FINGERPRINTS: &[(&str, &[&str])] = &[
    (
        "--input-format",
        &["b0ae98420d9eebb3fb9263daf1df82633671f4b9b1ade878717a573a3d655d83"],
    ),
    (
        "--output-format",
        &["b2d535163c0d13c97ebd3fd6786e8b3ceebdd912a249ba9d60e058e1cd23a228"],
    ),
    (
        "--no-session-persistence",
        &["f35d4ae1474b3ca852f1e62972bc6a78b424d81c71f0de4c8d77ff967da6ce93"],
    ),
    (
        "--setting-sources",
        &["78894352ccc74945bd471f0f8470184378ca6a8b43dd0f4a41a976c4ada5829f"],
    ),
    (
        "--max-turns",
        &["eef1a761fca627d8cf5ca9e220749b48091f76d1f46d325c737a722d98da6b66"],
    ),
    (
        "--safe-mode",
        &["3050b3ad84044607c784e63610956d9537f3d1d62a1a466d3ad726584c01c038"],
    ),
    (
        "--disable-slash-commands",
        &["0d3b2791f93287bd8507ba6a7687f9ee4c917b50d55fa020b5d2a5909ee83a2f"],
    ),
    (
        "--no-chrome",
        &["08501c728357cc16389a745faf08b3434e10419bbb1c71abb5eb47afe6fa7c6d"],
    ),
    (
        "--prompt-suggestions",
        &["c16469774837592b7aaaf3849a9c9dd902ee3919f26ed401d3799b74eb4a4a9e"],
    ),
];

#[test]
fn selected_mapped_surface_has_no_delta_across_the_window() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        protocol["mapped_deltas_from_2_1_257"],
        serde_json::json!([])
    );
    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_session"], false);
    assert_eq!(protocol["host_install_changed"], false);
    assert_eq!(protocol["decoder_corpus"], "claude-code-2.1.220");
    assert_exact_string_set(
        &protocol["help_selected_flags_present"],
        &[
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
        ],
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
        strings(&protocol["permission_mode_effective_choices"]),
        [
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "manual",
            "dontAsk",
            "plan",
        ]
    );
    assert_eq!(
        strings(&protocol["permission_mode_wire_values"]),
        [
            "acceptEdits",
            "auto",
            "bypassPermissions",
            "default",
            "dontAsk",
            "plan",
        ]
    );
    assert_eq!(protocol["selected_permission_mode"], "plan");
    assert_eq!(protocol["selected_tools_headless"], "Read,Glob,Grep");
    assert_eq!(protocol["selected_tools_response_only"], "");
    assert_eq!(protocol["selected_setting_sources"], "user,project,local");
    assert_eq!(protocol["selected_mcp_isolation"], r#"{"mcpServers":{}}"#);
    assert_eq!(protocol["include_partial_messages_selected"], false);
}

#[test]
fn option_definition_fingerprints_are_shared_by_every_hop_and_platform() {
    let protocol = json(PROTOCOL);
    let fingerprints = protocol["option_definition_fingerprints"]
        .as_object()
        .expect("fingerprints are an object");
    let mut expected: Vec<&str> = SELECTED_FLAG_DEFINITION_COUNTS
        .iter()
        .map(|(flag, _)| *flag)
        .collect();
    expected.sort_unstable();
    let mut actual: Vec<&str> = fingerprints.keys().map(String::as_str).collect();
    actual.sort_unstable();
    assert_eq!(actual, expected);
    assert_eq!(SELECTED_FLAGS.len(), SELECTED_FLAG_DEFINITION_COUNTS.len());
    for (flag, count) in SELECTED_FLAG_DEFINITION_COUNTS {
        let darwin = strings(&fingerprints[*flag]["darwin-arm64"]);
        let linux = strings(&fingerprints[*flag]["linux-x64"]);
        assert_eq!(darwin.len(), *count, "{flag}");
        assert_eq!(linux, darwin, "{flag}");
        for digest in &darwin {
            assert_sha256(&serde_json::Value::String((*digest).to_owned()), digest);
        }
    }
    for (flag, digests) in EXACT_OPTION_FINGERPRINTS {
        assert_exact_string_set(&fingerprints[*flag]["darwin-arm64"], digests);
        assert_exact_string_set(&fingerprints[*flag]["linux-x64"], digests);
    }
}

#[test]
fn stream_json_construction_anchors_are_stable() {
    let protocol = json(PROTOCOL);
    let anchors = &protocol["stream_json_stream_anchors"];
    assert_sha256(
        &anchors["init_record_shape_sha256"],
        INIT_RECORD_SHAPE_SHA256,
    );
    assert_exact_string_set(
        &anchors["hook_started_normalized_fingerprints"],
        &["38baad191c4435427633063b299daade1a6422285f51a845e3fa50a5fa04336d"],
    );
    assert_exact_string_set(
        &anchors["result_uuid_normalized_fingerprints"],
        &["53f760099f661fd3d237d96cbdad85ac9f679faed5d7b4feffcf74347adbb41f"],
    );
    assert_eq!(
        anchors["stream_event_normalized_fingerprints"]
            .as_array()
            .expect("stream-event fingerprints are an array")
            .len(),
        12
    );
    assert_eq!(
        anchors["thinking_tokens_normalized_fingerprints"]
            .as_array()
            .expect("thinking-token fingerprints are an array")
            .len(),
        3
    );
    assert!(
        anchors["thinking_tokens_note"]
            .as_str()
            .expect("note is text")
            .contains("SessionsV2Client")
    );
}

#[test]
fn unmapped_boundaries_are_explicit() {
    let protocol = json(PROTOCOL);
    let boundaries = &protocol["unmapped_boundaries"];
    assert_exact_string_set(
        &boundaries["watcher_flags"],
        &["--bare", "--settings", "--add-dir", "--include-hook-events"],
    );
    assert_exact_string_set(
        &boundaries["background_session_commands"],
        &["attach", "logs", "stop", "kill", "respawn", "rm"],
    );
    assert_eq!(boundaries["sdk_tools_declarations"], "sdk-tools.d.ts");
    for flag in [
        "--permission-prompts",
        "--system-prompt-snapshot",
        "--json-schema",
        "--max-budget-usd",
        "--restricted",
        "--forward-subagent-text",
    ] {
        assert!(
            strings(&boundaries["new_help_flags"]).contains(&flag),
            "{flag}"
        );
    }
    for setting in [
        "managedMcpServers",
        "maxEffortLevel",
        "gatewayInternalNetworks",
    ] {
        assert!(
            strings(&boundaries["managed_settings"]).contains(&setting),
            "{setting}"
        );
    }
}

#[test]
fn response_only_selected_flags_are_exact() {
    let response_only = json(RESPONSE_ONLY);
    assert_eq!(response_only["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(response_only["version"], "2.1.270");
    assert_exact_string_set(
        &response_only["help_selected_flags_present"],
        &[
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
        ],
    );
    assert_eq!(response_only["selected_tools"], "");
    assert_eq!(response_only["selected_prompt_suggestions"], "false");
    assert_eq!(response_only["include_partial_messages_selected"], false);
    assert_eq!(response_only["selected_mapped_subset_unchanged"], true);
    assert_eq!(response_only["provider_prompt_sent"], false);
    assert_eq!(
        response_only["qualified_decoder_corpus"],
        serde_json::json!(["claude-code-2.1.227", "claude-code-2.1.228"])
    );
}
