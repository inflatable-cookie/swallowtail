use crate::support::{FixtureServer, StreamFixture, ThreadServices};
use std::sync::Arc;
use swallowtail_adapter_kimi_platform::kimi_platform_direct_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    ProtocolFacadeId, ProviderId, ReasoningMode, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    CredentialService, EndpointRef, HostServices, NetworkPolicyService, RuntimeFailure, ScopeId,
    ScopedTaskService, TimeService,
};
use swallowtail_testkit::ExecutionTopologyFixture;

pub struct Fixture {
    pub server: FixtureServer,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
    release_after_blocking: Arc<std::sync::Mutex<Vec<usize>>>,
    thread: ThreadServices,
}

impl Fixture {
    pub fn new() -> Self {
        Self::for_topology(&ExecutionTopologyFixture::local())
    }

    pub fn with_stream(stream: StreamFixture) -> Self {
        Self::with_topology_stream(&ExecutionTopologyFixture::local(), stream)
    }

    pub fn for_topology(topology: &ExecutionTopologyFixture) -> Self {
        Self::with_topology_stream(topology, StreamFixture::Success)
    }

    fn with_topology_stream(topology: &ExecutionTopologyFixture, stream: StreamFixture) -> Self {
        let server = FixtureServer::start(stream);
        let host_id = topology.execution_host_id().clone();
        let instance_id = topology.configured_instance_id().clone();
        let target = topology.instance_target().clone();
        let audience = EndpointAudience::new("api.moonshot.ai").expect("audience");
        let credential = CredentialRef::new("kimi-platform-fixture-key").expect("credential");
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
            host_id,
            instance_id,
            target,
            audience,
            credential,
            host,
            releases: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            release_after_blocking: Arc::new(std::sync::Mutex::new(Vec::new())),
            thread: ThreadServices::new(),
        }
    }

    pub fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                releases: Arc::clone(&self.releases),
                blocking: self.thread.clone(),
                release_after_blocking: Arc::clone(&self.release_after_blocking),
            }) as Arc<dyn CredentialService>)
    }

    pub fn releases(&self) -> usize {
        self.releases.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn release_after_blocking(&self) -> Vec<usize> {
        self.release_after_blocking
            .lock()
            .expect("release-order lock")
            .clone()
    }

    pub fn plan(&self, role: DriverRole) -> swallowtail_core::PreflightPlan {
        self.plan_with_audience(role, self.audience.clone())
    }

    pub fn plan_with_audience(
        &self,
        role: DriverRole,
        audience: EndpointAudience,
    ) -> swallowtail_core::PreflightPlan {
        let descriptor = kimi_platform_direct_descriptor();
        let access_id = AccessProfileId::new("access.kimi-platform.public").expect("access id");
        let requirements = capability_requirements(role);
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            self.instance_id.clone(),
            InstanceRevision::new("1").expect("revision"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("kimi-platform-chat-2026-07-21").expect("facade"),
            InstancePolicyId::new("public-platform-api-key").expect("policy"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("kimi-platform-k3").expect("route id"),
            ModelRouteRevision::new("1").expect("revision"),
            instance.id().clone(),
            ModelId::new("kimi-k3").expect("model id"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("moonshot").expect("provider id"));
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            audience,
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
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
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
        .with_capabilities(requirements);
        let context =
            PreflightContext::new(&descriptor, &instance, &access, &status, host_services);
        if role == DriverRole::ModelCatalog {
            preflight(&context, &operation).expect("catalogue preflight")
        } else {
            preflight(
                &context.with_model_route(&route),
                &operation.require_model_route(),
            )
            .expect("run preflight")
        }
    }
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
    blocking: ThreadServices,
    release_after_blocking: Arc<std::sync::Mutex<Vec<usize>>>,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.inner.acquire(scope, reference, audience)
    }

    fn release(&self, lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release(lease);
        let releases = Arc::clone(&self.releases);
        let blocking = self.blocking.clone();
        let release_after_blocking = Arc::clone(&self.release_after_blocking);
        Box::pin(async move {
            release_after_blocking
                .lock()
                .expect("release-order lock")
                .push(blocking.completed());
            let outcome = release.await;
            releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            outcome
        })
    }
}

fn capability_requirements(role: DriverRole) -> Vec<CapabilityRequirement> {
    if role == DriverRole::ModelCatalog {
        vec![CapabilityRequirement::new(Capability::ModelCatalog, [])]
    } else {
        vec![
            CapabilityRequirement::new(Capability::StructuredRun, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            CapabilityRequirement::new(Capability::UsageReporting, []),
            CapabilityRequirement::new(Capability::OutputTokenLimit, []),
            CapabilityRequirement::new(
                Capability::ReasoningSelection,
                [CapabilityConstraint::ReasoningMode(
                    ReasoningMode::new("high").expect("reasoning mode"),
                )],
            ),
        ]
    }
}
