#[path = "selection/interface.rs"]
mod interface;

pub use interface::{
    BEDROCK_CATALOGUE_SERVICE_REVISION, BEDROCK_RUNTIME_SERVICE_REVISION,
    bedrock_catalogue_interface_bindings, bedrock_catalogue_interface_claims,
    bedrock_runtime_interface_bindings, bedrock_runtime_interface_claims,
};

use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, ProtocolFacadeId,
    ProviderId, RuntimeReadiness, SupportAuthority,
};

pub const BEDROCK_RUNTIME_ENDPOINT_AUDIENCE: &str = "bedrock-runtime";
pub const BEDROCK_CONTROL_PLANE_ENDPOINT_AUDIENCE: &str = "bedrock";
pub const BEDROCK_RUNTIME_ACCESS_PROFILE_ID: &str =
    "amazon-bedrock.runtime.cloud-provider-identity";
pub const BEDROCK_CATALOGUE_ACCESS_PROFILE_ID: &str =
    "amazon-bedrock.catalogue.cloud-provider-identity";
pub const BEDROCK_RUNTIME_FACADE_REVISION: &str = "bedrock-converse-stream";
pub const BEDROCK_CATALOGUE_FACADE_REVISION: &str = "bedrock-list-foundation-models";
pub const BEDROCK_RUNTIME_INSTANCE_POLICY_ID: &str = "aws-delegated-runtime";
pub const BEDROCK_CATALOGUE_INSTANCE_POLICY_ID: &str = "aws-delegated-catalogue";

#[must_use]
pub fn bedrock_runtime_access_profile(credential: CredentialRef) -> AccessProfile {
    access_profile(
        BEDROCK_RUNTIME_ACCESS_PROFILE_ID,
        BEDROCK_RUNTIME_ENDPOINT_AUDIENCE,
        credential,
    )
}

#[must_use]
pub fn bedrock_catalogue_access_profile(credential: CredentialRef) -> AccessProfile {
    access_profile(
        BEDROCK_CATALOGUE_ACCESS_PROFILE_ID,
        BEDROCK_CONTROL_PLANE_ENDPOINT_AUDIENCE,
        credential,
    )
}

pub(crate) fn runtime_instance(
    id: ConfiguredInstanceId,
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id,
        revision,
        crate::bedrock_direct_descriptor().identity().id().clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access,
        SupportAuthority::ProviderSupported,
        valid(ProtocolFacadeId::new, BEDROCK_RUNTIME_FACADE_REVISION),
        valid(InstancePolicyId::new, BEDROCK_RUNTIME_INSTANCE_POLICY_ID),
        CapabilityProfile::new(runtime_capabilities()),
    )
    .with_interface_versions(bedrock_runtime_interface_bindings())
}

pub(crate) fn catalogue_instance(
    id: ConfiguredInstanceId,
    revision: InstanceRevision,
    host: ExecutionHostId,
    target: InstanceTargetRef,
    access: AccessProfileId,
) -> ConfiguredInstance {
    ConfiguredInstance::new(
        id,
        revision,
        crate::bedrock_catalogue_descriptor()
            .identity()
            .id()
            .clone(),
        host,
        target,
        InstanceOwnership::ExternalAttached,
        access,
        SupportAuthority::ProviderSupported,
        valid(ProtocolFacadeId::new, BEDROCK_CATALOGUE_FACADE_REVISION),
        valid(InstancePolicyId::new, BEDROCK_CATALOGUE_INSTANCE_POLICY_ID),
        CapabilityProfile::new(catalogue_capabilities()),
    )
    .with_interface_versions(bedrock_catalogue_interface_bindings())
}

pub(crate) fn runtime_model_route(
    instance: ConfiguredInstanceId,
    route: ModelRouteId,
    revision: ModelRouteRevision,
    model: ModelId,
    provider: ProviderId,
) -> ModelRoute {
    ModelRoute::new(
        route,
        revision,
        instance,
        model,
        CapabilityProfile::new(runtime_capabilities()),
    )
    .with_provider_id(provider)
}

pub(crate) fn runtime_requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
) -> OperationRequirements {
    requirements(
        host,
        access,
        DriverRole::StructuredRun,
        crate::bedrock_direct_descriptor().required_host_services(DriverRole::StructuredRun),
        runtime_capabilities(),
    )
    .with_interface_versions(bedrock_runtime_interface_bindings())
    .require_model_route()
}

pub(crate) fn catalogue_requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
) -> OperationRequirements {
    requirements(
        host,
        access,
        DriverRole::ModelCatalog,
        crate::bedrock_catalogue_descriptor().required_host_services(DriverRole::ModelCatalog),
        catalogue_capabilities(),
    )
    .with_interface_versions(bedrock_catalogue_interface_bindings())
}

fn access_profile(id: &str, audience: &str, credential: CredentialRef) -> AccessProfile {
    AccessProfile::new(
        valid(AccessProfileId::new, id),
        CredentialMechanism::CloudProviderIdentity,
        EntitlementMetering::CloudAccountBilling,
        valid(EndpointAudience::new, audience),
        SupportAuthority::ProviderSupported,
    )
    .with_credential_reference(credential)
}

fn requirements(
    host: ExecutionHostId,
    access: AccessProfileId,
    role: DriverRole,
    services: impl IntoIterator<Item = HostServiceKind>,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::DirectModelInference,
        OperationShape::StructuredRun,
        role,
        host,
        AccessRequirement::new(access)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services(services)
    .with_capabilities(capabilities)
}

fn runtime_capabilities() -> Vec<CapabilityRequirement> {
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

fn catalogue_capabilities() -> Vec<CapabilityRequirement> {
    vec![CapabilityRequirement::new(Capability::ModelCatalog, [])]
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static Bedrock identity is valid")
}
