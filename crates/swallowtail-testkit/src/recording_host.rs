use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{Diagnostic, EndpointAudience, ExecutionHostId, SafeDiagnostic};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, AuthorizedEndpoint, BlockingJob,
    BlockingWorkService, BoxFuture, CleanupOutcome, CredentialLease, CredentialRef,
    CredentialService, Deadline, DeadlineObservation, DelegatedCredential, DiagnosticObserver,
    EndpointRef, HostServices, JoinedTask, MaterializedFileRef, MaterializedResourceRef,
    MonotonicInstant, NetworkGrant, NetworkPolicyService, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, ProcessRequest, ProcessService, ResourceAccess,
    ResourceLease, ResourceRepresentation, RuntimeFailure, SchemaDocument, SchemaFileLease,
    SchemaService, ScopeId, ScopedTaskService, TimeService, WorkingResourceRef,
    WorkingResourceService,
};

mod serving;
mod working_resource_io;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RecordedHostCall {
    TaskSpawn,
    TaskJoin,
    BlockingWork,
    TimeNow,
    TimeWaitUntil,
    ProcessStart,
    ProcessGracefulStop,
    ProcessForceStop,
    ProcessWait,
    NetworkAuthorize,
    CredentialAcquire,
    CredentialRelease,
    WorkingResourceResolve,
    WorkingResourceCreateTemporary,
    WorkingResourceRelease,
    WorkingResourceReadText,
    WorkingResourceWriteText,
    AttachmentMaterializeFile,
    AttachmentFileRelease,
    ModelArtifactAcquire,
    ModelArtifactRelease,
    ServingEndpointPublish,
    ServingEndpointRelease,
    SchemaMaterializeFile,
    SchemaFileRelease,
    DiagnosticObserve,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordingOutcome {
    Succeed,
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
}

pub struct RecordingHostServices {
    state: Arc<RecordingState>,
    services: HostServices,
}

impl RecordingHostServices {
    #[must_use]
    pub fn new(outcome: RecordingOutcome) -> Self {
        Self::for_host(
            ExecutionHostId::new("fixture.host.local").expect("fixture host id is valid"),
            outcome,
        )
    }

    #[must_use]
    pub fn for_host(execution_host_id: ExecutionHostId, outcome: RecordingOutcome) -> Self {
        let state = Arc::new(RecordingState::default());
        let service = Arc::new(RecordingService {
            state: Arc::clone(&state),
            outcome,
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
        Self { state, services }
    }

    #[must_use]
    pub const fn services(&self) -> &HostServices {
        &self.services
    }

    #[must_use]
    pub fn calls(&self) -> Vec<RecordedHostCall> {
        self.state.calls()
    }

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

pub fn poll_immediate<T>(future: impl Future<Output = T>) -> T {
    let mut future = Box::pin(future);
    let mut context = Context::from_waker(Waker::noop());
    match Pin::as_mut(&mut future).poll(&mut context) {
        Poll::Ready(value) => value,
        Poll::Pending => panic!("recording fixture future was not immediately ready"),
    }
}
