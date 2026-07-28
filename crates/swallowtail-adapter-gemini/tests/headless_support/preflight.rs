use swallowtail_adapter_gemini::{
    GEMINI_CLI_HEADLESS_AXIS, GeminiCliPreparationInput, GeminiCliPreparationProbe,
    GeminiCliPreparedDriver, GeminiHeadlessPreparationInput, GeminiHeadlessPreparationProbe,
    gemini_headless_descriptor,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    InterfaceVersion, InterfaceVersionAxis, InterfaceVersionBinding, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    PreflightPlan, ProtocolFacadeId, ProviderId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, OperationPolicy, PreparedAccessEvidence,
    ProviderRetentionPolicy, RequestId, ScopeId, StructuredRunRequest, WorkingResourceRef,
};

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
    let descriptor = gemini_headless_descriptor();
    let access = access_profile();
    let status = access_status(&access);
    let requirements = capabilities();
    let profile = CapabilityProfile::new(requirements.clone());
    let version = version_binding("0.52.0");
    let instance = ConfiguredInstance::new(
        instance_id,
        InstanceRevision::new("1").expect("revision is valid"),
        descriptor.identity().id().clone(),
        host.clone(),
        target,
        InstanceOwnership::HostOwnedEphemeral,
        access.id().clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("gemini-headless-stream-json-v1").expect("facade is valid"),
        InstancePolicyId::new("gemini-headless-ambient-plan").expect("policy is valid"),
        profile.clone(),
    )
    .with_interface_versions([version.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let route = ModelRoute::new(
        ModelRouteId::new("gemini-headless-model-route").expect("route id is valid"),
        ModelRouteRevision::new("1").expect("route revision is valid"),
        instance.id().clone(),
        ModelId::new("gemini-2.5-flash").expect("model id is valid"),
        profile,
    )
    .with_provider_id(ProviderId::new("gemini").expect("provider id is valid"));
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
        AccessRequirement::new(access.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services(host_services)
    .with_capabilities(requirements)
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_interface_versions([version])
    .require_model_route();
    preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
            .with_model_route(&route),
        &operation,
    )
    .expect("Gemini headless fixture preflight succeeds")
}

pub fn request_for(id: &str, working_resource: WorkingResourceRef) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("fixture-private-prompt").expect("content is valid"),
        OperationPolicy::offline()
            .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
            .with_harness_isolation(HarnessIsolation::AmbientHost)
            .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient),
    )
    .with_working_resource(working_resource)
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

pub fn preparation_input(host: ExecutionHostId) -> GeminiHeadlessPreparationInput {
    let access = access_profile();
    let status = access_status(&access);
    GeminiHeadlessPreparationInput::new(
        ConfiguredInstanceId::new("gemini-headless.prepared").expect("instance id is valid"),
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.fixture.executable").expect("executable is valid"),
            InterfaceVersionAxis::new(GEMINI_CLI_HEADLESS_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("gemini.fixture.environment").expect("environment is valid"),
        access,
        PreparedAccessEvidence::caller_asserted(status),
    )
}

pub fn probe() -> GeminiHeadlessPreparationProbe {
    GeminiHeadlessPreparationProbe::new(
        RequestId::new("gemini-headless-preparation").expect("request id is valid"),
        ScopeId::new("gemini-headless-preparation").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

pub fn cli_preparation_input(host: ExecutionHostId) -> GeminiCliPreparationInput {
    let access = access_profile();
    let status = access_status(&access);
    GeminiCliPreparationInput::new(
        GeminiCliPreparedDriver::Headless,
        ConfiguredInstanceId::new("gemini-headless.prepared").expect("instance id is valid"),
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.fixture.executable").expect("executable is valid"),
            InterfaceVersionAxis::new(GEMINI_CLI_HEADLESS_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("gemini.fixture.environment").expect("environment is valid"),
        access,
        PreparedAccessEvidence::caller_asserted(status),
    )
}

pub fn cli_probe() -> GeminiCliPreparationProbe {
    GeminiCliPreparationProbe::new(
        RequestId::new("gemini-headless-preparation").expect("request id is valid"),
        ScopeId::new("gemini-headless-preparation").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("access.gemini-headless").expect("access id is valid"),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new("gemini-developer-api").expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(
        CredentialRef::new("gemini.fixture.api-key").expect("credential is valid"),
    )
}

fn access_status(access: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn version_binding(version: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        InterfaceVersionAxis::new(GEMINI_CLI_HEADLESS_AXIS).expect("axis is valid"),
        InterfaceVersion::new(version).expect("version is valid"),
    )
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
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
