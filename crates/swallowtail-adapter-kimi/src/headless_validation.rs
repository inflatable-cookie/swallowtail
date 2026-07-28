use crate::failure::failure;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism, CredentialRef,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure, StreamReattachmentPolicy,
    StructuredRunRequest, validate_harness_configuration_policy, validate_harness_isolation_policy,
};

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
    credential: &CredentialRef,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::headless::DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    if plan.credential_mechanism() != &CredentialMechanism::InteractiveOauth
        || plan.credential_reference() != Some(credential)
    {
        return Err(plan_mismatch("delegated membership access"));
    }
    crate::selection::select_kimi_headless_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    for (kind, available) in [
        (HostServiceKind::Task, services.task().is_some()),
        (HostServiceKind::Process, services.process().is_some()),
        (HostServiceKind::Time, services.time().is_some()),
    ] {
        require_service(plan, available, kind)?;
    }
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("instance or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient harness authority"));
    }
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("provider lifecycle policy"));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().reasoning_mode().is_some()
    {
        return Err(unsupported(
            "provider search, external network, or reasoning selection",
        ));
    }
    if request.working_resource().is_none() || request.deadline().is_none() {
        return Err(unsupported("missing working resource or host deadline"));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "attachments, consumer tools, schema, or output-token limit",
        ));
    }
    require_capability(plan, Capability::StructuredRun)?;
    require_capability(plan, Capability::StreamingEvents)?;
    require_capability(plan, Capability::ProviderDurableRetention)?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )
}

fn require_service(
    plan: &PreflightPlan,
    available: bool,
    kind: HostServiceKind,
) -> Result<(), RuntimeFailure> {
    if !plan
        .requirements()
        .host_services()
        .any(|required| required == kind)
    {
        Err(plan_mismatch("host service"))
    } else if !available {
        Err(failure(
            "swallowtail.kimi.headless.host_service_missing",
            "Kimi headless requires its preflight-bound host services",
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
    failure(
        "swallowtail.kimi.headless.request_plan_mismatch",
        format!("Kimi headless request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.kimi.headless.unsupported_input",
        format!("Kimi headless does not support {feature}"),
    )
}
