#[test]
fn range_corpus_freezes_official_0_151_0_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_151_0_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.151.0");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-08-29T09:59:26.300Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-mhtWmOZRdmWD1jPbLDnQb59BsaVP/V+lXe/OFNR9ZcLZU0UCiBwn98Fcav1ss7sDIlHkuqj6nWd44IPeXoOhJA=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "515ca678dd9eec6afd4a7dc34f571c6536b3f282"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.151.0");
    assert_eq!(
        corpus["tag_commit"],
        "78c290807ce710180111df227df3b7a4fe845452"
    );
    assert!(corpus["local_cli"].is_null());
    assert_eq!(corpus["official_cli"], "codex-cli 0.151.0");
    assert_eq!(
        strings(&corpus["published_intermediates"]),
        ["0.150.0", "0.150.1"]
    );
    assert_eq!(
        strings(&corpus["unpublished_gaps"]),
        ["0.149.2", "0.150.2"]
    );
    assert_sha256(&corpus["schema"]["v2_bundle_sha256"]);
    assert_eq!(
        corpus["schema"]["v2_bundle_sha256"],
        "2442b15801bc019ad55987ad03e0f0ae60c51417825b9b6d708db640e6c2651c"
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
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.149.1");
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
    assert_eq!(decision["extend_through"], "0.151.0");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.151.1"
    );
}
