use crate::failure::failure;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    InterfaceVersionBinding, PreflightPlan,
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
) -> Result<InterfaceVersionBinding, RuntimeFailure> {
    let observed_version = validate_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    for (available, service, name) in [
        (services.task().is_some(), HostServiceKind::Task, "task"),
        (
            services.process().is_some(),
            HostServiceKind::Process,
            "process",
        ),
        (services.time().is_some(), HostServiceKind::Time, "time"),
    ] {
        require_service(plan, available, service, name)?;
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
        != Some(HarnessConfigurationPosture::ProviderSuppressed)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("response-only harness authority"));
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
    if request.policy().harness_mode().is_some() {
        return Err(unsupported("harness mode"));
    }
    if let Some(reasoning) = request.policy().reasoning_mode() {
        require_constraint(
            plan,
            Capability::ReasoningSelection,
            CapabilityConstraint::ReasoningMode(reasoning.clone()),
        )?;
    }
    if request.working_resource().is_some() || request.deadline().is_none() {
        return Err(unsupported("working resource or missing host deadline"));
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
    for required in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::UsageReporting,
    ] {
        require_capability(plan, required)?;
    }
    for forbidden in [
        Capability::WorkingResource,
        Capability::StructuredOutput,
        Capability::HarnessModeSelection,
    ] {
        reject_capability(plan, forbidden)?;
    }
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    Ok(observed_version)
}

pub(crate) fn validate_plan(
    plan: &PreflightPlan,
) -> Result<InterfaceVersionBinding, RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::claude_code_response::DRIVER_ID {
        return Err(failure(
            "swallowtail.claude_code.response_only.plan_driver_mismatch",
            "Claude Code response-only plan is bound to a different driver",
        ));
    }
    if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::claude_code::ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.claude_code.response_only.access_profile_rejected",
            "Claude Code response-only execution requires local subscription access",
        ));
    }
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::ProviderSuppressed)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(failure(
            "swallowtail.claude_code.response_only.authority_rejected",
            "Claude Code response-only execution requires suppressed configuration and ambient isolation truth",
        ));
    }
    crate::claude_code_response_selection::select_response_only_plan(plan)
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
            "swallowtail.claude_code.response_only.host_service_missing",
            format!(
                "Claude Code response-only execution requires the preflight-bound {name} service"
            ),
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

fn reject_capability(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
    {
        Err(plan_mismatch("forbidden capability"))
    } else {
        Ok(())
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
        "swallowtail.claude_code.response_only.request_plan_mismatch",
        format!("Claude Code response-only request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.claude_code.response_only.unsupported_input",
        format!("Claude Code response-only execution does not support {feature}"),
    )
}
