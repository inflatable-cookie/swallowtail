use std::cell::Cell;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, ExtensionNamespace, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    ModelId, ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, PreflightFailure, PreflightPlan, ProtocolFacadeId, ResourceAccess,
    ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy, SupportAuthority,
    TransportFamilyId, preflight,
};
use swallowtail_runtime::WorkingResourceRef;

const ACCESS_ID: &str = "fixture.access.session";
const DRIVER_ID: &str = "fixture.session.driver";
const INSTANCE_ID: &str = "fixture.session.instance";
const ROUTE_ID: &str = "fixture.session.route";
const OBSERVED_EXTENSION: &str = "fixture.session/provider-request-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionAccessFixtureCase {
    ReadOnly,
    BoundedWorkspace,
    MissingWriteCapability,
    MissingWorkingResourceService,
    UnboundProviderRequest,
}

pub struct SessionAccessPreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    available_services: Vec<HostServiceKind>,
    policy: SessionAccessPolicy,
    working_resource: WorkingResourceRef,
    provider_side_effects: Cell<usize>,
}

impl SessionAccessPreflightFixture {
    #[must_use]
    pub fn for_case(case: SessionAccessFixtureCase, host_id: ExecutionHostId) -> Self {
        let access_id = AccessProfileId::new(ACCESS_ID).expect("fixture access id is valid");
        let extension =
            ExtensionNamespace::new(OBSERVED_EXTENSION).expect("fixture extension is valid");
        let bounded = case != SessionAccessFixtureCase::ReadOnly;
        let policy = if bounded {
            SessionAccessPolicy::bounded_workspace([extension.clone()])
        } else {
            SessionAccessPolicy::read_only()
        };

        let mut capabilities = vec![CapabilityRequirement::new(
            Capability::InteractiveSession,
            [],
        )];
        if bounded && case != SessionAccessFixtureCase::MissingWriteCapability {
            capabilities.push(CapabilityRequirement::new(
                Capability::WorkingResource,
                [
                    CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
                    CapabilityConstraint::ResourceRepresentation(
                        ResourceRepresentation::Filesystem,
                    ),
                ],
            ));
        }
        let capability_profile = CapabilityProfile::new(capabilities.clone());
        let descriptor = DriverDescriptor::new(
            AdapterIdentity::new(
                AdapterId::new(DRIVER_ID).expect("fixture driver id is valid"),
                AdapterVersion::new("fixture-version-1").expect("fixture version is valid"),
            ),
            IntegrationFamilyId::new("fixture-session-family").expect("fixture family is valid"),
            TransportFamilyId::new("fixture-session-transport")
                .expect("fixture transport is valid"),
        )
        .with_roles([DriverRole::InteractiveSession])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::InteractiveSession])
        .with_required_host_services(
            DriverRole::InteractiveSession,
            [HostServiceKind::Task, HostServiceKind::Process],
        )
        .with_extension_namespaces([extension.clone()]);
        let instance_id =
            ConfiguredInstanceId::new(INSTANCE_ID).expect("fixture instance id is valid");
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            InstanceRevision::new("fixture-revision-1").expect("fixture revision is valid"),
            descriptor.identity().id().clone(),
            host_id.clone(),
            InstanceTargetRef::new("fixture-session-target").expect("fixture target is valid"),
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture-session-facade").expect("fixture facade is valid"),
            InstancePolicyId::new("fixture-session-policy").expect("fixture policy is valid"),
            capability_profile.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new(ROUTE_ID).expect("fixture route id is valid"),
            ModelRouteRevision::new("fixture-route-revision-1")
                .expect("fixture route revision is valid"),
            instance_id,
            ModelId::new("fixture-session-model").expect("fixture model id is valid"),
            capability_profile,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::InteractiveOauth,
            EntitlementMetering::SubscriptionAllowance,
            EndpointAudience::new("fixture-session-audience").expect("fixture audience is valid"),
            SupportAuthority::ProviderSupported,
        );
        let access_status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let access_requirement = AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let mut required_services = vec![HostServiceKind::Task, HostServiceKind::Process];
        if bounded && case != SessionAccessFixtureCase::MissingWorkingResourceService {
            required_services.push(HostServiceKind::WorkingResource);
        }
        let mut requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            host_id,
            access_requirement,
        )
        .with_ownership_modes([InstanceOwnership::HostOwnedPersistent])
        .with_host_services(required_services.clone())
        .with_capabilities(capabilities)
        .with_session_access_policy(policy.clone())
        .require_model_route();
        if bounded && case != SessionAccessFixtureCase::UnboundProviderRequest {
            requirements = requirements.with_extension_namespaces([extension]);
        }

        Self {
            driver: descriptor,
            instance,
            route,
            access_profile,
            access_status,
            requirements,
            available_services: required_services,
            policy,
            working_resource: WorkingResourceRef::new("fixture.session.resource")
                .expect("fixture resource is valid"),
            provider_side_effects: Cell::new(0),
        }
    }

    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                &self.access_status,
                self.available_services.iter().copied(),
            )
            .with_model_route(&self.route),
            &self.requirements,
        )
    }

    #[must_use]
    pub const fn policy(&self) -> &SessionAccessPolicy {
        &self.policy
    }

    #[must_use]
    pub const fn working_resource(&self) -> &WorkingResourceRef {
        &self.working_resource
    }

    pub fn record_provider_side_effect(&self) {
        self.provider_side_effects
            .set(self.provider_side_effects.get() + 1);
    }

    #[must_use]
    pub fn provider_side_effect_count(&self) -> usize {
        self.provider_side_effects.get()
    }
}
