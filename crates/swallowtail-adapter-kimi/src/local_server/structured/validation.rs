use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, OperationShape, PreflightPlan,
    ResourceAccess, ResourceRepresentation,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure, StreamReattachmentPolicy,
    StructuredRunRequest, validate_harness_configuration_policy, validate_harness_isolation_policy,
};

pub(super) fn validate(
    driver: &super::super::KimiLocalServerDriver,
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    let configuration = driver.configuration()?;
    if plan.driver_identity().id() != crate::kimi_local_server_descriptor().identity().id()
        || plan.requirements().execution_layer() != ExecutionLayer::HarnessInteraction
        || plan.requirements().operation_shape() != OperationShape::StructuredRun
        || plan.requirements().driver_role() != DriverRole::StructuredRun
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("driver, operation, or model route"));
    }
    let claim = crate::kimi_local_server_claim();
    let mut bindings = plan
        .interface_versions()
        .filter(|binding| binding.axis() == claim.axis());
    let binding = bindings.next().ok_or_else(|| plan_mismatch("version"))?;
    if bindings.next().is_some()
        || claim.assess(binding.version()) != plan.assess_interface_version(binding)
        || !claim.permits(binding.version())
    {
        return Err(plan_mismatch("version"));
    }
    services.require_execution_host(plan.execution_host_id())?;
    for kind in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ] {
        if !plan
            .requirements()
            .host_services()
            .any(|required| required == kind)
            || !services.available_kinds().contains(&kind)
        {
            return Err(failure(
                "swallowtail.kimi.local_server.run_host_service_missing",
                "Kimi local-server structured run requires its preflight-bound host services",
            ));
        }
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    if request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(plan_mismatch("ambient harness authority"));
    }
    if request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::ManagedAllowed
        || !configuration.managed_recovery()
        || request.policy().stream_reattachment()
            != match configuration.maximum_reattachments() {
                0 => StreamReattachmentPolicy::Disabled,
                1 => {
                    StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
                }
                _ => return Err(unsupported("stream reattachment maximum")),
            }
    {
        return Err(unsupported("provider lifecycle policy"));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
    {
        return Err(unsupported("external network or search policy"));
    }
    if request.working_resource().is_none() || request.deadline().is_none() {
        return Err(unsupported("missing working resource or deadline"));
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
    if plan.requirements().session_access_policy().is_some()
        || plan
            .requirements()
            .session_provider_state_policy()
            .is_some()
    {
        return Err(plan_mismatch("operation-private session authority"));
    }
    require_capability(plan, Capability::StructuredRun)?;
    require_capability(plan, Capability::StreamingEvents)?;
    require_capability(plan, Capability::ProviderDurableRetention)?;
    require_capability(plan, Capability::ProviderManagedRecovery)?;
    if configuration.maximum_reattachments() == 1 {
        require_constraint(
            plan,
            Capability::StreamReattachment,
            CapabilityConstraint::ReattachmentMaximumCount(1),
        )?;
    }
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    match request.policy().reasoning_mode() {
        Some(mode) => require_constraint(
            plan,
            Capability::ReasoningSelection,
            CapabilityConstraint::ReasoningMode(mode.clone()),
        ),
        None => Ok(()),
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
        "swallowtail.kimi.local_server.run_plan_mismatch",
        format!("Kimi local-server run does not match its preflight-bound {dimension}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.run_input_unsupported",
        format!("Kimi local-server structured run does not support {feature}"),
    )
}
