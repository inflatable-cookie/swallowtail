//! Card 204 / Research 221: the exact `3.0.55` `run_result.model` shape.
//!
//! Kept separate from the broad identity corpus so neither file grows into a
//! god-file finding.

use serde_json::Value;

const IDENTITY: &str = include_str!("fixtures/cline-headless-3.0.55/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cline-headless-3.0.55/protocol.json");
const SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");

#[test]
fn run_result_model_is_the_object_shaped_request_echo() {
    // Exact `3.0.55` builds `AgentResult.model` as `{id, provider, info?}` from
    // `messageModelInfo`, itself derived from the requested `modelId`/`providerId`
    // (`sdk/packages/shared/src/agents/types.ts`,
    // `sdk/packages/agents/src/agent-runtime.ts`). It is never a bare string and it
    // is never a provider-confirmed applied model. Research 221.
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture");
    let shape = &protocol["run_result_model_shape"];
    assert_eq!(shape["id"], "string");
    assert_eq!(shape["provider"], "string");
    assert_eq!(shape["info"], "optional-model-info");
    assert_eq!(protocol["run_result_model_is_request_echo"], true);
    assert_eq!(protocol["run_start_requires_unselected_verbose"], true);

    let identity: Value = serde_json::from_str(IDENTITY).expect("identity fixture");
    assert_eq!(
        identity["identity_decision"]["run_result_model_is_object"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["run_result_model_is_request_echo"],
        true
    );
    for source in [
        "sdk/packages/shared/src/agents/types.ts",
        "sdk/packages/agents/src/agent-runtime.ts",
    ] {
        assert!(
            identity["tagged_headless_sources"][source].is_string(),
            "{source} must stay frozen behind the run_result.model shape"
        );
    }

    let run_result = SUCCESS
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("success line"))
        .find(|value| value["type"] == "run_result")
        .expect("success corpus must carry one run_result envelope");
    let model = &run_result["model"];
    assert!(
        model.is_object(),
        "run_result.model is an object at 3.0.55, not a bare string"
    );
    assert!(model["id"].is_string(), "run_result.model.id is a string");
    assert!(
        model["provider"].is_string(),
        "run_result.model.provider is a string"
    );

    // The echo is of the same requested identity the verbose-only `run_start`
    // would have reported, so it adds no applied-model evidence.
    let run_start = SUCCESS
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str::<Value>(line).expect("success line"))
        .find(|value| value["type"] == "run_start")
        .expect("success corpus must carry one run_start envelope");
    assert_eq!(model["id"], run_start["modelId"]);
    assert_eq!(model["provider"], run_start["providerId"]);
}
