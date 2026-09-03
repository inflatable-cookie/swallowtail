use crate::support::{FixtureServer, ServerScenario, ThreadServices};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_adapter_deepseek::{
    DEEPSEEK_ENDPOINT, DEEPSEEK_FACADE_REVISION, DEEPSEEK_MODEL_ID, DeepSeekPreparationInput,
    deepseek_direct_descriptor, deepseek_facade_binding, deepseek_v4_requirements,
    deepseek_v4_run_requirements,
};
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
    CredentialService, Deadline, EndpointRef, HostServices, InteractiveSessionHandle,
    MonotonicInstant, NetworkPolicyService, PreparedAccessEvidence, RuntimeFailure, ScopeId,
    ScopedTaskService, SessionCleanupRequest, TimeService,
};
use swallowtail_testkit::ExecutionTopologyFixture;

pub struct Fixture {
    pub server: FixtureServer,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    access_id: AccessProfileId,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    releases: Arc<AtomicUsize>,
    release_after_blocking: Arc<Mutex<Vec<usize>>>,
    thread: ThreadServices,
}

impl Fixture {
    pub fn new() -> Self {
        Self::for_topology(&ExecutionTopologyFixture::local())
    }

    pub fn with_scenario(scenario: ServerScenario) -> Self {
        Self::with_topology_scenario(&ExecutionTopologyFixture::local(), scenario)
    }

    pub fn for_topology(topology: &ExecutionTopologyFixture) -> Self {
        Self::with_topology_scenario(topology, ServerScenario::Success)
    }

    pub fn with_topology_scenario(
        topology: &ExecutionTopologyFixture,
        scenario: ServerScenario,
    ) -> Self {
        let server = FixtureServer::start(scenario);
        let host_id = topology.execution_host_id().clone();
        let instance_id = topology.configured_instance_id().clone();
        let target = InstanceTargetRef::new(DEEPSEEK_ENDPOINT).expect("exact target");
        let access_id = AccessProfileId::new("access.deepseek.public").expect("access id");
        let audience = EndpointAudience::new("api.deepseek.com").expect("audience");
        let credential = CredentialRef::new("deepseek-fixture-key").expect("credential");
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
            access_id,
            audience,
            credential,
            host,
            releases: Arc::new(AtomicUsize::new(0)),
            release_after_blocking: Arc::new(Mutex::new(Vec::new())),
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

    pub fn close_session(
        &self,
        session: Box<dyn InteractiveSessionHandle>,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let deadline = Deadline::at(MonotonicInstant::from_ticks(
            self.thread.now().ticks() + 10_000,
        ));
        session.close(SessionCleanupRequest::new(deadline), self.services())
    }

    pub fn releases(&self) -> usize {
        self.releases.load(Ordering::SeqCst)
    }

    pub fn release_after_blocking(&self) -> Vec<usize> {
        self.release_after_blocking
            .lock()
            .expect("release-order lock")
            .clone()
    }

    #[allow(dead_code)]
    pub fn preparation_input(&self) -> DeepSeekPreparationInput {
        let access = AccessProfile::new(
            self.access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        DeepSeekPreparationInput::new(
            self.instance_id.clone(),
            InstanceRevision::new("prepared-1").expect("revision"),
            self.host_id.clone(),
            self.target.clone(),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        )
    }

    pub fn plan(&self, role: DriverRole) -> swallowtail_core::PreflightPlan {
        self.plan_with_reasoning(role, "high")
    }

    pub fn plan_with_reasoning(
        &self,
        role: DriverRole,
        reasoning: &str,
    ) -> swallowtail_core::PreflightPlan {
        let descriptor = deepseek_direct_descriptor();
        let session_requirements =
            deepseek_v4_requirements(self.host_id.clone(), self.access_id.clone());
        let run_requirements =
            deepseek_v4_run_requirements(self.host_id.clone(), self.access_id.clone());
        let mut capabilities: Vec<_> = session_requirements.capabilities().cloned().collect();
        capabilities.extend(run_requirements.capabilities().cloned());
        capabilities.push(CapabilityRequirement::new(Capability::ModelCatalog, []));
        let profile = CapabilityProfile::new(capabilities.clone());
        let instance = ConfiguredInstance::new(
            self.instance_id.clone(),
            InstanceRevision::new("1").expect("revision"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            self.access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new(DEEPSEEK_FACADE_REVISION).expect("facade"),
            InstancePolicyId::new("deepseek-public-api-key").expect("policy"),
            profile.clone(),
        )
        .with_interface_versions([deepseek_facade_binding()]);
        let route = ModelRoute::new(
            ModelRouteId::new("deepseek-v4-pro").expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("route revision"),
            self.instance_id.clone(),
            ModelId::new(DEEPSEEK_MODEL_ID).expect("model"),
            profile,
        )
        .with_provider_id(ProviderId::new("deepseek").expect("provider"));
        let access = AccessProfile::new(
            self.access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            self.access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let operation = match role {
            DriverRole::InteractiveSession => {
                exact_reasoning_requirements(session_requirements, reasoning)
            }
            DriverRole::StructuredRun => exact_reasoning_requirements(run_requirements, reasoning),
            _ => OperationRequirements::new(
                ExecutionLayer::DirectModelInference,
                OperationShape::InteractiveSession,
                DriverRole::ModelCatalog,
                self.host_id.clone(),
                access_requirement(self.access_id.clone()),
            )
            .with_ownership_modes([InstanceOwnership::ExternalAttached])
            .with_host_services(descriptor.required_host_services(role))
            .with_capabilities([CapabilityRequirement::new(Capability::ModelCatalog, [])]),
        };
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let context =
            PreflightContext::new(&descriptor, &instance, &access, &status, host_services);
        if role == DriverRole::ModelCatalog {
            preflight(&context, &operation).expect("catalogue preflight")
        } else {
            preflight(&context.with_model_route(&route), &operation).expect("route preflight")
        }
    }
}

fn exact_reasoning_requirements(
    requirements: OperationRequirements,
    mode: &str,
) -> OperationRequirements {
    let selected = ReasoningMode::new(mode).expect("fixture reasoning mode");
    let capabilities = requirements
        .capabilities()
        .map(|requirement| {
            if requirement.capability() == Capability::ReasoningSelection {
                CapabilityRequirement::new(
                    Capability::ReasoningSelection,
                    [CapabilityConstraint::ReasoningMode(selected.clone())],
                )
            } else {
                requirement.clone()
            }
        })
        .collect::<Vec<_>>();
    requirements.with_capabilities(capabilities)
}

fn access_requirement(id: AccessProfileId) -> AccessRequirement {
    AccessRequirement::new(id)
        .with_credential_states([CredentialState::Ready])
        .with_entitlement_states([EntitlementState::Available])
        .with_endpoint_authorizations([EndpointAuthorization::Allowed])
        .with_runtime_readiness([RuntimeReadiness::Ready])
        .with_support_authorities([SupportAuthority::ProviderSupported])
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<AtomicUsize>,
    blocking: ThreadServices,
    release_after_blocking: Arc<Mutex<Vec<usize>>>,
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
        let order = Arc::clone(&self.release_after_blocking);
        Box::pin(async move {
            order
                .lock()
                .expect("release-order lock")
                .push(blocking.completed());
            let outcome = release.await;
            releases.fetch_add(1, Ordering::SeqCst);
            outcome
        })
    }
}
