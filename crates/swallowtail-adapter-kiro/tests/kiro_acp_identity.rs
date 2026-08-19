use serde_json::Value;
use swallowtail_adapter_kiro::{
    KIRO_CLI_RELEASE_VERSION, kiro_acp_claim, kiro_cli_release_binding,
};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};

const IDENTITY: &str = include_str!("fixtures/kiro-acp-2.18.1/identity.json");
const PROTOCOL: &str = include_str!("fixtures/kiro-acp-2.18.1/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/kiro-acp-2.18.1/initialize.json");
const SESSION_NEW: &str = include_str!("fixtures/kiro-acp-2.18.1/session-new.json");
const SESSION_PROMPT: &str = include_str!("fixtures/kiro-acp-2.18.1/session-prompt.json");
const SESSION_CANCEL: &str = include_str!("fixtures/kiro-acp-2.18.1/session-cancel.json");
const PERMISSION: &str = include_str!("fixtures/kiro-acp-2.18.1/permission.json");
const NEGATIVE: &str = include_str!("fixtures/kiro-acp-2.18.1/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/kiro-acp-2.18.1/corpus-plan.json");

#[test]
fn frozen_identity_keeps_acp_separate_from_headless_cloud_and_content_field() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "kiro-cli.release");
    assert_eq!(identity["route"], "kiro.acp");
    assert_eq!(identity["official"]["version"], KIRO_CLI_RELEASE_VERSION);
    assert_eq!(
        identity["identity_decision"]["flatten_onto_kiro_headless"],
        false
    );
    assert_eq!(identity["identity_decision"]["flatten_onto_cloud"], false);
    assert_eq!(identity["identity_decision"]["pass_agent_flag"], false);
    assert_eq!(identity["identity_decision"]["run_kiro_login"], false);
    assert_eq!(
        identity["identity_decision"]["bind_kiro_api_key_lease"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_session_load"], false);
    assert_eq!(
        identity["identity_decision"]["send_prompt_field_content"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["kiro-cli", "acp"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["kiro-cli", "acp"])
    );
    assert_eq!(protocol["prompt"]["field"], "prompt");
    assert_eq!(
        protocol["prompt"]["docs_example_field_content_rejected"],
        true
    );
    assert_eq!(protocol["authority"]["swallowtail_passes_cloud"], false);
    assert_eq!(
        protocol["authority"]["swallowtail_passes_agent_flag"],
        false
    );
    assert_eq!(protocol["permission"]["allow_always_unselected"], true);

    let claim = kiro_acp_claim();
    let version = InterfaceVersion::new(KIRO_CLI_RELEASE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(kiro_cli_release_binding(KIRO_CLI_RELEASE_VERSION).is_some());
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
        initialize["official_example"]["agentInfo"]["name"],
        "kiro-cli"
    );
    assert_eq!(
        initialize["official_example"]["agentInfo"]["version"],
        "1.5.0"
    );
    assert_eq!(initialize["response_recovered_from_public_source"], false);
    assert_eq!(
        initialize["request"]["params"]["clientCapabilities"]["fs"]["readTextFile"],
        false
    );
    let prompt: Value = serde_json::from_str(SESSION_PROMPT).expect("session prompt");
    assert_eq!(prompt["selected_field"], "prompt");
    assert!(prompt["request"]["params"].get("prompt").is_some());
    assert!(prompt["request"]["params"].get("content").is_none());
    let permission: Value = serde_json::from_str(PERMISSION).expect("permission");
    assert_eq!(
        permission["swallowtail_default"],
        "do not auto-select allow_always or map --trust-all-tools onto ACP"
    );
    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "docs-content-field-rejected",
        "headless-is-not-stdio-acp",
        "cloud-unmapped",
        "agent-flag-unmapped",
        "trust-all-tools-not-swallowtail-authority",
        "session-load-unmapped",
        "kiro-dev-extensions-unmapped",
        "login-not-swallowtail",
        "malformed-json",
        "oversized-frame",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_kiro_run_invariants() {
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
