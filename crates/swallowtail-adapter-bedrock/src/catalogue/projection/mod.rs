mod values;

use aws_sdk_bedrock::operation::list_foundation_models::ListFoundationModelsOutput;
use aws_sdk_bedrock::types::FoundationModelSummary;
use swallowtail_core::{ModelCatalogEntry, ModelCatalogObservations, ModelId, ModelMetadata};
use values::{project_customization, project_inference_type, project_lifecycle, project_modality};

pub(super) const MAX_CATALOGUE_ENTRIES: usize = 1_024;
const MAX_MODEL_ID_BYTES: usize = 512;
const MAX_MODEL_NAME_BYTES: usize = 512;
const MAX_PROVIDER_NAME_BYTES: usize = 256;
pub(super) const MAX_OBSERVATIONS_PER_KIND: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CatalogueProjectionError {
    TooManyEntries,
    InvalidModelId,
    FieldBoundExceeded,
    TooManyObservationValues,
    InvalidProviderObservation,
}

pub(super) fn project_output(
    output: &ListFoundationModelsOutput,
) -> Result<Vec<ModelCatalogEntry>, CatalogueProjectionError> {
    let summaries = output.model_summaries();
    if summaries.len() > MAX_CATALOGUE_ENTRIES {
        return Err(CatalogueProjectionError::TooManyEntries);
    }
    summaries.iter().map(project_summary).collect()
}

fn project_summary(
    summary: &FoundationModelSummary,
) -> Result<ModelCatalogEntry, CatalogueProjectionError> {
    validate_required(summary.model_id(), MAX_MODEL_ID_BYTES)?;
    let id =
        ModelId::new(summary.model_id()).map_err(|_| CatalogueProjectionError::InvalidModelId)?;
    let mut metadata = match summary.model_name() {
        Some(name) => {
            validate_optional(name, MAX_MODEL_NAME_BYTES)?;
            ModelMetadata::with_display_name(name)
                .map_err(|_| CatalogueProjectionError::FieldBoundExceeded)?
        }
        None => ModelMetadata::default(),
    };

    let mut observations = ModelCatalogObservations::new(values::source());
    if let Some(provider_name) = summary.provider_name() {
        validate_optional(provider_name, MAX_PROVIDER_NAME_BYTES)?;
        observations = observations
            .with_provider_display_name(provider_name)
            .map_err(|_| CatalogueProjectionError::FieldBoundExceeded)?;
    }
    if let Some(modalities) = summary.input_modalities.as_ref() {
        validate_observation_count(modalities.len())?;
        observations = observations.with_input_modalities(
            modalities
                .iter()
                .map(project_modality)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    if let Some(modalities) = summary.output_modalities.as_ref() {
        validate_observation_count(modalities.len())?;
        observations = observations.with_output_modalities(
            modalities
                .iter()
                .map(project_modality)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    if let Some(supported) = summary.response_streaming_supported() {
        observations = observations.with_response_streaming_supported(supported);
    }
    if let Some(inference_types) = summary.inference_types_supported.as_ref() {
        validate_observation_count(inference_types.len())?;
        observations = observations.with_inference_types(
            inference_types
                .iter()
                .map(project_inference_type)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    if let Some(customizations) = summary.customizations_supported.as_ref() {
        validate_observation_count(customizations.len())?;
        observations = observations.with_customization_types(
            customizations
                .iter()
                .map(project_customization)
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    if let Some(lifecycle) = summary.model_lifecycle() {
        observations = observations.with_lifecycle(project_lifecycle(lifecycle)?);
    }

    metadata = metadata.with_catalog_observations(observations);
    Ok(ModelCatalogEntry::new(id, metadata))
}

fn validate_required(value: &str, maximum: usize) -> Result<(), CatalogueProjectionError> {
    if value.trim().is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return Err(CatalogueProjectionError::InvalidModelId);
    }
    if value.len() > maximum {
        return Err(CatalogueProjectionError::FieldBoundExceeded);
    }
    Ok(())
}

fn validate_optional(value: &str, maximum: usize) -> Result<(), CatalogueProjectionError> {
    if value.trim().is_empty()
        || value.trim() != value
        || value.chars().any(char::is_control)
        || value.len() > maximum
    {
        return Err(CatalogueProjectionError::FieldBoundExceeded);
    }
    Ok(())
}

fn validate_observation_count(count: usize) -> Result<(), CatalogueProjectionError> {
    if count > MAX_OBSERVATIONS_PER_KIND {
        Err(CatalogueProjectionError::TooManyObservationValues)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests;
