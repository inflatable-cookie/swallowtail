use super::CatalogueProjectionError;
use aws_sdk_bedrock::primitives::DateTime;
use aws_sdk_bedrock::types::{
    FoundationModelLifecycle, FoundationModelLifecycleStatus, InferenceType, ModelCustomization,
    ModelModality as AwsModelModality,
};
use swallowtail_core::{
    CatalogObservation, CatalogTimestamp, IntegrationFamilyId, ModelCustomizationType,
    ModelInferenceType, ModelLifecycleObservation, ModelLifecycleStatus, ModelLifecycleTransition,
    ModelModality, ProviderCatalogValue,
};

pub(super) fn project_modality(
    modality: &AwsModelModality,
) -> Result<CatalogObservation<ModelModality>, CatalogueProjectionError> {
    match modality {
        AwsModelModality::Text => Ok(CatalogObservation::Known(ModelModality::Text)),
        AwsModelModality::Image => Ok(CatalogObservation::Known(ModelModality::Image)),
        AwsModelModality::Embedding => Ok(CatalogObservation::Known(ModelModality::Embedding)),
        value => provider_observation(value.as_str()),
    }
}

pub(super) fn project_inference_type(
    inference_type: &InferenceType,
) -> Result<CatalogObservation<ModelInferenceType>, CatalogueProjectionError> {
    match inference_type {
        InferenceType::OnDemand => Ok(CatalogObservation::Known(ModelInferenceType::OnDemand)),
        InferenceType::Provisioned => {
            Ok(CatalogObservation::Known(ModelInferenceType::Provisioned))
        }
        value => provider_observation(value.as_str()),
    }
}

pub(super) fn project_customization(
    customization: &ModelCustomization,
) -> Result<CatalogObservation<ModelCustomizationType>, CatalogueProjectionError> {
    match customization {
        ModelCustomization::FineTuning => Ok(CatalogObservation::Known(
            ModelCustomizationType::FineTuning,
        )),
        ModelCustomization::ContinuedPreTraining => Ok(CatalogObservation::Known(
            ModelCustomizationType::ContinuedPreTraining,
        )),
        ModelCustomization::Distillation => Ok(CatalogObservation::Known(
            ModelCustomizationType::Distillation,
        )),
        value => provider_observation(value.as_str()),
    }
}

pub(super) fn project_lifecycle(
    lifecycle: &FoundationModelLifecycle,
) -> Result<ModelLifecycleObservation, CatalogueProjectionError> {
    let status = match lifecycle.status() {
        FoundationModelLifecycleStatus::Active => {
            CatalogObservation::Known(ModelLifecycleStatus::Active)
        }
        FoundationModelLifecycleStatus::Legacy => {
            CatalogObservation::Known(ModelLifecycleStatus::Legacy)
        }
        value => provider_observation(value.as_str())?,
    };
    let mut observation = ModelLifecycleObservation::new(status);
    observation = with_transition(
        observation,
        ModelLifecycleTransition::StartOfLife,
        lifecycle.start_of_life_time(),
    )?;
    observation = with_transition(
        observation,
        ModelLifecycleTransition::Legacy,
        lifecycle.legacy_time(),
    )?;
    observation = with_transition(
        observation,
        ModelLifecycleTransition::PublicExtendedAccess,
        lifecycle.public_extended_access_time(),
    )?;
    with_transition(
        observation,
        ModelLifecycleTransition::EndOfLife,
        lifecycle.end_of_life_time(),
    )
}

fn with_transition(
    observation: ModelLifecycleObservation,
    transition: ModelLifecycleTransition,
    timestamp: Option<&DateTime>,
) -> Result<ModelLifecycleObservation, CatalogueProjectionError> {
    let Some(timestamp) = timestamp else {
        return Ok(observation);
    };
    let timestamp = CatalogTimestamp::new(timestamp.secs(), timestamp.subsec_nanos())
        .map_err(|_| CatalogueProjectionError::InvalidProviderObservation)?;
    Ok(observation.with_transition(transition, timestamp))
}

fn provider_observation<T>(value: &str) -> Result<CatalogObservation<T>, CatalogueProjectionError> {
    ProviderCatalogValue::new(source(), value)
        .map(CatalogObservation::ProviderDefined)
        .map_err(|_| CatalogueProjectionError::InvalidProviderObservation)
}

pub(super) fn source() -> IntegrationFamilyId {
    IntegrationFamilyId::new("amazon-bedrock").expect("Bedrock family id is valid")
}
