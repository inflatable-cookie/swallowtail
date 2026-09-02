use super::SDK_DRIVER_ID;
use crate::sdk::failure::{failure, unsupported};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism, DriverRole,
    HarnessBackgroundAction, HarnessConfigurationPosture, HarnessConfigurationSource,
    HarnessIsolation, HostServiceKind, InstanceOwnership, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, RuntimeFailure, TurnRequest, validate_session_plan_agreement,
};

pub(crate) const ACCESS_NAMESPACE: &str = "claude/delegated-subscription-auth";
pub(crate) const ENDPOINT_AUDIENCE: &str = "claude-agent-sdk";

pub(super) fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    // A platform that cannot prove descendant-tree ownership is unsupported
    // for this route, not best-effort.
    if !crate::sdk::claude_agent_sdk_platform_supported() {
        return Err(unsupported("this execution host platform"));
    }
    if plan.driver_identity().id().as_str() != SDK_DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::sdk::selection::validate_claude_agent_sdk_plan_versions(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    for (service, present) in [
        (HostServiceKind::Task, services.task().is_some()),
        (HostServiceKind::Process, services.process().is_some()),
        (HostServiceKind::Credential, services.credential().is_some()),
        (
            HostServiceKind::WorkingResource,
            services.working_resource().is_some(),
        ),
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
        _ => return Err(plan_mismatch("delegated subscription access")),
    }
    if plan.credential_reference() != Some(credential)
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(plan_mismatch("access profile"));
    }
    validate_suppression(plan)?;
    if plan.requirements().driver_role() != DriverRole::InteractiveSession {
        return Err(plan_mismatch("driver role"));
    }
    if plan.model_route_id().is_none() || plan.model_id().is_none() {
        return Err(plan_mismatch("model route"));
    }
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient read access"));
    }
    if request.provider_state_policy() != Some(SessionProviderStatePolicy::Prohibited) {
        return Err(plan_mismatch("provider-state policy"));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("resource-free session"));
    }
    // Open, startup, and every provider await are raced against this bound.
    if request.deadline().is_none() {
        return Err(unsupported("session open without a host deadline"));
    }
    if has_unsupported_options(request.options()) {
        return Err(unsupported("session options"));
    }
    require_capability(plan, Capability::InteractiveSession)?;
    require_capability(plan, Capability::StreamingEvents)?;
    require_capability(plan, Capability::ToolCalls)?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::ActiveTurn),
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

pub(super) fn validate_turn(request: &TurnRequest) -> Result<(), RuntimeFailure> {
    if request.deadline().is_none() {
        return Err(unsupported("turn without a host deadline"));
    }
    if request.attachments().len() != 0 || request.structured_output().is_some() {
        return Err(unsupported("turn attachments or structured output"));
    }
    Ok(())
}

fn validate_suppression(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::ProviderSuppressed)
    {
        return Err(plan_mismatch("harness configuration posture"));
    }
    let policy = plan
        .harness_rpc_policy()
        .ok_or_else(|| plan_mismatch("harness RPC policy"))?;
    let bounds = policy.scheduling();
    if bounds.maximum_active_operations().get() != 1
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

fn has_unsupported_options(options: &swallowtail_runtime::SessionOptions) -> bool {
    options.reasoning_mode().is_some()
        || options.harness_mode().is_some()
        || options.developer_instructions().is_some()
        || options.idioms().is_some()
        || options.tools().len() != 0
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
        "swallowtail.claude-agent.sdk.request_plan_mismatch",
        format!("Claude Agent SDK sidecar request does not match its preflight-bound {dimension}"),
    )
}
