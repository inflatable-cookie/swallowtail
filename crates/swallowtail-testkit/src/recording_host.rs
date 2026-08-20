use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{Diagnostic, EndpointAudience, ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    ApprovedUrlRef, AttachmentDescriptor, AttachmentFileLease, AttachmentService,
    AuthorizedEndpoint, BlockingJob, BlockingWorkService, BoxFuture, CleanupOutcome,
    CredentialLease, CredentialRef, CredentialService, Deadline, DeadlineObservation,
    DebugObservation, DelegatedCredential, DeviceAuthorizationId, DeviceAuthorizationReceipt,
    DeviceCodeDisplayService, DeviceCodePrompt, DiagnosticObserver, EndpointRef, HostServices,
    JoinedTask, LoopbackCallbackId, LoopbackCallbackLease, LoopbackCallbackReceipt,
    LoopbackCallbackService, MaterializedFileRef, MaterializedResourceRef, MonotonicInstant,
    NetworkGrant, NetworkPolicyService, ProcessExit, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessRequest, ProcessService, ResourceAccess, ResourceLease,
    ResourceRepresentation, RuntimeFailure, SchemaDocument, SchemaFileLease, SchemaService,
    ScopeId, ScopedTaskService, TimeService, UrlOpenService, WorkingResourceRef,
    WorkingResourceService,
};

mod serving;
mod working_resource_io;

/// One host-service interaction recorded by [`RecordingHostServices`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecordedHostCall {
    /// A scoped task was spawned.
    TaskSpawn,
    /// A scoped task handle was joined.
    TaskJoin,
    /// Blocking work was requested.
    BlockingWork,
    /// Monotonic time was observed.
    TimeNow,
    /// A deadline wait was requested.
    TimeWaitUntil,
    /// A child process was started.
    ProcessStart,
    /// Graceful process stop was requested.
    ProcessGracefulStop,
    /// Forced process stop was requested.
    ProcessForceStop,
    /// Process completion was awaited.
    ProcessWait,
    /// Network access was authorized.
    NetworkAuthorize,
    /// A credential lease was acquired.
    CredentialAcquire,
    /// A credential lease was released.
    CredentialRelease,
    /// A working resource was resolved.
    WorkingResourceResolve,
    /// A temporary working resource was created.
    WorkingResourceCreateTemporary,
    /// A working-resource lease was released.
    WorkingResourceRelease,
    /// Text was read through working-resource I/O.
    WorkingResourceReadText,
    /// Text was written through working-resource I/O.
    WorkingResourceWriteText,
    /// An attachment was materialized as a file.
    AttachmentMaterializeFile,
    /// A materialized attachment was released.
    AttachmentFileRelease,
    /// A model-artifact lease was acquired.
    ModelArtifactAcquire,
    /// A model-artifact lease was released.
    ModelArtifactRelease,
    /// A serving endpoint was published.
    ServingEndpointPublish,
    /// A serving endpoint was released.
    ServingEndpointRelease,
    /// A schema was materialized as a file.
    SchemaMaterializeFile,
    /// A materialized schema was released.
    SchemaFileRelease,
    /// A safe diagnostic was observed.
    DiagnosticObserve,
    /// A structured debug observation was observed.
    DebugObserve,
    /// A host-approved sign-in URL was opened.
    UrlOpen,
    /// A sign-in loopback callback was bound.
    LoopbackBind,
    /// Sign-in loopback arrival was polled.
    LoopbackPoll,
    /// A credential reference was materialized from a loopback receipt.
    LoopbackMaterialize,
    /// A sign-in loopback lease was released.
    LoopbackRelease,
    /// A device code was displayed.
    DeviceCodeDisplay,
    /// Device authorization was polled.
    DevicePoll,
    /// A credential reference was materialized from a device receipt.
    DeviceMaterialize,
}

/// Configured result returned by recording host services.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordingOutcome {
    /// Every fallible service interaction succeeds.
    Succeed,
    /// Every fallible service interaction returns this safe diagnostic.
    Fail(SafeDiagnostic),
}

impl RecordingOutcome {
    fn result(&self) -> Result<(), RuntimeFailure> {
        match self {
            Self::Succeed => Ok(()),
            Self::Fail(diagnostic) => Err(RuntimeFailure::new(diagnostic.clone())),
        }
    }
}

#[derive(Default)]
struct RecordingState {
    calls: Mutex<Vec<RecordedHostCall>>,
}

impl RecordingState {
    fn record(&self, call: RecordedHostCall) {
        self.calls
            .lock()
            .expect("recording host call lock poisoned")
            .push(call);
    }

    fn calls(&self) -> Vec<RecordedHostCall> {
        self.calls
            .lock()
            .expect("recording host call lock poisoned")
            .clone()
    }
}

#[derive(Clone)]
struct RecordingService {
    state: Arc<RecordingState>,
    outcome: RecordingOutcome,
}

impl RecordingService {
    fn record(&self, call: RecordedHostCall) -> Result<(), RuntimeFailure> {
        self.state.record(call);
        self.outcome.result()
    }

    fn cleanup(&self, call: RecordedHostCall) -> CleanupOutcome {
        self.state.record(call);
        match &self.outcome {
            RecordingOutcome::Succeed => CleanupOutcome::Clean,
            RecordingOutcome::Fail(diagnostic) => CleanupOutcome::Failed(diagnostic.clone()),
        }
    }
}

struct RecordingJoinedTask(RecordingService);

impl JoinedTask for RecordingJoinedTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self.0.record(RecordedHostCall::TaskJoin);
        Box::pin(async move { result })
    }
}

impl ScopedTaskService for RecordingService {
    fn spawn(
        &self,
        _scope: ScopeId,
        _task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        self.record(RecordedHostCall::TaskSpawn)?;
        Ok(Box::new(RecordingJoinedTask(self.clone())))
    }
}

impl BlockingWorkService for RecordingService {
    fn run(
        &self,
        _scope: ScopeId,
        _job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self.record(RecordedHostCall::BlockingWork);
        Box::pin(async move { result })
    }
}

impl TimeService for RecordingService {
    fn now(&self) -> MonotonicInstant {
        self.state.record(RecordedHostCall::TimeNow);
        MonotonicInstant::from_ticks(17)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        self.state.record(RecordedHostCall::TimeWaitUntil);
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

struct RecordingProcessHandle(RecordingService);

impl ProcessHandle for RecordingProcessHandle {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.record(RecordedHostCall::ProcessGracefulStop);
        Box::pin(async move { result })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.record(RecordedHostCall::ProcessForceStop);
        Box::pin(async move { result })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        let result = self.0.record(RecordedHostCall::ProcessWait);
        Box::pin(async move { result.map(|()| ProcessExit::new(true, Some(0))) })
    }
}

impl ProcessService for RecordingService {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::ProcessStart)
            .map(|()| Box::new(RecordingProcessHandle(self.clone())) as Box<dyn ProcessHandle>);
        Box::pin(async move { result })
    }
}

impl NetworkPolicyService for RecordingService {
    fn authorize(
        &self,
        scope: ScopeId,
        endpoint: EndpointRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<NetworkGrant, RuntimeFailure>> {
        let result = self.record(RecordedHostCall::NetworkAuthorize).map(|()| {
            NetworkGrant::new(
                scope,
                endpoint,
                audience,
                AuthorizedEndpoint::new("https://recording.invalid/v1")
                    .expect("recording endpoint is valid"),
            )
        });
        Box::pin(async move { result })
    }
}

impl CredentialService for RecordingService {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        let result = self.record(RecordedHostCall::CredentialAcquire).map(|()| {
            CredentialLease::Delegated(DelegatedCredential::new(scope, reference, audience))
        });
        Box::pin(async move { result })
    }

    fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self.cleanup(RecordedHostCall::CredentialRelease);
        Box::pin(async move { outcome })
    }
}

impl WorkingResourceService for RecordingService {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::WorkingResourceResolve)
            .map(|()| ResourceLease::consumer_owned(scope, reference, access, representation))
            .and_then(|lease| recording_resource_lease(lease, representation));
        Box::pin(async move { result })
    }

    fn create_temporary(
        &self,
        scope: ScopeId,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::WorkingResourceCreateTemporary)
            .map(|()| {
                ResourceLease::operation_scoped(
                    scope,
                    WorkingResourceRef::new("recording.temporary-resource")
                        .expect("recording reference is valid"),
                    access,
                    representation,
                )
            })
            .and_then(|lease| recording_resource_lease(lease, representation));
        Box::pin(async move { result })
    }

    fn release(&self, lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome =
            if lease.cleanup_authority() == swallowtail_runtime::LeaseCleanupAuthority::Consumer {
                self.state.record(RecordedHostCall::WorkingResourceRelease);
                CleanupOutcome::NotApplicable
            } else {
                self.cleanup(RecordedHostCall::WorkingResourceRelease)
            };
        Box::pin(async move { outcome })
    }
}

fn recording_resource_lease(
    lease: ResourceLease,
    representation: ResourceRepresentation,
) -> Result<ResourceLease, RuntimeFailure> {
    if representation == ResourceRepresentation::Filesystem {
        Ok(lease.with_filesystem(
            MaterializedResourceRef::new("/private/recording/workspace")
                .expect("recording filesystem root is valid"),
        ))
    } else {
        Ok(lease)
    }
}

impl AttachmentService for RecordingService {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
        let reference = descriptor.reference().clone();
        let result = self
            .record(RecordedHostCall::AttachmentMaterializeFile)
            .map(|()| {
                AttachmentFileLease::operation_scoped(
                    scope,
                    reference,
                    MaterializedFileRef::new("/private/recording/attachment.png")
                        .expect("recording file is valid"),
                )
            });
        Box::pin(async move { result })
    }

    fn release_file(&self, _lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self.cleanup(RecordedHostCall::AttachmentFileRelease);
        Box::pin(async move { outcome })
    }
}

impl SchemaService for RecordingService {
    fn materialize_file(
        &self,
        scope: ScopeId,
        _document: SchemaDocument,
    ) -> BoxFuture<'static, Result<SchemaFileLease, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::SchemaMaterializeFile)
            .map(|()| {
                SchemaFileLease::operation_scoped(
                    scope,
                    MaterializedFileRef::new("/private/recording/schema.json")
                        .expect("recording file is valid"),
                )
            });
        Box::pin(async move { result })
    }

    fn release_file(&self, _lease: SchemaFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self.cleanup(RecordedHostCall::SchemaFileRelease);
        Box::pin(async move { outcome })
    }
}

impl DiagnosticObserver for RecordingService {
    fn observe(&self, _diagnostic: &Diagnostic) {
        self.state.record(RecordedHostCall::DiagnosticObserve);
    }

    fn observe_debug(&self, _observation: &DebugObservation) {
        self.state.record(RecordedHostCall::DebugObserve);
    }
}

impl UrlOpenService for RecordingService {
    fn open(
        &self,
        _scope: ScopeId,
        _url: ApprovedUrlRef,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self.record(RecordedHostCall::UrlOpen);
        Box::pin(async move { result })
    }
}

impl LoopbackCallbackService for RecordingService {
    fn bind(
        &self,
        scope: ScopeId,
    ) -> BoxFuture<'static, Result<LoopbackCallbackLease, RuntimeFailure>> {
        let result = self.record(RecordedHostCall::LoopbackBind).map(|()| {
            LoopbackCallbackLease::new(
                scope,
                LoopbackCallbackId::new("recording.loopback").expect("callback id is valid"),
            )
        });
        Box::pin(async move { result })
    }

    fn poll(
        &self,
        lease: &LoopbackCallbackLease,
    ) -> BoxFuture<'static, Result<Option<LoopbackCallbackReceipt>, RuntimeFailure>> {
        let result = self
            .record(RecordedHostCall::LoopbackPoll)
            .map(|()| Some(LoopbackCallbackReceipt::new(lease.callback_id().clone())));
        Box::pin(async move { result })
    }

    fn materialize_credential(
        &self,
        _receipt: &LoopbackCallbackReceipt,
        _audience: &swallowtail_core::EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record(RecordedHostCall::LoopbackMaterialize)?;
        Ok(CredentialRef::new("recording.sign-in.credential")
            .expect("recording credential is valid"))
    }

    fn release(&self, _lease: LoopbackCallbackLease) -> BoxFuture<'static, CleanupOutcome> {
        let outcome = self.cleanup(RecordedHostCall::LoopbackRelease);
        Box::pin(async move { outcome })
    }
}

impl DeviceCodeDisplayService for RecordingService {
    fn display(
        &self,
        _scope: ScopeId,
        _prompt: DeviceCodePrompt,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let result = self.record(RecordedHostCall::DeviceCodeDisplay);
        Box::pin(async move { result })
    }

    fn poll_authorization(
        &self,
        _scope: &ScopeId,
    ) -> BoxFuture<'static, Result<Option<DeviceAuthorizationReceipt>, RuntimeFailure>> {
        let result = self.record(RecordedHostCall::DevicePoll).map(|()| {
            Some(DeviceAuthorizationReceipt::new(
                DeviceAuthorizationId::new("recording.device").expect("authorization id is valid"),
            ))
        });
        Box::pin(async move { result })
    }

    fn materialize_credential(
        &self,
        _receipt: &DeviceAuthorizationReceipt,
        _audience: &swallowtail_core::EndpointAudience,
    ) -> Result<CredentialRef, RuntimeFailure> {
        self.record(RecordedHostCall::DeviceMaterialize)?;
        Ok(CredentialRef::new("recording.sign-in.credential")
            .expect("recording credential is valid"))
    }
}

/// Complete in-memory host service registry that records every interaction.
pub struct RecordingHostServices {
    state: Arc<RecordingState>,
    outcome: RecordingOutcome,
    services: HostServices,
}

impl RecordingHostServices {
    /// Creates recording services under the canonical fixture host identity.
    #[must_use]
    pub fn new(outcome: RecordingOutcome) -> Self {
        Self::for_host(
            ExecutionHostId::new("fixture.host.local").expect("fixture host id is valid"),
            outcome,
        )
    }

    /// Creates recording services under an explicit execution host identity.
    #[must_use]
    pub fn for_host(execution_host_id: ExecutionHostId, outcome: RecordingOutcome) -> Self {
        let state = Arc::new(RecordingState::default());
        let service = Arc::new(RecordingService {
            state: Arc::clone(&state),
            outcome: outcome.clone(),
        });
        let services = HostServices::new(execution_host_id)
            .with_task(service.clone())
            .with_blocking_work(service.clone())
            .with_time(service.clone())
            .with_process(service.clone())
            .with_network(service.clone())
            .with_credential(service.clone())
            .with_working_resource(service.clone())
            .with_working_resource_io(service.clone())
            .with_attachment(service.clone())
            .with_model_artifact(service.clone())
            .with_serving_endpoint(service.clone())
            .with_schema(service.clone())
            .with_diagnostic_observer(service);
        Self {
            state,
            outcome,
            services,
        }
    }

    /// Registers interactive sign-in ports. Registration does not start sign-in.
    #[must_use]
    pub fn with_sign_in_ports(mut self) -> Self {
        let service = Arc::new(RecordingService {
            state: Arc::clone(&self.state),
            outcome: self.outcome.clone(),
        });
        self.services = self
            .services
            .with_url_open(service.clone())
            .with_loopback_callback(service.clone())
            .with_device_code_display(service);
        self
    }

    /// Returns the provider-neutral host service registry.
    #[must_use]
    pub const fn services(&self) -> &HostServices {
        &self.services
    }

    /// Returns an ordered snapshot of all recorded calls.
    #[must_use]
    pub fn calls(&self) -> Vec<RecordedHostCall> {
        self.state.calls()
    }

    /// Counts occurrences of one host-service interaction.
    #[must_use]
    pub fn count(&self, call: RecordedHostCall) -> usize {
        self.calls().iter().filter(|seen| **seen == call).count()
    }
}

impl Default for RecordingHostServices {
    fn default() -> Self {
        Self::new(RecordingOutcome::Succeed)
    }
}

/// Resolves a fixture future that is required to be immediately ready.
pub fn poll_immediate<T>(future: impl Future<Output = T>) -> T {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("recording fixture future was not immediately ready"),
    }
}

#[cfg(test)]
mod tests {
    use super::{RecordedHostCall, RecordingHostServices};
    use swallowtail_core::HostServiceKind;

    #[test]
    fn sign_in_ports_are_opt_in_and_registration_records_no_calls() {
        let default_host = RecordingHostServices::default();
        assert!(
            !default_host
                .services()
                .available_kinds()
                .contains(&HostServiceKind::UrlOpen)
        );

        let recording = RecordingHostServices::default().with_sign_in_ports();
        assert!(
            recording
                .services()
                .available_kinds()
                .contains(&HostServiceKind::UrlOpen)
        );
        assert!(
            recording
                .services()
                .available_kinds()
                .contains(&HostServiceKind::LoopbackCallback)
        );
        assert!(
            recording
                .services()
                .available_kinds()
                .contains(&HostServiceKind::DeviceCodeDisplay)
        );
        assert_eq!(recording.count(RecordedHostCall::UrlOpen), 0);
        assert_eq!(recording.count(RecordedHostCall::LoopbackBind), 0);
        assert_eq!(recording.count(RecordedHostCall::DeviceCodeDisplay), 0);
    }
}
