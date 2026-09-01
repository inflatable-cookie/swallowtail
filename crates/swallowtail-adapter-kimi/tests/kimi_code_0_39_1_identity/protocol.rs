use super::support::{PROTOCOL, ROUTING_PROTOCOL, assert_sha1, json, strings};

#[test]
fn the_executing_path_names_the_agent_core_v2_implementations_swallowtail_reaches() {
    let executing = &json(PROTOCOL)["executing_path"];
    assert_eq!(executing["npm_bin"], "dist/main.mjs");
    assert_eq!(executing["github_archive_member"], "kimi");
    assert_eq!(
        executing["archive_format"],
        "node-single-executable-application"
    );
    assert_eq!(strings(&executing["acp_argv"]), ["acp"]);
    assert_eq!(
        strings(&executing["headless_argv"]),
        [
            "--model",
            "<model>",
            "--prompt",
            "<content>",
            "--output-format",
            "stream-json"
        ]
    );
    assert_eq!(
        executing["legacy_env_not_set_by_swallowtail"],
        "KIMI_CODE_LEGACY_FLAG"
    );
    assert_eq!(
        executing["acp_default_implementation"],
        "packages/acp-server (agent-core-v2) from 0.33.0"
    );
    assert_eq!(executing["acp_native_since"], "0.33.0");
    assert_eq!(
        executing["headless_default_implementation"],
        "apps/kimi-code/src/cli/v2/run-v2-print.ts (agent-core-v2 runV2Print) from 0.33.0"
    );
    assert_eq!(executing["headless_v2_default_since"], "0.33.0");
    assert_eq!(
        executing["routing_corpus"],
        "kimi-code-0.33.0-headless-routing"
    );
}

#[test]
fn the_recorded_argv_selects_no_resume_goal_or_bypass_flag() {
    // The production link lives in `headless_command`'s own unit tests, which
    // can reach the private constructor. This asserts only what the corpus
    // itself must state: the selected argv shape and the absence of any
    // unselected flag.
    let executing = &json(PROTOCOL)["executing_path"];
    let argv = strings(&executing["headless_argv"]);
    assert_eq!(argv[0], "--model");
    assert_eq!(argv[2], "--prompt");
    assert_eq!(argv[4], "--output-format");
    assert_eq!(argv[5], "stream-json");
    assert!(
        !argv.iter().any(|value| value.starts_with("--session")
            || value.starts_with("--goal")
            || value.contains("dangerous")),
        "no resume, goal, or bypass flag is selected"
    );
}

#[test]
fn selected_acp_and_headless_source_spans_are_named_honestly() {
    let protocol = json(PROTOCOL);
    let acp = &protocol["selected_acp_source"];
    assert_eq!(acp["native_acp_first_shipped"], "0.33.0");
    // server.ts is the one mapped acp-server surface that moves inside the
    // native span; both spans are named rather than a single false range.
    assert_sha1(
        &acp["acp_server_server_ts"]["0.33.0..=0.36.1"],
        "4f1b3e464027d3b80a6d560aa042d173d747d671",
    );
    assert_sha1(
        &acp["acp_server_server_ts"]["0.37.0..=0.39.1"],
        "6e4ee87840af172117d66673bafe3426479c3b5b",
    );
    assert_sha1(
        &acp["acp_server_events_map_ts_unchanged_0_33_0_through_0_39_1"],
        "cb549b736199c4857f43791fb9153894f3f18a0b",
    );
    assert_sha1(
        &acp["acp_server_auth_methods_ts_unchanged_0_33_0_through_0_39_1"],
        "7b537995c99b5dcb5e065f4fde86e8d1600e3925",
    );
    assert_sha1(
        &acp["acp_server_config_options_ts_unchanged_0_33_0_through_0_39_1"],
        "beeb4300833e6db38bd16f35dd5e57607c841fad",
    );
    assert_sha1(
        &acp["dispatch_acp_ts"]["0.38.0..=0.39.1"],
        "d0803a852fdaca4fcb68cb1794a626854fcd1224",
    );

    let headless = &protocol["selected_headless_source"];
    assert_sha1(
        &headless["prompt_render_ts_unchanged_0_30_0_through_0_39_1"],
        "0e2f35238db066a13b53ad2cfff11bdff2f76724",
    );
    assert_sha1(
        &headless["options_ts_unchanged_0_32_0_through_0_39_1"],
        "004fd7cabdf622dba31ec8c5c3037c0b797fdb95",
    );
    assert_sha1(
        &headless["run_prompt_ts_unchanged_0_33_0_through_0_39_1"],
        "cd519b223ea79355a0656a0254b8803576c3deb0",
    );
    assert_sha1(
        &headless["experimental_v2_ts_unchanged_0_33_0_through_0_39_1"],
        "09deacc9c2f891fa27944171662116ea5e2e2c83",
    );
    // The routing corpus names the same two engine-gate blobs; a fabricated
    // edit has to be made consistently in both files.
    let routing = json(ROUTING_PROTOCOL);
    assert_eq!(
        routing["engine_gate"]["blob_0_33_0_through_0_39_1"],
        headless["experimental_v2_ts_unchanged_0_33_0_through_0_39_1"]
    );
    assert_eq!(
        routing["dispatch_site"]["blob_0_33_0_through_0_39_1"],
        headless["run_prompt_ts_unchanged_0_33_0_through_0_39_1"]
    );
}

#[test]
fn every_changed_selected_adjacent_file_is_classified_and_stays_unmapped() {
    let protocol = json(PROTOCOL);
    let changed_acp = protocol["changed_acp_source"]
        .as_object()
        .expect("changed ACP ledger");
    assert_eq!(changed_acp.len(), 4);
    for (name, entry) in changed_acp {
        assert_eq!(entry["mapped"], false, "{name} stays unmapped");
        assert_ne!(
            entry["from_0_38_0"], entry["at_0_39_0_and_0_39_1"],
            "{name} records a real blob move"
        );
        assert!(
            !entry["delta"].as_str().expect("delta is text").is_empty(),
            "{name} names its delta"
        );
    }
    assert_eq!(
        changed_acp["acp_server_convert_ts"]["inert_reason"],
        "Swallowtail sends mcpServers [] on session/new and session/load, so the converted loop body is unreachable"
    );
    // The terminal runner is the one changed file that blocks qualification.
    let runner = &changed_acp["acp_server_acp_terminal_runner_ts"];
    assert_eq!(runner["blocks_acp_qualification_of_0_39_x"], true);
    assert_eq!(
        runner["contract_029_trigger"],
        "capability and failure behavior"
    );

    let v2 = &protocol["changed_headless_source"]["run_v2_print_ts"];
    assert_eq!(v2["mapped"], false);
    assert_eq!(v2["emission_sites_unchanged"], true);
    assert_sha1(
        &v2["from_0_38_0"],
        "40a8c31ac64a0ab737ca6cc308cda84376629ce1",
    );
    assert_sha1(
        &v2["at_0_39_0_and_0_39_1"],
        "299e926caa6d749c873d5c3fc277aa1e96303887",
    );
}

#[test]
fn unmapped_extras_stay_unmapped() {
    let unmapped = json(PROTOCOL);
    let unmapped = strings(&unmapped["unmapped_deltas"]);
    for required in [
        "acp-server local process fallback when the client advertises no terminal capability",
        "acp-server stdio MCP server default runtime identity",
        "kimi rc / kimi web --remote-control / /remote-control experimental Remote Control behind KIMI_CODE_EXPERIMENTAL_REMOTE_CONTROL",
        "experimental tower multi-agent orchestration behind KIMI_CODE_EXPERIMENTAL_TOWER",
    ] {
        assert!(
            unmapped.contains(&required),
            "missing unmapped delta {required}"
        );
    }
}
