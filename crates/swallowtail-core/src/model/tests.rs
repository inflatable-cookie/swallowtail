use super::{
    ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, ProviderId, ReasoningMetadata,
    ReasoningMode,
};
use crate::{IntegrationFamilyId, ModelCatalogObservations};

#[test]
fn stable_identity_is_separate_from_display_metadata() {
    let id = ModelId::new("provider/model-1").expect("model id is valid");
    let entry = ModelCatalogEntry::new(
        id.clone(),
        ModelMetadata::with_display_name("Model One")
            .expect("display name is valid")
            .with_description("A model")
            .expect("description is valid")
            .with_default(true),
    );

    assert_eq!(entry.id(), &id);
    assert_eq!(entry.metadata().display_name(), Some("Model One"));
    assert_eq!(entry.metadata().description(), Some("A model"));
    assert!(entry.metadata().is_default());
}

#[test]
fn harness_provider_and_model_ids_remain_separate() {
    let provider = ProviderId::new("anthropic").expect("provider id is valid");
    let model = ModelId::new("claude-sonnet").expect("model id is valid");
    let entry = ModelCatalogEntry::new(model.clone(), ModelMetadata::default())
        .with_provider_id(provider.clone());

    assert_eq!(entry.provider_id(), Some(&provider));
    assert_eq!(entry.id(), &model);
}

#[test]
fn reasoning_metadata_is_evidence_not_an_implicit_selection() {
    let low = ReasoningMode::new("low").expect("mode is valid");
    let high = ReasoningMode::new("high").expect("mode is valid");
    let metadata = ModelMetadata::default().with_reasoning(ReasoningMetadata::new(
        [low.clone(), high],
        Some(low.clone()),
    ));

    let reasoning = metadata.reasoning().expect("reasoning evidence is present");
    assert!(reasoning.supports(&low));
    assert_eq!(reasoning.default_mode(), Some(&low));
}

#[test]
fn absent_token_limits_are_unknown_and_observed_limits_are_mutable_metadata() {
    let unknown = ModelMetadata::default();
    assert_eq!(unknown.token_limits(), None);

    let observed = unknown.with_token_limits(ModelTokenLimits::new(Some(200_000), Some(8_192)));
    assert_eq!(
        observed
            .token_limits()
            .and_then(|limits| limits.maximum_input_tokens()),
        Some(200_000)
    );
    assert_eq!(
        observed
            .token_limits()
            .and_then(|limits| limits.maximum_output_tokens()),
        Some(8_192)
    );
}

#[test]
fn catalogue_observations_do_not_change_model_or_provider_identity() {
    let observations = ModelCatalogObservations::new(
        IntegrationFamilyId::new("gateway").expect("source is valid"),
    )
    .with_provider_display_name("Underlying Provider")
    .expect("provider display name is valid");
    let entry = ModelCatalogEntry::new(
        ModelId::new("model-1").expect("model id is valid"),
        ModelMetadata::default().with_catalog_observations(observations),
    );

    assert_eq!(entry.provider_id(), None);
    assert_eq!(
        entry
            .metadata()
            .catalog_observations()
            .and_then(ModelCatalogObservations::provider_display_name),
        Some("Underlying Provider")
    );
}
