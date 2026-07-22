use std::cell::Cell;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, AudioEncoding, Capability, CapabilityConstraint, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    MediaDirection, MediaFormat, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightFailure, PreflightPlan,
    ProtocolFacadeId, RealtimeMediaConfig, RealtimeMediaRequirements, RuntimeReadiness,
    SessionAccessPolicy, SupportAuthority, TransportFamilyId, preflight,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RealtimeMediaPreflightCase {
    Canonical,
    WrongRole,
    WrongLayer,
    WrongShape,
    MissingRoute,
    WrongModel,
    MissingFormat,
    WrongChunkBound,
    RejectedAccess,
    MissingHostService,
}

pub struct RealtimeMediaPreflightFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
    requirements: OperationRequirements,
    services: Vec<HostServiceKind>,
    include_route: bool,
    provider_side_effects: Cell<usize>,
}

impl RealtimeMediaPreflightFixture {
    #[must_use]
    pub fn for_case(case: RealtimeMediaPreflightCase) -> Self {
        let adapter_id = valid(AdapterId::new, "fixture.realtime-media");
        let instance_id = valid(ConfiguredInstanceId::new, "fixture.realtime-instance");
        let profile_id = valid(AccessProfileId::new, "fixture.realtime-access");
        let host_id = valid(ExecutionHostId::new, "fixture.realtime-host");
        let role = if case == RealtimeMediaPreflightCase::WrongRole {
            DriverRole::InteractiveSession
        } else {
            DriverRole::RealtimeMediaSession
        };
        let layer = if case == RealtimeMediaPreflightCase::WrongLayer {
            ExecutionLayer::HarnessInteraction
        } else {
            ExecutionLayer::DirectModelInference
        };
        let shape = if case == RealtimeMediaPreflightCase::WrongShape {
            OperationShape::StructuredRun
        } else {
            OperationShape::InteractiveSession
        };
        let advertised = advertised_capabilities(case);
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(adapter_id.clone(), valid(AdapterVersion::new, "fixture-v1")),
            valid(IntegrationFamilyId::new, "fixture-realtime-provider"),
            valid(TransportFamilyId::new, "websocket-realtime-media"),
        )
        .with_roles([role])
        .with_execution_layers([layer])
        .with_operation_shapes([shape])
        .with_required_host_services(role, host_services());
        let instance = ConfiguredInstance::new(
            instance_id.clone(),
            valid(InstanceRevision::new, "fixture-revision-1"),
            adapter_id,
            host_id.clone(),
            valid(InstanceTargetRef::new, "fixture-realtime-endpoint"),
            InstanceOwnership::ExternalAttached,
            profile_id.clone(),
            SupportAuthority::ProviderSupported,
            valid(ProtocolFacadeId::new, "fixture-realtime-v1"),
            valid(InstancePolicyId::new, "fixture-realtime-policy"),
            CapabilityProfile::new(advertised.clone()),
        );
        let route_model = if case == RealtimeMediaPreflightCase::WrongModel {
            "fixture-other-model"
        } else {
            "fixture-realtime-model"
        };
        let route = ModelRoute::new(
            valid(ModelRouteId::new, "fixture-realtime-route"),
            valid(ModelRouteRevision::new, "fixture-route-revision-1"),
            instance_id,
            valid(ModelId::new, route_model),
            CapabilityProfile::new(advertised),
        );
        let access_profile = AccessProfile::new(
            profile_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            valid(EndpointAudience::new, "fixture.realtime.example"),
            SupportAuthority::ProviderSupported,
        );
        let credential = if case == RealtimeMediaPreflightCase::RejectedAccess {
            CredentialState::Required
        } else {
            CredentialState::Ready
        };
        let access_status = AccessStatus::new(
            profile_id.clone(),
            credential,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let access = AccessRequirement::new(profile_id)
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]);
        let media = RealtimeMediaRequirements::new(
            valid(ModelId::new, "fixture-realtime-model"),
            realtime_media_config(),
        );
        let requirements = OperationRequirements::new(layer, shape, role, host_id, access)
            .with_ownership_modes([InstanceOwnership::ExternalAttached])
            .with_host_services(host_services())
            .with_capabilities(common_capabilities())
            .with_realtime_media(media)
            .require_model_route();
        let requirements = if shape == OperationShape::InteractiveSession {
            requirements.with_session_access_policy(SessionAccessPolicy::resource_free())
        } else {
            requirements
        };
        let mut services = host_services().to_vec();
        if case == RealtimeMediaPreflightCase::MissingHostService {
            services.retain(|service| *service != HostServiceKind::Credential);
        }
        Self {
            driver,
            instance,
            route,
            access_profile,
            access_status,
            requirements,
            services,
            include_route: case != RealtimeMediaPreflightCase::MissingRoute,
            provider_side_effects: Cell::new(0),
        }
    }

    pub fn preflight(&self) -> Result<PreflightPlan, PreflightFailure> {
        let context = PreflightContext::new(
            &self.driver,
            &self.instance,
            &self.access_profile,
            &self.access_status,
            self.services.iter().copied(),
        );
        if self.include_route {
            preflight(&context.with_model_route(&self.route), &self.requirements)
        } else {
            preflight(&context, &self.requirements)
        }
    }

    #[must_use]
    pub fn provider_side_effect_count(&self) -> usize {
        self.provider_side_effects.get()
    }
}

pub(crate) fn realtime_media_config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("sample rate is nonzero"),
        NonZeroU16::new(1).expect("channel count is nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
        NonZeroU32::new(2).expect("turn bound is nonzero"),
    )
}

fn advertised_capabilities(case: RealtimeMediaPreflightCase) -> Vec<CapabilityRequirement> {
    let config = realtime_media_config();
    let mut media: Vec<_> = config
        .capability_requirement()
        .constraints()
        .cloned()
        .collect();
    if case == RealtimeMediaPreflightCase::MissingFormat {
        media.retain(|constraint| {
            !matches!(
                constraint,
                CapabilityConstraint::RealtimeMediaFormat(MediaDirection::Output, _)
            )
        });
    }
    if case == RealtimeMediaPreflightCase::WrongChunkBound {
        media.retain(|constraint| {
            !matches!(
                constraint,
                CapabilityConstraint::RealtimeMediaMaximumChunkBytes(_)
            )
        });
        media.push(CapabilityConstraint::RealtimeMediaMaximumChunkBytes(16_384));
    }
    let mut capabilities = common_capabilities();
    capabilities.push(CapabilityRequirement::new(Capability::RealtimeMedia, media));
    capabilities
}

fn common_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveResponse,
            )],
        ),
    ]
}

fn host_services() -> [HostServiceKind; 5] {
    [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ]
}

fn valid<T, E>(constructor: impl FnOnce(String) -> Result<T, E>, value: &str) -> T
where
    E: std::fmt::Debug,
{
    constructor(value.to_owned()).expect("static realtime fixture text must be valid")
}
