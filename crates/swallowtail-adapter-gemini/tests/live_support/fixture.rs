use super::server::{LiveFixtureServer, LiveScenario};
use super::services::{CallLog, ThreadServices, TimeMode, TrackingCredential, TrackingNetwork};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use std::sync::Arc;
use swallowtail_adapter_gemini::gemini_live_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AudioEncoding, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, MediaFormat, ModelId,
    ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PlannedConnectionRolloverPolicy, PreflightContext, ProtocolFacadeId, ProviderId,
    RealtimeMediaConfig, RealtimeMediaRequirements, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    NetworkPolicyService, ScopedTaskService, TimeService,
};

pub struct LiveFixture {
    pub server: LiveFixtureServer,
    pub calls: CallLog,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    threads: ThreadServices,
    fail_release: bool,
}

impl LiveFixture {
    pub fn new(scenario: LiveScenario, mode: TimeMode) -> Self {
        Self::build(scenario, mode, "host.gemini-live", false)
    }

    pub fn for_host(scenario: LiveScenario, mode: TimeMode, host: &str) -> Self {
        Self::build(scenario, mode, host, false)
    }

    pub fn with_release_failure(scenario: LiveScenario, mode: TimeMode) -> Self {
        Self::build(scenario, mode, "host.gemini-live", true)
    }

    fn build(scenario: LiveScenario, mode: TimeMode, host: &str, fail_release: bool) -> Self {
        let server = LiveFixtureServer::start(scenario);
        let calls = CallLog::default();
        let host_id = ExecutionHostId::new(host).unwrap();
        let target = InstanceTargetRef::new("gemini-live-fixture-endpoint").unwrap();
        let audience = EndpointAudience::new("generativelanguage.googleapis.com").unwrap();
        let credential = CredentialRef::new("gemini-live-fixture-key").unwrap();
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .approve_secret_credential(
                credential.clone(),
                audience.clone(),
                b"fixture-secret".to_vec(),
            )
            .build();
        Self {
            server,
            calls: calls.clone(),
            host_id,
            target,
            audience,
            credential,
            host,
            threads: ThreadServices::new(mode, calls),
            fail_release,
        }
    }

    pub fn services(&self) -> HostServices {
        let threads = Arc::new(self.threads.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&threads) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&threads) as Arc<dyn BlockingWorkService>)
            .with_time(threads as Arc<dyn TimeService>)
            .with_network(Arc::new(TrackingNetwork {
                inner: self.host.clone(),
                calls: self.calls.clone(),
            }) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                calls: self.calls.clone(),
                fail_release: self.fail_release,
            }) as Arc<dyn CredentialService>)
    }

    pub fn plan(&self) -> swallowtail_core::PreflightPlan {
        let descriptor = gemini_live_descriptor();
        let access_id = AccessProfileId::new("gemini.authorization-api-key.project").unwrap();
        let requirements = capabilities();
        let profile = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("gemini.public.live-preview").unwrap(),
            InstanceRevision::new("1").unwrap(),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new(
                "google.generativelanguage.v1beta.GenerativeService.BidiGenerateContent",
            )
            .unwrap(),
            InstancePolicyId::new("gemini-live-preview-authorization-key-manual-audio").unwrap(),
            profile.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("gemini-3-1-flash-live-preview").unwrap(),
            ModelRouteRevision::new("1").unwrap(),
            instance.id().clone(),
            ModelId::new("gemini-3.1-flash-live-preview").unwrap(),
            profile,
        )
        .with_provider_id(ProviderId::new("gemini").unwrap());
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let role = DriverRole::RealtimeMediaSession;
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::InteractiveSession,
            role,
            self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::ProviderSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(host_services.clone())
        .with_capabilities(requirements)
        .with_session_access_policy(SessionAccessPolicy::resource_free())
        .with_realtime_media(RealtimeMediaRequirements::new(
            ModelId::new("gemini-3.1-flash-live-preview").unwrap(),
            config(),
        ))
        .with_planned_connection_rollover(rollover_policy())
        .require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route),
            &operation,
        )
        .expect("Gemini Live preflight succeeds")
    }
}

pub fn config() -> RealtimeMediaConfig {
    let input = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(16_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    let output = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).unwrap(),
        NonZeroU16::new(1).unwrap(),
    );
    RealtimeMediaConfig::new(
        input,
        output,
        NonZeroU64::new(32_768).unwrap(),
        NonZeroU32::new(2).unwrap(),
    )
}

pub fn rollover_policy() -> PlannedConnectionRolloverPolicy {
    PlannedConnectionRolloverPolicy::Bounded(NonZeroU32::new(1).unwrap())
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveResponse,
            )],
        ),
        config().capability_requirement(),
        CapabilityRequirement::new(
            Capability::PlannedConnectionRollover,
            [CapabilityConstraint::PlannedConnectionRolloverMaximumCount(
                1,
            )],
        ),
    ]
}
