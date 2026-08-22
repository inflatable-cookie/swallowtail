use crate::failure::failure;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ReasoningMode, ResourceAccess, ResourceRepresentation, SupportAuthority,
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
) -> Result<ResourceAccess, RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::HEADLESS_DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::selection::validate_cursor_headless_plan(plan)?;
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
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::CURSOR_SUBSCRIPTION_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("instance, access, or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
        || request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            "provider lifecycle or ambient authority policy",
        ));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
    {
        return Err(unsupported("consumer network or search policy"));
    }
    validate_reasoning_selection(
        plan,
        request.policy().reasoning_mode(),
        plan.model_id().expect("validated above").as_str(),
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
    require_capability(plan, Capability::ObservableActivity)?;
    require_capability(plan, Capability::UsageReporting)?;
    require_capability(plan, Capability::ProviderDurableRetention)?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    let read = has_constraint(
        plan,
        Capability::WorkingResource,
        &CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    );
    let write = has_constraint(
        plan,
        Capability::WorkingResource,
        &CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
    );
    match (read, write) {
        (true, false) => Ok(ResourceAccess::Read),
        (false, true) => Ok(ResourceAccess::ReadWrite),
        _ => Err(plan_mismatch("working-resource authority")),
    }
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
            "swallowtail.cursor.headless.host_service_missing",
            format!("Cursor headless requires the preflight-bound {name} service"),
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

fn require_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if has_constraint(plan, capability, &constraint) {
        Ok(())
    } else {
        Err(plan_mismatch("capability constraint"))
    }
}

fn has_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: &CapabilityConstraint,
) -> bool {
    plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == constraint)
    })
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    failure(
        "swallowtail.cursor.headless.request_plan_mismatch",
        format!("Cursor headless request does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.cursor.headless.unsupported_input",
        format!("Cursor headless does not support {feature}"),
    )
}

fn validate_reasoning_selection(
    plan: &PreflightPlan,
    requested: Option<&ReasoningMode>,
    model_id: &str,
) -> Result<(), RuntimeFailure> {
    let parsed = crate::headless_model_parameters::parse_plan_model_id(model_id)?;
    let encoded = parsed.parameters.effort().cloned();
    let mut planned = plan
        .requirements()
        .capabilities()
        .filter(|required| required.capability() == Capability::ReasoningSelection);
    let Some(planned_requirement) = planned.next() else {
        if requested.is_some() || encoded.is_some() {
            return Err(unsupported("unplanned reasoning selection"));
        }
        return Ok(());
    };
    if planned.next().is_some() {
        return Err(plan_mismatch("reasoning capability"));
    }
    let Some(requested) = requested else {
        return Err(unsupported("missing reasoning selection"));
    };
    let mut constraints = planned_requirement.constraints();
    let Some(planned_constraint) = constraints.next() else {
        return Err(plan_mismatch("reasoning constraint"));
    };
    if constraints.next().is_some() {
        return Err(plan_mismatch("reasoning constraint"));
    }
    let CapabilityConstraint::ReasoningMode(planned_effort) = planned_constraint else {
        return Err(plan_mismatch("reasoning constraint"));
    };
    let Some(encoded) = encoded else {
        return Err(plan_mismatch("rendered model effort"));
    };
    if planned_effort != requested || planned_effort != &encoded {
        return Err(plan_mismatch("reasoning selection"));
    }
    Ok(())
}
