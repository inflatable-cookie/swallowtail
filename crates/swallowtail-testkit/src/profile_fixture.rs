use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, CancellationScope, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, DriverDescriptor,
    EndpointAudience, EndpointAuthorization, EntitlementState, ExecutionHostId, ExtensionNamespace,
    HostServiceKind, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, PreflightContext,
    PreflightFailure, PreflightPlan, ProtocolFacadeId, RuntimeReadiness, SupportAuthority,
    TransportFamilyId, preflight,
};

use crate::{SyntheticProfile, profile_shape::ProfileShape};

pub(crate) struct ProfilePreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    available_services: Vec<HostServiceKind>,
}

impl ProfilePreflightFixture {
    pub(crate) fn new(profile: SyntheticProfile) -> Self {
        let shape = ProfileShape::for_profile(profile);
        let adapter_id = valid(AdapterId::new, shape.adapter_id);
        let access_profile_id = valid(AccessProfileId::new, shape.access_profile_id);
        let host_id = valid(ExecutionHostId::new, "fixture.host.profile");
        let capabilities = capability_profile(profile);
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                valid(AdapterVersion::new, "fixture-version-1"),
            ),
            valid(IntegrationFamilyId::new, shape.integration_family),
            valid(TransportFamilyId::new, shape.transport_family),
        )
        .with_roles([shape.role])
        .with_execution_layers([shape.layer])
        .with_operation_shapes([shape.operation_shape])
        .with_required_host_services(shape.role, shape.required_services.iter().copied())
        .with_extension_namespaces([extension_namespace()]);
        let instance_id = valid(ConfiguredInstanceId::new, shape.instance_id);
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            valid(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            host_id.clone(),
            valid(InstanceTargetRef::new, "fixture-profile-target"),
            shape.ownership,
            access_profile_id.clone(),
            SupportAuthority::ProviderSupported,
            valid(ProtocolFacadeId::new, shape.transport_family),
            valid(InstancePolicyId::new, "fixture-profile-policy"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            valid(ModelRouteId::new, shape.route_id),
            valid(ModelRouteRevision::new, "fixture-route-revision-1"),
            instance_id,
            valid(ModelId::new, shape.model_id),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_profile_id.clone(),
            shape.credential,
            shape.metering,
            valid(EndpointAudience::new, shape.audience),
            SupportAuthority::ProviderSupported,
        );
        let access_status = AccessStatus::new(
            access_profile_id.clone(),
            shape.credential_state,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let access_requirement = AccessRequirement::new(access_profile_id)
            .with_credential_states([shape.credential_state])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let requirements = OperationRequirements::new(
            shape.layer,
            shape.operation_shape,
            shape.role,
            host_id,
            access_requirement,
        )
        .with_ownership_modes([shape.ownership])
        .with_host_services(shape.required_services.iter().copied())
        .with_capabilities(capability_requirements(profile))
        .with_extension_namespaces([extension_namespace()])
        .require_model_route();

        Self {
            driver,
            instance,
            route,
            access_profile,
            access_status,
            requirements,
            available_services: shape.required_services,
        }
    }

    pub(crate) fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(&self.context(), &self.requirements)
    }

    pub(crate) fn context(&self) -> PreflightContext<'_> {
        PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            self.available_services.iter().copied(),
        )
        .with_model_route(&self.route)
    }

    pub(crate) const fn driver(&self) -> &DriverDescriptor {
        &self.driver
    }

    pub(crate) const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }
}

fn capability_profile(profile: SyntheticProfile) -> CapabilityProfile {
    CapabilityProfile::new(capability_requirements(profile))
}

fn capability_requirements(profile: SyntheticProfile) -> Vec<CapabilityRequirement> {
    let interruption_scope = match profile {
        SyntheticProfile::LongLivedRpcHarness => CancellationScope::ActiveTurn,
        SyntheticProfile::OwnedSelfHosted => CancellationScope::OwnedServingInstance,
        _ => CancellationScope::StructuredRun,
    };
    let mut capabilities = vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(interruption_scope)],
        ),
    ];
    match profile {
        SyntheticProfile::LongLivedRpcHarness => {
            capabilities.push(CapabilityRequirement::new(
                Capability::InteractiveSession,
                [],
            ));
            capabilities.push(CapabilityRequirement::new(Capability::Resume, []));
        }
        SyntheticProfile::AttachedSelfHosted | SyntheticProfile::OwnedSelfHosted => {}
        SyntheticProfile::OneShotStructuredCli | SyntheticProfile::HostedDirectApi => {
            capabilities.push(CapabilityRequirement::new(Capability::StructuredRun, []));
        }
    }
    capabilities
}

fn extension_namespace() -> ExtensionNamespace {
    valid(ExtensionNamespace::new, "fixture.profile/v1")
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static profile fixture text must be valid")
}
