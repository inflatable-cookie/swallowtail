fn validate_run_reasoning(
    plan: &PreflightPlan,
    requested: Option<&swallowtail_core::ReasoningMode>,
) -> Result<(), RuntimeFailure> {
    let requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| {
            requirement.capability() == swallowtail_core::Capability::ReasoningSelection
        })
        .collect::<Vec<_>>();
    if requested.is_none() && requirements.is_empty() {
        return Ok(());
    }
    let [requirement] = requirements.as_slice() else {
        return Err(failure(
            "swallowtail.claude_agent.acp.run_reasoning_mismatch",
            "Claude Agent run reasoning does not match its preflight plan",
        ));
    };
    let constraints = requirement.constraints().collect::<Vec<_>>();
    let [swallowtail_core::CapabilityConstraint::ReasoningMode(planned)] = constraints.as_slice()
    else {
        return Err(failure(
            "swallowtail.claude_agent.acp.run_reasoning_mismatch",
            "Claude Agent run reasoning does not match its preflight plan",
        ));
    };
    if requested == Some(planned) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.claude_agent.acp.run_reasoning_mismatch",
            "Claude Agent run reasoning does not match its preflight plan",
        ))
    }
}

pub(super) fn validate_initialize(
    response: &Value,
    selected: &ClaudeAgentPlanSelection,
) -> Result<ClaudeAgentLifecycleCapabilities, RuntimeFailure> {
    let info = response.get("agentInfo").ok_or_else(malformed)?;
    if info.get("name").and_then(Value::as_str) != Some("@agentclientprotocol/claude-agent-acp")
        || info.get("version").and_then(Value::as_str) != Some(selected.version().as_str())
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.agent_version_rejected",
            "Claude Agent ACP identity does not match the preflight version",
        ));
    }
    if response
        .get("authMethods")
        .and_then(Value::as_array)
        .is_none_or(|methods| !methods.is_empty())
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.terminal_auth_rejected",
            "Claude Agent advertised authentication outside the API-key subset",
        ));
    }
    let capabilities = response.get("agentCapabilities").ok_or_else(malformed)?;
    let providers = capabilities.get("providers").is_some();
    let steering = response
        .get("_meta")
        .and_then(|meta| meta.get("steering"))
        .and_then(|value| value.get("supported"))
        .and_then(Value::as_bool)
        == Some(true);
    match selected.behavior() {
        ClaudeAgentBehavior::Baseline | ClaudeAgentBehavior::SessionConfig
            if providers || steering =>
        {
            return Err(capability_drift());
        }
        ClaudeAgentBehavior::ProviderCapability if !providers || steering => {
            return Err(capability_drift());
        }
        ClaudeAgentBehavior::SteeringMetadata if !providers || !steering => {
            return Err(capability_drift());
        }
        _ => {}
    }
    let session = capabilities.get("sessionCapabilities");
    let lifecycle = ClaudeAgentLifecycleCapabilities {
        close: advertised(session, "close"),
        delete: advertised(session, "delete"),
        load: capabilities.get("loadSession").and_then(Value::as_bool) == Some(true),
        resume: advertised(session, "resume"),
    };
    if !lifecycle.close || !lifecycle.delete || !lifecycle.load || !lifecycle.resume {
        return Err(failure(
            "swallowtail.claude_agent.acp.lifecycle_capability_drift",
            "Claude Agent lifecycle capabilities do not match the qualified adapter behavior",
        ));
    }
    Ok(lifecycle)
}

pub(super) fn validate_attachment(
    plan: &PreflightPlan,
    binding: &swallowtail_runtime::SessionResumeBinding,
    working_resource: &swallowtail_runtime::WorkingResourceRef,
    access_policy: &SessionAccessPolicy,
    deadline: Option<swallowtail_runtime::Deadline>,
    options: &swallowtail_runtime::SessionOptions,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    for service in [
        HostServiceKind::Task,
        HostServiceKind::Time,
        HostServiceKind::Process,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ] {
        if !services.available_kinds().contains(&service) {
            return Err(failure(
                "swallowtail.claude_agent.acp.attachment_service_missing",
                "Claude Agent ACP attachment requires its preflight-bound host services",
            ));
        }
    }
    if plan.credential_mechanism() == &CredentialMechanism::ApiKey
        && services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.credential_service_missing",
            "Claude Agent ACP API-key access requires a credential service",
        ));
    }
    if !binding.matches_attachment(plan, working_resource, access_policy) {
        return Err(failure(
            "swallowtail.claude_agent.acp.session_binding_mismatch",
            "Claude Agent ACP session binding does not match the requested attachment",
        ));
    }
    if access_policy != &session_access_policy(plan)? {
        return Err(failure(
            "swallowtail.claude_agent.acp.access_policy_rejected",
            "Claude Agent ACP requires its preflight-bound ambient read-only access policy",
        ));
    }
    if deadline.is_some() {
        return Err(unsupported("session deadline"));
    }
    if options.developer_instructions().is_some()
        || options.reasoning_mode().is_some()
        || options.tools().len() != 0
    {
        return Err(unsupported("attachment session options"));
    }
    Ok(())
}

pub(super) fn validate_turn(
    request: &TurnRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if let Some(deadline) = request.deadline()
        && services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    {
        return Err(failure(
            "swallowtail.claude_agent.acp.deadline_elapsed",
            "Claude Agent turn deadline elapsed before provider work",
        ));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    Ok(())
}

fn capability_drift() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.capability_drift",
        "Claude Agent capabilities do not match the selected behavior revision",
    )
}

fn advertised(session: Option<&Value>, capability: &str) -> bool {
    session
        .and_then(|session| session.get(capability))
        .is_some_and(Value::is_object)
}
