fn validate_common(
    plan: &PreflightPlan,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::selection::validate_pi_plan_version(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    for (service, present) in [
        (HostServiceKind::Task, services.task().is_some()),
        (HostServiceKind::Process, services.process().is_some()),
        (HostServiceKind::Credential, services.credential().is_some()),
        (HostServiceKind::Time, services.time().is_some()),
    ] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
            || !present
        {
            return Err(plan_mismatch("host service"));
        }
    }
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral {
        return Err(plan_mismatch("instance ownership"));
    }
    match plan.credential_mechanism() {
        CredentialMechanism::ProviderSpecific(namespace)
            if namespace.as_str() == ACCESS_NAMESPACE => {}
        _ => return Err(plan_mismatch("delegated harness access")),
    }
    if plan.credential_reference() != Some(credential)
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(plan_mismatch("access profile"));
    }
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::ProviderSuppressed)
    {
        return Err(plan_mismatch("harness configuration posture"));
    }
    let policy = plan
        .harness_rpc_policy()
        .ok_or_else(|| plan_mismatch("harness RPC policy"))?;
    let bounds = policy.scheduling();
    if bounds.maximum_active_operations().get() != 1
        || bounds.maximum_completed_prompts().get() != 2
        || bounds.maximum_pending_steering().get() != 1
        || bounds.maximum_pending_follow_up().get() != 1
    {
        return Err(plan_mismatch("harness scheduling policy"));
    }
    for source in [
        HarnessConfigurationSource::Extensions,
        HarnessConfigurationSource::Skills,
        HarnessConfigurationSource::PromptTemplates,
        HarnessConfigurationSource::ContextFiles,
    ] {
        if policy.permits_configuration_source(source) {
            return Err(plan_mismatch("disabled configuration source"));
        }
    }
    for action in [
        HarnessBackgroundAction::UpdateCheck,
        HarnessBackgroundAction::Telemetry,
        HarnessBackgroundAction::PackageMutation,
        HarnessBackgroundAction::AutomaticRetry,
    ] {
        if policy.permits_background_action(action) {
            return Err(plan_mismatch("disabled background action"));
        }
    }
    Ok(())
}

