use super::super::super::ClaudeCodePreparedIntegration;
use swallowtail_core::{
    AccessRequirement, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    CredentialState, Diagnostic, EndpointAuthorization, EntitlementState, ExecutionLayer,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, ModelRoute,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ReasoningMode,
    RuntimeReadiness, SafeDiagnostic, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

pub(super) fn instance_with_capabilities(
    prepared: &ClaudeCodePreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    let base = prepared.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        base.driver_id().clone(),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        capabilities,
    )
    .with_interface_versions(base.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn requirements(
    prepared: &ClaudeCodePreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    watchers: bool,
) -> OperationRequirements {
    let mut host_services = vec![
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ];
    if watchers {
        host_services.extend([
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
            HostServiceKind::Watcher,
            HostServiceKind::WatcherBridge,
        ]);
    }
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::StructuredRun,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route()
}

pub(super) fn build_plan(
    prepared: &ClaudeCodePreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    let descriptor = crate::claude_code_headless_descriptor();
    let context = PreflightContext::new(
        &descriptor,
        instance,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
    .with_model_route(route);
    preflight(&context, requirements).map_err(|error| {
        PreparationFailure::new(
            PreparationStage::Preflight,
            Diagnostic::new(error.diagnostic().clone()),
        )
    })
}

pub(super) fn failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}

/// Reports whether the prepared Claude Code version is exactly qualified.
///
/// Research 226 probed every published version in the maintained window.
/// `AllowUnverified` also permits later stable points on the same axis, but
/// no artifact for one has been probed, so a maximum-turn selection must
/// reject them before process work rather than assume the parser, the loop
/// guard, and the terminal shape all survived.
pub(super) fn qualifies_maximum_turns(prepared: &ClaudeCodePreparedIntegration) -> bool {
    crate::claude_code_maximum_turns::admits(prepared.observation().version())
}

pub(super) fn qualifies_watchers(prepared: &ClaudeCodePreparedIntegration) -> bool {
    crate::claude_code_watcher::admits(prepared.observation().version())
}

pub(super) fn operation_capabilities(
    available: &swallowtail_core::CapabilityProfile,
    reasoning: Option<&ReasoningMode>,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| *capability != swallowtail_core::Capability::ReasoningSelection)
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = reasoning {
        capabilities.push(CapabilityRequirement::new(
            swallowtail_core::Capability::ReasoningSelection,
            [swallowtail_core::CapabilityConstraint::ReasoningMode(
                mode.clone(),
            )],
        ));
    }
    capabilities
}
