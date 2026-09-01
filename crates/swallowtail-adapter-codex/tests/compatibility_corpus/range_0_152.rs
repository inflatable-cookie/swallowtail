#[test]
fn range_corpus_freezes_official_0_152_0_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_152_0_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.152.0");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-09-01T02:02:46.635Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-Vx0tg/J5SbxYYGJazTtL/XySK9Dlqc5KW1MZM71NMwVci/4F1ap+FfSKPFTlrICEtOTuq3KNcWSdv9oMGdPuRw=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "a9f8948612ef63fd7441b0c551d8805e1213cbb9"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.152.0");
    assert_eq!(
        corpus["tag_commit"],
        "316795b3cf2a45e90d121d9f46499d4658b2645c"
    );
    assert_eq!(corpus["local_cli"], "codex-cli 0.150.1");
    assert_eq!(corpus["official_cli"], "codex-cli 0.152.0");
    assert_eq!(strings(&corpus["published_intermediates"]), ["0.152.0"]);
    assert_eq!(
        strings(&corpus["unpublished_gaps"]),
        ["0.149.2", "0.150.2", "0.151.1"]
    );
    assert_sha256(&corpus["schema"]["v2_bundle_sha256"]);
    assert_eq!(
        corpus["schema"]["v2_bundle_sha256"],
        "08fa1b1072c5d8a889f00fdd96d1c853084e288d89d246552c1c47c23142adbb"
    );
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
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.151.0");
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
    assert_eq!(decision["extend_through"], "0.152.0");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.152.1"
    );
}

#[test]
fn identity_protocol_keeps_thread_resume_exclude_turns_selected_and_unchanged() {
    let protocol = json(include_str!(
        "../fixtures/codex-cli-0.152.0/protocol.json"
    ));
    let unused = strings(&protocol["unused_deltas"]);
    assert!(
        !unused.iter().any(|delta| delta.contains("excludeTurns")),
        "already-selected mapped excludeTurns must not be listed unused"
    );
    assert_eq!(
        protocol["schema"]["thread_resume_params_sha256"],
        "8ac68582a81d60940b10b330be8546123f56bfe246b56f8a4f121da00f347cf2"
    );
    assert_eq!(
        protocol["schema"]["thread_resume_params_byte_identical_to_0_151_0"],
        true
    );
    assert_eq!(
        protocol["schema"]["thread_resume_exclude_turns"],
        "already-selected-mapped"
    );
    assert_eq!(
        strings(&protocol["schema"]["thread_resume_required"]),
        ["threadId"]
    );
    let properties = strings(&protocol["schema"]["thread_resume_properties"]);
    assert!(properties.contains(&"excludeTurns"));
    assert!(properties.contains(&"threadId"));
}
