use std::num::NonZeroU32;
use swallowtail_adapter_pi::pi_rpc_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, ExtensionNamespace, HarnessIsolation, HarnessRpcPolicy,
    HarnessSchedulingBounds, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ProviderId, ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    Deadline, OpenSessionRequest, OperationContent, RequestId, RuntimeTurnId, TurnRequest,
    WorkingResourceRef,
};
use swallowtail_testkit::ExecutionTopologyFixture;

pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub credential: CredentialRef,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
    build_selection(
        host,
        ConfiguredInstanceId::new("pi.fixture.instance").expect("valid instance"),
        InstanceTargetRef::new("pi.fixture.pinned-executable").expect("valid target"),
        WorkingResourceRef::new("pi.fixture.workspace").expect("valid resource"),
    )
}

pub fn selection_for_topology(topology: &ExecutionTopologyFixture) -> FixtureSelection {
    build_selection(
        topology.execution_host_id().clone(),
        topology.configured_instance_id().clone(),
        topology.instance_target().clone(),
        topology.working_resource().clone(),
    )
}

fn build_selection(
    host: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    instance_target: InstanceTargetRef,
    resource: WorkingResourceRef,
) -> FixtureSelection {
    let descriptor = pi_rpc_descriptor();
    let credential = CredentialRef::new("pi.fixture.delegated-auth").expect("valid credential");
    let access_id = AccessProfileId::new("pi.fixture.harness-auth").expect("valid access id");
    let capability_requirements = capabilities();
    let capabilities = CapabilityProfile::new(capability_requirements.clone());
    let version = InterfaceVersionBinding::new(
        InterfaceVersionAxis::new("pi.package").expect("valid axis"),
        InterfaceVersion::new("0.80.10").expect("valid version"),
    );
    let rpc_policy = rpc_policy();
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        descriptor.identity().id().clone(),
        host.clone(),
        instance_target,
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("pi-rpc-0.80.10-strict-lf").expect("valid facade"),
        InstancePolicyId::new("pi.fixture.ambient-read").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_rpc_policy(rpc_policy.clone());
    let route = ModelRoute::new(
        ModelRouteId::new("pi.fixture.route").expect("valid route"),
        ModelRouteRevision::new("fixture-route-revision").expect("valid route revision"),
        instance_id,
        ModelId::new("fixture-model").expect("valid model"),
        capabilities,
    )
    .with_provider_id(ProviderId::new("fixture-provider").expect("valid provider"));
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
    let services = service_kinds();
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(services)
    .with_capabilities(capability_requirements)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
    .with_interface_versions([version])
    .with_harness_rpc_policy(rpc_policy)
    .require_model_route();
    let plan = preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, services)
            .with_model_route(&route),
        &requirements,
    )
    .expect("Pi fixture preflight succeeds");
    FixtureSelection {
        plan,
        credential,
        resource,
    }
}

pub fn open_request(id: &str, resource: WorkingResourceRef) -> OpenSessionRequest {
    OpenSessionRequest::new(RequestId::new(id).expect("valid request"), resource, None)
        .with_access_policy(SessionAccessPolicy::ambient_harness(ResourceAccess::Read))
}

pub fn turn_request(id: &str, deadline: Deadline) -> TurnRequest {
    TurnRequest::new(
        RuntimeTurnId::new(id).expect("valid turn"),
        OperationContent::new("fixture private prompt").expect("valid prompt"),
    )
    .with_deadline(deadline)
}

fn service_kinds() -> [HostServiceKind; 5] {
    [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::Time,
    ]
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

fn capabilities() -> Vec<CapabilityRequirement> {
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
