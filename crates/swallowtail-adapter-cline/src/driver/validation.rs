fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.cline.acp.task_service_missing",
            "Cline ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.cline.acp.process_service_missing",
            "Cline ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.cline.acp.resource_service_missing",
            "Cline ACP requires a working-resource service",
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
            "swallowtail.cline.acp.access_policy_rejected",
            "Cline ACP requires exact ambient working-resource access",
        ));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free session"));
    }
    if request.deadline().is_some() {
        return Err(unsupported("session deadline"));
    }
    if request.options().developer_instructions().is_some()
        || request.options().reasoning_mode().is_some()
        || request.options().tools().len() != 0
        || request.options().harness_mode().is_some()
    {
        return Err(unsupported("session options other than the first prompt"));
    }
    Ok(())
}

fn validate_initialize(
    response: &Value,
    selected_version: &swallowtail_core::InterfaceVersion,
) -> Result<(), RuntimeFailure> {
    let info = response.get("agentInfo").ok_or_else(malformed)?;
    if info.get("name").and_then(Value::as_str) != Some("cline")
        || info.get("version").and_then(Value::as_str) != Some(selected_version.as_str())
    {
        return Err(failure(
            "swallowtail.cline.acp.agent_version_rejected",
            "Cline ACP identity does not match the preflight version",
        ));
    }
    Ok(())
}

fn parse_new_session(response: Value) -> Result<String, RuntimeFailure> {
    response
        .get("sessionId")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(malformed)
}

fn session_resource_access(plan: &PreflightPlan) -> Result<ResourceAccess, RuntimeFailure> {
    plan.requirements()
        .session_access_policy()
        .and_then(SessionAccessPolicy::resource_access)
        .ok_or_else(|| {
            failure(
                "swallowtail.cline.acp.resource_access_missing",
                "Cline ACP requires explicit working-resource access",
            )
        })
}
