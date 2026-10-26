fn prepare(
    route: CursorPreparedDriver,
    host_id: ExecutionHostId,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    prepare_release(route, host_id, VERSION)
}

fn prepare_release(
    route: CursorPreparedDriver,
    host_id: ExecutionHostId,
    release: &str,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    let access_id = AccessProfileId::new("cursor.fixture.access").expect("access id");
    let input = CursorPreparationInput::new(
        route,
        swallowtail_core::ConfiguredInstanceId::new(format!("cursor.fixture.{route:?}"))
            .expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("cursor.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new(CURSOR_AGENT_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        cursor_subscription_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let host = support::FixtureHost::completed([stdout(release)]);
    block_on(prepare_cursor(
        input,
        probe(),
        services_with_resource(&host, host_id),
    ))
}

fn evidence(access_id: AccessProfileId) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access_id,
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

fn headless_input(access: ResourceAccess) -> CursorHeadlessRunProfileInput {
    parameterized_input(
        parameterized_selection("fixture-model").expect("selection"),
        access,
    )
}

fn parameterized_selection(
    model: &str,
) -> Result<CursorHeadlessModelSelection, PreparationFailure> {
    Ok(CursorHeadlessModelSelection::new(
        ModelRouteId::new("cursor.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("cursor").expect("provider"),
        ModelId::new(model).expect("model"),
    ))
}

fn parameterized_input(
    model: CursorHeadlessModelSelection,
    access: ResourceAccess,
) -> CursorHeadlessRunProfileInput {
    CursorHeadlessRunProfileInput::new(
        request_id(match access {
            ResourceAccess::Read => "headless-read",
            ResourceAccess::ReadWrite => "headless-write",
        }),
        model,
        OperationContent::new("fixture-private-prompt").expect("prompt"),
        working_resource(),
        access,
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn probe() -> CursorPreparationProbe {
    CursorPreparationProbe::new(
        request_id("probe"),
        ScopeId::new("cursor-preparation-probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn services_with_resource(host: &support::FixtureHost, host_id: ExecutionHostId) -> HostServices {
    host.services(host_id)
        .with_working_resource(Arc::new(UnusedResourceService))
        .with_working_resource_io(Arc::new(UnusedResourceIoService))
}

struct UnusedResourceService;

impl WorkingResourceService for UnusedResourceService {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        MaterializedResourceRef::new("/fixture/cursor").expect("materialized"),
                    ),
            )
        })
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
        Box::pin(async { CleanupOutcome::Clean })
    }
}

struct UnusedResourceIoService;

impl WorkingResourceIoService for UnusedResourceIoService {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }

    fn write_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.cursor.prepared.unused",
        "Unused Cursor prepared fixture service was called",
    ))
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("cursor.fixture.host").expect("host id")
}

fn request_id(value: &str) -> RequestId {
    RequestId::new(value).expect("request id")
}

fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource")
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}

#[derive(Clone)]
struct AcpHandshakeHost {
    agent: Arc<AcpHandshakeAgent>,
    arguments: Arc<Mutex<Option<Vec<String>>>>,
}

struct AcpHandshakeAgent {
    state: Mutex<AcpHandshakeState>,
    changed: Condvar,
}

#[derive(Default)]
struct AcpHandshakeState {
    output: VecDeque<ProcessOutputChunk>,
    stopped: bool,
}

impl AcpHandshakeHost {
    fn new() -> Self {
        Self {
            agent: Arc::new(AcpHandshakeAgent {
                state: Mutex::new(AcpHandshakeState::default()),
                changed: Condvar::new(),
            }),
            arguments: Arc::new(Mutex::new(None)),
        }
    }

    fn services(&self, host_id: ExecutionHostId) -> HostServices {
        HostServices::new(host_id)
            .with_time(Arc::new(AcpHandshakeTime))
            .with_task(Arc::new(AcpHandshakeTaskService))
            .with_process(Arc::new(self.clone()))
            .with_working_resource(Arc::new(UnusedResourceService))
            .with_working_resource_io(Arc::new(UnusedResourceIoService))
    }

    fn arguments(&self) -> Vec<String> {
        self.arguments
            .lock()
            .expect("ACP handshake arguments lock")
            .clone()
            .expect("ACP handshake process started")
    }
}

struct AcpHandshakeTime;

impl TimeService for AcpHandshakeTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

struct AcpHandshakeTaskService;

impl ScopedTaskService for AcpHandshakeTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(AcpHandshakeTask(Some(std::thread::spawn(
            move || {
                block_on(task);
            },
        )))))
    }
}

struct AcpHandshakeTask(Option<JoinHandle<()>>);

impl JoinedTask for AcpHandshakeTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.0.take().expect("ACP handshake task joins once");
        Box::pin(async move { handle.join().map_err(|_| handshake_failure()) })
    }
}

impl ProcessService for AcpHandshakeHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.arguments.lock().expect("ACP handshake arguments lock") =
            Some(request.arguments().map(str::to_owned).collect());
        let handle =
            Box::new(AcpHandshakeProcess(Arc::clone(&self.agent))) as Box<dyn ProcessHandle>;
        Box::pin(async move { Ok(handle) })
    }
}

struct AcpHandshakeProcess(Arc<AcpHandshakeAgent>);

impl AcpHandshakeAgent {
    fn enqueue(state: &mut AcpHandshakeState, message: Value) {
        let mut bytes = serde_json::to_vec(&message).expect("ACP handshake serializes");
        bytes.push(b'\n');
        state
            .output
            .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
    }

    fn write(&self, chunk: ProcessInputChunk) -> Result<(), RuntimeFailure> {
        let message: Value =
            serde_json::from_slice(chunk.bytes()).map_err(|_| handshake_failure())?;
        let mut state = self.state.lock().expect("ACP handshake lock");
        let id = message.get("id").cloned();
        match message.get("method").and_then(Value::as_str) {
            Some("initialize") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {
                        "protocolVersion": 1,
                        "agentCapabilities": {
                            "loadSession": true,
                            "mcpCapabilities": {"http": true, "sse": true},
                            "promptCapabilities": {
                                "audio": false,
                                "embeddedContext": false,
                                "image": true
                            },
                            "sessionCapabilities": {"list": {}}
                        },
                        "authMethods": [
                            {"id": "cursor_login", "name": "Cursor Login"}
                        ]
                    }
                }),
            ),
            Some("session/new") => Self::enqueue(
                &mut state,
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "result": {"sessionId": "cursor-fixture-session"}
                }),
            ),
            _ => {}
        }
        self.changed.notify_all();
        Ok(())
    }

    fn stop(&self) {
        let mut state = self.state.lock().expect("ACP handshake lock");
        state.stopped = true;
        self.changed.notify_all();
    }
}

impl ProcessHandle for AcpHandshakeProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = self.0.write(chunk);
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.stop();
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self.0.state.lock().expect("ACP handshake lock");
            while state.output.is_empty() && !state.stopped {
                state = self.0.changed.wait(state).expect("ACP handshake wait lock");
            }
            Ok(state.output.pop_front())
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.stop();
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.stop();
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

fn handshake_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.cursor.prepared.acp_handshake",
        "Cursor prepared ACP handshake fixture failed",
    ))
}
