use crate::failure::failure;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialRef,
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
    crate::headless::validate_headless_plan(plan, credential)?;
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
        || plan.provider_id().is_none()
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("instance, provider, or model route"));
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
    let owned_transcript_cleanup = owns_transcript_cleanup(plan)?;
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention()
            != if owned_transcript_cleanup {
                ProviderRetentionPolicy::TemporaryAllowed
            } else {
                ProviderRetentionPolicy::DurableAllowed
            }
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
    )
}

pub(crate) fn owns_transcript_cleanup(plan: &PreflightPlan) -> Result<bool, RuntimeFailure> {
    let durable = plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::ProviderDurableRetention);
    let temporary = plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::ProviderTemporaryRetention);
    let deletion = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::OwnedRemoteResourceDeletion);
    let exact_deletion = deletion.is_some_and(|required| {
        required
            .constraints()
            .eq([&CapabilityConstraint::OwnedRemoteResource(
                swallowtail_core::OwnedRemoteResourceKind::Session,
            )])
    });
    match (durable, temporary, deletion.is_some(), exact_deletion) {
        (true, false, false, false) => Ok(false),
        (false, true, true, true) => Ok(true),
        _ => Err(plan_mismatch("retention and transcript cleanup capability")),
    }
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
        Err(failure(
            "swallowtail.gemini.headless.host_service_missing",
            format!("Gemini headless requires the preflight-bound {name} service"),
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
        "swallowtail.gemini.headless.request_plan_mismatch",
        format!("Gemini headless request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.gemini.headless.unsupported_input",
        format!("Gemini headless does not support {feature}"),
    )
}
