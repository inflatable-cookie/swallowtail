fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.cursor.acp.task_service_missing",
            "Cursor Agent ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.cursor.acp.process_service_missing",
            "Cursor Agent ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.cursor.acp.resource_service_missing",
            "Cursor Agent ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.cursor.acp.resource_io_service_missing",
            "Cursor Agent ACP requires a working-resource I/O service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    if plan.requirements().operation_shape()
        != swallowtail_core::OperationShape::InteractiveSession
    {
        return Err(failure(
            "swallowtail.cursor.acp.operation_shape_rejected",
            "Cursor ACP requires an interactive-session plan",
        ));
    }
    swallowtail_runtime::validate_session_plan_agreement(plan, request.plan_agreement())?;
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite) {
        return Err(failure(
            "swallowtail.cursor.acp.access_policy_rejected",
            "Cursor Agent requires exact ambient read-write workspace access",
        ));
    }
    if request.provider_state_policy()
        != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
    {
        return Err(failure(
            "swallowtail.cursor.acp.provider_state_rejected",
            "Cursor Agent session state must be preserved on attachment close",
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

fn validate_recovery(
    plan: &PreflightPlan,
    request: &ResumeSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for (present, code, message) in [
        (
            services.task().is_some(),
            "swallowtail.cursor.acp.task_service_missing",
            "Cursor Agent ACP requires a scoped task service",
        ),
        (
            services.process().is_some(),
            "swallowtail.cursor.acp.process_service_missing",
            "Cursor Agent ACP requires a process service",
        ),
        (
            services.working_resource().is_some(),
            "swallowtail.cursor.acp.resource_service_missing",
            "Cursor Agent ACP requires a working-resource service",
        ),
        (
            services.working_resource_io().is_some(),
            "swallowtail.cursor.acp.resource_io_service_missing",
            "Cursor Agent ACP requires a working-resource I/O service",
        ),
    ] {
        if !present {
            return Err(failure(code, message));
        }
    }
    let requirement = plan
        .requirements()
        .capabilities()
        .find(|requirement| {
            requirement.capability() == Capability::ProviderSessionAttachmentRecovery
        })
        .ok_or_else(|| {
            failure(
                "swallowtail.cursor.acp.attachment_recovery_capability_mismatch",
                "Cursor Agent attachment recovery does not match its preflight plan",
            )
        })?;
    for constraint in [
        CapabilityConstraint::ReplayMaximumItems(
            crate::MAXIMUM_ATTACHMENT_RECOVERY_UPDATES as u32,
        ),
        CapabilityConstraint::ReplayMaximumBytes(crate::MAXIMUM_ATTACHMENT_RECOVERY_BYTES as u64),
    ] {
        if !requirement
            .constraints()
            .any(|present| present == &constraint)
        {
            return Err(failure(
                "swallowtail.cursor.acp.attachment_recovery_capability_mismatch",
                "Cursor Agent attachment recovery bounds do not match its preflight plan",
            ));
        }
    }
    if !request.resume_binding().matches_attachment(
        plan,
        request.working_resource(),
        request.access_policy(),
    ) || request.access_policy()
        != &SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
        || request.provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || request.deadline().is_some()
        || !request.options().is_empty()
    {
        return Err(failure(
            "swallowtail.cursor.acp.attachment_recovery_binding_mismatch",
            "Cursor Agent attachment recovery does not match its durable binding",
        ));
    }
    Ok(())
}

fn validate_turn(request: &TurnRequest, services: &HostServices) -> Result<(), RuntimeFailure> {
    if let Some(deadline) = request.deadline() {
        let time = services.time().ok_or_else(|| {
            failure(
                "swallowtail.cursor.acp.time_service_missing",
                "Deadline-bound Cursor work requires a time service",
            )
        })?;
        if time.now() >= deadline.instant() {
            return Err(failure(
                "swallowtail.cursor.acp.deadline_elapsed",
                "Cursor run deadline elapsed before provider work",
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

fn validate_initialize(response: &Value) -> Result<(), RuntimeFailure> {
    let capabilities = response
        .get("agentCapabilities")
        .and_then(Value::as_object)
        .ok_or_else(malformed)?;
    if capabilities.get("loadSession").and_then(Value::as_bool) != Some(true)
        || capabilities
            .get("promptCapabilities")
            .and_then(|value| value.get("embeddedContext"))
            .and_then(Value::as_bool)
            != Some(false)
        || capabilities
            .get("promptCapabilities")
            .and_then(|value| value.get("image"))
            .and_then(Value::as_bool)
            != Some(true)
        || capabilities
            .get("promptCapabilities")
            .and_then(|value| value.get("audio"))
            .and_then(Value::as_bool)
            != Some(false)
        || capabilities
            .get("mcpCapabilities")
            .and_then(|value| value.get("http"))
            .and_then(Value::as_bool)
            != Some(true)
        || capabilities
            .get("mcpCapabilities")
            .and_then(|value| value.get("sse"))
            .and_then(Value::as_bool)
            != Some(true)
        || !capabilities
            .get("sessionCapabilities")
            .and_then(|value| value.get("list"))
            .is_some_and(Value::is_object)
    {
        return Err(failure(
            "swallowtail.cursor.acp.capabilities_rejected",
            "Cursor Agent did not advertise the qualified ACP capabilities",
        ));
    }
    let cursor_login = response
        .get("authMethods")
        .and_then(Value::as_array)
        .is_some_and(|methods| {
            methods
                .iter()
                .any(|method| method.get("id").and_then(Value::as_str) == Some(AUTH_METHOD))
        });
    if !cursor_login {
        return Err(failure(
            "swallowtail.cursor.acp.local_login_unavailable",
            "Cursor Agent did not advertise delegated local login",
        ));
    }
    Ok(())
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
            "swallowtail.cursor.acp.resource_release_failed",
            "Cursor Agent working-resource service disappeared during cleanup",
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
