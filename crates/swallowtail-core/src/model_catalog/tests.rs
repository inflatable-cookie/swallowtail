use super::*;
use crate::IntegrationFamilyId;
use std::collections::BTreeSet;

fn source() -> IntegrationFamilyId {
    IntegrationFamilyId::new("fixture-catalogue").expect("source is valid")
}

#[test]
fn absent_observations_stay_unknown_and_false_stays_observed() {
    let unknown = ModelCatalogObservations::new(source());
    assert_eq!(unknown.input_modalities(), None);
    assert_eq!(unknown.response_streaming_supported(), None);

    let observed = ModelCatalogObservations::new(source())
        .with_input_modalities([])
        .with_response_streaming_supported(false);
    assert!(observed.input_modalities().is_some_and(BTreeSet::is_empty));
    assert_eq!(observed.response_streaming_supported(), Some(false));
}

#[test]
fn provider_values_are_namespaced_and_bounded() {
    let value =
        ProviderCatalogValue::new(source(), "FUTURE_MODALITY").expect("provider value is valid");
    assert_eq!(value.source().as_str(), "fixture-catalogue");
    assert_eq!(value.as_str(), "FUTURE_MODALITY");
    assert!(ProviderCatalogValue::new(source(), " ").is_err());
    assert!(ProviderCatalogValue::new(source(), " FUTURE_MODALITY").is_err());
    assert!(ProviderCatalogValue::new(source(), "bad\nvalue").is_err());
    assert!(ProviderCatalogValue::new(source(), "x".repeat(129)).is_err());
}

#[test]
fn lifecycle_timestamps_remain_source_scoped_metadata() {
    let timestamp = CatalogTimestamp::new(1_700_000_000, 42).expect("timestamp is valid");
    let observations = ModelCatalogObservations::new(source()).with_lifecycle(
        ModelLifecycleObservation::new(CatalogObservation::Known(ModelLifecycleStatus::Legacy))
            .with_transition(ModelLifecycleTransition::Legacy, timestamp),
    );

    assert_eq!(observations.source().as_str(), "fixture-catalogue");
    assert_eq!(
        observations
            .lifecycle()
            .and_then(|lifecycle| lifecycle.transition(ModelLifecycleTransition::Legacy)),
        Some(timestamp)
    );
    assert!(CatalogTimestamp::new(0, 1_000_000_000).is_err());
}
