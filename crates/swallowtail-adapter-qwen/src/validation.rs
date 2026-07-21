use crate::DRIVER_ID;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism, HarnessIsolation,
    HostServiceKind, InstanceOwnership, PreflightPlan, ResourceAccess, ResourceRepresentation,
    SafeDiagnostic,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure, StreamReattachmentPolicy,
    StructuredRunRequest, validate_harness_isolation_policy,
};

const ACCESS_NAMESPACE: &str = "qwen-code/delegated-harness-auth";
const ENDPOINT_AUDIENCE: &str = "qwen-code";

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
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

    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral {
        return Err(plan_mismatch("instance ownership"));
    }
    match plan.credential_mechanism() {
        CredentialMechanism::ProviderSpecific(namespace)
            if namespace.as_str() == ACCESS_NAMESPACE => {}
        _ => return Err(plan_mismatch("delegated harness access")),
    }
    if plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE {
        return Err(plan_mismatch("endpoint audience"));
    }
    if plan.provider_id().is_none() || plan.model_id().is_none() || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("provider and model route"));
    }

    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
        return Err(plan_mismatch("ambient harness isolation"));
    }
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
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
            "provider network, search, or reasoning selection",
        ));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("missing working resource"));
    }
    if request.deadline().is_none() {
        return Err(unsupported("missing host deadline"));
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
        Err(host_missing(name))
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
        "swallowtail.qwen.headless.request_plan_mismatch",
        format!("Qwen headless request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.unsupported_input",
        format!("Qwen headless does not support {feature}"),
    )
}

fn host_missing(service: &str) -> RuntimeFailure {
    failure(
        "swallowtail.qwen.headless.host_service_missing",
        format!("Qwen headless requires the preflight-bound {service} service"),
    )
}

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
