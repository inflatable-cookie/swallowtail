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
    AttachmentRef, AttachmentService, BlockingWorkService, CleanupOutcome, CredentialLease,
    CredentialService, Deadline, EndpointRef, HostServices, MonotonicInstant, NetworkPolicyService,
    OperationContent, PreparedAccessEvidence, RequestId, ScopedTaskService, SessionCleanupRequest,
    TimeService,
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
    attachment_releases: Arc<std::sync::atomic::AtomicUsize>,
    attachment_ref: AttachmentRef,
    attachment_path: std::path::PathBuf,
}

static NEXT_ATTACHMENT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

impl PreparedFixture {
    pub fn new(host_id: ExecutionHostId) -> Self {
        Self::with_stream(host_id, crate::support::StreamFixture::Success)
    }

    pub fn with_stream(host_id: ExecutionHostId, stream: crate::support::StreamFixture) -> Self {
        let server = FixtureServer::start_with(stream);
        let target = InstanceTargetRef::new("anthropic.prepared.endpoint").unwrap();
        let audience = EndpointAudience::new("api.anthropic.com").unwrap();
        let credential = CredentialRef::new("anthropic.prepared.key").unwrap();
        let attachment_ref = AttachmentRef::new("anthropic.fixture.image").unwrap();
        let attachment_path = std::env::temp_dir().join(format!(
            "swallowtail-anthropic-{}-{}-fixture.png",
            std::process::id(),
            NEXT_ATTACHMENT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        ));
        std::fs::write(&attachment_path, b"\x89PNG\r\n\x1a\n").expect("fixture image writes");
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
            .approve_attachment(attachment_ref.clone(), &attachment_path)
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
            attachment_releases: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            attachment_ref,
            attachment_path,
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
            .with_attachment(Arc::new(TrackingAttachment {
                inner: self.host.clone(),
                releases: Arc::clone(&self.attachment_releases),
            }) as Arc<dyn AttachmentService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                releases: Arc::clone(&self.releases),
            }) as Arc<dyn CredentialService>)
    }

    pub fn cleanup_request(&self) -> SessionCleanupRequest {
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(
            self.thread.now().ticks().saturating_add(30_000),
        )))
    }

    pub fn attempt_input(&self, id: &str) -> AnthropicInferenceAttemptInput {
        self.attempt_input_for_model(id, "claude-fixture-primary")
    }

    pub fn attempt_input_for_model(
        &self,
        id: &str,
        model_id: &str,
    ) -> AnthropicInferenceAttemptInput {
        AnthropicInferenceAttemptInput::new(
            RequestId::new(id).unwrap(),
            AnthropicModelSelection::new(
                ModelRouteId::new("anthropic.prepared.route").unwrap(),
                ModelRouteRevision::new("1").unwrap(),
                ModelId::new(model_id).unwrap(),
            ),
            OperationContent::new("prepared fixture prompt").unwrap(),
            NonZeroU64::new(64).unwrap(),
        )
    }

    pub fn releases(&self) -> usize {
        self.releases.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn attachment_ref(&self) -> AttachmentRef {
        self.attachment_ref.clone()
    }

    pub fn attachment_releases(&self) -> usize {
        self.attachment_releases
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl Drop for PreparedFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.attachment_path);
    }
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
}

struct TrackingAttachment {
    inner: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
}

impl AttachmentService for TrackingAttachment {
    fn materialize_file(
        &self,
        scope: swallowtail_runtime::ScopeId,
        descriptor: swallowtail_runtime::AttachmentDescriptor,
    ) -> swallowtail_runtime::BoxFuture<
        'static,
        Result<swallowtail_runtime::AttachmentFileLease, swallowtail_runtime::RuntimeFailure>,
    > {
        self.inner.materialize_file(scope, descriptor)
    }

    fn release_file(
        &self,
        lease: swallowtail_runtime::AttachmentFileLease,
    ) -> swallowtail_runtime::BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release_file(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            outcome
        })
    }
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
