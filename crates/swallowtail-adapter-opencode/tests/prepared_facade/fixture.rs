use crate::http_support::{FixtureServer, StreamFixture, ThreadServices};
use futures_channel::oneshot;
use futures_executor::block_on;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_adapter_opencode::{
    OpenCodeModelSelection, OpenCodePreparationInput, OpenCodePreparationProbe,
    OpenCodePreparedIntegration, prepare_opencode_attached,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, InstanceRevision, InstanceTargetRef,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness, SupportAuthority,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentRef, AttachmentRole, AttachmentService,
    BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialService, Deadline,
    DeadlineObservation, DiscoveryCancellation, EndpointRef, HostServices, MonotonicInstant,
    NetworkPolicyService, PreparedAccessEvidence, RuntimeFailure, ScopeId, ScopedTaskService,
    TimeService, WorkingResourceRef, WorkingResourceService,
};

pub(super) struct PreparedFixture {
    pub(super) server: FixtureServer,
    pub(super) host_id: ExecutionHostId,
    pub(super) target: InstanceTargetRef,
    pub(super) resource: WorkingResourceRef,
    host: LocalProcessHost,
    access: AccessProfile,
    evidence: PreparedAccessEvidence,
    thread: ThreadServices,
    clock: TestClock,
    pub(super) releases: Arc<AtomicUsize>,
    attachment: AttachmentDescriptor,
    attachment_path: PathBuf,
    pub(super) attachment_releases: Arc<AtomicUsize>,
}

impl PreparedFixture {
    pub(super) fn new(host_id: &str, version: &str) -> Self {
        Self::new_with_fixture(host_id, version, StreamFixture::Success)
    }

    pub(super) fn new_with_fixture(
        host_id: &str,
        version: &str,
        stream_fixture: StreamFixture,
    ) -> Self {
        let server = FixtureServer::start_with_version(stream_fixture, version);
        let host_id = ExecutionHostId::new(host_id).unwrap();
        let target = InstanceTargetRef::new("opencode.prepared.endpoint").unwrap();
        let audience = EndpointAudience::new("opencode.prepared.server").unwrap();
        let credential = CredentialRef::new("opencode.prepared.delegated").unwrap();
        let resource = WorkingResourceRef::new("opencode.prepared.workspace").unwrap();
        let attachment_ref = AttachmentRef::new("opencode.prepared.image").unwrap();
        let attachment_path = std::env::temp_dir().join(format!(
            "swallowtail-opencode-{}-{}.png",
            std::process::id(),
            host_id.as_str().replace(['.', ':'], "-")
        ));
        std::fs::write(&attachment_path, b"\x89PNG\r\n\x1a\n").expect("fixture image writes");
        let attachment =
            AttachmentDescriptor::new(attachment_ref.clone(), "image/png", AttachmentRole::Input)
                .unwrap()
                .with_known_length(8);
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .approve_delegated_credential(credential.clone(), audience.clone())
            .approve_working_resource(resource.clone(), std::env::temp_dir())
            .approve_attachment(attachment_ref, &attachment_path)
            .build();
        let access = AccessProfile::new(
            AccessProfileId::new("opencode.prepared.access").unwrap(),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("opencode/delegated-auth").unwrap(),
            ),
            EntitlementMetering::Unknown,
            audience,
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(credential);
        let status = AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        );
        Self {
            server,
            host_id,
            target,
            resource,
            host,
            access,
            evidence: PreparedAccessEvidence::caller_asserted(status),
            thread: ThreadServices::new(),
            clock: TestClock::new(),
            releases: Arc::new(AtomicUsize::new(0)),
            attachment,
            attachment_path,
            attachment_releases: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(super) fn prepared(&self) -> OpenCodePreparedIntegration {
        block_on(prepare_opencode_attached(
            self.preparation_input(),
            self.probe(DiscoveryCancellation::new()),
            self.services(),
        ))
        .expect("OpenCode attached integration prepares")
    }

    pub(super) fn preparation_input(&self) -> OpenCodePreparationInput {
        OpenCodePreparationInput::new(
            ConfiguredInstanceId::new("opencode.prepared").unwrap(),
            InstanceRevision::new("3").unwrap(),
            self.host_id.clone(),
            self.target.clone(),
            self.access.clone(),
            self.evidence.clone(),
        )
    }

    pub(super) fn probe(&self, cancellation: DiscoveryCancellation) -> OpenCodePreparationProbe {
        OpenCodePreparationProbe::new(
            ScopeId::new("opencode-prepared-probe").unwrap(),
            self.clock.deadline_after(Duration::from_secs(1)),
            cancellation,
        )
    }

    pub(super) fn services(&self) -> HostServices {
        self.services_with_release_failure(false)
    }

    pub(super) fn services_with_release_failure(&self, fail_release: bool) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(Arc::new(self.clock.clone()) as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                releases: Arc::clone(&self.releases),
                fail_release,
            }) as Arc<dyn CredentialService>)
            .with_working_resource(Arc::new(self.host.clone()) as Arc<dyn WorkingResourceService>)
            .with_attachment(Arc::new(TrackingAttachment {
                inner: self.host.clone(),
                releases: Arc::clone(&self.attachment_releases),
            }) as Arc<dyn AttachmentService>)
    }

    pub(super) fn services_with_denied_network(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        let denied = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_delegated_credential(
                self.access
                    .credential_reference()
                    .expect("fixture access has credential")
                    .clone(),
                self.access.endpoint_audience().clone(),
            )
            .approve_working_resource(self.resource.clone(), std::env::temp_dir())
            .build();
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(Arc::new(self.clock.clone()) as Arc<dyn TimeService>)
            .with_network(Arc::new(denied.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(denied.clone()) as Arc<dyn CredentialService>)
            .with_working_resource(Arc::new(denied) as Arc<dyn WorkingResourceService>)
    }

    pub(super) fn deadline_after(&self, duration: Duration) -> Deadline {
        self.clock.deadline_after(duration)
    }

    pub(super) fn model(&self) -> OpenCodeModelSelection {
        OpenCodeModelSelection::new(
            ModelRouteId::new("opencode.prepared.route").unwrap(),
            ModelRouteRevision::new("1").unwrap(),
            ProviderId::new("anthropic").unwrap(),
            ModelId::new("claude-sonnet").unwrap(),
        )
    }

    pub(super) fn attachment(&self) -> AttachmentDescriptor {
        self.attachment.clone()
    }
}

impl Drop for PreparedFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.attachment_path);
    }
}

#[derive(Clone)]
struct TestClock {
    origin: Instant,
}

impl TestClock {
    fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }

    fn deadline_after(&self, duration: Duration) -> Deadline {
        Deadline::at(MonotonicInstant::from_ticks(
            self.origin.elapsed().as_millis() as u64 + duration.as_millis() as u64,
        ))
    }
}

impl TimeService for TestClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(self.origin.elapsed().as_millis() as u64)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let wait = deadline
            .instant()
            .ticks()
            .saturating_sub(self.now().ticks());
        let (sender, receiver) = oneshot::channel();
        thread::spawn(move || {
            if wait != 0 {
                thread::sleep(Duration::from_millis(wait));
            }
            let _ = sender.send(DeadlineObservation::new(deadline, deadline.instant()));
        });
        Box::pin(async move {
            receiver
                .await
                .unwrap_or_else(|_| DeadlineObservation::new(deadline, deadline.instant()))
        })
    }
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<AtomicUsize>,
    fail_release: bool,
}

struct TrackingAttachment {
    inner: LocalProcessHost,
    releases: Arc<AtomicUsize>,
}

impl AttachmentService for TrackingAttachment {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
        self.inner.materialize_file(scope, descriptor)
    }

    fn release_file(&self, lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release_file(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, Ordering::SeqCst);
            outcome
        })
    }
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: ScopeId,
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
        let release = CredentialService::release(&self.inner, lease);
        let releases = Arc::clone(&self.releases);
        let fail_release = self.fail_release;
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, Ordering::SeqCst);
            if fail_release {
                CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "fixture.opencode.cleanup_failed",
                    "Fixture OpenCode credential cleanup failed",
                ))
            } else {
                outcome
            }
        })
    }
}
