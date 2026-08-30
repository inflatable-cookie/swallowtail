use crate::failure::failure;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, HarnessConfigurationPosture,
    HarnessIsolation, HostServiceKind, InstanceOwnership, PreflightPlan, ResourceAccess,
    ResourceRepresentation,
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
) -> Result<(), RuntimeFailure> {
    crate::claude_code::validate_headless_plan(plan)?;
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
    require_optional_service(
        plan,
        services.working_resource().is_some(),
        HostServiceKind::WorkingResource,
        "working resource",
    )?;
    require_optional_service(
        plan,
        services.working_resource_io().is_some(),
        HostServiceKind::WorkingResourceIo,
        "working-resource I/O",
    )?;
    require_optional_service(
        plan,
        services.watcher().is_some(),
        HostServiceKind::Watcher,
        "watcher",
    )?;
    require_optional_service(
        plan,
        services.watcher_bridge().is_some(),
        HostServiceKind::WatcherBridge,
        "watcher bridge",
    )?;
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
        || request.policy().provider_retention() != ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("provider lifecycle policy"));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
    {
        return Err(unsupported("provider search or external tool network"));
    }
    if let Some(reasoning) = request.policy().reasoning_mode() {
        require_constraint(
            plan,
            Capability::ReasoningSelection,
            CapabilityConstraint::ReasoningMode(reasoning.clone()),
        )?;
    }
    if request.policy().harness_mode() != Some(swallowtail_core::HarnessMode::Plan) {
        return Err(plan_mismatch("plan mode"));
    }
    require_constraint(
        plan,
        Capability::HarnessModeSelection,
        CapabilityConstraint::HarnessMode(swallowtail_core::HarnessMode::Plan),
    )?;
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
        Err(missing_service(name))
    } else {
        Ok(())
    }
}

fn require_optional_service(
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
        Ok(())
    } else if !available {
        Err(missing_service(name))
    } else {
        Ok(())
    }
}

fn missing_service(name: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.host_service_missing",
        format!("Claude Code headless requires the preflight-bound {name} service"),
    )
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
        "swallowtail.claude_code.headless.request_plan_mismatch",
        format!("Claude Code headless request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.headless.unsupported_input",
        format!("Claude Code headless does not support {feature}"),
    )
}
