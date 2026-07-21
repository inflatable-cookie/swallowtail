use crate::server::{FixtureServer, ServerMode};
use crate::services::{ThreadServices, TimeMode};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use swallowtail_adapter_openai::openai_background_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    ProtocolFacadeId, ProviderId, RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    NetworkPolicyService, ScopedTaskService, TimeService,
};

pub struct Fixture {
    pub server: FixtureServer,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    releases: Arc<AtomicUsize>,
    thread: ThreadServices,
}

impl Fixture {
    pub fn new(mode: ServerMode, host: &str, time: TimeMode) -> Self {
        let server = FixtureServer::start_with(mode);
        let host_id = ExecutionHostId::new(host).expect("host id is valid");
        let target = InstanceTargetRef::new("openai-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("api.openai.com").expect("audience is valid");
        let credential = CredentialRef::new("openai-fixture-key").expect("credential is valid");
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
            target,
            audience,
            credential,
            host,
            releases: Arc::new(AtomicUsize::new(0)),
            thread: ThreadServices::new(time),
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
            }) as Arc<dyn CredentialService>)
    }

    pub fn releases(&self) -> usize {
        self.releases.load(Ordering::SeqCst)
    }

    pub fn plan(&self) -> swallowtail_core::PreflightPlan {
        let descriptor = openai_background_descriptor();
        let access_id = AccessProfileId::new("access.openai").expect("access id is valid");
        let requirements = capability_requirements();
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("openai.public").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("openai-responses-background-2026-07-21")
                .expect("facade is valid"),
            InstancePolicyId::new("public-api-key").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("openai-gpt-5-6").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new("gpt-5.6").expect("model id is valid"),
            capabilities,
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
        let host_services: Vec<_> = descriptor
            .required_host_services(DriverRole::StructuredRun)
            .collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
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
        .require_model_route();
        let context =
            PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route);
        preflight(&context, &operation).expect("OpenAI background preflight succeeds")
    }
}

fn capability_requirements() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::OutputTokenLimit, []),
        CapabilityRequirement::new(Capability::ProviderBackgroundExecution, []),
        CapabilityRequirement::new(Capability::ProviderTemporaryRetention, []),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::StructuredRun,
            )],
        ),
    ]
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<AtomicUsize>,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: swallowtail_runtime::ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> swallowtail_runtime::BoxFuture<
        'static,
        Result<swallowtail_runtime::CredentialLease, swallowtail_runtime::RuntimeFailure>,
    > {
        self.inner.acquire(scope, reference, audience)
    }

    fn release(
        &self,
        lease: swallowtail_runtime::CredentialLease,
    ) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::CleanupOutcome> {
        let release = self.inner.release(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, Ordering::SeqCst);
            outcome
        })
    }
}
