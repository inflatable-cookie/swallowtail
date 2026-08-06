use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ReasoningMode, ResourceAccess, ResourceRepresentation, SupportAuthority,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure, StreamReattachmentPolicy,
    StructuredRunRequest, validate_harness_configuration_policy, validate_harness_isolation_policy,
};

pub(crate) struct ValidatedInput {
    pub(crate) effort: ReasoningMode,
}

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<ValidatedInput, RuntimeFailure> {
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
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan
            .model_id()
            .is_some_and(|model| model.as_str() != crate::MUSE_SPARK_MODEL_ID)
    {
        return Err(plan_mismatch("instance, local account, or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::ProviderEnforced)
    {
        return Err(plan_mismatch("ambient configuration or provider sandbox"));
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
        || request.policy().harness_mode().is_some()
    {
        return Err(crate::failure::unsupported(
            "consumer network, search, or harness mode",
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
    let effort = request
        .policy()
        .reasoning_mode()
        .cloned()
        .ok_or_else(|| crate::failure::unsupported("implicit reasoning effort"))?;
    if !matches!(
        effort.as_str(),
        "none" | "minimal" | "low" | "medium" | "high" | "xhigh" | "ultra"
    ) {
        return Err(crate::failure::unsupported("reasoning effort"));
    }
    require_constraint(
        plan,
        Capability::ReasoningSelection,
        CapabilityConstraint::ReasoningMode(effort.clone()),
    )?;
    Ok(ValidatedInput { effort })
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
            "swallowtail.muse_code.headless.host_service_missing",
            format!("Muse Code headless requires the preflight-bound {name} service"),
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
        "swallowtail.muse_code.headless.request_plan_mismatch",
        format!("Muse Code headless request does not match its preflight-bound {dimension}"),
    )
}
