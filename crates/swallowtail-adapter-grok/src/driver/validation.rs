fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.grok.acp.task_service_missing",
            "Grok Build ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.grok.acp.process_service_missing",
            "Grok Build ACP requires a process service",
        ),
        (
            services.credential().is_some(),
            "swallowtail.grok.acp.credential_service_missing",
            "Grok Build ACP requires a credential service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.grok.acp.resource_service_missing",
            "Grok Build ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.grok.acp.resource_io_service_missing",
            "Grok Build ACP requires a working-resource I/O service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    if plan.requirements().operation_shape()
        == swallowtail_core::OperationShape::InteractiveSession
    {
        swallowtail_runtime::validate_session_plan_agreement(plan, request.plan_agreement())?;
    } else if plan.requirements().operation_shape()
        != swallowtail_core::OperationShape::StructuredRun
    {
        return Err(failure(
            "swallowtail.grok.acp.run_plan_mismatch",
            "Grok private session projection requires a structured-run plan",
        ));
    }
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite) {
        return Err(failure(
            "swallowtail.grok.acp.access_policy_rejected",
            "Grok Build requires exact ambient read-write workspace access",
        ));
    }
    if request.provider_state_policy()
        != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    {
        return Err(failure(
            "swallowtail.grok.acp.provider_state_rejected",
            "Grok Build session state must be preserved on attachment close",
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

fn validate_turn(request: &TurnRequest, services: &HostServices) -> Result<(), RuntimeFailure> {
    if let Some(deadline) = request.deadline() {
        let time = services.time().ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.time_service_missing",
                "Deadline-bound Grok work requires a time service",
            )
        })?;
        if time.now() >= deadline.instant() {
            return Err(failure(
                "swallowtail.grok.acp.deadline_elapsed",
                "Grok run deadline elapsed before provider work",
            ));
        }
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    Ok(())
}

include!("validation/run.rs");

fn validate_initialize(
    response: &Value,
    selected_version: &swallowtail_core::InterfaceVersion,
    selected_model: &str,
) -> Result<NegotiatedSessionModelOptions, RuntimeFailure> {
    let metadata = response.get("_meta").ok_or_else(malformed)?;
    if metadata.get("agentVersion").and_then(Value::as_str) != Some(selected_version.as_str()) {
        return Err(failure(
            "swallowtail.grok.acp.agent_version_rejected",
            "Grok Build ACP identity does not match the preflight version",
        ));
    }
    let capabilities = response
        .get("agentCapabilities")
        .and_then(Value::as_object)
        .ok_or_else(malformed)?;
    if capabilities.get("loadSession").and_then(Value::as_bool) != Some(true)
        || capabilities
            .get("promptCapabilities")
            .and_then(|value| value.get("embeddedContext"))
            .and_then(Value::as_bool)
            != Some(true)
    {
        return Err(failure(
            "swallowtail.grok.acp.capabilities_rejected",
            "Grok Build did not advertise the qualified ACP capabilities",
        ));
    }
    let cached_token = response
        .get("authMethods")
        .and_then(Value::as_array)
        .is_some_and(|methods| {
            methods
                .iter()
                .any(|method| method.get("id").and_then(Value::as_str) == Some(AUTH_METHOD))
        });
    if !cached_token
        || metadata.get("defaultAuthMethodId").and_then(Value::as_str) != Some(AUTH_METHOD)
    {
        return Err(failure(
            "swallowtail.grok.acp.cached_token_unavailable",
            "Grok Build did not advertise cached_token as its default access method",
        ));
    }
    parse_model_options(metadata, selected_model)
}

fn parse_model_options(
    metadata: &Value,
    selected_model: &str,
) -> Result<NegotiatedSessionModelOptions, RuntimeFailure> {
    let state = metadata.get("modelState").ok_or_else(malformed)?;
    let current = state
        .get("currentModelId")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    if current != selected_model {
        return Err(failure(
            "swallowtail.grok.acp.model_drift",
            "Grok Build current model does not match the preflight model",
        ));
    }
    let available = state
        .get("availableModels")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    let options = available
        .iter()
        .map(|model| {
            let value = model
                .get("modelId")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            let display_name = model.get("name").and_then(Value::as_str).map(str::to_owned);
            NegotiatedSessionModelOption::new(value, display_name)
        })
        .collect::<Result<Vec<_>, _>>()?;
    NegotiatedSessionModelOptions::new(current, options)
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("ACP active-task lock poisoned");
        if active
            .as_ref()
            .is_some_and(|active| active.turn.is_finished())
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut finished) = finished
        && let Some(task) = finished.task.take()
    {
        task.join().await?;
    }
    Ok(())
}

async fn release_resource(lease: Option<ResourceLease>, services: &HostServices) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "swallowtail.grok.acp.resource_release_failed",
            "Grok Build working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

async fn release_credential(
    lease: Option<CredentialLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "swallowtail.grok.acp.credential_release_failed",
            "Grok Build credential service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(code, message))
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}
