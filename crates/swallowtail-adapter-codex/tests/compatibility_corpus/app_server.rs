#[test]
fn app_server_corpus_keeps_stable_experimental_and_milestones_separate() {
    let corpus = json(APP_SERVER_RELEASES);
    assert_eq!(corpus["facade"], "codex-app-server-v2");
    assert_eq!(
        strings(&corpus["candidate_versions"]),
        [
            "0.110.0", "0.120.0", "0.131.0", "0.140.0", "0.144.6", "0.145.0", "0.146.0", "0.147.0", "0.148.0", "0.149.0"
        ]
    );
    let methods = string_set(&corpus["stable_methods"]);
    for method in [
        "initialize",
        "model/list",
        "thread/start",
        "thread/resume",
        "turn/start",
        "turn/interrupt",
        "turn/started",
        "turn/completed",
        "item/completed",
        "item/agentMessage/delta",
    ] {
        assert!(methods.contains(method), "missing stable method {method}");
    }
    let transcript_coverage = string_set(&corpus["transcript_coverage"]);
    for exchange in [
        "initialize",
        "catalogue",
        "session-open",
        "session-resume",
        "turn",
        "callback",
        "interrupt",
        "provider-failure",
        "disconnect",
        "close",
    ] {
        assert!(
            transcript_coverage.contains(exchange),
            "missing transcript exchange {exchange}"
        );
    }

    let releases = corpus["releases"]
        .as_array()
        .expect("release corpus is an array");
    for release in releases {
        assert_exact_evidence(release);
        assert_sha256(&release["stable_bundle_sha256"]);
        assert_sha256(&release["experimental_bundle_sha256"]);
        assert_ne!(
            release["stable_bundle_sha256"], release["experimental_bundle_sha256"],
            "stable schema cannot stand in for experimental schema"
        );
        assert_sha256(&release["model_list_schema_sha256"]);
        let version = release["version"].as_str().expect("version is text");
        let fields = string_set(&release["experimental_thread_fields"]);
        assert!(fields.contains("dynamicTools"));
        if matches!(
            version,
            "0.131.0" | "0.140.0" | "0.144.6" | "0.145.0" | "0.146.0" | "0.147.0" | "0.148.0" | "0.149.0"
        ) {
            assert_eq!(release["runtime_workspace_roots"], true);
            assert!(fields.contains("runtimeWorkspaceRoots"));
            assert_eq!(
                release["behavior_revision"],
                "codex.app-server.v2.workspace-roots"
            );
        } else {
            assert_eq!(release["runtime_workspace_roots"], false);
            assert!(!fields.contains("runtimeWorkspaceRoots"));
            assert_eq!(release["behavior_revision"], "codex.app-server.v2.base");
        }
        if matches!(version, "0.144.6" | "0.145.0" | "0.146.0" | "0.147.0" | "0.148.0" | "0.149.0") {
            assert_eq!(release["allow_provider_model_fallback"], true);
            assert!(fields.contains("allowProviderModelFallback"));
        } else {
            assert_eq!(release["allow_provider_model_fallback"], false);
            assert!(!fields.contains("allowProviderModelFallback"));
        }
    }

    let rejections = corpus["rejections"]
        .as_array()
        .expect("rejections are an array");
    let rejected: BTreeSet<_> = rejections
        .iter()
        .map(|entry| entry["version"].as_str().expect("version is text"))
        .collect();
    assert_eq!(
        rejected,
        BTreeSet::from([
            "0.107.0",
            "0.108.0",
            "0.109.0",
            "0.146.0-alpha.4",
            "not-a-version",
        ])
    );
    assert_unverified_newer(&corpus);
}

#[test]
fn experimental_gate_cases_reject_fields_without_capability_opt_in() {
    let cases = json(APP_SERVER_GATES);
    for case in cases.as_array().expect("gate cases are an array") {
        let enabled = case["experimental_api"]
            .as_bool()
            .expect("experimental flag is boolean");
        let accepted = !requires_experimental_api(&case["message"]) || enabled;
        assert_eq!(
            accepted,
            case["accepted"].as_bool().expect("accepted is boolean"),
            "gate case {}",
            case["name"].as_str().expect("case name is text")
        );
    }
}

#[test]
fn core_transcript_preserves_additive_unknowns_and_required_field_failures() {
    let messages: Vec<Value> = APP_SERVER_TRANSCRIPT.lines().map(json).collect();
    let additive = messages
        .iter()
        .find(|message| message["method"] == "future/additive")
        .expect("additive notification is frozen");
    assert!(notification_is_structurally_valid(additive));

    let malformed = serde_json::json!({
        "method": "item/agentMessage/delta",
        "params": {
            "threadId": "thread-fixture",
            "turnId": "turn-fixture"
        }
    });
    assert!(!notification_is_structurally_valid(&malformed));
}

