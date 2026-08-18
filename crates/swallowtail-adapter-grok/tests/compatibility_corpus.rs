use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_grok::{GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, grok_build_acp_claim};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/grok-build-0.2.114-0.2.117/compatibility.json");
const RECOVERY_CORPUS: &str =
    include_str!("fixtures/grok-build-0.2.114-0.2.117/continuation-recovery.json");
const IDENTITY_1_0_4: &str = include_str!("fixtures/grok-1-0-4-identity.json");
const HANDSHAKE_1_0_4: &str = include_str!("fixtures/grok-1-0-4/compatibility.json");

fn corpus() -> Value {
    serde_json::from_str(CORPUS).expect("Grok Build compatibility corpus is valid JSON")
}

fn recovery_corpus() -> Value {
    serde_json::from_str(RECOVERY_CORPUS)
        .expect("Grok Build continuation-recovery corpus is valid JSON")
}

fn records(corpus: &Value) -> &[Value] {
    corpus["stable_versions"]
        .as_array()
        .expect("stable versions are an array")
}

#[test]
fn corpus_freezes_every_stable_artifact_and_executable_identity() {
    let corpus = corpus();
    let records = records(&corpus);
    let versions = records
        .iter()
        .map(|record| record["version"].as_str().expect("version is text"))
        .collect::<Vec<_>>();

    assert_eq!(versions, ["0.2.114", "0.2.115", "0.2.116", "0.2.117"]);
    assert_eq!(versions.iter().copied().collect::<BTreeSet<_>>().len(), 4);
    assert!(records.iter().all(|record| {
        let revision = record["source_revision"].as_str().unwrap();
        let launcher = record["launcher_npm_integrity"].as_str().unwrap();
        let platform = record["platform_npm_integrity"].as_str().unwrap();
        let platform_sha = record["platform_tar_sha256"].as_str().unwrap();
        let executable_sha = record["executable_sha256"].as_str().unwrap();
        (7..=40).contains(&revision.len())
            && revision.bytes().all(|byte| byte.is_ascii_hexdigit())
            && launcher.starts_with("sha512-")
            && platform.starts_with("sha512-")
            && is_sha256(platform_sha)
            && is_sha256(executable_sha)
    }));
}

#[test]
fn selected_acp_initialization_is_stable_across_the_interval() {
    let corpus = corpus();
    for record in records(&corpus) {
        assert_eq!(record["acp_protocol_version"], 1);
        assert_eq!(record["load_session"], true);
        assert_eq!(
            record["auth_methods"],
            serde_json::json!(["cached_token", "grok.com"])
        );
        assert_eq!(record["default_auth_method"], "cached_token");
        assert_eq!(record["models"], serde_json::json!(["grok-4.5"]));
        assert_eq!(
            record["efforts"],
            serde_json::json!(["high", "medium", "low"])
        );
    }
}

#[test]
fn task_control_behavior_changes_only_at_0_2_117_without_new_authority() {
    let corpus = corpus();
    let records = records(&corpus);
    for record in records {
        let latest = record["version"] == "0.2.117";
        assert_eq!(
            record["acp_task_control_revision"],
            if latest { 2 } else { 1 }
        );
        assert_eq!(
            record["candidate_behavior_revision"],
            if latest {
                "grok-build.acp-v1.cached-token-task-control-v2"
            } else {
                "grok-build.acp-v1.cached-token-activation-v1"
            }
        );
    }

    let absences = corpus["selected_absences"].as_array().unwrap();
    for absent in [
        "direct_task_control",
        "direct_subagent_control",
        "api_key_fallback",
        "interactive_login_fallback",
        "implicit_sandbox",
    ] {
        assert!(absences.iter().any(|value| value == absent));
    }
}

#[test]
fn production_claim_matches_the_frozen_segments() {
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, "1.0.4");
    let claim = grok_build_acp_claim();
    for candidate in ["0.2.114", "0.2.115", "0.2.116", "0.2.117", "1.0.4"] {
        assert!(claim.supports(&version(candidate)));
    }
    assert!(matches!(
        claim.assess(&version("0.2.117")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(matches!(
        claim.assess(&version("1.0.4")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert!(matches!(
        claim.assess(&version("1.0.6")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    for gap in ["0.2.118", "0.2.121", "1.0.0", "1.0.3"] {
        assert!(!claim.permits(&version(gap)));
    }
    assert!(!claim.permits(&version("0.2.117-alpha.1")));
}

#[test]
fn identity_and_handshake_qualify_1_0_4_as_same_axis_milestone() {
    let identity: Value =
        serde_json::from_str(IDENTITY_1_0_4).expect("Grok 1.0.4 identity corpus is valid JSON");
    assert_eq!(identity["axis"], "grok-build.executable");
    assert_eq!(identity["version"], "1.0.4");
    assert_eq!(identity["npm_package"], "@xai-official/grok");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-Nu3SFXTqwvCQr/LQFwrQYgngJhUQwX2h9ZSgzW4HowidjbPBWtMVO0xI88d2z6/zlDSNaT5YP/uk+2DthKQMsg=="
    );
    assert_eq!(identity["source_revision"], "d846eb93d94d");
    assert_eq!(identity["local_cli"], "grok 1.0.4 (d846eb93d94d) [stable]");
    assert!(is_sha256(
        identity["local_executable_sha256"]
            .as_str()
            .expect("executable digest is text")
    ));
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "same-axis-milestone");
    assert_eq!(decision["flatten_as_0_2_unverified_newer"], false);
    assert_eq!(decision["fail_closed"], false);
    assert_eq!(decision["qualify_1_0"], true);

    let handshake: Value =
        serde_json::from_str(HANDSHAKE_1_0_4).expect("Grok 1.0.4 handshake corpus is valid JSON");
    assert_eq!(handshake["version"], "1.0.4");
    assert_eq!(handshake["acp_protocol_version"], 1);
    assert_eq!(handshake["agent_version"], "1.0.4");
    assert_eq!(handshake["load_session"], true);
    assert_eq!(
        handshake["auth_methods"],
        serde_json::json!(["cached_token", "grok.com"])
    );
    assert_eq!(handshake["default_auth_method"], "cached_token");
    assert_eq!(handshake["models"], serde_json::json!(["grok-4.6"]));
    assert_eq!(
        handshake["efforts"],
        serde_json::json!(["xhigh", "high", "medium", "low"])
    );
    assert_eq!(
        handshake["candidate_behavior_revision"],
        "grok-build.acp-v1.cached-token-model-4-6-v3"
    );
    assert_eq!(handshake["authenticate_ok"], true);
    assert_eq!(handshake["session_new_ok"], true);
    assert_eq!(handshake["provider_prompt_sent"], false);
    assert_eq!(handshake["session_resume_qualified"], false);

    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, "1.0.4");
    let claim = grok_build_acp_claim();
    assert!(matches!(
        claim.assess(&version("1.0.4")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert_eq!(
        claim
            .assess(&version("1.0.4"))
            .behavior_revision()
            .unwrap()
            .as_str(),
        "grok-build.acp-v1.cached-token-model-4-6-v3"
    );
}

#[test]
fn load_replay_remains_blocked_without_complete_client_visible_evidence() {
    let corpus = recovery_corpus();
    let artifacts = corpus["qualified_artifacts"]
        .as_array()
        .expect("qualified artifacts are an array");
    assert_eq!(artifacts.len(), 4);
    assert_eq!(
        artifacts
            .iter()
            .map(|artifact| artifact["version"].as_str().expect("version is text"))
            .collect::<Vec<_>>(),
        ["0.2.114", "0.2.115", "0.2.116", "0.2.117"]
    );
    assert!(artifacts.iter().all(|artifact| {
        is_sha256(
            artifact["executable_sha256"]
                .as_str()
                .expect("executable digest is text"),
        )
    }));

    assert_eq!(corpus["load"]["advertised"], true);
    assert_eq!(
        corpus["load"]["artifact_form"],
        "official_stripped_native_executables"
    );
    assert_eq!(corpus["load"]["public_control_flow_source"], false);
    assert_eq!(corpus["load"]["deterministic_load_transcript"], false);
    assert_eq!(corpus["load"]["client_visible_replay_integrity"], false);
    assert_eq!(
        corpus["load"]["embedded_replay_paths"],
        serde_json::json!([
            "completion_drain",
            "cursor_missing_falls_back_to_full_replay",
            "unparseable_replay_records_are_skipped",
            "post_replay_flush_failure_skips_delta_replay"
        ])
    );
    assert_eq!(corpus["decision"]["continuation_recovery"], "blocked");
    assert_eq!(corpus["decision"]["production_mapping"], false);
    assert_eq!(
        corpus["unqualified_negative_cases"]
            .as_array()
            .expect("negative cases are an array")
            .len(),
        10
    );
    assert_eq!(corpus["provider_prompt_sent"], false);
    assert_eq!(corpus["provider_session_loaded"], false);
    assert_eq!(corpus["authenticated_work_performed"], false);
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
