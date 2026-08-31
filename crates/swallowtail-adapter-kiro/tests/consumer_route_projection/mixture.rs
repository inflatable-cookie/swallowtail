use swallowtail_adapter_kiro::{KiroPreparedIntegration, kiro_acp_descriptor};
use swallowtail_core::{
    AccessRequirement, AccessStatus, AdapterId, AdapterIdentity, ConfiguredInstance,
    DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, IntegrationFamilyId, OperationRequirements, OperationShape, PreflightContext,
    PreflightFailure, PreflightPlan, ResourceAccess, SessionProviderStatePolicy, TransportFamilyId,
    preflight,
};
use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow, SessionAccessPolicy,
};

/// Exact published driver identity of the other candidate H ACP route.
///
/// `deepagents.acp` is a separate release-leaf package, so this proof rebuilds its
/// exact route identity from public core types instead of adding a
/// cross-adapter dependency the release staging does not admit. Every other
/// applicability dimension is copied from this prepared session, so the
/// composer rejection isolates route identity alone.
pub(super) const NEIGHBOUR_DRIVER_ID: &str = "swallowtail.deepagents.acp";

/// One source id both snapshots name, so only route or access evidence differs.
pub(super) const SHARED_SOURCE: &str = "candidate-h.shared-source";

/// Exact host services the prepared interactive session requires.
pub(super) const HOST_SERVICES: [HostServiceKind; 3] = [
    HostServiceKind::Task,
    HostServiceKind::Process,
    HostServiceKind::WorkingResource,
];

/// Requires the mixed assembly to fail closed rather than publish the row.
pub(super) fn assert_rejects(
    applicability: ConsumerRouteApplicability,
    mine: &ConsumerRouteProjectionContribution,
    row: ConsumerRouteProjectionRow,
) {
    let rejection = ConsumerRouteProjectionContribution::new(
        applicability,
        mine.sources().cloned().collect::<Vec<_>>(),
        [row],
        [],
        [],
    )
    .expect_err("a row proved under other evidence cannot join this snapshot");
    assert_eq!(
        rejection.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

/// Copies one published row onto another exact applicability.
pub(super) fn rebind(
    row: &ConsumerRouteProjectionRow,
    applicability: ConsumerRouteApplicability,
) -> ConsumerRouteProjectionRow {
    ConsumerRouteProjectionRow::new(
        row.identity().clone(),
        applicability,
        row.source().clone(),
        row.source_class(),
        row.evidence_strength(),
        row.lifecycle(),
    )
    .with_support(row.support())
    .with_availability(row.availability())
    .with_actor_posture(row.actor_posture())
    .with_state_support(row.state_support())
}

pub(super) fn assert_route_identity_is_the_only_difference(
    shifted: &ConsumerRouteApplicability,
    ready: &ConsumerRouteApplicability,
) {
    assert_ne!(shifted.driver_identity(), ready.driver_identity());
    assert_eq!(shifted.instance_id(), ready.instance_id());
    assert_eq!(shifted.instance_revision(), ready.instance_revision());
    assert_eq!(shifted.instance_policy_id(), ready.instance_policy_id());
    assert_eq!(shifted.protocol_facade_id(), ready.protocol_facade_id());
    assert_shared_dimensions(shifted, ready);
    assert_ne!(shifted, ready, "one exact route identity separates them");
}

pub(super) fn assert_access_is_the_only_difference(
    shifted: &ConsumerRouteApplicability,
    ready: &ConsumerRouteApplicability,
) {
    assert_eq!(shifted.driver_identity(), ready.driver_identity());
    assert_eq!(shifted.instance_id(), ready.instance_id());
    assert_eq!(shifted.instance_revision(), ready.instance_revision());
    assert_eq!(shifted.instance_policy_id(), ready.instance_policy_id());
    assert_eq!(shifted.protocol_facade_id(), ready.protocol_facade_id());
    assert_shared_dimensions(shifted, ready);
    assert_ne!(shifted, ready, "one exact access dimension separates them");
}

fn assert_shared_dimensions(
    shifted: &ConsumerRouteApplicability,
    ready: &ConsumerRouteApplicability,
) {
    assert_eq!(shifted.execution_host_id(), ready.execution_host_id());
    assert_eq!(shifted.driver_role(), ready.driver_role());
    assert_eq!(shifted.execution_layer(), ready.execution_layer());
    assert_eq!(shifted.operation_shape(), ready.operation_shape());
    assert_eq!(shifted.model(), ready.model());
    assert_eq!(shifted.access_profile_id(), ready.access_profile_id());
    assert_eq!(shifted.credential_mechanism(), ready.credential_mechanism());
    assert_eq!(shifted.resource_access(), ready.resource_access());
    assert_eq!(shifted.filesystem_boundary(), ready.filesystem_boundary());
}

pub(super) fn ready(integration: &KiroPreparedIntegration) -> &AccessStatus {
    integration.access_evidence().status()
}

pub(super) fn neighbour_driver_id() -> AdapterId {
    AdapterId::new(NEIGHBOUR_DRIVER_ID).expect("neighbour driver id is valid")
}

pub(super) fn neighbour_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            neighbour_driver_id(),
            kiro_acp_descriptor().identity().version().clone(),
        ),
        IntegrationFamilyId::new("deepagents-acp").expect("family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("transport id is valid"),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(DriverRole::InteractiveSession, HOST_SERVICES)
}

/// Rebuilds the prepared configured instance, optionally under another route.
pub(super) fn instance(
    integration: &KiroPreparedIntegration,
    driver_id: Option<AdapterId>,
) -> ConfiguredInstance {
    let base = integration.instance();
    ConfiguredInstance::new(
        base.id().clone(),
        base.revision().clone(),
        driver_id.unwrap_or_else(|| base.driver_id().clone()),
        base.execution_host_id().clone(),
        base.target_reference().clone(),
        base.ownership(),
        base.access_profile_id().clone(),
        base.support_authority(),
        base.protocol_facade_id().clone(),
        base.policy_id().clone(),
        base.capabilities().clone(),
    )
    .with_interface_versions(base.interface_versions().cloned())
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

pub(super) fn plan_with(
    descriptor: &DriverDescriptor,
    instance: &ConfiguredInstance,
    integration: &KiroPreparedIntegration,
    observed: &AccessStatus,
) -> Result<PreflightPlan, PreflightFailure> {
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        instance.execution_host_id().clone(),
        AccessRequirement::new(instance.access_profile_id().clone())
            .with_credential_states([observed.credential()])
            .with_entitlement_states([observed.entitlement()])
            .with_endpoint_authorizations([observed.endpoint_authorization()])
            .with_runtime_readiness([observed.runtime_readiness()])
            .with_support_authorities([observed.support_authority()]),
    )
    .with_ownership_modes([instance.ownership()])
    .with_host_services(HOST_SERVICES)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
    let context = PreflightContext::new(
        descriptor,
        instance,
        integration.access_profile(),
        observed,
        integration.available_host_services(),
    );
    preflight(&context, &requirements)
}
