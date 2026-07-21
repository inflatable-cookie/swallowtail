#[derive(Clone)]
pub struct FixtureHost {
    agent: Arc<SharedAgent>,
    process: Arc<Mutex<Option<ProcessRequest>>>,
    credentials: Arc<AtomicUsize>,
    credential_releases: Arc<AtomicUsize>,
    resource_releases: Arc<AtomicUsize>,
    resource_writes: Arc<Mutex<Vec<(String, String)>>>,
    cleanup_events: Arc<Mutex<Vec<CleanupEvent>>>,
}

impl FixtureHost {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            agent: Arc::new(SharedAgent {
                state: Mutex::new(AgentState::default()),
                changed: Condvar::new(),
                scenario,
            }),
            process: Arc::new(Mutex::new(None)),
            credentials: Arc::new(AtomicUsize::new(0)),
            credential_releases: Arc::new(AtomicUsize::new(0)),
            resource_releases: Arc::new(AtomicUsize::new(0)),
            resource_writes: Arc::new(Mutex::new(Vec::new())),
            cleanup_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    pub fn process_arguments(&self) -> Option<Vec<String>> {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .as_ref()
            .map(|request| request.arguments().map(str::to_owned).collect())
    }

    pub fn process_started(&self) -> bool {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .is_some()
    }

    pub fn credential_acquisitions(&self) -> usize {
        self.credentials.load(Ordering::SeqCst)
    }

    pub fn cleanup_counts(&self) -> (usize, usize) {
        (
            self.resource_releases.load(Ordering::SeqCst),
            self.credential_releases.load(Ordering::SeqCst),
        )
    }

    pub fn resource_writes(&self) -> Vec<(String, String)> {
        self.resource_writes
            .lock()
            .expect("fixture writes lock poisoned")
            .clone()
    }

    pub fn cleanup_events(&self) -> Vec<CleanupEvent> {
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .clone()
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.process.lock().expect("fixture process lock poisoned") = Some(request);
        let process = Box::new(FixtureProcess {
            agent: Arc::clone(&self.agent),
            cleanup_events: Arc::clone(&self.cleanup_events),
        }) as Box<dyn ProcessHandle>;
        Box::pin(async move { Ok(process) })
    }
}

struct FixtureProcess {
    agent: Arc<SharedAgent>,
    cleanup_events: Arc<Mutex<Vec<CleanupEvent>>>,
}

impl FixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self
            .agent
            .state
            .lock()
            .expect("fixture agent lock poisoned");
        state.stopped = true;
        self.agent.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.agent.handle_write(chunk);
        Box::pin(async move { result })
    }
    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }
    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self
                .agent
                .state
                .lock()
                .expect("fixture agent lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .agent
                    .changed
                    .wait(state)
                    .expect("fixture agent wait lock poisoned");
            }
            Ok(state.output.pop_front())
        })
    }
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }
    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .push(CleanupEvent::ProcessWait);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl CredentialService for FixtureHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.credentials.fetch_add(1, Ordering::SeqCst);
        let lease = CredentialLease::Delegated(DelegatedCredential::new(
            scope, reference, audience,
        ));
        Box::pin(async move { Ok(lease) })
    }

    fn release(&self, _lease: CredentialLease) -> BoxFuture<'static, CleanupOutcome> {
        self.credential_releases.fetch_add(1, Ordering::SeqCst);
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .push(CleanupEvent::CredentialRelease);
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
                swallowtail_runtime::MaterializedResourceRef::new("/fixture/workspace")
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
        self.cleanup_events
            .lock()
            .expect("fixture cleanup-event lock poisoned")
            .push(CleanupEvent::ResourceRelease);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

impl WorkingResourceIoService for FixtureHost {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        let result = WorkingResourceText::new("fixture".to_owned(), request.maximum_bytes())
            .map_err(|_| fixture_failure());
        Box::pin(async move { result })
    }

    fn write_text(
        &self,
        lease: &ResourceLease,
        request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        if lease.access() != ResourceAccess::ReadWrite {
            return Box::pin(async { Err(fixture_failure()) });
        }
        self.resource_writes
            .lock()
            .expect("fixture writes lock poisoned")
            .push((
                request.locator().as_host_value().to_owned(),
                request.content().as_driver_value().to_owned(),
            ));
        Box::pin(async { Ok(()) })
    }
}

struct ThreadTaskService;
struct ThreadTask(Option<JoinHandle<()>>);

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

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.kimi_acp.failed",
        "Kimi ACP fixture failed",
    ))
}
