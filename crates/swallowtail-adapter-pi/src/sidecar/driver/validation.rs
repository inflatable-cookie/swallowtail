use super::SIDECAR_DRIVER_ID;
use crate::sidecar::failure::{failure, unsupported};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism, DriverRole,
    HarnessBackgroundAction, HarnessConfigurationPosture, HarnessConfigurationSource,
    HarnessIsolation, HostServiceKind, InstanceOwnership, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    HostServices, OpenSessionRequest, RuntimeFailure, TurnRequest, validate_session_plan_agreement,
};

pub(super) const ACCESS_NAMESPACE: &str = "pi/delegated-harness-auth";
pub(super) const ENDPOINT_AUDIENCE: &str = "pi-harness";

pub(super) fn validate_catalogue(
    plan: &PreflightPlan,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    validate_common(plan, services, credential)?;
    if plan.requirements().driver_role() != DriverRole::ModelCatalog
        || plan.provider_id().is_some()
        || plan.model_id().is_some()
        || plan.model_route_id().is_some()
    {
        return Err(plan_mismatch("catalogue operation"));
    }
    require_capability(plan, Capability::ModelCatalog)
}

pub(super) fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    validate_common(plan, services, credential)?;
    if plan.requirements().driver_role() != DriverRole::InteractiveSession {
        return Err(plan_mismatch("driver role"));
    }
    if !plan
        .requirements()
        .host_services()
        .any(|required| required == HostServiceKind::WorkingResource)
        || services.working_resource().is_none()
    {
        return Err(plan_mismatch("host service"));
    }
    if plan.provider_id().is_none() || plan.model_id().is_none() || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("provider and model route"));
    }
    validate_session_plan_agreement(plan, request.plan_agreement())?;
    if request.access_policy() != &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient read access"));
    }
    if request.provider_state_policy() != Some(SessionProviderStatePolicy::Prohibited) {
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
    )?;
    validate_planned_attachment_services(plan, services)
}

pub(super) fn validate_turn(
    request: &TurnRequest,
    services: &HostServices,
    image_attachments: bool,
) -> Result<(), RuntimeFailure> {
    if request.deadline().is_none() {
        return Err(unsupported("turn without a host deadline"));
    }
    if request.attachments().len() > 1 || request.structured_output().is_some() {
        return Err(unsupported(
            "more than one turn attachment or structured output",
        ));
    }
    if let Some(attachment) = request.attachments().next() {
        if !image_attachments {
            return Err(plan_mismatch("attachments"));
        }
        if attachment.media_type() != "image/png" {
            return Err(unsupported("non-PNG turn attachment"));
        }
        if attachment
            .known_length()
            .is_some_and(|length| length > 1024 * 1024)
        {
            return Err(unsupported("turn attachment larger than one MiB"));
        }
        if services.attachment().is_none() || services.blocking_work().is_none() {
            return Err(plan_mismatch("attachment host services"));
        }
    }
    Ok(())
}

fn validate_common(
    plan: &PreflightPlan,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != SIDECAR_DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::sidecar::selection::validate_pi_sdk_sidecar_plan_versions(plan)?;
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

fn validate_planned_attachment_services(
    plan: &PreflightPlan,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let Some(attachment) = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::Attachments)
    else {
        return Ok(());
    };
    for constraint in [
        CapabilityConstraint::attachment_media_type("image/png")
            .expect("static media type is valid"),
        CapabilityConstraint::AttachmentMaximumBytes(1024 * 1024),
        CapabilityConstraint::AttachmentMaximumCount(1),
    ] {
        if !attachment
            .constraints()
            .any(|required| required == &constraint)
        {
            return Err(plan_mismatch("attachment constraint"));
        }
    }
    for service in [HostServiceKind::Attachment, HostServiceKind::BlockingWork] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
        {
            return Err(plan_mismatch("attachment host service"));
        }
    }
    if services.attachment().is_none() || services.blocking_work().is_none() {
        return Err(plan_mismatch("attachment host service"));
    }
    Ok(())
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.pi.sdk-sidecar.request_plan_mismatch",
        format!("Pi SDK sidecar request does not match its preflight-bound {dimension}"),
    )
}
