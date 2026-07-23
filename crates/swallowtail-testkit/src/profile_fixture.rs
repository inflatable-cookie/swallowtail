use std::num::NonZeroU32;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, AttachedModelObservation, ConfiguredInstance, ConfiguredInstanceId,
    DriverDescriptor, EndpointAudience, EndpointAuthorization, EntitlementState, ExecutionHostId,
    ExtensionNamespace, HarnessConfigurationPosture, HarnessRpcPolicy, HarnessSchedulingBounds,
    HostServiceKind, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    ModelArtifactBinding, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, PreflightContext, PreflightFailure, PreflightPlan, ProtocolFacadeId,
    ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion, RuntimeReadiness,
    SupportAuthority, TransportFamilyId, preflight,
};

use crate::{SyntheticProfile, profile_shape::ProfileShape};

mod artifact;
pub(crate) mod attached_runtime;
mod capabilities;
mod interface_version;
mod managed;

pub(crate) struct ProfilePreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    artifact: Option<ModelArtifactBinding>,
    attached_model_observation: Option<AttachedModelObservation>,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    available_services: Vec<HostServiceKind>,
}

impl ProfilePreflightFixture {
    pub(crate) fn new(profile: SyntheticProfile) -> Self {
        Self::for_host(profile, valid(ExecutionHostId::new, "fixture.host.profile"))
    }

    pub(crate) fn for_host(profile: SyntheticProfile, host_id: ExecutionHostId) -> Self {
        let shape = ProfileShape::for_profile(profile);
        let adapter_id = valid(AdapterId::new, shape.adapter_id);
        let access_profile_id = valid(AccessProfileId::new, shape.access_profile_id);
        let capabilities = capabilities::profile(profile);
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
        let instance = if profile == SyntheticProfile::ProviderManagedRemoteHarness {
            instance.with_provider_agent(ProviderAgentBinding::new(
                valid(ProviderAgentId::new, "fixture-managed-agent"),
                valid(ProviderAgentVersion::new, "7"),
            ))
        } else {
            instance
        };
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
        let artifact =
            (profile == SyntheticProfile::OwnedSelfHosted).then(artifact::fixture_artifact);
        let mut requirements = OperationRequirements::new(
            shape.layer,
            shape.operation_shape,
            shape.role,
            host_id,
            access_requirement,
        )
        .with_ownership_modes([shape.ownership])
        .with_host_services(shape.required_services.iter().copied())
        .with_capabilities(capabilities::requirements(profile))
        .with_extension_namespaces([extension_namespace()])
        .require_model_route();
        if shape.operation_shape == swallowtail_core::OperationShape::InteractiveSession {
            requirements = requirements
                .with_session_access_policy(crate::profile_session_access::policy(profile));
        }
        if profile == SyntheticProfile::RealtimeMediaDirectSession {
            requirements =
                requirements.with_realtime_media(swallowtail_core::RealtimeMediaRequirements::new(
                    valid(ModelId::new, shape.model_id),
                    crate::realtime_media_fixture::realtime_media_config(),
                ));
        }
        if profile == SyntheticProfile::LocallyContinuedDirectSession {
            requirements = requirements.with_direct_continuation(
                swallowtail_core::DirectContinuationRequirements::new(
                    valid(ModelId::new, shape.model_id),
                    crate::direct_continuation_fixture::config(),
                ),
            );
        }

        Self {
            driver,
            instance,
            route,
            artifact,
            attached_model_observation: None,
            access_profile,
            access_status,
            requirements,
            available_services: shape.required_services,
        }
    }

    pub(crate) fn require_harness_rpc_policy(&mut self, policy: HarnessRpcPolicy) {
        self.requirements = self.requirements.clone().with_harness_rpc_policy(policy);
    }

    pub(crate) fn bind_harness_configuration(&mut self, posture: HarnessConfigurationPosture) {
        self.instance = self
            .instance
            .clone()
            .with_harness_configuration_posture(posture);
        self.requirements = self
            .requirements
            .clone()
            .with_harness_configuration_posture(posture);
    }

    pub(crate) fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        preflight(&self.context(), &self.requirements)
    }

    pub(crate) fn preflight_without_artifact(&self) -> Result<PreflightPlan, PreflightFailure> {
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

    pub(crate) fn context(&self) -> PreflightContext<'_> {
        let mut context = PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            self.available_services.iter().copied(),
        )
        .with_model_route(&self.route);
        if let Some(artifact) = &self.artifact {
            context = context.with_model_artifact(artifact);
        }
        if let Some(observation) = &self.attached_model_observation {
            context = context.with_attached_model_observation(observation);
        }
        context
    }

    pub(crate) const fn driver(&self) -> &DriverDescriptor {
        &self.driver
    }

    pub(crate) const fn instance(&self) -> &ConfiguredInstance {
        &self.instance
    }

    pub(crate) const fn artifact(&self) -> Option<&ModelArtifactBinding> {
        self.artifact.as_ref()
    }
}

pub(crate) fn restrictive_rpc_policy() -> HarnessRpcPolicy {
    HarnessRpcPolicy::restrictive(HarnessSchedulingBounds::new(
        NonZeroU32::new(1).unwrap(),
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(1).unwrap(),
        NonZeroU32::new(1).unwrap(),
    ))
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
