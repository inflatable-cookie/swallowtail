fn validate_request(
    plan: &PreflightPlan,
    access_policy: &SessionAccessPolicy,
    deadline: Option<swallowtail_runtime::Deadline>,
    options: &swallowtail_runtime::SessionOptions,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.kimi.acp.task_service_missing",
            "Kimi ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.kimi.acp.process_service_missing",
            "Kimi ACP requires a process service",
        ),
        (
            services.credential().is_some(),
            "swallowtail.kimi.acp.credential_service_missing",
            "Kimi ACP requires a delegated credential service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.kimi.acp.resource_service_missing",
            "Kimi ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.kimi.acp.resource_io_service_missing",
            "Kimi ACP requires a working-resource I/O service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    validate_session_access_plan(plan, access_policy)?;
    require_capability(plan, Capability::WorkingResourceTextWrite)?;
    require_constraint(
        plan,
        Capability::WorkingResourceTextWrite,
        &CapabilityConstraint::WorkingResourceMaximumBytes(crate::MAXIMUM_WRITE_BYTES as u64),
    )?;
    if access_policy != &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite) {
        return Err(failure(
            "swallowtail.kimi.acp.access_policy_rejected",
            "Kimi ACP requires explicit ambient read-write access",
        ));
    }
    if deadline.is_some() {
        return Err(unsupported("session deadline"));
    }
    if !options.is_empty() {
        return Err(unsupported("session options"));
    }
    Ok(())
}

fn require_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: &CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().capabilities().any(|requirement| {
        requirement.capability() == capability
            && requirement.constraints().any(|candidate| candidate == constraint)
    }) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.acp.capability_constraint_missing",
            "Kimi ACP operation bounds were not preflight-bound",
        ))
    }
}

fn validate_bound_request(
    plan: &PreflightPlan,
    binding: &SessionResumeBinding,
    working_resource: &swallowtail_runtime::WorkingResourceRef,
    access_policy: &SessionAccessPolicy,
) -> Result<(), RuntimeFailure> {
    if binding.matches_attachment(plan, working_resource, access_policy) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.acp.session_binding_mismatch",
            "Kimi ACP session binding does not match the requested attachment",
        ))
    }
}

fn require_capability(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .capabilities()
        .any(|requirement| requirement.capability() == capability)
    {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.acp.capability_missing",
            "Kimi ACP attachment operation was not preflight-bound",
        ))
    }
}

fn validate_initialize(response: &Value) -> Result<(), RuntimeFailure> {
    let info = response.get("agentInfo").ok_or_else(malformed)?;
    if info.get("name").and_then(Value::as_str) != Some("Kimi Code CLI")
        || info.get("version").and_then(Value::as_str) != Some(KIMI_VERSION)
    {
        return Err(failure(
            "swallowtail.kimi.acp.agent_version_rejected",
            "Kimi ACP requires the pinned Kimi Code version",
        ));
    }
    let capabilities = response.get("agentCapabilities").ok_or_else(malformed)?;
    if capabilities.get("loadSession").and_then(Value::as_bool) != Some(true)
        || capabilities
            .get("sessionCapabilities")
            .and_then(|value| value.get("resume"))
            .is_none()
    {
        return Err(failure(
            "swallowtail.kimi.acp.capability_drift",
            "Kimi Code did not advertise the pinned persistent-session subset",
        ));
    }
    Ok(())
}

fn parse_session(
    response: &Value,
    model: &swallowtail_core::ModelId,
) -> Result<String, RuntimeFailure> {
    validate_session_configuration(response, model)?;
    response
        .get("sessionId")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(malformed)
}

fn validate_session_configuration(
    response: &Value,
    model: &swallowtail_core::ModelId,
) -> Result<(), RuntimeFailure> {
    let configured = response
        .get("configOptions")
        .and_then(Value::as_array)
        .and_then(|options| {
            options
                .iter()
                .find(|option| option.get("id").and_then(Value::as_str) == Some("model"))
        })
        .and_then(|option| option.get("currentValue"))
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    if configured == model.as_str() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.kimi.acp.model_mismatch",
            "Kimi Code session model does not match the preflight route",
        ))
    }
}

fn validate_turn(request: &TurnRequest) -> Result<(), RuntimeFailure> {
    if request.deadline().is_some() {
        return Err(unsupported("turn deadline"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    Ok(())
}
