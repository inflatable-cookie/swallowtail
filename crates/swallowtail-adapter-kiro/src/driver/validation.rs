fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.kiro.acp.task_service_missing",
            "Kiro ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.kiro.acp.process_service_missing",
            "Kiro ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.kiro.acp.resource_service_missing",
            "Kiro ACP requires a working-resource service",
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
            "swallowtail.kiro.acp.access_policy_rejected",
            "Kiro ACP requires exact ambient working-resource access",
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
    // Initialize result fields are unrecovered from public Kiro source.
    // Do not invent agentInfo. Fail closed if a present identity drifts.
    if let Some(info) = response.get("agentInfo") {
        if !info.is_object() {
            return Err(malformed());
        }
        if let Some(name) = info.get("name")
            && name.as_str() != Some("kiro-cli")
        {
            return Err(failure(
                "swallowtail.kiro.acp.agent_version_rejected",
                "Kiro ACP identity does not match the preflight version",
            ));
        }
        if let Some(version) = info.get("version")
            && version.as_str() != Some(selected_version.as_str())
        {
            return Err(failure(
                "swallowtail.kiro.acp.agent_version_rejected",
                "Kiro ACP identity does not match the preflight version",
            ));
        }
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
                "swallowtail.kiro.acp.resource_access_missing",
                "Kiro ACP requires explicit working-resource access",
            )
        })
}
