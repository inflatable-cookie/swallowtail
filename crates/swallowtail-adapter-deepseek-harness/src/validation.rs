use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ResourceAccess, ResourceRepresentation, SupportAuthority,
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
    if plan.driver_identity().id().as_str() != crate::DRIVER_ID
        || plan.requirements().execution_layer()
            != swallowtail_core::ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != swallowtail_core::OperationShape::StructuredRun
    {
        return Err(plan_mismatch("driver or operation"));
    }
    crate::selection::validate_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    for (service, present, name) in [
        (HostServiceKind::Task, services.task().is_some(), "task"),
        (
            HostServiceKind::Process,
            services.process().is_some(),
            "process",
        ),
        (HostServiceKind::Time, services.time().is_some(), "time"),
    ] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == service)
            || !present
        {
            return Err(crate::failure::failure(
                "swallowtail.deepseek_harness.host_service_missing",
                format!("DeepSeek Harness requires the preflight-bound {name} service"),
            ));
        }
    }
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::DEEPSEEK_HARNESS_CONFIG_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
        || plan.provider_id().is_none()
    {
        return Err(plan_mismatch("host configuration access or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient configuration or host isolation"));
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
        || request.policy().reasoning_mode().is_some()
    {
        return Err(crate::failure::unsupported(
            "consumer network, search, or reasoning selection",
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
    for capability in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::ObservableActivity,
        Capability::UsageReporting,
    ] {
        require_capability(plan, capability)?;
    }
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
            && required.constraints().any(|bound| bound == &constraint)
    }) {
        Ok(())
    } else {
        Err(plan_mismatch("capability constraint"))
    }
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.deepseek_harness.request_plan_mismatch",
        format!("DeepSeek Harness request does not match its preflight-bound {dimension}"),
    )
}
