#[test]
fn range_corpus_freezes_official_0_155_1_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_155_1_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.155.1");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-09-18T20:09:23.628Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-02fAAGyBtlA1zPjEo3kTj/bOSYbPz5DvjLwRZJdV7weFFEDzNFOMjQGmZ/+5CuirYV0hE+AZTrnjzwXYU4AdAQ=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "7190987abfcabc8fcba7774518539d694a490fc8"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.155.1");
    assert_eq!(
        corpus["tag_commit"],
        "be2951ea34f0d295ed0becf97079f92fa5f6950e"
    );
    assert_eq!(corpus["local_cli"], "not-on-PATH");
    assert_eq!(corpus["official_cli"], "codex-cli 0.155.1");
    assert_eq!(
        strings(&corpus["published_intermediates"]),
        ["0.155.0", "0.155.1"]
    );
    assert_eq!(
        strings(&corpus["unpublished_gaps"]),
        ["0.149.2", "0.150.2", "0.151.1", "0.152.2", "0.154.1"]
    );
    let source_delta = &corpus["source_delta"];
    assert_eq!(
        source_delta["base_commit"],
        "6b9826e3aa83b1a5947db50f4332cb9c65f1b340"
    );
    assert_eq!(
        source_delta["head_commit"],
        "be2951ea34f0d295ed0becf97079f92fa5f6950e"
    );
    assert_eq!(source_delta["changed_files"], 1330);
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
        "f0402dc8ce8d278108f1e68e9d46ec7e59ddd9d153f5e70668d84d56f258dda3"
    );
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.154.0");
    assert_eq!(
        corpus["claim_at_observation"]["classification"],
        "unverified_newer"
    );
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
    assert_eq!(decision["extend_through"], "0.155.1");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.155.2"
    );
}

#[test]
fn identity_protocol_keeps_selected_surfaces_compatible_past_0_154_0() {
    let protocol = json(include_str!(
        "../fixtures/codex-cli-0.155.1/protocol.json"
    ));
    let frozen = json(include_str!(
        "../fixtures/codex-cli-0.154.0/protocol.json"
    ));
    let unused = strings(&protocol["unused_deltas"]);
    let frozen_unused = strings(&frozen["unused_deltas"]);
    for entry in frozen_unused {
        assert!(unused.contains(&entry), "{entry} must survive");
    }
    assert!(unused.contains(&"experimental /voice plus shipped voice-host/gstreamer tree"));
    assert!(unused.contains(&"thread/attachment add/list/remove and updated notification"));
    assert!(unused.contains(&"app-server --managed-daemon hidden flag"));
    assert_eq!(
        strings(&protocol["exec_selected_flags_present"]),
        strings(&frozen["exec_selected_flags_present"])
    );
    assert_eq!(
        strings(&protocol["app_server_selected_flags_present"]),
        strings(&frozen["app_server_selected_flags_present"])
    );
    assert_eq!(
        protocol["schema"]["methods_present"],
        frozen["schema"]["methods_present"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_properties"],
        frozen["schema"]["thread_resume_properties"]
    );
    assert_eq!(
        strings(&protocol["schema"]["thread_resume_required"]),
        ["threadId"]
    );
    assert_eq!(
        protocol["schema"]["thread_resume_exclude_turns"],
        "already-selected-mapped"
    );
    let properties = strings(&protocol["schema"]["thread_resume_properties"]);
    assert!(properties.contains(&"excludeTurns"));
    assert!(properties.contains(&"threadId"));
    assert_eq!(
        protocol["github_source_delta"]["mapped_feeding_changed_files"],
        serde_json::json!([])
    );
    assert_eq!(protocol["downloaded_binaries_executed"], false);
}
