use crate::connection::CommandResult;
use crate::failure::failure;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{
    CatalogObservation, IntegrationFamilyId, ModelCatalogEntry, ModelCatalogObservations, ModelId,
    ModelMetadata, ModelModality, ModelTokenLimits, ProviderCatalogValue, ProviderId,
};
use swallowtail_runtime::RuntimeFailure;

const MAXIMUM_MODELS: usize = 512;
const MAXIMUM_MODALITIES: usize = 16;
const MAXIMUM_TEXT_BYTES: usize = 256;

pub(crate) fn parse_catalogue(
    response: CommandResult,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    if !response.success {
        return Err(failure(
            "swallowtail.oh_my_pi.rpc.catalogue_rejected",
            "OhMyPi RPC rejected model catalogue discovery",
        ));
    }
    let models = response
        .data
        .as_ref()
        .and_then(|data| data.get("models"))
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let mut identities = BTreeSet::new();
    models
        .iter()
        .map(|model| parse_model(model, &mut identities))
        .collect()
}

fn parse_model(
    model: &Value,
    identities: &mut BTreeSet<(String, String)>,
) -> Result<ModelCatalogEntry, RuntimeFailure> {
    let id = bounded_text(model, "id")?;
    let provider = bounded_text(model, "provider")?;
    if !identities.insert((provider.to_owned(), id.to_owned())) {
        return Err(protocol_failure());
    }
    let id = ModelId::new(id).map_err(|_| protocol_failure())?;
    let provider = ProviderId::new(provider).map_err(|_| protocol_failure())?;
    let mut metadata = match optional_bounded_text(model, "name")? {
        Some(name) => ModelMetadata::with_display_name(name).map_err(|_| protocol_failure())?,
        None => ModelMetadata::default(),
    };
    let maximum_input = optional_u64(model, "contextWindow")?;
    let maximum_output = optional_u64(model, "maxTokens")?;
    if maximum_input.is_some() || maximum_output.is_some() {
        metadata = metadata.with_token_limits(ModelTokenLimits::new(maximum_input, maximum_output));
    }
    let mut observations = ModelCatalogObservations::new(source());
    let mut has_observations = false;
    if let Some(input) = model.get("input") {
        let modalities = input.as_array().ok_or_else(protocol_failure)?;
        if modalities.len() > MAXIMUM_MODALITIES {
            return Err(protocol_failure());
        }
        let source = source();
        let modalities = modalities
            .iter()
            .map(|modality| parse_modality(modality, &source))
            .collect::<Result<Vec<_>, _>>()?;
        observations = observations.with_input_modalities(modalities);
        has_observations = true;
    }
    if let Some(supported) = optional_bool(model, "reasoning")? {
        observations = observations.with_reasoning_supported(supported);
        has_observations = true;
    }
    if has_observations {
        metadata = metadata.with_catalog_observations(observations);
    }
    Ok(ModelCatalogEntry::new(id, metadata).with_provider_id(provider))
}

fn parse_modality(
    value: &Value,
    source: &IntegrationFamilyId,
) -> Result<CatalogObservation<ModelModality>, RuntimeFailure> {
    let value = value.as_str().ok_or_else(protocol_failure)?;
    bounded(value)?;
    match value {
        "text" => Ok(CatalogObservation::Known(ModelModality::Text)),
        "image" => Ok(CatalogObservation::Known(ModelModality::Image)),
        other => ProviderCatalogValue::new(source.clone(), other)
            .map(CatalogObservation::ProviderDefined)
            .map_err(|_| protocol_failure()),
    }
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(protocol_failure)?;
    bounded(value)?;
    Ok(value)
}

fn optional_bounded_text<'a>(
    value: &'a Value,
    field: &str,
) -> Result<Option<&'a str>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => {
            let value = value.as_str().ok_or_else(protocol_failure)?;
            bounded(value)?;
            Ok(Some(value))
        }
    }
}

fn bounded(value: &str) -> Result<(), RuntimeFailure> {
    if value.is_empty()
        || value.len() > MAXIMUM_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(protocol_failure());
    }
    Ok(())
}

fn optional_u64(value: &Value, field: &str) -> Result<Option<u64>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .filter(|value| *value > 0)
            .map(Some)
            .ok_or_else(protocol_failure),
    }
}

fn optional_bool(value: &Value, field: &str) -> Result<Option<bool>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_bool().map(Some).ok_or_else(protocol_failure),
    }
}

fn source() -> IntegrationFamilyId {
    IntegrationFamilyId::new("oh-my-pi").expect("static OhMyPi family is valid")
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.catalogue_invalid",
        "OhMyPi RPC returned an invalid bounded model catalogue",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_catalogue;
    use crate::connection::CommandResult;
    use serde_json::json;

    #[test]
    fn parses_configured_provider_models_without_exposing_raw_route_data() {
        let entries = parse_catalogue(CommandResult {
            success: true,
            data: Some(json!({
                "models": [{
                    "id": "fixture-model",
                    "name": "Fixture Model",
                    "api": "anthropic-messages",
                    "provider": "fixture-provider",
                    "baseUrl": "https://private.invalid",
                    "reasoning": true,
                    "input": ["text", "image", "audio"],
                    "contextWindow": 200000,
                    "maxTokens": 8192,
                    "cost": {"input": 1.0}
                }]
            })),
        })
        .expect("catalogue parses");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].id().as_str(), "fixture-model");
        assert_eq!(
            entries[0].provider_id().map(|provider| provider.as_str()),
            Some("fixture-provider")
        );
        assert_eq!(entries[0].metadata().display_name(), Some("Fixture Model"));
        assert_eq!(
            entries[0]
                .metadata()
                .token_limits()
                .and_then(|limits| limits.maximum_input_tokens()),
            Some(200_000)
        );
        assert_eq!(
            entries[0]
                .metadata()
                .catalog_observations()
                .and_then(|observations| observations.reasoning_supported()),
            Some(true)
        );
        assert!(!format!("{entries:?}").contains("private.invalid"));
    }

    #[test]
    fn rejects_duplicate_or_unbounded_catalogue_evidence() {
        for models in [
            json!([
                {"id": "same", "provider": "provider"},
                {"id": "same", "provider": "provider"}
            ]),
            json!([{"id": "model", "provider": "provider", "input": [1]}]),
        ] {
            let error = parse_catalogue(CommandResult {
                success: true,
                data: Some(json!({"models": models})),
            })
            .expect_err("invalid catalogue is rejected");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.oh_my_pi.rpc.catalogue_invalid"
            );
        }
        let error = parse_catalogue(CommandResult {
            success: true,
            data: Some(json!({
                "models": (0..=super::MAXIMUM_MODELS)
                    .map(|index| json!({
                        "id": format!("model-{index}"),
                        "provider": "provider"
                    }))
                    .collect::<Vec<_>>()
            })),
        })
        .expect_err("overflow is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.oh_my_pi.rpc.catalogue_invalid"
        );
    }
}
