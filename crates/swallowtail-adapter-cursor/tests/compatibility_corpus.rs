use serde_json::Value;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_BASELINE_BUILD_REVISION, CURSOR_AGENT_BASELINE_VERSION,
    CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION, CURSOR_AGENT_LATEST_QUALIFIED_VERSION,
    cursor_acp_claim, cursor_agent_release_binding, cursor_catalogue_claim, cursor_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const CORPUS: &str = include_str!("fixtures/cursor-agent-2026.07.01-2026.07.23/compatibility.json");
const RECOVERY_CORPUS: &str =
    include_str!("fixtures/cursor-agent-2026.07.01-2026.07.23/continuation-recovery.json");

fn corpus() -> Value {
    serde_json::from_str(CORPUS).expect("Cursor compatibility corpus is valid JSON")
}

fn recovery_corpus() -> Value {
    serde_json::from_str(RECOVERY_CORPUS)
        .expect("Cursor continuation-recovery corpus is valid JSON")
}

#[test]
fn corpus_freezes_both_exact_release_and_build_identities() {
    let corpus = corpus();
    let releases = corpus["qualified_releases"]
        .as_array()
        .expect("qualified releases are an array");
    assert_eq!(releases.len(), 2);
    assert_eq!(releases[0]["release_date"], CURSOR_AGENT_BASELINE_VERSION);
    assert_eq!(
        releases[0]["build_revision"],
        CURSOR_AGENT_BASELINE_BUILD_REVISION
    );
    assert_eq!(
        releases[1]["release_date"],
        CURSOR_AGENT_LATEST_QUALIFIED_VERSION
    );
    assert_eq!(
        releases[1]["build_revision"],
        CURSOR_AGENT_LATEST_QUALIFIED_BUILD_REVISION
    );
    for release in releases {
        for field in [
            "artifact_sha256",
            "runtime_index_sha256",
            "acp_chunk_sha256",
            "headless_chunk_sha256",
        ] {
            assert!(is_sha256(release[field].as_str().expect("digest is text")));
        }
    }
}

#[test]
fn selected_route_evidence_retains_three_behaviors_and_no_new_authority() {
    let corpus = corpus();
    assert_eq!(corpus["acp"]["protocol_version"], 1);
    assert_eq!(
        corpus["acp"]["auth_methods"],
        serde_json::json!(["cursor_login"])
    );
    assert_eq!(corpus["acp"]["selected_capabilities_unchanged"], true);
    assert_eq!(corpus["headless"]["output_format_module_identical"], true);
    assert_eq!(corpus["headless"]["prompt_builder_module_identical"], true);
    assert_eq!(corpus["catalogue"]["command_surface_unchanged"], true);
    assert_eq!(
        corpus["unselected_additions"],
        serde_json::json!(["auto-review", "private-cloud-worker", "create-chat"])
    );
    assert_eq!(corpus["provider_prompt_sent"], false);
    assert_eq!(corpus["authenticated_catalogue_called"], false);
}

#[test]
fn production_claims_use_two_singletons_and_keep_later_dates_unverified() {
    for claim in [
        cursor_catalogue_claim(),
        cursor_acp_claim(),
        cursor_headless_claim(),
    ] {
        assert!(claim.supports(&version("2026-07-01")));
        assert!(claim.supports(&version("2026-07-23")));
        assert!(!claim.permits(&version("2026-07-15")));
        assert!(matches!(
            claim.assess(&version("2026-07-24")),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

#[test]
fn every_qualified_date_requires_its_exact_build_revision() {
    for accepted in ["2026.07.01-41b2de7", "2026.07.23-e383d2b"] {
        assert!(cursor_agent_release_binding(accepted).is_some());
    }
    for rejected in ["2026.07.01-deadbee", "2026.07.23-deadbee"] {
        assert!(cursor_agent_release_binding(rejected).is_none());
    }
    assert!(cursor_agent_release_binding("2026.07.24-a1b2c3d").is_some());
}

#[test]
fn load_replay_is_blocked_when_source_suppresses_replay_failures() {
    let corpus = recovery_corpus();
    let artifacts = corpus["qualified_artifacts"]
        .as_array()
        .expect("qualified artifacts are an array");
    assert_eq!(artifacts.len(), 2);
    assert_eq!(artifacts[0]["version"], "2026.07.01-41b2de7");
    assert_eq!(artifacts[1]["version"], "2026.07.23-e383d2b");
    assert!(artifacts.iter().all(|artifact| is_sha256(
        artifact["acp_chunk_sha256"]
            .as_str()
            .expect("ACP chunk digest is text")
    )));

    assert_eq!(corpus["load"]["advertised"], true);
    assert_eq!(corpus["load"]["replay_awaited_before_response"], true);
    assert_eq!(
        corpus["load"]["history_read_failure"],
        "logged_and_suppressed"
    );
    assert_eq!(
        corpus["load"]["turn_replay_failure"],
        "logged_and_suppressed"
    );
    assert_eq!(corpus["load"]["failure_visible_to_client"], false);
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
    InterfaceVersion::new(value).expect("fixture Cursor release date is valid")
}
