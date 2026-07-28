use crate::failure::failure;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{
    IntegrationFamilyId, ModelCatalogEntry, ModelCatalogObservations, ModelId, ModelMetadata,
    ModelTokenLimits, ProviderId,
};
use swallowtail_runtime::RuntimeFailure;

use super::protocol::{RestReply, decode_rest};

const MAXIMUM_MODELS: usize = 512;
const MAXIMUM_CAPABILITIES: usize = 32;
const MAXIMUM_TEXT_BYTES: usize = 256;

pub(super) fn decode_catalogue(
    status: u16,
    bytes: &[u8],
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(status, bytes)? else {
        return Err(failure(
            "swallowtail.kimi.local_server.catalogue_rejected",
            "Kimi local server rejected model catalogue discovery",
        ));
    };
    let items = data
        .as_object()
        .and_then(|data| data.get("items"))
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if items.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let mut identities = BTreeSet::new();
    items
        .iter()
        .map(|item| parse_model(item, &mut identities))
        .collect()
}

fn parse_model(
    value: &Value,
    identities: &mut BTreeSet<String>,
) -> Result<ModelCatalogEntry, RuntimeFailure> {
    let model = bounded_text(value, "model")?;
    if !identities.insert(model.to_owned()) {
        return Err(protocol_failure());
    }
    let provider = bounded_text(value, "provider")?;
    let maximum_input = required_positive_u64(value, "max_context_size")?;
    let maximum_output = optional_positive_u64(value, "max_output_size")?;
    let mut metadata = match optional_bounded_text(value, "display_name")? {
        Some(display_name) => {
            ModelMetadata::with_display_name(display_name).map_err(|_| protocol_failure())?
        }
        None => ModelMetadata::default(),
    };
    metadata =
        metadata.with_token_limits(ModelTokenLimits::new(Some(maximum_input), maximum_output));
    if let Some(capabilities) = value.get("capabilities") {
        let capabilities = capabilities.as_array().ok_or_else(protocol_failure)?;
        if capabilities.len() > MAXIMUM_CAPABILITIES {
            return Err(protocol_failure());
        }
        let mut reasoning = false;
        for capability in capabilities {
            let capability = capability.as_str().ok_or_else(protocol_failure)?;
            bounded(capability)?;
            reasoning |= capability == "thinking";
        }
        metadata = metadata.with_catalog_observations(
            ModelCatalogObservations::new(source()).with_reasoning_supported(reasoning),
        );
    }
    Ok(ModelCatalogEntry::new(
        ModelId::new(model).map_err(|_| protocol_failure())?,
        metadata,
    )
    .with_provider_id(ProviderId::new(provider).map_err(|_| protocol_failure())?))
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
        Err(protocol_failure())
    } else {
        Ok(())
    }
}

fn required_positive_u64(value: &Value, field: &str) -> Result<u64, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .filter(|value| *value > 0)
        .ok_or_else(protocol_failure)
}

fn optional_positive_u64(value: &Value, field: &str) -> Result<Option<u64>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .filter(|value| *value > 0)
            .map(Some)
            .ok_or_else(protocol_failure),
    }
}

fn source() -> IntegrationFamilyId {
    IntegrationFamilyId::new("kimi-code").expect("static Kimi family is valid")
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.catalogue_invalid",
        "Kimi local server returned an invalid bounded model catalogue",
    )
}

#[cfg(test)]
mod tests {
    use super::decode_catalogue;

    #[test]
    fn configured_aliases_are_projected_without_provider_configuration() {
        let entries = decode_catalogue(
            200,
            br#"{"code":0,"msg":"success","data":{"items":[{"provider":"kimi","model":"k2","display_name":"Kimi K2","max_context_size":131072,"max_output_size":8192,"capabilities":["thinking"]},{"provider":"openai","model":"gpt4o","display_name":"GPT-4o","max_context_size":128000}]},"request_id":"fixture"}"#,
        )
        .expect("catalogue parses");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id().as_str(), "k2");
        assert_eq!(
            entries[0].provider_id().map(|provider| provider.as_str()),
            Some("kimi")
        );
        assert_eq!(
            entries[0]
                .metadata()
                .catalog_observations()
                .and_then(|observations| observations.reasoning_supported()),
            Some(true)
        );
        assert_eq!(
            entries[0]
                .metadata()
                .token_limits()
                .and_then(|limits| limits.maximum_output_tokens()),
            Some(8192)
        );
    }

    #[test]
    fn later_catalogue_fixture_keeps_the_selected_read_shape() {
        let entries = decode_catalogue(
            200,
            include_bytes!(concat!(
                "../../tests/fixtures/kimi-code-0.29.1-0.29.2/",
                "model-catalogue.json"
            )),
        )
        .expect("later catalogue parses");
        assert_eq!(entries.len(), 2);
        assert!(
            entries
                .iter()
                .all(|entry| entry.id().as_str() != "secondary")
        );
    }

    #[test]
    fn duplicate_aliases_and_unbounded_shapes_reject() {
        for body in [
            br#"{"code":0,"msg":"success","data":{"items":[{"provider":"a","model":"same","max_context_size":1},{"provider":"b","model":"same","max_context_size":1}]},"request_id":"fixture"}"#.as_slice(),
            br#"{"code":0,"msg":"success","data":{"items":[{"provider":"a","model":"bad\nmodel","max_context_size":1}]},"request_id":"fixture"}"#.as_slice(),
            br#"{"code":0,"msg":"success","data":{"items":[{"provider":"a","model":"model","max_context_size":0}]},"request_id":"fixture"}"#.as_slice(),
        ] {
            assert!(decode_catalogue(200, body).is_err());
        }
    }
}
