use serde_json::Value;
use std::collections::BTreeSet;

const EXEC_RELEASES: &str = include_str!("fixtures/compatibility/exec-releases.json");
const APP_SERVER_RELEASES: &str = include_str!("fixtures/compatibility/app-server-releases.json");
const APP_SERVER_GATES: &str = include_str!("fixtures/compatibility/app-server-gate-cases.json");
const APP_SERVER_TRANSCRIPT: &str = include_str!("fixtures/compatibility/app-server-core.jsonl");

#[test]
fn exec_corpus_freezes_baseline_checkpoints_and_rejections() {
    let corpus = json(EXEC_RELEASES);
    assert_eq!(corpus["axis"], "codex.cli");
    assert_eq!(
        strings(&corpus["candidate_versions"]),
        ["0.122.0", "0.130.0", "0.140.0", "0.144.6", "0.145.0"]
    );
    let required_argv = string_set(&corpus["required_argv"]);
    for required in [
        "exec",
        "--json",
        "--ephemeral",
        "--ignore-user-config",
        "--ignore-rules",
        "--skip-git-repo-check",
        "--sandbox",
        "read-only",
    ] {
        assert!(required_argv.contains(required), "missing {required}");
    }

    let releases = corpus["releases"]
        .as_array()
        .expect("release corpus is an array");
    for release in releases {
        assert_exact_evidence(release);
        assert_eq!(release["json"], true);
        assert_eq!(release["ephemeral"], true);
        match release["classification"]
            .as_str()
            .expect("classification is text")
        {
            "candidate" => {
                assert_eq!(release["ignore_user_config"], true);
                assert_eq!(release["ignore_rules"], true);
            }
            "below_baseline" => {
                assert_eq!(release["version"], "0.121.0");
                assert_eq!(release["ignore_user_config"], false);
                assert_eq!(release["ignore_rules"], false);
            }
            "prerelease" => assert_eq!(release["version"], "0.146.0-alpha.4"),
            other => panic!("unexpected classification {other}"),
        }
    }
    assert_eq!(
        strings(&corpus["synthetic_rejections"]),
        ["not-a-version", "0.146.0"]
    );
}

#[test]
fn app_server_corpus_keeps_stable_experimental_and_milestones_separate() {
    let corpus = json(APP_SERVER_RELEASES);
    assert_eq!(corpus["facade"], "codex-app-server-v2");
    assert_eq!(
        strings(&corpus["candidate_versions"]),
        [
            "0.110.0", "0.120.0", "0.131.0", "0.140.0", "0.144.6", "0.145.0"
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
        if matches!(version, "0.131.0" | "0.140.0" | "0.144.6" | "0.145.0") {
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
        if matches!(version, "0.144.6" | "0.145.0") {
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
            "0.146.0",
            "0.146.0-alpha.4",
            "not-a-version",
        ])
    );
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

fn requires_experimental_api(message: &Value) -> bool {
    const EXPERIMENTAL_FIELDS: &[&str] = &[
        "allowProviderModelFallback",
        "collaborationMode",
        "dynamicTools",
        "runtimeWorkspaceRoots",
    ];
    message["params"].as_object().is_some_and(|params| {
        EXPERIMENTAL_FIELDS
            .iter()
            .any(|field| params.contains_key(*field))
    })
}

fn notification_is_structurally_valid(message: &Value) -> bool {
    match message["method"].as_str() {
        Some("item/agentMessage/delta") => message["params"]["delta"].is_string(),
        Some(_) => true,
        None => false,
    }
}

fn assert_exact_evidence(release: &Value) {
    assert_eq!(
        release["tag_commit"]
            .as_str()
            .expect("tag commit is text")
            .len(),
        40
    );
    assert!(
        release["npm_integrity"]
            .as_str()
            .expect("integrity is text")
            .starts_with("sha512-")
    );
    assert_eq!(
        release["npm_shasum"]
            .as_str()
            .expect("npm shasum is text")
            .len(),
        40
    );
    for key in ["help_sha256", "cli_source_sha256", "events_source_sha256"] {
        if !release[key].is_null() {
            assert_sha256(&release[key]);
        }
    }
}

fn assert_sha256(value: &Value) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|value| value.as_str().expect("array value is text"))
        .collect()
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    strings(value).into_iter().collect()
}
