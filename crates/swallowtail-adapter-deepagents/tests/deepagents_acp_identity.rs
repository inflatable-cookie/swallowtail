use serde_json::Value;
use swallowtail_adapter_deepagents::{
    DEEPAGENTS_ACP_PACKAGE_VERSION, deepagents_acp_claim, deepagents_acp_package_binding,
};
use swallowtail_core::InterfaceVersion;
use swallowtail_testkit::{ConformanceAssertion, run_acp_single_turn_projection_assertions};

const IDENTITY: &str = include_str!("fixtures/deepagents-acp-0.1.25/identity.json");
const PROTOCOL: &str = include_str!("fixtures/deepagents-acp-0.1.25/protocol.json");
const INITIALIZE: &str = include_str!("fixtures/deepagents-acp-0.1.25/initialize.json");
const SESSION_NEW: &str = include_str!("fixtures/deepagents-acp-0.1.25/session-new.json");
const SESSION_PROMPT: &str = include_str!("fixtures/deepagents-acp-0.1.25/session-prompt.json");
const SESSION_CANCEL: &str = include_str!("fixtures/deepagents-acp-0.1.25/session-cancel.json");
const PERMISSION: &str = include_str!("fixtures/deepagents-acp-0.1.25/permission.json");
const NEGATIVE: &str = include_str!("fixtures/deepagents-acp-0.1.25/negative-cases.json");
const CORPUS_PLAN: &str = include_str!("fixtures/deepagents-acp-0.1.25/corpus-plan.json");

#[test]
fn frozen_identity_keeps_acp_separate_from_npx_library_and_content_field() {
    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(identity["axis"], "deepagents-acp.package");
    assert_eq!(identity["route"], "deepagents.acp");
    assert_eq!(
        identity["official"]["version"],
        DEEPAGENTS_ACP_PACKAGE_VERSION
    );
    assert_eq!(identity["identity_decision"]["wrap_npx"], false);
    assert_eq!(
        identity["identity_decision"]["wrap_library_start_server"],
        false
    );
    assert_eq!(identity["identity_decision"]["wrap_custom_tsx"], false);
    assert_eq!(identity["identity_decision"]["pass_model_flag"], false);
    assert_eq!(identity["identity_decision"]["pass_workspace_flag"], false);
    assert_eq!(
        identity["identity_decision"]["bind_anthropic_api_key_lease"],
        false
    );
    assert_eq!(identity["identity_decision"]["map_session_load"], false);
    assert_eq!(
        identity["identity_decision"]["treat_agentinfo_version_as_package"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["entrypoint"],
        serde_json::json!(["deepagents-acp"])
    );

    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    assert_eq!(
        protocol["selected_command"],
        serde_json::json!(["deepagents-acp"])
    );
    assert_eq!(protocol["prompt"]["field"], "prompt");
    assert_eq!(protocol["initialize"]["cli_agent_info"]["version"], "0.0.1");
    assert_eq!(
        protocol["initialize"]["cli_agent_info"]["version_is_constructor_default_not_npm"],
        true
    );
    assert_eq!(protocol["authority"]["swallowtail_passes_npx"], false);
    assert_eq!(
        protocol["authority"]["swallowtail_passes_model_flag"],
        false
    );
    assert_eq!(
        protocol["authority"]["swallowtail_passes_workspace_flag"],
        false
    );
    assert_eq!(protocol["permission"]["allow_always_unselected"], true);

    let claim = deepagents_acp_claim();
    let version = InterfaceVersion::new(DEEPAGENTS_ACP_PACKAGE_VERSION).expect("qualified version");
    assert!(claim.assess(&version).is_permitted());
    assert!(deepagents_acp_package_binding(DEEPAGENTS_ACP_PACKAGE_VERSION).is_some());
    assert!(deepagents_acp_package_binding("0.0.1").is_none());
    assert!(deepagents_acp_package_binding("0.1.7").is_none());
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
        initialize["cli_result_shape"]["agentInfo"]["name"],
        "deepagents-acp"
    );
    assert_eq!(
        initialize["cli_result_shape"]["agentInfo"]["version"],
        "0.0.1"
    );
    assert_eq!(initialize["response_recovered_from_public_source"], true);
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
        "observe and cancel; never select allow_always; do not inherit source allow-on-permission-error"
    );
    let negatives: Value = serde_json::from_str(NEGATIVE).expect("negatives");
    let ids: Vec<_> = negatives["cases"]
        .as_array()
        .expect("cases")
        .iter()
        .map(|case| case["id"].as_str().expect("id"))
        .collect();
    for required in [
        "content-field-rejected",
        "npx-is-not-swallowtail-spawn",
        "library-embed-unmapped",
        "custom-tsx-unmapped",
        "model-flag-unmapped",
        "workspace-flag-unmapped",
        "session-load-unmapped",
        "set-mode-unmapped",
        "slash-commands-unmapped",
        "allow-always-unselected",
        "api-key-not-swallowtail-lease",
        "registry-0-1-7-not-frozen-package",
        "malformed-json",
        "oversized-frame",
    ] {
        assert!(ids.contains(&required), "missing negative case {required}");
    }
}

#[test]
fn provider_neutral_acp_projection_assertions_cover_deepagents_run_invariants() {
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
