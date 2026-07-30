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

include!("validation/attachments.rs");
include!("validation/common.rs");

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

include!("validation/plan.rs");
