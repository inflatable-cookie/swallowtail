use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ResourceAccess, ResourceRepresentation, SessionProviderStatePolicy,
    SupportAuthority,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, OpenSessionRequest,
    ProviderExecutionPolicy, ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure,
    SessionAccessPolicy, StreamReattachmentPolicy, StructuredRunRequest,
    validate_harness_configuration_policy, validate_harness_isolation_policy,
    validate_session_plan_agreement,
};

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::selection::validate_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    require_service(
        plan,
        services.task().is_some(),
        HostServiceKind::Task,
        "task",
    )?;
    require_service(
        plan,
        services.process().is_some(),
        HostServiceKind::Process,
        "process",
    )?;
    require_service(
        plan,
        services.time().is_some(),
        HostServiceKind::Time,
        "time",
    )?;
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("instance, local account, or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient configuration or host isolation"));
    }
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(crate::failure::unsupported("provider lifecycle policy"));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().reasoning_mode().is_some()
    {
        return Err(crate::failure::unsupported(
            "consumer network, search, or reasoning selection",
        ));
    }
    if request.working_resource().is_none() || request.deadline().is_none() {
        return Err(crate::failure::unsupported(
            "missing working resource or deadline",
        ));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(crate::failure::unsupported(
            "attachments, consumer tools, schema, or output-token limit",
        ));
    }
    require_capability(plan, Capability::StructuredRun)?;
    require_capability(plan, Capability::StreamingEvents)?;
    require_capability(plan, Capability::ObservableActivity)?;
    require_capability(plan, Capability::UsageReporting)?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    Ok(())
}

pub(crate) fn validate_session(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::DRIVER_ID
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
    {
        return Err(session_plan_mismatch("driver or access"));
    }
    crate::selection::validate_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    if services.task().is_none()
        || services.process().is_none()
        || services.time().is_none()
        || services.working_resource().is_none()
    {
        return Err(crate::failure::failure(
            "swallowtail.command_code.headless.host_service_missing",
            "Command Code continuation requires task, process, time, and working-resource services",
        ));
    }
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if plan.requirements().operation_shape() != swallowtail_core::OperationShape::InteractiveSession
        || plan.requirements().driver_role() != swallowtail_core::DriverRole::InteractiveSession
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan.provider_id().is_none()
    {
        return Err(session_plan_mismatch("operation or model route"));
    }
    for capability in [
        Capability::InteractiveSession,
        Capability::StreamingEvents,
        Capability::ObservableActivity,
        Capability::UsageReporting,
        Capability::ProviderDurableRetention,
    ] {
        require_capability(plan, capability).map_err(|_| session_plan_mismatch("capability"))?;
    }
    require_constraint(
        plan,
        Capability::InteractiveSession,
        CapabilityConstraint::MaximumTurns(24),
    )
    .map_err(|_| session_plan_mismatch("capability constraint"))?;
    require_constraint(
        plan,
        Capability::StreamingEvents,
        CapabilityConstraint::StreamRecordMaximumCount(4096),
    )
    .map_err(|_| session_plan_mismatch("capability constraint"))?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::ActiveTurn),
    )
    .map_err(|_| session_plan_mismatch("capability constraint"))?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    )
    .map_err(|_| session_plan_mismatch("capability constraint"))?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )
    .map_err(|_| session_plan_mismatch("capability constraint"))?;
    if request.working_resource().is_none()
        || request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || request.provider_state_policy()
            != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        || request.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || !request.options().is_empty()
    {
        return Err(crate::failure::unsupported(
            "session access, provider state, configuration, isolation, or options",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time service").now() >= deadline.instant()
    {
        return Err(crate::failure::failure(
            "swallowtail.command_code.headless.deadline_elapsed",
            "Command Code session deadline elapsed before opening",
        ));
    }
    Ok(())
}

fn require_service(
    plan: &PreflightPlan,
    available: bool,
    service: HostServiceKind,
    name: &str,
) -> Result<(), RuntimeFailure> {
    if !plan
        .requirements()
        .host_services()
        .any(|required| required == service)
    {
        Err(plan_mismatch(name))
    } else if !available {
        Err(crate::failure::failure(
            "swallowtail.command_code.headless.host_service_missing",
            format!("Command Code headless requires the preflight-bound {name} service"),
        ))
    } else {
        Ok(())
    }
}

fn require_capability(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
    {
        Ok(())
    } else {
        Err(plan_mismatch("capability"))
    }
}

fn require_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == &constraint)
    }) {
        Ok(())
    } else {
        Err(plan_mismatch("capability constraint"))
    }
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.command_code.headless.request_plan_mismatch",
        format!("Command Code headless request does not match its preflight-bound {dimension}"),
    )
}

fn session_plan_mismatch(dimension: &str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.command_code.headless.session_plan_mismatch",
        format!("Command Code continuation plan did not match {dimension}"),
    )
}
