use crate::DRIVER_ID;
use crate::failure::{failure, unsupported};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism, DriverRole,
    ExecutionLayer, HarnessBackgroundAction, HarnessConfigurationPosture,
    HarnessConfigurationSource, HarnessIsolation, HostServiceKind, InstanceOwnership,
    OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, OpenSessionRequest,
    ProviderExecutionPolicy, ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure,
    StreamReattachmentPolicy, StructuredRunRequest, TurnRequest, validate_session_plan_agreement,
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

pub(super) fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
    credential: &swallowtail_core::CredentialRef,
) -> Result<(), RuntimeFailure> {
    validate_common(plan, services, credential)?;
    if plan.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != OperationShape::StructuredRun
        || plan.requirements().driver_role() != DriverRole::StructuredRun
        || plan.provider_id().is_none()
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("structured-run role and model route"));
    }
    if !plan
        .requirements()
        .host_services()
        .any(|required| required == HostServiceKind::WorkingResource)
        || services.working_resource().is_none()
    {
        return Err(plan_mismatch("working-resource host service"));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("resource-free structured run"));
    }
    if request.deadline().is_none() {
        return Err(unsupported("structured run without a host deadline"));
    }
    if request.tools().len() != 0
        || request.structured_output().is_some()
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "structured-run consumer tools, schema, or output-token limit",
        ));
    }
    validate_attachments(plan, request.attachments(), services)?;
    require_capability(plan, Capability::StructuredRun)?;
    require_capability(plan, Capability::StreamingEvents)?;
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
    let policy = request.policy();
    if policy.external_network() != ExternalNetworkPolicy::Denied
        || policy.external_search() != ExternalSearchPolicy::Disabled
        || policy.reasoning_mode().is_some()
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention() != ProviderRetentionPolicy::Prohibited
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
        || policy.harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || policy.harness_configuration_posture()
            != Some(HarnessConfigurationPosture::ProviderSuppressed)
    {
        return Err(unsupported("structured-run lifecycle or inference policy"));
    }
    if services.time().expect("validated Pi time service").now()
        >= request
            .deadline()
            .expect("validated Pi run deadline")
            .instant()
    {
        return Err(failure(
            "swallowtail.pi.rpc.run_deadline_elapsed",
            "Pi RPC structured-run deadline elapsed before provider work",
        ));
    }
    Ok(())
}

fn validate_attachments<'a>(
    plan: &PreflightPlan,
    attachments: impl ExactSizeIterator<Item = &'a swallowtail_runtime::AttachmentDescriptor>,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let count = attachments.len();
    let planned = plan
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::Attachments);
    if (count != 0) != planned.is_some() {
        return Err(plan_mismatch("attachments"));
    }
    if count == 0 {
        return Ok(());
    }
    let requirement = planned.expect("attachment request is planned");
    if count > 1
        || !requirement
            .constraints()
            .any(|constraint| matches!(constraint, CapabilityConstraint::AttachmentMaximumCount(1)))
    {
        return Err(plan_mismatch("attachment count"));
    }
    for attachment in attachments {
        if attachment.media_type() != "image/png"
            || !requirement.constraints().any(|constraint| {
                matches!(constraint, CapabilityConstraint::AttachmentMediaType(media) if media == "image/png")
            })
        {
            return Err(unsupported("non-PNG attachment"));
        }
        if attachment
            .known_length()
            .is_some_and(|length| length > 1024 * 1024)
        {
            return Err(unsupported("attachment larger than one MiB"));
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
    if request.attachments().len() == 0 && image_attachments {
        // The session capability authorizes later image-bearing turns; an
        // individual text-only turn does not need to repeat that input.
    }
    Ok(())
}

fn validate_planned_attachment_services(
    plan: &PreflightPlan,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if !plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == Capability::Attachments)
    {
        return Ok(());
    }
    let attachment = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::Attachments)
        .expect("attachment capability was found");
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
