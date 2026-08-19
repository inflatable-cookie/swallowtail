use serde_json::Value;
use swallowtail_adapter_copilot_cli::{
    COPILOT_CLI_ACP_MATURITY, COPILOT_CLI_PACKAGE_VERSION, copilot_cli_acp_claim,
    copilot_cli_package_binding,
};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};

const IDENTITY: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/identity.json");
const PROTOCOL: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/initialize.json");
const SESSION_NEW: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/session-new.json");
const SESSION_PROMPT: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/session-prompt.json");
const SESSION_CANCEL: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/session-cancel.json");
const PERMISSION: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/permission.json");
const NEGATIVE: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/copilot-cli-acp-1.0.80/corpus-plan.json");

#[test]
fn frozen_identity_keeps_acp_stdio_separate_from_tcp_and_yolo() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "copilot-cli.package");
    assert_eq!(identity["route"], "copilot-cli.acp");
    assert_eq!(identity["official"]["version"], COPILOT_CLI_PACKAGE_VERSION);
    assert_eq!(identity["maturity"], "public-preview");
    assert_eq!(
        identity["identity_decision"]["flatten_onto_tcp_port"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["pass_yolo_or_allow_all"],
        false
    );
    assert_eq!(identity["identity_decision"]["run_copilot_login"], false);
    assert_eq!(identity["identity_decision"]["map_session_load"], false);
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["copilot", "--acp", "--stdio"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["copilot", "--acp", "--stdio"])
    );
    assert_eq!(protocol["maturity"], "public-preview");
    assert_eq!(protocol["authority"]["swallowtail_passes_port"], false);
    assert_eq!(protocol["permission"]["allow_always_unselected"], true);
    assert_eq!(protocol["permission"]["yolo_unselected"], true);
    assert_eq!(
        protocol["initialize"]["result_recovered_from_public_source"],
        false
    );

    let claim = copilot_cli_acp_claim();
    let version = InterfaceVersion::new(COPILOT_CLI_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(copilot_cli_package_binding(COPILOT_CLI_PACKAGE_VERSION).is_some());
    assert_eq!(COPILOT_CLI_ACP_MATURITY, "public-preview");
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
    assert_eq!(initialize["response_recovered_from_public_source"], false);
    assert_eq!(
        initialize["request"]["params"]["clientCapabilities"]["fs"]["readTextFile"],
        false
    );
    let permission: Value = serde_json::from_str(PERMISSION).expect("permission");
    assert_eq!(permission["official_example_outcome"], "cancelled");
    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "tcp-port-is-not-stdio-acp",
        "yolo-not-swallowtail-authority",
        "allow-all-not-swallowtail-authority",
        "available-tools-unmapped",
        "effort-unmapped",
        "session-load-unmapped",
        "login-not-swallowtail",
        "interactive-slash-login-unmapped",
        "prerelease-ignored",
        "malformed-json",
        "oversized-frame",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_copilot_run_invariants() {
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
