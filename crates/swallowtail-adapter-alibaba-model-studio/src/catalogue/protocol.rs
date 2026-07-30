struct ParsedPage {
    models: Vec<ModelCatalogEntry>,
    last_page: bool,
}

fn parse_page(bytes: &[u8], expected_page: u32) -> Result<ParsedPage, RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let output = value.get("output").ok_or_else(protocol_failure)?;
    let page = output
        .get("page_no")
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(protocol_failure)?;
    let page_size = output
        .get("page_size")
        .and_then(Value::as_u64)
        .ok_or_else(protocol_failure)?;
    let total = output
        .get("total")
        .and_then(Value::as_u64)
        .ok_or_else(protocol_failure)?;
    if page != expected_page || page_size == 0 || page_size > 100 {
        return Err(protocol_failure());
    }
    let models = output
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > 100 {
        return Err(protocol_failure());
    }
    let models = models
        .iter()
        .map(|model| {
            let name = model
                .get("model_name")
                .and_then(Value::as_str)
                .ok_or_else(protocol_failure)?;
            bounded(name)?;
            Ok(ModelCatalogEntry::new(
                ModelId::new(name).map_err(|_| protocol_failure())?,
                ModelMetadata::default(),
            )
            .with_provider_id(
                ProviderId::new("alibaba-cloud").expect("static provider identity is valid"),
            ))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    let seen = u64::from(page.saturating_sub(1))
        .saturating_mul(page_size)
        .saturating_add(models.len() as u64);
    Ok(ParsedPage {
        models,
        last_page: seen >= total,
    })
}

fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference().is_none()
        || plan.endpoint_audience().as_str() != crate::ALIBABA_DEPLOYABLE_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.models.preflight_rejected",
            "Alibaba deployable models require their exact international API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.models.host_service_missing",
            "Alibaba deployable models require blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
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
            "swallowtail.alibaba_model_studio.models.deadline_elapsed",
            "Alibaba deployable-model deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.models.protocol_invalid",
        "Alibaba returned an invalid bounded deployable-model catalogue",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.models.cleanup_failed",
        "Alibaba deployable-model credential cleanup failed",
    )
}

