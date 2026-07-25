use crate::support::{FixtureServer, ThreadServices};
use std::num::NonZeroU64;
use std::sync::Arc;
use swallowtail_adapter_anthropic::{
    AnthropicInferenceAttemptInput, AnthropicModelSelection, AnthropicPreparationInput,
    AnthropicPreparedIntegration, prepare_anthropic_direct,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CleanupOutcome, CredentialLease, CredentialService, EndpointRef,
    HostServices, NetworkPolicyService, OperationContent, PreparedAccessEvidence, RequestId,
    ScopedTaskService, TimeService,
};

pub struct PreparedFixture {
    pub server: FixtureServer,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    host: LocalProcessHost,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    thread: ThreadServices,
    releases: Arc<std::sync::atomic::AtomicUsize>,
}

impl PreparedFixture {
    pub fn new(host_id: ExecutionHostId) -> Self {
        let server = FixtureServer::start();
        let target = InstanceTargetRef::new("anthropic.prepared.endpoint").unwrap();
        let audience = EndpointAudience::new("api.anthropic.com").unwrap();
        let credential = CredentialRef::new("anthropic.prepared.key").unwrap();
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
        let access = AccessProfile::new(
            AccessProfileId::new("anthropic.prepared.access").unwrap(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            audience,
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(credential);
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        Self {
            server,
            host_id,
            target,
            host,
            access,
            evidence: PreparedAccessEvidence::caller_asserted(status),
            thread: ThreadServices::new(),
            releases: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    pub fn prepared(&self) -> AnthropicPreparedIntegration {
        prepare_anthropic_direct(
            AnthropicPreparationInput::new(
                ConfiguredInstanceId::new("anthropic.prepared").unwrap(),
                InstanceRevision::new("1").unwrap(),
                self.host_id.clone(),
                self.target.clone(),
                self.access.clone(),
                self.evidence.clone(),
            ),
            &self.services(),
        )
        .expect("Anthropic direct integration prepares")
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

    pub fn attempt_input(&self, id: &str) -> AnthropicInferenceAttemptInput {
        AnthropicInferenceAttemptInput::new(
            RequestId::new(id).unwrap(),
            AnthropicModelSelection::new(
                ModelRouteId::new("anthropic.prepared.route").unwrap(),
                ModelRouteRevision::new("1").unwrap(),
                ModelId::new("claude-fixture-primary").unwrap(),
            ),
            OperationContent::new("prepared fixture prompt").unwrap(),
            NonZeroU64::new(64).unwrap(),
        )
    }

    pub fn releases(&self) -> usize {
        self.releases.load(std::sync::atomic::Ordering::SeqCst)
    }
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: swallowtail_runtime::ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> swallowtail_runtime::BoxFuture<
        'static,
        Result<CredentialLease, swallowtail_runtime::RuntimeFailure>,
    > {
        self.inner.acquire(scope, reference, audience)
    }

    fn release(
        &self,
        lease: CredentialLease,
    ) -> swallowtail_runtime::BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            outcome
        })
    }
}
