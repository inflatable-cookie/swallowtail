use super::support::{
    FROZEN_0_152_1_PROTOCOL, PROTOCOL, assert_exact_string_set, assert_sha256, json, strings,
};

#[test]
fn upstream_published_schema_digests_are_the_frozen_0_154_0_values() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_1_PROTOCOL);
    assert_sha256(
        &protocol["schema"]["v2_bundle_sha256"],
        "f3487938786b729cb6773dbc9e83a7efab9c78c845db7094e8f539f373cbacc9",
    );
    assert_sha256(
        &protocol["schema"]["experimental_bundle_sha256"],
        "d71ddf3bf5484f8de2799f7a4793c2e66808a9ec1a330e2307accb088ab5948a",
    );
    assert_sha256(
        &protocol["schema"]["model_list_params_sha256"],
        frozen["schema"]["model_list_params_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["model_list_params_sha256"],
        "de29a536c00a5b8f46f34dba417dabd93365305571a8ed200e33bea85db68b5a",
    );
    assert_sha256(
        &protocol["schema"]["thread_resume_params_sha256"],
        "36b2854eb802559e17b0e5639385a1e39d5279326c50e2cdb836749b6dd7cb6f",
    );
    assert_sha256(
        &protocol["schema"]["turn_start_params_sha256"],
        frozen["schema"]["turn_start_params_sha256"]
            .as_str()
            .expect("digest"),
    );
    assert_sha256(
        &protocol["schema"]["turn_start_params_sha256"],
        "a3835e8c1e942e4b358e1a670939b89918b16c4d13105a579899892b7ade6dea",
    );
}

#[test]
fn selected_flags_methods_and_resume_shape_carry_over() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_1_PROTOCOL);
    assert_eq!(
        protocol["exec_selected_flags_present"],
        frozen["exec_selected_flags_present"]
    );
    assert_exact_string_set(
        &protocol["app_server_selected_flags_present"],
        &["app-server", "--listen", "stdio://"],
    );
    assert_eq!(
        protocol["app_server_selected_flags_present"],
        frozen["app_server_selected_flags_present"]
    );
    assert_eq!(
        protocol["schema"]["methods_present"],
        frozen["schema"]["methods_present"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_required"],
        frozen["schema"]["thread_resume_required"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_properties"],
        frozen["schema"]["thread_resume_properties"]
    );
    let methods = strings(&protocol["schema"]["methods_present"]);
    assert_eq!(methods.len(), 15);
    let properties = strings(&protocol["schema"]["thread_resume_properties"]);
    assert!(properties.contains(&"excludeTurns"));
    assert!(properties.contains(&"threadId"));
    assert_eq!(
        protocol["schema"]["thread_resume_exclude_turns"],
        "already-selected-mapped"
    );
    assert_eq!(
        protocol["schema"]["thread_list_params_originators"],
        "optional-additive-unmapped"
    );
}

#[test]
fn byte_identical_mapped_sources_hold_across_all_seven_tags() {
    let source = &json(PROTOCOL)["github_source_delta"];
    assert_exact_string_set(&source["mapped_feeding_changed_files"], &[]);
    let byte_identical = &source["byte_identical_mapped_sources"];
    for (path, sha) in [
        ("codex-rs/exec/src/exec_events.rs", "30df7f176a02c5283405a70fac2d5ef9acdcb66e"),
        ("codex-rs/exec/src/event_processor_with_jsonl_output.rs", "488cbc52e9ca31cea1203fb8ba923af50e2b39df"),
        ("codex-rs/app-server-protocol/schema/json/v1/InitializeParams.json", "7acc76386c6cdeb534160fc9854e79495f199841"),
        ("codex-rs/app-server-protocol/schema/json/v1/InitializeResponse.json", "1de65f82f4df9fa810fbda3ff199eca49430a29f"),
        ("codex-rs/app-server-protocol/schema/json/v2/ModelListParams.json", "11a3476240ca9dc59a3478a2a87530764d903878"),
        ("codex-rs/app-server-protocol/schema/json/v2/ThreadArchiveParams.json", "49322b60a45ff90c43cf6119ec3a0346985e9ea1"),
        ("codex-rs/app-server-protocol/schema/json/v2/ThreadDeleteParams.json", "1711e11a2f3ba6e0b68520873b69099b340dce0a"),
        ("codex-rs/app-server-protocol/schema/json/v2/ThreadReadParams.json", "920e6c346d6b6fca1773e7ea4b68c69dee46dec1"),
        ("codex-rs/app-server-protocol/schema/json/v2/ThreadStartParams.json", "8bf5ae8bef72a0769bc1b9f66f7ce4cc0148dcb7"),
        ("codex-rs/app-server-protocol/schema/json/v2/TurnInterruptParams.json", "9181428a10e17186272d2cdd74fc2c4f07c76504"),
        ("codex-rs/app-server-protocol/schema/json/v2/TurnInterruptResponse.json", "5d8a0f9ce22e0387218fd390c362de6a61247b8c"),
        ("codex-rs/app-server-protocol/schema/json/v2/TurnStartParams.json", "2ba2e1f4830ad3639715e678e70df9640b7ff405"),
    ] {
        assert_eq!(byte_identical[path], serde_json::json!(sha), "{path} blob SHA");
    }
    let bounded = source["bounded_feeding_changes"]
        .as_array()
        .expect("bounded feeding changes");
    assert_eq!(bounded.len(), 15);
    let files: Vec<&str> = bounded
        .iter()
        .map(|entry| entry["file"].as_str().expect("file"))
        .collect();
    assert_eq!(
        files,
        vec![
            "codex-rs/exec/src/cli.rs",
            "codex-rs/cli/src/main.rs",
            "codex-rs/exec/src/lib.rs",
            "codex-rs/app-server/src/main.rs",
            "codex-rs/app-server/src/lib.rs",
            "codex-rs/app-server/src/message_processor.rs",
            "codex-rs/app-server/src/request_processors/thread_lifecycle.rs",
            "codex-rs/app-server/src/request_processors/turn_processor.rs",
            "codex-rs/app-server/src/request_processors/thread_resume_redaction.rs",
            "codex-rs/app-server/src/bespoke_event_handling.rs",
            "codex-rs/app-server/src/outgoing_message.rs",
            "codex-rs/app-server/src/in_process.rs",
            "codex-rs/app-server/src/transport.rs",
            "codex-rs/app-server/src/request_processors/catalog_processor.rs",
            "codex-rs/app-server/src/request_processors/account_processor.rs",
        ]
    );
    let classifications = source["classifications"]
        .as_array()
        .expect("classifications");
    assert_eq!(classifications.len(), 18);
}

#[test]
fn help_proves_selected_argv_without_claiming_full_help_identity() {
    let protocol = json(PROTOCOL);
    let help = &protocol["help_surface"];
    assert_eq!(help["exec_help_selected_argv_unchanged"], true);
    assert_eq!(help["app_server_help_selected_argv_unchanged"], true);
    assert_eq!(
        protocol["decoder_corpus"],
        "existing-codex-jsonl-and-app-server-v2"
    );
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_session"], false);
    assert_eq!(protocol["host_install_changed"], false);
}

#[test]
fn unmapped_deltas_extend_the_frozen_set_with_bounded_additions() {
    let protocol = json(PROTOCOL);
    let frozen = json(FROZEN_0_152_1_PROTOCOL);
    let unused = strings(&protocol["unused_deltas"]);
    let frozen_unused = strings(&frozen["unused_deltas"]);
    assert!(
        unused.len() > frozen_unused.len(),
        "0.154.0 records the frozen set plus bounded additions"
    );
    assert_eq!(unused.len(), frozen_unused.len() + 15);
    for entry in &frozen_unused {
        assert!(unused.contains(entry), "{entry} must survive");
    }
    let new_entries: Vec<&str> = unused
        .iter()
        .copied()
        .filter(|entry| !frozen_unused.contains(entry))
        .collect();
    assert_eq!(
        new_entries,
        vec![
            "exec --worktree global opt-in with selected-combination guards",
            "exec-server remote and AWS flags",
            "plugin reconcile method",
            "user-verification ceremony",
            "thread/list optional originators",
            "ThreadResume ConfigurationReasoning definitions",
            "item optional questions",
            "thread metadata nullable model and reasoningEffort",
            "turn optional approvalsReviewer consumption",
            "review/start detached deprecation notice",
            "deprecated codex mcp-server entry-point removal",
            "app-server daemon lifecycle commands",
            "thread unload-delay configurability",
            "codex.js vite-plus ownership detection",
            "vendored bwrap refresh",
        ]
    );
    assert_eq!(
        protocol["schema"]["authority"],
        "upstream-published checked-in schema files"
    );
}
