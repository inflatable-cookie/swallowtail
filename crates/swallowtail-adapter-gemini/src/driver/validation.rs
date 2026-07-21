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
    validate_session_access_plan(plan, request.access_policy())?;
    if request.access_policy()
        != &SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read)
    {
        return Err(failure(
            "swallowtail.gemini.acp.access_policy_rejected",
            "Gemini ACP first proof requires explicit ambient read access",
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

fn validate_initialize(response: &Value) -> Result<(), RuntimeFailure> {
    let info = response.get("agentInfo").ok_or_else(malformed)?;
    if info.get("name").and_then(Value::as_str) != Some("gemini-cli")
        || info.get("version").and_then(Value::as_str) != Some(GEMINI_VERSION)
    {
        return Err(failure(
            "swallowtail.gemini.acp.agent_version_rejected",
            "Gemini ACP requires the pinned Gemini CLI version",
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

fn parse_new_session(response: &Value) -> Result<String, RuntimeFailure> {
    let session_id = response
        .get("sessionId")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    let mode = response
        .get("modes")
        .and_then(|modes| modes.get("currentModeId"))
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    if mode != "plan" {
        return Err(failure(
            "swallowtail.gemini.acp.mode_rejected",
            "Gemini CLI did not open in read-only plan mode",
        ));
    }
    Ok(session_id.to_owned())
}
