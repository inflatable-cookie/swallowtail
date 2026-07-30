fn validate(
    plan: &PreflightPlan,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.driver_identity().id().as_str() != DRIVER_ID
        || plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference().is_none()
        || plan.endpoint_audience().as_str() != crate::GEMINI_MODELS_ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.gemini.models.preflight_rejected",
            "Gemini Models requires its exact Developer API-key plan",
        ));
    }
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.gemini.models.host_service_missing",
            "Gemini Models requires blocking-work, time, network, and credential services",
        ));
    }
    ensure_before_deadline(request, services)
}

fn ensure_before_deadline(
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some_and(|deadline| {
        services.time().expect("validated time service").now() >= deadline.instant()
    }) {
        Err(failure(
            "swallowtail.gemini.models.deadline_elapsed",
            "Gemini Models deadline elapsed before dispatch",
        ))
    } else {
        Ok(())
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
        Some(Value::String(value)) => {
            bounded(value)?;
            Ok(Some(value))
        }
        Some(_) => Err(protocol_failure()),
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

fn optional_bool(value: &Value, field: &str) -> Result<Option<bool>, RuntimeFailure> {
    match value.get(field) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_bool().map(Some).ok_or_else(protocol_failure),
    }
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.protocol_invalid",
        "Gemini returned an invalid bounded model catalogue",
    )
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.transport_failed",
        "Gemini Models HTTP transport failed",
    )
}

fn cleanup_failure() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.models.cleanup_failed",
        "Gemini Models credential cleanup failed",
    )
}

