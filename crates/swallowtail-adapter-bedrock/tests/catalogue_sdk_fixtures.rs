use aws_credential_types::Credentials;
use aws_sdk_bedrock::config::{Region, retry::RetryConfig};
use aws_sdk_bedrock::operation::list_foundation_models::{
    ListFoundationModelsInput, ListFoundationModelsOutput,
};
use aws_sdk_bedrock::types::{FoundationModelSummary, ModelModality};
use serde_json::{Value, json};
use swallowtail_adapter_bedrock::{
    CATALOGUE_SDK_CRATE, CATALOGUE_SDK_VERSION, CATALOGUE_SERVICE_API,
};

const PROTOCOL: &str = include_str!("fixtures/bedrock-control-plane-1.148.0/protocol.json");

#[test]
fn manifest_binds_control_plane_access_attempt_bounds_and_exclusions() {
    let fixture: Value = serde_json::from_str(PROTOCOL).expect("fixture manifest is valid JSON");
    assert_eq!(fixture["sdk_crate"], CATALOGUE_SDK_CRATE);
    assert_eq!(fixture["sdk_version"], CATALOGUE_SDK_VERSION);
    assert_eq!(fixture["service_api"], CATALOGUE_SERVICE_API);
    assert_eq!(fixture["endpoint_audience"], "bedrock");
    assert_eq!(fixture["authentication"]["default_chain_allowed"], false);
    assert_eq!(
        fixture["authorization"]["iam_action"],
        "bedrock:ListFoundationModels"
    );
    assert_eq!(fixture["request"]["filters"], json!([]));
    assert_eq!(fixture["request"]["pagination"], "none");
    assert_eq!(fixture["request"]["maximum_provider_requests"], 1);
    assert_eq!(fixture["request"]["maximum_sdk_attempts"], 1);
    assert_eq!(fixture["request"]["maximum_entries"], 1_024);
    assert_eq!(
        fixture["unknown_enum_policy"],
        "bounded_namespaced_observation"
    );
    assert_eq!(fixture["provider_resource_arn"], "adapter_private");
    assert!(
        fixture["excluded"]
            .as_array()
            .expect("exclusions are an array")
            .contains(&json!("bedrock_runtime_endpoint_grant"))
    );
    assert!(
        fixture["excluded"]
            .as_array()
            .expect("exclusions are an array")
            .contains(&json!("bedrock_mantle_models"))
    );
}

#[test]
fn generated_request_is_unfiltered_and_output_is_one_collection() {
    let input = ListFoundationModelsInput::builder()
        .build()
        .expect("unfiltered request builds");
    assert_eq!(input.by_provider(), None);
    assert_eq!(input.by_customization_type(), None);
    assert_eq!(input.by_output_modality(), None);
    assert_eq!(input.by_inference_type(), None);

    let summary = FoundationModelSummary::builder()
        .model_arn("private-provider-resource")
        .model_id("provider.model-v1")
        .input_modalities(ModelModality::Text)
        .build()
        .expect("summary builds");
    let output = ListFoundationModelsOutput::builder()
        .model_summaries(summary)
        .build();
    assert_eq!(output.model_summaries().len(), 1);
    assert_eq!(output.model_summaries()[0].model_id(), "provider.model-v1");
}

#[test]
fn generated_config_is_explicit_and_one_attempt_without_default_chain_loading() {
    let config = aws_sdk_bedrock::Config::builder()
        .region(Region::new("eu-west-2"))
        .endpoint_url("https://bedrock.eu-west-2.amazonaws.com")
        .credentials_provider(Credentials::new(
            "fixture-access-key",
            "fixture-secret-key",
            None,
            None,
            "fixture",
        ))
        .retry_config(RetryConfig::standard().with_max_attempts(1))
        .build();

    assert_eq!(config.region().map(Region::as_ref), Some("eu-west-2"));
    assert_eq!(
        config.retry_config().map(RetryConfig::max_attempts),
        Some(1)
    );
    assert_eq!(config.signing_name(), "bedrock");
    assert!(!format!("{config:?}").contains("fixture-secret-key"));
}
