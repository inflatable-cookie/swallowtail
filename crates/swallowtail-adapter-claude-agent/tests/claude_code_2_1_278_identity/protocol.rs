use super::support::{
    PROTOCOL, RESPONSE_ONLY, SELECTED_FLAGS, assert_exact_string_set, assert_sha256, json, strings,
};
use swallowtail_adapter_claude_agent::CLAUDE_CODE_RESPONSE_ONLY_AXIS;

/// Selected flag and the number of distinct stable definitions naming it.
const SELECTED_FLAG_DEFINITION_COUNTS: &[(&str, usize)] = &[
    ("--input-format", 1),
    ("--output-format", 1),
    ("--verbose", 2),
    ("--no-session-persistence", 1),
    ("--model", 3),
    ("--effort", 1),
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
        &["f26dc0d788d94feac33d8f30060bb5416cdfdfa4a3ec4d78b316dfdc8fd5528f"],
    ),
    (
        "--output-format",
        &["b32bbd9c831797e25c5eabdcafeab99d174f9d448b97ea560976360cf452acb0"],
    ),
    (
        "--verbose",
        &[
            "d961c9b1881c1e610334f7d926ccbef8c618e84d5af4f70830120267a61bc8f8",
            "e0788c8eada3f4becdc418b14fe582f463a4212516f6da51efd65e0cfa2947ac",
        ],
    ),
    (
        "--no-session-persistence",
        &["029d3ea9379f2788423da63a83f8542b730164fa0e1bc5755c16f283ed7902f4"],
    ),
    (
        "--model",
        &[
            "2849261318acb904e312534629c3c64e65467a865a518f035844a1e559b03773",
            "35f2e0c6b94c1f4a55af58e3967b9ec57b603e3d9e08f48df21d76d7d6983221",
            "cd14efd58fa6be8bee20260026b473be0c37d8c41fc295f62973b45504d3b54e",
        ],
    ),
    (
        "--effort",
        &["5f50151d4cf956937ce5bee5ad8f3124d76df85c0a9a4597494dbb6fcafc2ee4"],
    ),
    (
        "--permission-mode",
        &[
            "28c6f427ad24bbee51f4d7a18852c187c1c4f88143d99023ff291466246c4704",
            "d060015ed466b05d185c46339f3a9d063d9bf15f62ef7e0075cb7e3ea3086bff",
        ],
    ),
    (
        "--tools",
        &["ad526a29bd4dd9efcf62903ae7b3a4e1d1017053ad8ae262b9eae91e78482d33"],
    ),
    (
        "--setting-sources",
        &["c247ed4cc5764ecd03a1bea80e09c5e6dfcf033b6dca357ad5dec827e89415a7"],
    ),
    (
        "--mcp-config",
        &[
            "40105f32ecaf5f3802f1f0095ab324fe7b758d0d5abe9d14badc2bf09c1a84e7",
            "765ac557df02c8e462e51419e9d2851a53830383092aa53a00348d7bb4d34e0f",
        ],
    ),
    (
        "--strict-mcp-config",
        &[
            "01d8e598f542a35a2af4432301458cd8fedb9d5097dcde8294c2d2f3011d295a",
            "1ab3b8f8c89250453defe9f7efb80a668f1fad3acf03286e1a9ad10c391458f4",
        ],
    ),
    (
        "--max-turns",
        &["bc1c0fd8faa0d58ecbce88a14a99ec312bf5822f158aee093dd687ad29b36948"],
    ),
    (
        "--safe-mode",
        &["7ed106d819194854909416d883056dc33d73965069ce4047066e1e88d8167f38"],
    ),
    (
        "--disable-slash-commands",
        &["9bc209af39c0eec0a9bd9c17feb47606e79489ed2cb7e40f2024b9cbed99cb8d"],
    ),
    (
        "--no-chrome",
        &["a5e9f07fb2ea8845dd7d02214c1213bd16ba6a71d58e71909dbe53d564193861"],
    ),
    (
        "--prompt-suggestions",
        &["0d5cc328554c2e855d19ab3628e11bc2ecefe9b4d93348a0818aa3f4d50f5b77"],
    ),
];

#[test]
fn selected_mapped_surface_has_no_delta_across_the_window() {
    let protocol = json(PROTOCOL);
    assert_eq!(
        protocol["mapped_deltas_from_2_1_270"],
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
fn option_definition_fingerprints_are_the_stable_intersection() {
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
    assert_exact_string_set(
        &protocol["remangling_flags"],
        &["--verbose", "--model", "--effort"],
    );
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
fn stream_json_construction_anchors_stay() {
    let protocol = json(PROTOCOL);
    let anchors = &protocol["stream_json_stream_anchors"];
    assert_exact_string_set(
        &anchors["init_required_keys"],
        &[
            "type",
            "subtype",
            "cwd",
            "session_id",
            "tools",
            "mcp_servers",
            "model",
            "permissionMode",
            "slash_commands",
            "claude_code_version",
        ],
    );
    assert_eq!(anchors["init_optional_mcp_servers_source_from"], "2.1.274");
    assert_eq!(anchors["has_stream_event_on_every_hop"], true);
    assert_eq!(anchors["has_hook_started_on_every_hop"], true);
    assert_eq!(anchors["has_result_on_every_hop"], true);
    assert_eq!(anchors["thinking_sites_present_on_every_hop"], true);
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
    assert_eq!(response_only["version"], "2.1.278");
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
