use super::{
    CallLog, FixtureServer, ServerScenario, ThreadServices, TrackingCredential, TrackingNetwork,
};
use std::sync::Arc;
use swallowtail_adapter_xai::xai_websocket_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ProviderId, RuntimeReadiness, SessionAccessPolicy, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    NetworkPolicyService, ScopedTaskService, TimeService,
};

pub struct DriverFixture {
    pub server: FixtureServer,
    pub calls: CallLog,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    thread: ThreadServices,
}

impl DriverFixture {
    pub fn new(scenario: ServerScenario) -> Self {
        Self::for_host(
            scenario,
            ExecutionHostId::new("host.xai").expect("host id is valid"),
        )
    }

    pub fn for_host(scenario: ServerScenario, host_id: ExecutionHostId) -> Self {
        let server = FixtureServer::start(scenario);
        let target = InstanceTargetRef::new("xai-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("api.x.ai").expect("audience is valid");
        let credential = CredentialRef::new("xai-fixture-key").expect("credential is valid");
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
        let calls = CallLog::default();
        Self {
            server,
            thread: ThreadServices::new(calls.clone()),
            calls,
            host_id,
            target,
            audience,
            credential,
            host,
        }
    }

    pub fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(TrackingNetwork {
                inner: self.host.clone(),
                calls: self.calls.clone(),
            }) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                calls: self.calls.clone(),
            }) as Arc<dyn CredentialService>)
    }

    pub fn plan(&self) -> PreflightPlan {
        let descriptor = xai_websocket_descriptor();
        let access_id = AccessProfileId::new("access.xai.public").expect("access id is valid");
        let requirements = capability_requirements();
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("xai.public.websocket").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("xai-responses-websocket-2026-04-23").expect("facade is valid"),
            InstancePolicyId::new("public-api-key-resource-free").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("xai-grok-fixture").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            instance.id().clone(),
            ModelId::new("grok-fixture-exact").expect("model id is valid"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("xai").expect("provider id is valid"));
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
        let role = swallowtail_core::DriverRole::InteractiveSession;
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
        .require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route),
            &operation,
        )
        .expect("xAI session preflight succeeds")
    }
}

fn capability_requirements() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [swallowtail_core::CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::BilledCostReporting, []),
    ]
}
