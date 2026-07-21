use swallowtail_adapter_qwen::qwen_headless_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    ExtensionNamespace, HarnessIsolation, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ProviderId, ResourceAccess, ResourceRepresentation, RuntimeReadiness, SupportAuthority,
    preflight,
};
use swallowtail_runtime::{
    Deadline, MonotonicInstant, OperationContent, OperationPolicy, ProviderRetentionPolicy,
    RequestId, StructuredRunRequest, WorkingResourceRef,
};

pub fn plan() -> PreflightPlan {
    bound_plan(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        ConfiguredInstanceId::new("qwen-headless.local").expect("instance id is valid"),
        InstanceTargetRef::new("qwen-executable").expect("target is valid"),
    )
}

pub fn plan_for(topology: &swallowtail_testkit::ExecutionTopologyFixture) -> PreflightPlan {
    bound_plan(
        topology.execution_host_id().clone(),
        topology.configured_instance_id().clone(),
        topology.instance_target().clone(),
    )
}

fn bound_plan(
    host: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
) -> PreflightPlan {
    let descriptor = qwen_headless_descriptor();
    let access_id = AccessProfileId::new("access.qwen-headless").expect("access id is valid");
    let requirements = capabilities();
    let profile = CapabilityProfile::new(requirements.clone());
    let instance = ConfiguredInstance::new(
        instance_id,
        InstanceRevision::new("1").expect("revision is valid"),
        descriptor.identity().id().clone(),
        host.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("qwen-code-v0.19.11-stream-json").expect("facade is valid"),
        InstancePolicyId::new("read-only-ambient-host").expect("policy is valid"),
        profile.clone(),
    );
    let route = ModelRoute::new(
        ModelRouteId::new("qwen-model-route").expect("route id is valid"),
        ModelRouteRevision::new("1").expect("route revision is valid"),
        instance.id().clone(),
        ModelId::new("qwen3-coder-plus").expect("model id is valid"),
        profile,
    )
    .with_provider_id(ProviderId::new("alibaba-modelstudio").expect("provider id is valid"));
    let namespace = ExtensionNamespace::new("qwen-code/delegated-harness-auth")
        .expect("access namespace is valid");
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ProviderSpecific(namespace),
        EntitlementMetering::Unknown,
        EndpointAudience::new("qwen-code").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let host_services = [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
    ];
    let operation = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::StructuredRun,
        DriverRole::StructuredRun,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Unknown])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(host_services)
    .with_capabilities(requirements)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
            .with_model_route(&route),
        &operation,
    )
    .expect("Qwen fixture preflight succeeds")
}

pub fn request(id: &str) -> StructuredRunRequest {
    request_for(id, working_resource())
}

pub fn request_for(id: &str, resource: WorkingResourceRef) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("fixture-private-prompt").expect("content is valid"),
        OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost),
    )
    .with_working_resource(resource)
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

pub fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource is valid")
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
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
