use super::agent::FixtureProcessHandle;
use super::*;
use swallowtail_runtime::CredentialService;

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.process.lock().expect("fixture process lock poisoned") = Some(ObservedProcess {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environment_count: request.environment().len(),
            working_resource: request.working_resource().cloned(),
        });
        let handle =
            Box::new(FixtureProcessHandle(Arc::clone(&self.agent))) as Box<dyn ProcessHandle>;
        Box::pin(async move { Ok(handle) })
    }
}

impl CredentialService for FixtureHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.credential_acquires.fetch_add(1, Ordering::SeqCst);
        let lease = CredentialLease::Secret(SecretLease::new(
            scope,
            reference,
            b"fixture-key".to_vec(),
            audience,
        ));
        Box::pin(async move { Ok(lease) })
    }

    fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        self.credential_releases.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl WorkingResourceService for FixtureHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let lease = ResourceLease::consumer_owned(scope, reference, access, representation)
            .with_filesystem(
                swallowtail_runtime::MaterializedResourceRef::new("/private/fixture")
                    .expect("fixture path is valid"),
            );
        Box::pin(async move { Ok(lease) })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        self.resource_releases.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl WorkingResourceIoService for FixtureHost {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        if request.locator().as_host_value() != "/private/fixture/src/lib.rs" {
            return Box::pin(async { Err(fixture_failure()) });
        }
        self.reads.fetch_add(1, Ordering::SeqCst);
        let content = WorkingResourceText::new("fixture file".to_owned(), request.maximum_bytes())
            .map_err(|_| fixture_failure());
        Box::pin(async move { content })
    }
}

pub(super) struct ThreadTaskService;
struct ThreadTask(Option<std::thread::JoinHandle<()>>);

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Some(std::thread::spawn(move || {
            block_on(task);
        })))))
    }
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.0.take().expect("fixture task joins once");
        Box::pin(async move { handle.join().map_err(|_| fixture_failure()) })
    }
}

pub(super) struct FixtureTime {
    immediate: bool,
}

impl FixtureTime {
    pub(super) const fn new(immediate: bool) -> Self {
        Self { immediate }
    }
}

impl TimeService for FixtureTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        if self.immediate {
            Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
        } else {
            Box::pin(std::future::pending())
        }
    }
}
