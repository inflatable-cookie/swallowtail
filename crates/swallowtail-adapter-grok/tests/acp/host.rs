#[derive(Clone)]
struct FixtureHost {
    agent: Arc<Agent>,
    process: Arc<Mutex<Option<ProcessObservation>>>,
    credential_acquires: Arc<AtomicUsize>,
    credential_releases: Arc<AtomicUsize>,
    resource_releases: Arc<AtomicUsize>,
    deadline_gate: Arc<Mutex<Option<Arc<std::sync::atomic::AtomicBool>>>>,
}

#[derive(Clone, Debug)]
struct ProcessObservation {
    arguments: Vec<String>,
    environment_count: usize,
    resource: Option<WorkingResourceRef>,
}

impl FixtureHost {
    fn new(scenario: Scenario) -> Self {
        Self::with_version(scenario, "0.2.114")
    }

    fn with_version(scenario: Scenario, version: &str) -> Self {
        Self {
            agent: Arc::new(Agent {
                state: Mutex::new(AgentState::default()),
                changed: Condvar::new(),
                scenario,
                version: version.to_owned(),
            }),
            process: Arc::new(Mutex::new(None)),
            credential_acquires: Arc::new(AtomicUsize::new(0)),
            credential_releases: Arc::new(AtomicUsize::new(0)),
            resource_releases: Arc::new(AtomicUsize::new(0)),
            deadline_gate: Arc::new(Mutex::new(None)),
        }
    }

    /// Holds the registered-open deadline until this flag is set.
    ///
    /// Without it a deadline that is ready immediately can expire during
    /// `initialize` and never reach the case the test names.
    fn fire_deadline_when(&self, gate: Arc<std::sync::atomic::AtomicBool>) {
        *self.deadline_gate.lock().expect("deadline gate lock") = Some(gate);
    }

    fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(self.clone()))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    fn writes(&self) -> Vec<Value> {
        self.agent
            .state
            .lock()
            .expect("agent lock poisoned")
            .writes
            .clone()
    }

    fn wait_for_write<F>(&self, predicate: F) -> bool
    where
        F: Fn(&[Value]) -> bool,
    {
        let mut state = self.agent.state.lock().expect("agent lock poisoned");
        while !predicate(&state.writes) && !state.stopped {
            state = self
                .agent
                .changed
                .wait(state)
                .expect("agent wait lock poisoned");
        }
        predicate(&state.writes)
    }

    fn emit_permission_without_turn(&self) {
        let mut state = self.agent.state.lock().expect("agent lock poisoned");
        Agent::enqueue(
            &mut state,
            json!({
                "jsonrpc": "2.0",
                "id": 902,
                "method": "session/request_permission",
                "params": {
                    "sessionId": "grok-fixture-session",
                    "toolCall": {"toolCallId": "no-active-turn"},
                    "options": [{
                        "optionId": "reject-once",
                        "name": "Reject once",
                        "kind": "reject_once"
                    }]
                }
            }),
        );
        self.agent.changed.notify_all();
    }

    /// Returns the courier Grok spawned from the declared ACP MCP server.
    fn spawned_registered_courier(&self) -> Option<Arc<SpawnedMcpChild>> {
        self.agent
            .state
            .lock()
            .expect("agent lock poisoned")
            .spawned_mcp
            .first()
            .map(Arc::clone)
    }

    /// Completes the held prompt, exactly as Grok ends a turn normally.
    fn complete_turn(&self) {
        let mut state = self.agent.state.lock().expect("agent lock poisoned");
        if let Some(prompt_id) = state.prompt_id.take() {
            Agent::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": prompt_id,
                    "result": {"stopReason": "end_turn"}
                }),
            );
        }
        self.agent.changed.notify_all();
    }

    fn release_deadline(&self) {
        let mut state = self.agent.state.lock().expect("agent lock poisoned");
        state.deadline_released = true;
        self.agent.changed.notify_all();
    }
}

impl TimeService for FixtureHost {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        if matches!(self.agent.scenario, Scenario::Deadline) {
            Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
        } else if matches!(
            self.agent.scenario,
            Scenario::RegisteredOpenUnanswered | Scenario::RegisteredOpenBlockedCall
        ) {
            // The registered-open deadline fires only once the provider has
            // actually received `session/new` and, when a test asks for it,
            // once its registered call is outstanding. A deadline that expired
            // during `initialize` would not exercise the named case.
            //
            // The waiting happens on its own thread: the bounded open polls
            // this future from its very first step, on the same thread that
            // must later send `session/new`, so blocking inside the poll would
            // deadlock the open it is meant to bound.
            let agent = Arc::clone(&self.agent);
            let gate = self
                .deadline_gate
                .lock()
                .expect("deadline gate lock")
                .clone();
            let fired = Arc::new(std::sync::atomic::AtomicBool::new(false));
            let started = Arc::new(std::sync::atomic::AtomicBool::new(false));
            Box::pin(async move {
                std::future::poll_fn(move |context| {
                    if fired.load(Ordering::SeqCst) {
                        return std::task::Poll::Ready(());
                    }
                    if !started.swap(true, Ordering::SeqCst) {
                        let waker = context.waker().clone();
                        let agent = Arc::clone(&agent);
                        let gate = gate.clone();
                        let fired = Arc::clone(&fired);
                        std::thread::spawn(move || {
                            {
                                let mut state =
                                    agent.state.lock().expect("agent lock poisoned");
                                while !state.session_new_seen && !state.stopped {
                                    state = agent
                                        .changed
                                        .wait(state)
                                        .expect("agent wait lock poisoned");
                                }
                            }
                            if let Some(gate) = gate {
                                while !gate.load(Ordering::SeqCst) {
                                    std::thread::sleep(std::time::Duration::from_millis(5));
                                }
                            }
                            fired.store(true, Ordering::SeqCst);
                            waker.wake();
                        });
                    }
                    std::task::Poll::Pending
                })
                .await;
                DeadlineObservation::new(deadline, deadline.instant())
            })
        } else if matches!(self.agent.scenario, Scenario::PermissionTimeout) {
            let agent = Arc::clone(&self.agent);
            Box::pin(async move {
                let mut state = agent.state.lock().expect("agent lock poisoned");
                while (!state.permission_emitted || !state.deadline_released) && !state.stopped {
                    state = agent
                        .changed
                        .wait(state)
                        .expect("agent wait lock poisoned");
                }
                DeadlineObservation::new(deadline, deadline.instant())
            })
        } else {
            Box::pin(std::future::pending())
        }
    }
}

struct FixtureProcess(Arc<Agent>);

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.write(chunk);
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self.0.state.lock().expect("agent lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .0
                    .changed
                    .wait(state)
                    .expect("agent wait lock poisoned");
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
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

impl FixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let mut state = self.0.state.lock().expect("agent lock poisoned");
        state.stopped = true;
        for child in state.spawned_mcp.drain(..) {
            child.kill();
        }
        self.0.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.process.lock().expect("process lock poisoned") = Some(ProcessObservation {
            arguments: request.arguments().map(str::to_owned).collect(),
            environment_count: request.environment().len(),
            resource: request.working_resource().cloned(),
        });
        let handle = Box::new(FixtureProcess(Arc::clone(&self.agent))) as Box<dyn ProcessHandle>;
        Box::pin(async move { Ok(handle) })
    }
}

impl CredentialService for FixtureHost {
    fn acquire(
        &self,
        scope: ScopeId,
        reference: CredentialRef,
        audience: swallowtail_core::EndpointAudience,
    ) -> BoxFuture<'static, Result<CredentialLease, RuntimeFailure>> {
        self.credential_acquires.fetch_add(1, Ordering::SeqCst);
        Box::pin(async move {
            Ok(CredentialLease::Delegated(DelegatedCredential::new(
                scope, reference, audience,
            )))
        })
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
                swallowtail_runtime::MaterializedResourceRef::new("/private/grok-fixture")
                    .expect("fixture path"),
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
        let result = WorkingResourceText::new("fixture".to_owned(), request.maximum_bytes())
            .map_err(|_| fixture_failure());
        Box::pin(async move { result })
    }

    fn write_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
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
        let handle = self.0.take().expect("task joins once");
        Box::pin(async move { handle.join().map_err(|_| fixture_failure()) })
    }
}
