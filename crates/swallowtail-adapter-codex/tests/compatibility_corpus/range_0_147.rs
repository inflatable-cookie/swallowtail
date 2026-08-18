#[test]
fn range_corpus_freezes_official_0_147_identity_and_compatible_extension() {
    let corpus = json(CODEX_0_147_RANGE);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(corpus["version"], "0.147.0");
    assert_eq!(corpus["npm_package"], "@openai/codex");
    assert_eq!(corpus["npm_latest"], true);
    assert_eq!(corpus["published_at"], "2026-08-07T01:47:21.081Z");
    assert_eq!(
        corpus["npm_integrity"],
        "sha512-EQLEXecAG2ptxI7UpBMo2TR/ga5596/c/OsYF/0LoUDh5JANZ7IoGqlzBEWbuEVQ76JePIbtTW/ihCkp1a7Z3w=="
    );
    assert_eq!(
        corpus["npm_shasum"],
        "1792030d147156695a2b86db0ec1a000ab9a83fc"
    );
    assert_eq!(corpus["git_tag"], "rust-v0.147.0");
    assert_eq!(
        corpus["tag_commit"],
        "be6e8eac029b183056b7e4402879f15d2c85f61b"
    );
    assert_eq!(corpus["local_cli"], "codex-cli 0.147.0");
    assert_sha256(&corpus["schema"]["v2_bundle_sha256"]);
    assert_eq!(
        corpus["schema"]["v2_bundle_sha256"],
        "f3dec1e031d99a420b137b903f02196d4325eece57620c925bb7130b25f168d2"
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
    assert_eq!(corpus["claim_at_observation"]["latest_qualified"], "0.146.0");
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
    assert_eq!(decision["extend_through"], "0.147.0");
    assert_eq!(decision["raise_thread_catalogue_ceiling"], true);
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(
        decision["later_unverified_after_qualification"],
        "0.148.0"
    );
}
