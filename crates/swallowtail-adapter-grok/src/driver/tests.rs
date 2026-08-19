use super::validate_initialize;
use crate::selection::{GROK_BUILD_MODEL_4_5, GROK_BUILD_MODEL_4_6};
use serde_json::json;
use swallowtail_core::InterfaceVersion;

fn initialize(version: &str, model: &str) -> serde_json::Value {
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
            "agentVersion": version,
            "modelState": {
                "currentModelId": model,
                "availableModels": [
                    {"modelId": model, "name": model}
                ]
            }
        }
    })
}

#[test]
fn exact_initialize_binds_0_2_and_1_0_behavior_segments() {
    for (version, model) in [
        ("0.2.114", GROK_BUILD_MODEL_4_5),
        ("0.2.117", GROK_BUILD_MODEL_4_5),
        ("1.0.4", GROK_BUILD_MODEL_4_6),
        ("1.0.5", GROK_BUILD_MODEL_4_6),
    ] {
        let options = validate_initialize(
            &initialize(version, model),
            &InterfaceVersion::new(version).expect("version"),
            model,
        )
        .expect("qualified initialize");
        assert_eq!(options.current_value(), model);
        assert_eq!(options.options().count(), 1);
    }
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
        let mut response = initialize("0.2.114", GROK_BUILD_MODEL_4_5);
        response["_meta"][field] = json!(value);
        let error = validate_initialize(
            &response,
            &InterfaceVersion::new("0.2.114").expect("version"),
            GROK_BUILD_MODEL_4_5,
        )
        .expect_err("drift rejects");
        assert!(error.diagnostic().code().ends_with(code));
        assert!(!format!("{error:?}").contains("grok.com"));
    }

    let mut response = initialize("0.2.114", GROK_BUILD_MODEL_4_5);
    response["_meta"]["modelState"]["currentModelId"] = json!("private-unexpected-model");
    let error = validate_initialize(
        &response,
        &InterfaceVersion::new("0.2.114").expect("version"),
        GROK_BUILD_MODEL_4_5,
    )
    .expect_err("model drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.model_drift"
    );
    assert!(!format!("{error:?}").contains("private-unexpected-model"));
}
