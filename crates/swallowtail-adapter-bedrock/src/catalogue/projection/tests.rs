use super::*;
use crate::catalogue::failure::CatalogueFailureKind;
use crate::catalogue::sdk::classify_failure;
use aws_sdk_bedrock::operation::list_foundation_models::{
    ListFoundationModelsError, ListFoundationModelsOutput,
};
use aws_sdk_bedrock::primitives::DateTime;
use aws_sdk_bedrock::types::error::{
    AccessDeniedException, InternalServerException, ThrottlingException, ValidationException,
};
use aws_sdk_bedrock::types::{
    FoundationModelLifecycle, FoundationModelLifecycleStatus, FoundationModelSummary,
    InferenceType, ModelCustomization, ModelModality as AwsModelModality,
};
use swallowtail_core::{
    CatalogObservation, CatalogTimestamp, ModelLifecycleObservation, ModelLifecycleTransition,
    ModelModality,
};

fn summary() -> FoundationModelSummary {
    FoundationModelSummary::builder()
        .model_arn("arn:aws:bedrock:eu-west-2::foundation-model/provider.model-v1")
        .model_id("provider.model-v1")
        .model_name("Model One")
        .provider_name("Provider")
        .input_modalities(AwsModelModality::Text)
        .output_modalities(AwsModelModality::Text)
        .response_streaming_supported(true)
        .inference_types_supported(InferenceType::OnDemand)
        .customizations_supported(ModelCustomization::FineTuning)
        .model_lifecycle(
            FoundationModelLifecycle::builder()
                .status(FoundationModelLifecycleStatus::Active)
                .start_of_life_time(DateTime::from_secs_and_nanos(1_700_000_000, 42))
                .build()
                .expect("lifecycle fixture builds"),
        )
        .build()
        .expect("summary fixture builds")
}

#[test]
fn known_summary_projects_without_private_arn() {
    let output = ListFoundationModelsOutput::builder()
        .model_summaries(summary())
        .build();
    let entries = project_output(&output).expect("output projects");
    let entry = &entries[0];
    assert_eq!(entry.id().as_str(), "provider.model-v1");
    assert_eq!(entry.provider_id(), None);
    assert_eq!(entry.metadata().display_name(), Some("Model One"));
    let observations = entry
        .metadata()
        .catalog_observations()
        .expect("observations exist");
    assert_eq!(observations.source().as_str(), "amazon-bedrock");
    assert_eq!(observations.provider_display_name(), Some("Provider"));
    assert_eq!(observations.response_streaming_supported(), Some(true));
    assert!(observations.input_modalities().is_some_and(|values| {
        values.contains(&CatalogObservation::Known(ModelModality::Text))
    }));
    assert_eq!(
        observations
            .lifecycle()
            .and_then(|value| value.transition(ModelLifecycleTransition::StartOfLife)),
        Some(CatalogTimestamp::new(1_700_000_000, 42).expect("timestamp is valid"))
    );
    assert!(!format!("{entry:?}").contains("arn:aws:bedrock"));
}

#[test]
fn absent_fields_stay_unknown_and_unknown_enums_stay_namespaced() {
    let future = FoundationModelSummary::builder()
        .model_arn("private-arn")
        .model_id("future-model")
        .input_modalities(AwsModelModality::from("AUDIO"))
        .model_lifecycle(
            FoundationModelLifecycle::builder()
                .status(FoundationModelLifecycleStatus::from("END_OF_LIFE"))
                .build()
                .expect("future lifecycle builds"),
        )
        .build()
        .expect("future summary builds");
    let output = ListFoundationModelsOutput::builder()
        .model_summaries(future)
        .build();
    let entries = project_output(&output).expect("unknown values project");
    let observations = entries[0]
        .metadata()
        .catalog_observations()
        .expect("observations exist");
    assert_eq!(observations.output_modalities(), None);
    let CatalogObservation::ProviderDefined(modality) = observations
        .input_modalities()
        .and_then(|values| values.iter().next())
        .expect("modality exists")
    else {
        panic!("future modality stays provider-defined");
    };
    assert_eq!(modality.source().as_str(), "amazon-bedrock");
    assert_eq!(modality.as_str(), "AUDIO");
    assert!(matches!(
        observations.lifecycle().map(ModelLifecycleObservation::status),
        Some(CatalogObservation::ProviderDefined(value)) if value.as_str() == "END_OF_LIFE"
    ));
}

#[test]
fn response_and_observation_bounds_fail_without_truncation() {
    let output = ListFoundationModelsOutput::builder()
        .set_model_summaries(Some(vec![summary(); MAX_CATALOGUE_ENTRIES + 1]))
        .build();
    assert_eq!(
        project_output(&output),
        Err(CatalogueProjectionError::TooManyEntries)
    );
    let repeated = FoundationModelSummary::builder()
        .model_arn("private-arn")
        .model_id("model")
        .set_input_modalities(Some(vec![
            AwsModelModality::Text;
            MAX_OBSERVATIONS_PER_KIND + 1
        ]))
        .build()
        .expect("repeated summary builds");
    let output = ListFoundationModelsOutput::builder()
        .model_summaries(repeated)
        .build();
    assert_eq!(
        project_output(&output),
        Err(CatalogueProjectionError::TooManyObservationValues)
    );
}

#[test]
fn generated_errors_map_without_raw_provider_messages() {
    let cases = [
        (
            ListFoundationModelsError::AccessDeniedException(
                AccessDeniedException::builder()
                    .message("raw denied")
                    .build(),
            ),
            CatalogueFailureKind::PermissionDenied,
        ),
        (
            ListFoundationModelsError::ValidationException(
                ValidationException::builder()
                    .message("raw invalid")
                    .build(),
            ),
            CatalogueFailureKind::InvalidRequest,
        ),
        (
            ListFoundationModelsError::ThrottlingException(
                ThrottlingException::builder()
                    .message("raw throttle")
                    .build(),
            ),
            CatalogueFailureKind::RateLimited,
        ),
        (
            ListFoundationModelsError::InternalServerException(
                InternalServerException::builder()
                    .message("raw server")
                    .build(),
            ),
            CatalogueFailureKind::ProviderUnavailable,
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(classify_failure(&error), expected);
        assert!(!format!("{:?}", classify_failure(&error)).contains("raw"));
    }
}
