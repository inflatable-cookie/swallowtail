#[test]
fn range_corpus_freezes_official_0_154_0_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_154_0_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.154.0");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-09-09T22:40:10.746Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-FV/x1OHXYv/ifjf3mXj9ThTTAWcUZN6cGIRQRhRxkKNOPuImu1WW0c8ev1vUkE9XGH90dEnYG1tBjIkxRikg0w=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "eb1ec011cca427c606ef6cd4814014e59a1c3d70"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.154.0");
    assert_eq!(
        corpus["tag_commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
    );
    assert_eq!(corpus["local_cli"], "codex-cli 0.153.3");
    assert_eq!(corpus["official_cli"], "codex-cli 0.154.0");
    assert_eq!(
        strings(&corpus["published_intermediates"]),
        [
            "0.153.0",
            "0.153.1",
            "0.153.2",
            "0.153.3",
            "0.153.4",
            "0.154.0"
        ]
    );
    assert_eq!(
        strings(&corpus["unpublished_gaps"]),
        ["0.149.2", "0.150.2", "0.151.1", "0.152.2"]
    );
    let source_delta = &corpus["source_delta"];
    assert_eq!(
        source_delta["base_commit"],
        "5adb68a49933ae446bf11935662c83dba55a0804"
    );
    assert_eq!(
        source_delta["head_commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
    );
    assert_eq!(source_delta["changed_files"], 1812);
    assert_eq!(source_delta["mapped_feeding_changed_files"], 0);
    let methods = string_set(&corpus["schema"]["methods_present"]);
    for method in [
        "initialize",
        "model/list",
        "thread/list",
        "thread/read",
        "thread/start",
        "thread/resume",
        "thread/archive",
        "thread/delete",
        "turn/start",
        "turn/interrupt",
        "item/started",
        "item/completed",
        "item/plan/delta",
        "subAgentActivity",
        "collabAgentToolCall",
    ] {
        assert!(methods.contains(method), "missing {method}");
    }
    assert_sha256(&corpus["schema"]["v2_bundle_sha256"]);
    assert_eq!(
        corpus["schema"]["v2_bundle_sha256"],
        "f3487938786b729cb6773dbc9e83a7efab9c78c845db7094e8f539f373cbacc9"
    );
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.152.1");
    assert_eq!(corpus["claim_at_observation"]["classification"], "unverified_newer");
    let decision = &corpus["segment_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["exec_behavior"], "codex.exec.jsonl-v1");
    assert_eq!(
        decision["app_server_behavior"],
        "codex.app-server.v2.workspace-roots"
    );
    assert_eq!(
        decision["lifecycle_behavior"],
        "codex.app-server.lifecycle.v1.strict-descendant-hard-delete"
    );
    assert_eq!(decision["extend_through"], "0.154.0");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.154.1"
    );
}

#[test]
fn identity_protocol_keeps_selected_surfaces_compatible_past_0_152_1() {
    let protocol = json(include_str!(
        "../fixtures/codex-cli-0.154.0/protocol.json"
    ));
    let frozen = json(include_str!(
        "../fixtures/codex-cli-0.152.1/protocol.json"
    ));
    let unused = strings(&protocol["unused_deltas"]);
    let frozen_unused = strings(&frozen["unused_deltas"]);
    for entry in frozen_unused {
        assert!(unused.contains(&entry), "{entry} must survive");
    }
    assert!(unused.contains(&"exec --worktree global opt-in with selected-combination guards"));
    assert!(unused.contains(&"plugin reconcile method"));
    assert!(unused.contains(&"user-verification ceremony"));
    assert!(unused.contains(&"Guardian"));
    assert_eq!(strings(&protocol["exec_selected_flags_present"]), strings(&frozen["exec_selected_flags_present"]));
    assert_eq!(
        strings(&protocol["app_server_selected_flags_present"]),
        strings(&frozen["app_server_selected_flags_present"])
    );
    assert_eq!(protocol["schema"]["methods_present"], frozen["schema"]["methods_present"]);
    assert_eq!(protocol["schema"]["thread_resume_properties"], frozen["schema"]["thread_resume_properties"]);
    assert_eq!(
        strings(&protocol["schema"]["thread_resume_required"]),
        ["threadId"]
    );
    assert_eq!(protocol["schema"]["thread_resume_exclude_turns"], "already-selected-mapped");
    let properties = strings(&protocol["schema"]["thread_resume_properties"]);
    assert!(properties.contains(&"excludeTurns"));
    assert!(properties.contains(&"threadId"));
    assert_eq!(protocol["github_source_delta"]["mapped_feeding_changed_files"], serde_json::json!([]));
    assert_eq!(protocol["downloaded_binaries_executed"], false);
}
