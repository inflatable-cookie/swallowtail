use serde_json::Value;
use swallowtail_adapter_cline::{CLINE_PACKAGE_VERSION, cline_acp_claim, cline_package_binding};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};

const IDENTITY: &str = include_str!("fixtures/cline-acp-3.0.55/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cline-acp-3.0.55/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/cline-acp-3.0.55/initialize.json");
const SESSION_NEW: &str = include_str!("fixtures/cline-acp-3.0.55/session-new.json");
const SESSION_PROMPT: &str = include_str!("fixtures/cline-acp-3.0.55/session-prompt.json");
const SESSION_CANCEL: &str = include_str!("fixtures/cline-acp-3.0.55/session-cancel.json");
const PERMISSION: &str = include_str!("fixtures/cline-acp-3.0.55/permission.json");
const NEGATIVE: &str = include_str!("fixtures/cline-acp-3.0.55/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/cline-acp-3.0.55/corpus-plan.json");

#[test]
fn frozen_identity_keeps_acp_separate_from_headless_and_auto_approve() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "cline.package");
    assert_eq!(identity["route"], "cline.acp");
    assert_eq!(identity["official"]["version"], CLINE_PACKAGE_VERSION);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_headless_json"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["pass_auto_approve_true"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["run_authenticate_oauth"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_session_load"], false);
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["cline", "--acp"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["cline", "--acp"])
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_auto_approve_true"],
        false
    );
    assert_eq!(protocol["allow_always_unselected"], true);

    let claim = cline_acp_claim();
    let version = InterfaceVersion::new(CLINE_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(cline_package_binding(CLINE_PACKAGE_VERSION).is_some());
}

#[test]
fn named_corpus_files_stay_wired_to_the_first_driver() {
    for (name, body) in [
        ("initialize.json", INITIALIZE),
        ("session-new.json", SESSION_NEW),
        ("session-prompt.json", SESSION_PROMPT),
        ("session-cancel.json", SESSION_CANCEL),
        ("permission.json", PERMISSION),
        ("negative-cases.json", NEGATIVE),
        ("corpus-plan.json", CORPUS_PLAN),
    ] {
        let value: Value = serde_json::from_str(body).unwrap_or_else(|_| panic!("{name}"));
        assert!(value.is_object(), "{name} must be an object");
    }
    let initialize: Value = serde_json::from_str(INITIALIZE).expect("initialize");
    assert_eq!(
        initialize["response"]["result"]["agentInfo"]["name"],
        "cline"
    );
    assert_eq!(
        initialize["request"]["params"]["clientCapabilities"]["fs"]["readTextFile"],
        false
    );
    let permission: Value = serde_json::from_str(PERMISSION).expect("permission");
    assert_eq!(
        permission["swallowtail_default"],
        "do not auto-select allow_always or auto-approve"
    );
    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "json-flag-is-not-acp",
        "auto-approve-true-not-default",
        "id-resume-unmapped",
        "session-load-unmapped",
        "authenticate-oauth-not-swallowtail",
        "malformed-json",
        "oversized-frame",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_cline_run_invariants() {
    let report = run_acp_single_turn_projection_assertions();
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(
            report.covers(assertion),
            "{assertion:?} must remain a provider-neutral ACP assertion"
        );
    }
}
