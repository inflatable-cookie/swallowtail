#[path = "selection/interface.rs"]
mod interface;

pub use interface::{
    LLAMA_CPP_ATTACHED_RUNTIME_REVISION, LLAMA_CPP_OWNED_RUNTIME_REVISION,
    llama_cpp_attached_runtime_binding, llama_cpp_attached_runtime_claim,
    llama_cpp_owned_runtime_binding, llama_cpp_owned_runtime_claim,
};

use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    HostServiceKind, InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    ProtocolFacadeId, RuntimeReadiness, SupportAuthority,
};

/// Exact qualified build number for the externally attached route.
pub const LLAMA_CPP_ATTACHED_BUILD: &str = "9910";
/// Exact qualified source commit for the externally attached route.
pub const LLAMA_CPP_ATTACHED_COMMIT: &str = "f5525f7e7";
/// Exact qualified build number for the host-owned route.
pub const LLAMA_CPP_OWNED_BUILD: &str = "10069";
/// Exact qualified source commit for the host-owned route.
pub const LLAMA_CPP_OWNED_COMMIT: &str = "178a6c449";
/// Network-grant audience for an externally managed server.
pub const LLAMA_CPP_ATTACHED_ENDPOINT_AUDIENCE: &str = "llama.cpp.attached";
/// Loopback network-grant audience for a host-owned server.
pub const LLAMA_CPP_OWNED_ENDPOINT_AUDIENCE: &str = "llama.cpp.owned-loopback";
/// Local-unauthenticated access-profile identifier for attached serving.
pub const LLAMA_CPP_ATTACHED_ACCESS_PROFILE_ID: &str = "llama-cpp.attached.local-unauthenticated";
/// Local-unauthenticated access-profile identifier for owned serving.
pub const LLAMA_CPP_OWNED_ACCESS_PROFILE_ID: &str = "llama-cpp.owned.local-unauthenticated";

const ATTACHED_FACADE: &str = "llama.cpp.openai-chat-completions.b9910";
const OWNED_FACADE: &str = "llama.cpp.openai-chat-completions.b10069";

/// Builds the local-unauthenticated profile for an attached server.
#[must_use]
pub fn llama_cpp_attached_access_profile() -> AccessProfile {
    access_profile(
        LLAMA_CPP_ATTACHED_ACCESS_PROFILE_ID,
        LLAMA_CPP_ATTACHED_ENDPOINT_AUDIENCE,
    )
}

/// Builds the local-unauthenticated profile for a host-owned loopback server.
#[must_use]
pub fn llama_cpp_owned_access_profile() -> AccessProfile {
    access_profile(
        LLAMA_CPP_OWNED_ACCESS_PROFILE_ID,
        LLAMA_CPP_OWNED_ENDPOINT_AUDIENCE,
    )
}

pub(crate) fn attached_instance(
    id: ConfiguredInstanceId,
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id,
        revision,
        crate::llama_cpp_attached_descriptor()
            .identity()
            .id()
            .clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access,
        SupportAuthority::IntegrationMaintainerSupported,
        valid(ProtocolFacadeId::new, ATTACHED_FACADE),
        valid(InstancePolicyId::new, "attached-text-only"),
        CapabilityProfile::new(attached_all_capabilities()),
    )
    .with_interface_versions([llama_cpp_attached_runtime_binding()])
}

pub(crate) fn owned_instance(
    id: ConfiguredInstanceId,
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id,
        revision,
        crate::llama_cpp_owned_descriptor().identity().id().clone(),
        host,
        target,
        InstanceOwnership::HostOwnedEphemeral,
        access,
        SupportAuthority::IntegrationMaintainerSupported,
        valid(ProtocolFacadeId::new, OWNED_FACADE),
        valid(InstancePolicyId::new, "owned-offline-loopback"),
        CapabilityProfile::new(owned_capabilities()),
    )
    .with_interface_versions([llama_cpp_owned_runtime_binding()])
}

pub(crate) fn model_route(
    instance: ConfiguredInstanceId,
    route: ModelRouteId,
    revision: ModelRouteRevision,
    model: ModelId,
    capabilities: Vec<CapabilityRequirement>,
) -> ModelRoute {
    ModelRoute::new(
        route,
        revision,
        instance,
        model,
        CapabilityProfile::new(capabilities),
    )
}

pub(crate) fn attached_requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
    role: DriverRole,
) -> OperationRequirements {
    let capabilities = attached_capabilities(role);
    requirements(
        host,
        access,
        role,
        InstanceOwnership::ExternalAttached,
        crate::llama_cpp_attached_descriptor().required_host_services(role),
        capabilities,
    )
    .with_interface_versions([llama_cpp_attached_runtime_binding()])
}

pub(crate) fn owned_requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
) -> OperationRequirements {
    requirements(
        host,
        access,
        DriverRole::ServingInstanceLifecycle,
        InstanceOwnership::HostOwnedEphemeral,
        crate::llama_cpp_owned_descriptor()
            .required_host_services(DriverRole::ServingInstanceLifecycle),
        owned_capabilities(),
    )
    .with_interface_versions([llama_cpp_owned_runtime_binding()])
    .require_model_route()
}

pub(crate) fn attached_capabilities(role: DriverRole) -> Vec<CapabilityRequirement> {
    if role == DriverRole::ModelCatalog {
        vec![CapabilityRequirement::new(Capability::ModelCatalog, [])]
    } else {
        [
            Capability::StructuredRun,
            Capability::StreamingEvents,
            Capability::UsageReporting,
            Capability::OutputTokenLimit,
        ]
        .into_iter()
        .map(|capability| CapabilityRequirement::new(capability, []))
        .collect()
    }
}

pub(crate) fn owned_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::OwnedServingInstance,
            )],
        ),
    ]
}

fn attached_all_capabilities() -> Vec<CapabilityRequirement> {
    let mut capabilities = attached_capabilities(DriverRole::StructuredRun);
    capabilities.push(CapabilityRequirement::new(Capability::ModelCatalog, []));
    capabilities
}

fn access_profile(id: &str, audience: &str) -> AccessProfile {
    AccessProfile::new(
        valid(AccessProfileId::new, id),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::LocalCompute,
        valid(EndpointAudience::new, audience),
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
    role: DriverRole,
    ownership: InstanceOwnership,
    services: impl IntoIterator<Item = HostServiceKind>,
    capabilities: Vec<CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        role,
        host,
        AccessRequirement::new(access)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([ownership])
    .with_host_services(services)
    .with_capabilities(capabilities)
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static llama.cpp identity is valid")
}
