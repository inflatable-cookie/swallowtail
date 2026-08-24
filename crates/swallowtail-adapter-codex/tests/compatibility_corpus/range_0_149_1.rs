#[test]
fn range_corpus_freezes_official_0_149_1_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_149_1_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.149.1");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-08-24T00:32:45.066Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-6q5pbcpFbJbqOpkubSDBwXmktQ55aD8eUzGzBF1zASob2DjwhBKDSNGtdZKalfrNJUdTDTPDMmzCXEXs5tMBYA=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "37bc183ebd129e01e404e932bcec4ea861c70933"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.149.1");
    assert_eq!(
        corpus["tag_commit"],
        "ff29a44391deccde0aba0f8390337d7f3c319ea4"
    );
    assert!(corpus["local_cli"].is_null());
    assert_eq!(corpus["official_cli"], "codex-cli 0.149.1");
    assert_sha256(&corpus["schema"]["v2_bundle_sha256"]);
    assert_eq!(
        corpus["schema"]["v2_bundle_sha256"],
        "9b3de71a5a2ffc980b792a18aa8f8dec3f85f48829560222a0264fe494b679a9"
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
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.149.0");
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
    assert_eq!(decision["extend_through"], "0.149.1");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.149.2"
    );
}
