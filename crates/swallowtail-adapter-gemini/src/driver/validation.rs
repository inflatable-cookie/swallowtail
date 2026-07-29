fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.gemini.acp.task_service_missing",
            "Gemini ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.gemini.acp.process_service_missing",
            "Gemini ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.gemini.acp.resource_service_missing",
            "Gemini ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.gemini.acp.resource_io_service_missing",
            "Gemini ACP requires a working-resource I/O service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    swallowtail_runtime::validate_session_plan_agreement(plan, request.plan_agreement())?;
    let resource_access = session_resource_access(plan)?;
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(resource_access) {
        return Err(failure(
            "swallowtail.gemini.acp.access_policy_rejected",
            "Gemini ACP requires exact ambient working-resource access",
        ));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free session"));
    }
    if request.deadline().is_some() {
        return Err(unsupported("session deadline"));
    }
    if !request.options().is_empty() {
        return Err(unsupported("session options"));
    }
    Ok(())
}

fn validate_initialize(
    response: &Value,
    selected_version: &swallowtail_core::InterfaceVersion,
) -> Result<(), RuntimeFailure> {
    let info = response.get("agentInfo").ok_or_else(malformed)?;
    if info.get("name").and_then(Value::as_str) != Some("gemini-cli")
        || info.get("version").and_then(Value::as_str) != Some(selected_version.as_str())
    {
        return Err(failure(
            "swallowtail.gemini.acp.agent_version_rejected",
            "Gemini ACP identity does not match the preflight version",
        ));
    }
    let has_api_key = response
        .get("authMethods")
        .and_then(Value::as_array)
        .is_some_and(|methods| {
            methods
                .iter()
                .any(|method| method.get("id").and_then(Value::as_str) == Some("gemini-api-key"))
        });
    if !has_api_key {
        return Err(failure(
            "swallowtail.gemini.acp.api_key_unavailable",
            "Gemini CLI did not advertise the configured API-key access method",
        ));
    }
    Ok(())
}

struct OpenedSession {
    provider_id: String,
    model_options: Option<NegotiatedSessionModelOptions>,
}

fn parse_new_session(
    response: &Value,
    resource_access: swallowtail_core::ResourceAccess,
) -> Result<OpenedSession, RuntimeFailure> {
    let session_id = response
        .get("sessionId")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    let mode = response
        .get("modes")
        .and_then(|modes| modes.get("currentModeId"))
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    let expected_mode = provider_mode_id(resource_access);
    if mode != expected_mode {
        return Err(failure(
            "swallowtail.gemini.acp.mode_rejected",
            "Gemini CLI did not open in the preflight-bound access mode",
        ));
    }
    Ok(OpenedSession {
        provider_id: session_id.to_owned(),
        model_options: parse_model_options(response)?,
    })
}

fn parse_model_options(
    response: &Value,
) -> Result<Option<NegotiatedSessionModelOptions>, RuntimeFailure> {
    let Some(models) = response.get("models") else {
        return Ok(None);
    };
    let models = models.as_object().ok_or_else(malformed)?;
    let Some(available) = models.get("availableModels") else {
        return Ok(None);
    };
    let available = available.as_array().ok_or_else(malformed)?;
    let current = models
        .get("currentModelId")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    let options = available
        .iter()
        .map(|model| {
            let model = model.as_object().ok_or_else(malformed)?;
            let value = model
                .get("modelId")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            let display_name = model
                .get("name")
                .and_then(Value::as_str)
                .map(str::to_owned);
            NegotiatedSessionModelOption::new(value, display_name)
        })
        .collect::<Result<Vec<_>, _>>()?;
    NegotiatedSessionModelOptions::new(current, options).map(Some)
}
