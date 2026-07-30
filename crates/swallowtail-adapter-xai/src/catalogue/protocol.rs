fn parse_response(bytes: &[u8]) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let models = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > MAXIMUM_MODELS {
        return Err(protocol_failure());
    }
    let source = IntegrationFamilyId::new("xai").expect("static family id is valid");
    let mut identities = BTreeSet::new();
    models
        .iter()
        .map(|model| {
            if model.get("object").and_then(Value::as_str) != Some("model") {
                return Err(protocol_failure());
            }
            let id = bounded_text(model, "id")?;
            if !identities.insert(id.to_owned()) {
                return Err(protocol_failure());
            }
            let mut observations = ModelCatalogObservations::new(source.clone());
            observations = observations
                .with_input_modalities(parse_modalities(model, "input_modalities", &source)?)
                .with_output_modalities(parse_modalities(model, "output_modalities", &source)?);
            Ok(ModelCatalogEntry::new(
                ModelId::new(id).map_err(|_| protocol_failure())?,
                ModelMetadata::default().with_catalog_observations(observations),
            )
            .with_provider_id(ProviderId::new("xai").expect("static provider identity is valid")))
        })
        .collect()
}

fn parse_modalities(
    value: &Value,
    field: &str,
    source: &IntegrationFamilyId,
) -> Result<Vec<CatalogObservation<ModelModality>>, RuntimeFailure> {
    let values = value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if values.len() > 16 {
        return Err(protocol_failure());
    }
    values
        .iter()
        .map(|value| {
            let value = value.as_str().ok_or_else(protocol_failure)?;
            bounded(value)?;
            match value {
                "text" => Ok(CatalogObservation::Known(ModelModality::Text)),
                "image" => Ok(CatalogObservation::Known(ModelModality::Image)),
                "embedding" => Ok(CatalogObservation::Known(ModelModality::Embedding)),
                other => ProviderCatalogValue::new(source.clone(), other)
                    .map(CatalogObservation::ProviderDefined)
                    .map_err(|_| protocol_failure()),
            }
        })
        .collect()
}


fn validate(
    plan: &swallowtail_core::PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference().is_none()
        || plan.endpoint_audience().as_str() != crate::XAI_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.xai.models.preflight_rejected",
            "xAI Models requires its exact public API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.xai.models.host_service_missing",
            "xAI Models requires blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
}

fn bounded_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(protocol_failure)?;
    bounded(value)?;
    Ok(value)
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

fn ensure_before_deadline(
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        Err(failure(
            "swallowtail.xai.models.deadline_elapsed",
            "xAI Models deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.protocol_invalid",
        "xAI returned an invalid bounded language-model catalogue",
    )
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.transport_failed",
        "xAI Models HTTP transport failed",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.models.cleanup_failed",
        "xAI Models credential cleanup failed",
    )
}

