fn validate_run(
    plan: &PreflightPlan,
    request: &swallowtail_runtime::StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;
    if plan.requirements().execution_layer() != swallowtail_core::ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != swallowtail_core::OperationShape::StructuredRun
        || plan.requirements().driver_role() != swallowtail_core::DriverRole::StructuredRun
    {
        return Err(failure(
            "swallowtail.grok.acp.run_plan_mismatch",
            "Grok structured run does not match its preflight plan",
        ));
    }
    require_run_services(plan, services)?;
    require_run_capabilities(plan)?;
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free structured run"));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "structured-run attachments, consumer tools, schema, or output-token limit",
        ));
    }
    validate_run_policy(plan, request.policy())?;
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.grok.acp.deadline_elapsed",
            "Grok run deadline elapsed before provider work",
        ));
    }
    Ok(())
}

fn require_run_services(
    plan: &PreflightPlan,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    for service in [
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Time,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::Credential,
        swallowtail_core::HostServiceKind::WorkingResource,
        swallowtail_core::HostServiceKind::WorkingResourceIo,
    ] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
            || !services.available_kinds().contains(&service)
        {
            return Err(failure(
                "swallowtail.grok.acp.run_host_service_missing",
                "Grok structured run requires its preflight-bound host services",
            ));
        }
    }
    Ok(())
}

fn require_run_capabilities(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    for capability in [
        swallowtail_core::Capability::StructuredRun,
        swallowtail_core::Capability::StreamingEvents,
        swallowtail_core::Capability::WorkingResource,
        swallowtail_core::Capability::ProviderDurableRetention,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
        {
            return Err(run_capability_failure());
        }
    }
    let interruption = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == swallowtail_core::Capability::Interruption);
    if interruption.is_none_or(|required| {
        !required.constraints().any(|constraint| {
            constraint
                == &swallowtail_core::CapabilityConstraint::CancellationScope(
                    CancellationScope::StructuredRun,
                )
        })
    }) {
        return Err(run_capability_failure());
    }
    Ok(())
}

fn validate_run_policy(
    plan: &PreflightPlan,
    policy: &swallowtail_runtime::OperationPolicy,
) -> Result<(), RuntimeFailure> {
    swallowtail_runtime::validate_harness_isolation_policy(plan, policy).map_err(|_| {
        failure(
            "swallowtail.grok.acp.run_isolation_mismatch",
            "Grok structured-run isolation does not match its preflight plan",
        )
    })?;
    swallowtail_runtime::validate_harness_configuration_policy(plan, policy).map_err(|_| {
        failure(
            "swallowtail.grok.acp.run_configuration_mismatch",
            "Grok structured-run configuration does not match its preflight plan",
        )
    })?;
    if policy.external_network() != swallowtail_core::ExternalNetworkPolicy::Denied
        || policy.external_search() != swallowtail_core::ExternalSearchPolicy::Disabled
        || policy.provider_execution()
            != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || policy.provider_retention()
            != swallowtail_runtime::ProviderRetentionPolicy::DurableAllowed
        || policy.provider_recovery()
            != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment()
            != swallowtail_runtime::StreamReattachmentPolicy::Disabled
        || policy.reasoning_mode().is_some()
    {
        return Err(unsupported("structured-run lifecycle or inference policy"));
    }
    Ok(())
}

fn run_capability_failure() -> RuntimeFailure {
    failure(
        "swallowtail.grok.acp.run_capability_mismatch",
        "Grok structured-run capabilities do not match the preflight plan",
    )
}
