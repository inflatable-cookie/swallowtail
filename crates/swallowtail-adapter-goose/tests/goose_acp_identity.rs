use serde_json::Value;
use swallowtail_adapter_goose::{GOOSE_RELEASE_VERSION, goose_acp_claim, goose_release_binding};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};

const IDENTITY: &str = include_str!("fixtures/goose-acp-1.46.0/identity.json");
const PROTOCOL: &str = include_str!("fixtures/goose-acp-1.46.0/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/goose-acp-1.46.0/initialize.json");
const SESSION_NEW: &str = include_str!("fixtures/goose-acp-1.46.0/session-new.json");
const SESSION_PROMPT: &str = include_str!("fixtures/goose-acp-1.46.0/session-prompt.json");
const SESSION_CANCEL: &str = include_str!("fixtures/goose-acp-1.46.0/session-cancel.json");
const PERMISSION: &str = include_str!("fixtures/goose-acp-1.46.0/permission.json");
const NEGATIVE: &str = include_str!("fixtures/goose-acp-1.46.0/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/goose-acp-1.46.0/corpus-plan.json");

#[test]
fn frozen_identity_keeps_acp_separate_from_serve_and_with_builtin() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "goose.release");
    assert_eq!(identity["route"], "goose.acp");
    assert_eq!(identity["official"]["version"], GOOSE_RELEASE_VERSION);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_goose_serve"],
        false
    );
    assert_eq!(identity["identity_decision"]["pass_with_builtin"], false);
    assert_eq!(identity["identity_decision"]["run_goose_configure"], false);
    assert_eq!(identity["identity_decision"]["map_session_load"], false);
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["goose", "acp"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["goose", "acp"])
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_with_builtin"],
        false
    );
    assert_eq!(protocol["allow_always_unselected"], true);
    assert_eq!(protocol["auto_mode_unselected"], true);

    let claim = goose_acp_claim();
    let version = InterfaceVersion::new(GOOSE_RELEASE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(goose_release_binding(GOOSE_RELEASE_VERSION).is_some());
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
        "goose"
    );
    assert_eq!(
        initialize["request"]["params"]["clientCapabilities"]["fs"]["readTextFile"],
        false
    );
    let permission: Value = serde_json::from_str(PERMISSION).expect("permission");
    assert_eq!(
        permission["swallowtail_default"],
        "do not auto-select allow_always, reject_always, or GooseMode auto"
    );
    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "serve-is-not-stdio-acp",
        "with-builtin-unmapped",
        "enable-scheduler-unmapped",
        "session-load-unmapped",
        "configure-not-swallowtail",
        "acp-providers-not-this-route",
        "auto-mode-not-swallowtail-authority",
        "malformed-json",
        "oversized-frame",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_goose_run_invariants() {
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
