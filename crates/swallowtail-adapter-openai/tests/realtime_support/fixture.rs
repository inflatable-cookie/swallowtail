use super::server::{RealtimeFixtureServer, RealtimeScenario};
use super::services::{CallLog, ThreadServices, TimeMode, TrackingCredential, TrackingNetwork};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use std::sync::Arc;
use swallowtail_adapter_openai::openai_realtime_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AudioEncoding, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, MediaFormat, ModelId,
    ModelRoute, ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    PreflightContext, ProtocolFacadeId, ProviderId, RealtimeMediaConfig, RealtimeMediaRequirements,
    RuntimeReadiness, SessionAccessPolicy, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    NetworkPolicyService, ScopedTaskService, TimeService,
};

pub struct RealtimeFixture {
    pub server: RealtimeFixtureServer,
    pub calls: CallLog,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    threads: ThreadServices,
    fail_release: bool,
}

impl RealtimeFixture {
    pub fn new(scenario: RealtimeScenario, mode: TimeMode) -> Self {
        Self::for_host(scenario, mode, "host.openai-realtime")
    }

    pub fn for_host(scenario: RealtimeScenario, mode: TimeMode, host: &str) -> Self {
        Self::build(scenario, mode, host, false)
    }

    pub fn with_release_failure(scenario: RealtimeScenario, mode: TimeMode) -> Self {
        Self::build(scenario, mode, "host.openai-realtime", true)
    }

    fn build(scenario: RealtimeScenario, mode: TimeMode, host: &str, fail_release: bool) -> Self {
        let server = RealtimeFixtureServer::start(scenario);
        let calls = CallLog::default();
        let host_id = ExecutionHostId::new(host).expect("host id is valid");
        let target =
            InstanceTargetRef::new("openai-realtime-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("api.openai.com").expect("audience is valid");
        let credential =
            CredentialRef::new("openai-realtime-fixture-key").expect("credential is valid");
        let local = LocalProcessHost::builder(LocalProcessLimits::default())
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
            host: local,
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
        let descriptor = openai_realtime_descriptor();
        let access_id = AccessProfileId::new("access.openai.realtime").expect("access id is valid");
        let requirements = capabilities();
        let profile = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("openai.public.realtime").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("openai-realtime-2026-07-22").expect("facade is valid"),
            InstancePolicyId::new("public-api-key-audio-only").expect("policy is valid"),
            profile.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("openai-gpt-realtime-2-1").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new("gpt-realtime-2.1").expect("model id is valid"),
            profile,
        )
        .with_provider_id(ProviderId::new("openai").expect("provider id is valid"));
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
            ModelId::new("gpt-realtime-2.1").expect("model id is valid"),
            config(),
        ))
        .require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route),
            &operation,
        )
        .expect("OpenAI Realtime preflight succeeds")
    }
}

pub fn config() -> RealtimeMediaConfig {
    let format = MediaFormat::audio(
        AudioEncoding::Pcm16LittleEndian,
        NonZeroU32::new(24_000).expect("rate is nonzero"),
        NonZeroU16::new(1).expect("channels are nonzero"),
    );
    RealtimeMediaConfig::new(
        format,
        format,
        NonZeroU64::new(32_768).expect("chunk bound is nonzero"),
        NonZeroU32::new(2).expect("turn bound is nonzero"),
    )
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
    ]
}
