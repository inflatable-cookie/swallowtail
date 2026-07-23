use crate::DRIVER_ID;
use crate::failure::{failure, unsupported};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessBackgroundAction, HarnessConfigurationSource, HarnessIsolation, HostServiceKind,
    InstanceOwnership, PreflightPlan, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, RuntimeFailure, TurnRequest, validate_session_access_plan,
};

pub(super) const ACCESS_NAMESPACE: &str = "pi/delegated-harness-auth";
pub(super) const ENDPOINT_AUDIENCE: &str = "pi-harness";

pub(super) fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
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
        _ => return Err(plan_mismatch("delegated harness access")),
    }
    if plan.credential_reference() != Some(credential)
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(plan_mismatch("access profile"));
    }
    if plan.provider_id().is_none() || plan.model_id().is_none() || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("provider and model route"));
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
    validate_session_access_plan(plan, request.access_policy())?;
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient read access"));
    }
    if request.provider_state_policy() != SessionProviderStatePolicy::Prohibited {
        return Err(unsupported("provider session persistence"));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("resource-free session"));
    }
    if !request.options().is_empty() {
        return Err(unsupported("session options"));
    }
    require_capability(plan, Capability::InteractiveSession)?;
    require_capability(plan, Capability::StreamingEvents)?;
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
        "swallowtail.pi.rpc.request_plan_mismatch",
        format!("Pi RPC request does not match its preflight-bound {dimension}"),
    )
}
