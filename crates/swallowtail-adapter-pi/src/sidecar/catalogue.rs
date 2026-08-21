use super::failure::failure;
use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_core::{ModelCatalogEntry, ModelId, ModelMetadata, ProviderId};
use swallowtail_runtime::RuntimeFailure;

const MAXIMUM_MODELS: usize = 256;
const MAXIMUM_TEXT_BYTES: usize = 256;

pub(crate) fn parse_catalogue(
    data: Option<&Value>,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let models = data
        .and_then(|data| data.get("models"))
        .and_then(Value::as_array)
        .ok_or_else(catalogue_invalid)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(catalogue_invalid());
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
        return Err(catalogue_invalid());
    }
    let id = ModelId::new(id).map_err(|_| catalogue_invalid())?;
    let provider = ProviderId::new(provider).map_err(|_| catalogue_invalid())?;
    Ok(ModelCatalogEntry::new(id, ModelMetadata::default()).with_provider_id(provider))
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(catalogue_invalid)?;
    if value.is_empty()
        || value.len() > MAXIMUM_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(catalogue_invalid());
    }
    Ok(value)
}

fn catalogue_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.catalogue_invalid",
        "Pi SDK sidecar returned an invalid bounded model catalogue",
    )
}

#[cfg(test)]
mod tests {
    use super::parse_catalogue;
    use serde_json::json;

    #[test]
    fn parses_exact_bounded_provider_model_identities() {
        let entries = parse_catalogue(Some(&json!({
            "models": [
                {"provider": "fixture-provider", "id": "fixture-model"},
                {"provider": "fixture-provider", "id": "fixture-text-model"}
            ]
        })))
        .expect("catalogue parses");
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id().as_str(), "fixture-model");
        assert_eq!(
            entries[0].provider_id().map(|provider| provider.as_str()),
            Some("fixture-provider")
        );
    }

    #[test]
    fn rejects_duplicate_unbounded_or_overflowing_catalogue_evidence() {
        for data in [
            json!({"models": [
                {"provider": "p", "id": "same"},
                {"provider": "p", "id": "same"}
            ]}),
            json!({"models": [{"provider": "p", "id": ""}]}),
            json!({"models": [{"provider": "p"}]}),
            json!({"models": "invalid"}),
            json!({}),
        ] {
            let error = parse_catalogue(Some(&data)).expect_err("invalid catalogue is rejected");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.pi.sdk-sidecar.catalogue_invalid"
            );
        }
        let overflow = json!({
            "models": (0..=super::MAXIMUM_MODELS)
                .map(|index| json!({"provider": "p", "id": format!("model-{index}")}))
                .collect::<Vec<_>>()
        });
        let error = parse_catalogue(Some(&overflow)).expect_err("overflow is rejected");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.pi.sdk-sidecar.catalogue_invalid"
        );
        assert!(parse_catalogue(None).is_err());
    }
}
