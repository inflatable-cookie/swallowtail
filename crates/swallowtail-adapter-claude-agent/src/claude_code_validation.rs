use crate::claude_code_plan_checks::{
    HEADLESS_PLAN_ROUTE, plan_mismatch, require_capability, require_constraint,
    require_optional_service, require_service, unsupported,
};
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
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.task().is_some(),
        HostServiceKind::Task,
        "task",
    )?;
    require_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.process().is_some(),
        HostServiceKind::Process,
        "process",
    )?;
    require_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.time().is_some(),
        HostServiceKind::Time,
        "time",
    )?;
    require_optional_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.working_resource().is_some(),
        HostServiceKind::WorkingResource,
        "working resource",
    )?;
    require_optional_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.working_resource_io().is_some(),
        HostServiceKind::WorkingResourceIo,
        "working-resource I/O",
    )?;
    require_optional_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.watcher().is_some(),
        HostServiceKind::Watcher,
        "watcher",
    )?;
    require_optional_service(
        &HEADLESS_PLAN_ROUTE,
        plan,
        services.watcher_bridge().is_some(),
        HostServiceKind::WatcherBridge,
        "watcher bridge",
    )?;
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch(
            &HEADLESS_PLAN_ROUTE,
            "instance or model route",
        ));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch(&HEADLESS_PLAN_ROUTE, "harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch(&HEADLESS_PLAN_ROUTE, "harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch(
            &HEADLESS_PLAN_ROUTE,
            "ambient harness authority",
        ));
    }
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            &HEADLESS_PLAN_ROUTE,
            "provider lifecycle policy",
        ));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
    {
        return Err(unsupported(
            &HEADLESS_PLAN_ROUTE,
            "provider search or external tool network",
        ));
    }
    if let Some(reasoning) = request.policy().reasoning_mode() {
        require_constraint(
            &HEADLESS_PLAN_ROUTE,
            plan,
            Capability::ReasoningSelection,
            CapabilityConstraint::ReasoningMode(reasoning.clone()),
        )?;
    }
    if request.policy().harness_mode() != Some(swallowtail_core::HarnessMode::Plan) {
        return Err(plan_mismatch(&HEADLESS_PLAN_ROUTE, "plan mode"));
    }
    require_constraint(
        &HEADLESS_PLAN_ROUTE,
        plan,
        Capability::HarnessModeSelection,
        CapabilityConstraint::HarnessMode(swallowtail_core::HarnessMode::Plan),
    )?;
    if request.working_resource().is_none() || request.deadline().is_none() {
        return Err(unsupported(
            &HEADLESS_PLAN_ROUTE,
            "missing working resource or host deadline",
        ));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            &HEADLESS_PLAN_ROUTE,
            "attachments, consumer tools, schema, or output-token limit",
        ));
    }
    require_capability(&HEADLESS_PLAN_ROUTE, plan, Capability::StructuredRun)?;
    require_capability(&HEADLESS_PLAN_ROUTE, plan, Capability::StreamingEvents)?;
    require_capability(&HEADLESS_PLAN_ROUTE, plan, Capability::UsageReporting)?;
    require_constraint(
        &HEADLESS_PLAN_ROUTE,
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        &HEADLESS_PLAN_ROUTE,
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    )?;
    require_constraint(
        &HEADLESS_PLAN_ROUTE,
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )
}
