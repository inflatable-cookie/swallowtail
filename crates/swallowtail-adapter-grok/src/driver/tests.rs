use super::{EXPECTED_MODEL, validate_initialize};
use serde_json::json;
use swallowtail_core::InterfaceVersion;

fn initialize() -> serde_json::Value {
    json!({
        "protocolVersion": 1,
        "agentCapabilities": {
            "loadSession": true,
            "promptCapabilities": {"embeddedContext": true}
        },
        "authMethods": [
            {"id": "cached_token", "name": "cached_token"},
            {"id": "grok.com", "name": "Grok"}
        ],
        "_meta": {
            "defaultAuthMethodId": "cached_token",
            "agentVersion": "0.2.114",
            "modelState": {
                "currentModelId": "grok-4.5",
                "availableModels": [
                    {"modelId": "grok-4.5", "name": "Grok 4.5"}
                ]
            }
        }
    })
}

#[test]
fn exact_initialize_binds_version_access_capabilities_and_model() {
    let options = validate_initialize(
        &initialize(),
        &InterfaceVersion::new("0.2.114").expect("version"),
        EXPECTED_MODEL,
    )
    .expect("qualified initialize");
    assert_eq!(options.current_value(), "grok-4.5");
    assert_eq!(options.options().count(), 1);
}

#[test]
fn initialize_drift_fails_without_exposing_provider_payload() {
    for (field, value, code) in [
        ("agentVersion", "0.2.115", "agent_version_rejected"),
        (
            "defaultAuthMethodId",
            "grok.com",
            "cached_token_unavailable",
        ),
    ] {
        let mut response = initialize();
        response["_meta"][field] = json!(value);
        let error = validate_initialize(
            &response,
            &InterfaceVersion::new("0.2.114").expect("version"),
            EXPECTED_MODEL,
        )
        .expect_err("drift rejects");
        assert!(error.diagnostic().code().ends_with(code));
        assert!(!format!("{error:?}").contains("grok.com"));
    }

    let mut response = initialize();
    response["_meta"]["modelState"]["currentModelId"] = json!("private-unexpected-model");
    let error = validate_initialize(
        &response,
        &InterfaceVersion::new("0.2.114").expect("version"),
        EXPECTED_MODEL,
    )
    .expect_err("model drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.model_drift"
    );
    assert!(!format!("{error:?}").contains("private-unexpected-model"));
}
