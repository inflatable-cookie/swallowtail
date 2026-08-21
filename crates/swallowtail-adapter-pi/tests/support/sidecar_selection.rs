use std::num::NonZeroU32;
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, pi_sdk_sidecar_descriptor,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, HarnessRpcPolicy, HarnessSchedulingBounds, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, InterfaceVersion,
    InterfaceVersionAxis, InterfaceVersionBinding, ModelId, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext, PreflightPlan,
    ProtocolFacadeId, ProviderId, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::WorkingResourceRef;

pub struct SidecarFixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn sidecar_selection(host: ExecutionHostId) -> SidecarFixtureSelection {
    build_selection(
        host,
        DriverRole::InteractiveSession,
        sidecar_versions().to_vec(),
    )
}

/// Builds a session selection whose instance binds `instance_versions`. The
/// operation requirements request only the bindings the descriptor permits,
/// so driver-side version validation decides the outcome for off-point or
/// extra bindings.
pub fn sidecar_selection_with_instance_versions(
    host: ExecutionHostId,
    instance_versions: Vec<InterfaceVersionBinding>,
) -> SidecarFixtureSelection {
    build_selection(host, DriverRole::InteractiveSession, instance_versions)
}

pub fn sidecar_catalogue_selection(host: ExecutionHostId) -> SidecarFixtureSelection {
    build_selection(host, DriverRole::ModelCatalog, sidecar_versions().to_vec())
}

pub fn sidecar_versions() -> [InterfaceVersionBinding; 4] {
    [
        version(PI_SDK_SIDECAR_PACKAGE_AXIS, "0.84.2"),
        version(PI_SDK_SIDECAR_NODE_AXIS, "22.23.2"),
        version(PI_SDK_SIDECAR_WIRE_AXIS, "swallowtail-pi-sdk-jsonl-v1"),
        version(
            PI_SDK_SIDECAR_SIDECAR_AXIS,
            swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG,
        ),
    ]
}

fn version(axis: &str, value: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(axis).expect("valid axis"),
        InterfaceVersion::new(value).expect("valid version"),
    )
}

fn build_selection(
    host: ExecutionHostId,
    role: DriverRole,
    instance_versions: Vec<InterfaceVersionBinding>,
) -> SidecarFixtureSelection {
    let descriptor = pi_sdk_sidecar_descriptor();
    let requirement_versions: Vec<InterfaceVersionBinding> = instance_versions
        .iter()
        .filter(|binding| descriptor.permits_interface_version(binding))
        .cloned()
        .collect();
    let credential = CredentialRef::new("pi.fixture.delegated-auth").expect("valid credential");
    let access_id = AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id");
    let resource = WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource");
    let capability_requirements = capabilities(role);
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let instance_id =
        ConfiguredInstanceId::new("pi.fixture.sdk-sidecar-instance").expect("valid instance");
    let rpc_policy = rpc_policy();
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        InstanceTargetRef::new("pi.fixture.pinned-launch-recipe").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("pi-sdk-sidecar-jsonl-v1").expect("valid facade"),
        InstancePolicyId::new("pi-sdk-sidecar-ambient-read").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions(instance_versions)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_harness_rpc_policy(rpc_policy.clone());
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ProviderSpecific(
            ExtensionNamespace::new("pi/delegated-harness-auth").expect("valid namespace"),
        ),
        EntitlementMetering::Unknown,
        EndpointAudience::new("pi-harness").expect("valid audience"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(credential.clone());
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let services = service_kinds(role);
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        role,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services.clone())
    .with_capabilities(capability_requirements)
    .with_harness_configuration_posture(HarnessConfigurationPosture::ProviderSuppressed)
    .with_interface_versions(requirement_versions)
    .with_harness_rpc_policy(rpc_policy);
    let requirements = if role == DriverRole::InteractiveSession {
        requirements.with_harness_isolation(HarnessIsolation::AmbientHost)
    } else {
        requirements
    };
    let requirements = if role == DriverRole::InteractiveSession {
        let route = ModelRoute::new(
            ModelRouteId::new("pi.fixture.route").expect("valid route"),
            ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
            instance_id,
            ModelId::new("fixture-model").expect("valid model"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("fixture-provider").expect("valid provider"));
        let plan = preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, services)
                .with_model_route(&route),
            &requirements
                .with_session_access_policy(SessionAccessPolicy::ambient_harness(
                    ResourceAccess::Read,
                ))
                .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited)
                .require_model_route(),
        )
        .expect("sidecar fixture preflight succeeds");
        return SidecarFixtureSelection {
            plan,
            credential,
            resource,
        };
    } else {
        requirements
    };
    let plan = preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services),
        &requirements,
    )
    .expect("sidecar catalogue fixture preflight succeeds");
    SidecarFixtureSelection {
        plan,
        credential,
        resource,
    }
}

fn service_kinds(role: DriverRole) -> Vec<HostServiceKind> {
    let mut services = vec![
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::Time,
    ];
    if role == DriverRole::InteractiveSession {
        services.push(HostServiceKind::WorkingResource);
    }
    services
}

fn rpc_policy() -> HarnessRpcPolicy {
    let one = NonZeroU32::new(1).unwrap();
    HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(
        one,
        NonZeroU32::new(2).unwrap(),
        one,
        one,
    ))
}

fn capabilities(role: DriverRole) -> Vec<CapabilityRequirement> {
    if role == DriverRole::ModelCatalog {
        return vec![CapabilityRequirement::new(Capability::ModelCatalog, [])];
    }
    vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ]
}
