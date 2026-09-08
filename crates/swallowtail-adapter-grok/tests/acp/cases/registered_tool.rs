// Provider-free route-binding proofs for registered tools on `grok-build.acp`.
//
// The fake ACP agent spawns the reserved courier exactly as Grok was observed
// doing for a client-declared `mcpServers` entry. The real Contract 063
// proxy, courier, kernel, lease, and dispatcher carry one mediated call. No
// live Grok, credential, provider endpoint, or paid inference is used, and
// nothing here publishes a support claim.

const REGISTERED_NAMESPACE: &str = "desktop";
const REGISTERED_LOCAL_NAME: &str = "reconcile";
const REGISTERED_OPEN_DEADLINE_TICKS: u64 = 10_000_000_000;
const REGISTERED_TURN: &str = "grok-registered-turn";

struct BlockingDispatcher {
    entered: Arc<std::sync::atomic::AtomicBool>,
    release: Arc<std::sync::atomic::AtomicBool>,
    waker: Arc<Mutex<Option<std::task::Waker>>>,
}

impl swallowtail_runtime::RegisteredToolDispatcher for BlockingDispatcher {
    fn dispatch(
        &self,
        _call: swallowtail_runtime::RegisteredToolCall,
        _context: swallowtail_runtime::RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<swallowtail_runtime::RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let release = Arc::clone(&self.release);
        let waker = Arc::clone(&self.waker);
        Box::pin(std::future::poll_fn(move |context| {
            use std::task::Poll;
            entered.store(true, Ordering::SeqCst);
            if release.load(Ordering::SeqCst) {
                return Poll::Ready(Err(fixture_failure()));
            }
            *waker.lock().expect("blocking dispatcher waker lock") = Some(context.waker().clone());
            Poll::Pending
        }))
    }
}

impl BlockingDispatcher {
    fn release(&self) {
        self.release.store(true, Ordering::SeqCst);
        if let Some(waker) = self
            .waker
            .lock()
            .expect("blocking dispatcher waker lock")
            .take()
        {
            waker.wake();
        }
    }
}

struct CountingDispatcher {
    calls: Arc<AtomicUsize>,
}

impl swallowtail_runtime::RegisteredToolDispatcher for CountingDispatcher {
    fn dispatch(
        &self,
        call: swallowtail_runtime::RegisteredToolCall,
        _context: swallowtail_runtime::RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<swallowtail_runtime::RegisteredToolOutcome, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let payload = swallowtail_runtime::RegisteredToolPayload::new(
            swallowtail_runtime::RegisteredToolSchemaMediaType::new("application/json")
                .expect("media type"),
            br#"{"answer":"from-dispatcher"}"#.to_vec(),
            call.binding().effective_bounds().max_result_bytes(),
        )
        .expect("bounded dispatcher result");
        let result = swallowtail_runtime::RegisteredToolResult::new(
            payload,
            swallowtail_runtime::RegisteredToolSchemaDigest::new("sha256:output")
                .expect("output digest"),
        );
        Box::pin(std::future::ready(Ok(
            swallowtail_runtime::RegisteredToolOutcome::completed(&call, result),
        )))
    }
}

struct CourierClient {
    process: Arc<SpawnedMcpChild>,
}

impl CourierClient {
    fn request(&self, request: &[u8]) -> String {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("provider writes one JSONL request");
        self.read_stdout_line()
            .expect("provider reads one JSONL response")
    }

    fn notify(&self, request: &[u8]) {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("provider writes one notification");
    }

    fn handshake(&self) {
        let initialized = self.request(
            format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"{}\"}}}}",
                swallowtail_adapter_grok::registered_tool::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION
            )
            .as_bytes(),
        );
        assert!(
            initialized.contains(
                swallowtail_adapter_grok::registered_tool::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION
            ),
            "courier handshake: {initialized}"
        );
        self.notify(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    }

    fn list_tools(&self) -> String {
        self.request(br#"{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}"#)
    }

    fn call_tool(&self, name: &str, arguments: &str) -> String {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        self.request(request.as_bytes())
    }

    /// Issues one call that may get no answer at all.
    ///
    /// After the lease settles the courier may exit rather than answer, which
    /// is still a refusal: what matters is that no result is delivered and no
    /// dispatch happens.
    fn try_call_tool(&self, name: &str, arguments: &str) -> Option<String> {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        let mut bytes = request.into_bytes();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes))).ok()?;
        self.read_stdout_line()
    }

    fn read_stdout_line(&self) -> Option<String> {
        let chunk = block_on(self.process.read_output()).ok()??;
        Some(String::from_utf8(chunk.bytes().to_vec()).expect("JSON response is UTF-8"))
    }
}

/// Builds the real Contract 063 courier once for this test binary.
fn courier_binary() -> &'static std::path::Path {
    static PATH: std::sync::OnceLock<std::path::PathBuf> = std::sync::OnceLock::new();
    PATH.get_or_init(|| {
        let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir")
            .parent()
            .expect("workspace")
            .to_path_buf();
        // Nested cargo must not share the outer `cargo test` target lock.
        let nested_target = workspace.join("target").join("card118-courier");
        let status = std::process::Command::new("cargo")
            .args([
                "build",
                "-p",
                "swallowtail-host-local",
                "--features",
                "mediated-stdio-proxy",
                "--bin",
                "swallowtail-registered-tool-courier",
            ])
            .env("CARGO_TARGET_DIR", &nested_target)
            .current_dir(&workspace)
            .status()
            .expect("courier build starts");
        assert!(status.success(), "courier binary failed to build");
        let binary = nested_target
            .join("debug")
            .join("swallowtail-registered-tool-courier");
        assert!(binary.is_file(), "missing courier binary at {binary:?}");
        binary
    })
}

fn registered_tool_id() -> swallowtail_runtime::RegisteredToolId {
    named_registered_tool_id(REGISTERED_NAMESPACE, REGISTERED_LOCAL_NAME)
}

fn named_registered_tool_id(
    namespace: &str,
    local_name: &str,
) -> swallowtail_runtime::RegisteredToolId {
    swallowtail_runtime::RegisteredToolId::new(
        swallowtail_runtime::RegisteredToolNamespace::new(namespace).expect("namespace"),
        swallowtail_runtime::RegisteredToolLocalName::new(local_name).expect("local name"),
    )
}

fn registered_schema(digest: &str) -> swallowtail_runtime::RegisteredToolSchema {
    swallowtail_runtime::RegisteredToolSchema::new(
        swallowtail_runtime::RegisteredToolSchemaNamespace::new("desktop.registered-tools.schema")
            .expect("schema namespace"),
        swallowtail_runtime::RegisteredToolSchemaMediaType::new("application/json")
            .expect("media type"),
        swallowtail_runtime::RegisteredToolSchemaDialect::new("json-schema-2020-12")
            .expect("dialect"),
        swallowtail_runtime::RegisteredServerRevision::new("1").expect("revision"),
        swallowtail_runtime::RegisteredToolSchemaDigest::new(digest).expect("digest"),
        swallowtail_runtime::RegisteredToolSchemaDocument::new("{\"type\":\"object\"}")
            .expect("document"),
    )
}

fn registered_snapshot(
    host: &ExecutionHostId,
    executable: swallowtail_runtime::ExecutableRef,
    environment: EnvironmentRef,
    tool: swallowtail_runtime::RegisteredToolId,
    kind: swallowtail_runtime::RegisteredToolExecutionKind,
) -> swallowtail_runtime::RegisteredToolSnapshot {
    swallowtail_runtime::RegisteredToolSnapshot::new(
        swallowtail_runtime::RegisteredToolSnapshotInput {
            server_id: swallowtail_runtime::RegisteredServerId::new("desktop.registered-tools")
                .expect("server"),
            revision: swallowtail_runtime::RegisteredServerRevision::new("2026-09-07.1")
                .expect("revision"),
            execution_host_id: host.clone(),
            declarations: vec![
                swallowtail_runtime::RegisteredToolDeclaration::new(
                    tool,
                    kind,
                    registered_schema("sha256:input"),
                    registered_schema("sha256:output"),
                    swallowtail_runtime::RegisteredToolEffectPosture::Mutating,
                    swallowtail_runtime::RegisteredToolRetryPosture::ConsumerRetryable,
                    swallowtail_runtime::RegisteredToolBounds::ceiling(),
                )
                .expect("declaration"),
            ],
            transports: vec![
                swallowtail_runtime::RegisteredToolTransportSupport::new(
                    swallowtail_runtime::RegisteredToolTransport::PrivateLoopbackHttp,
                    [
                        swallowtail_runtime::RegisteredToolProtocolVersion::new(
                            swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
                        )
                        .expect("protocol"),
                    ],
                )
                .expect("transport"),
                swallowtail_runtime::RegisteredToolTransportSupport::new(
                    swallowtail_runtime::RegisteredToolTransport::HostMediatedCallback,
                    [
                        swallowtail_runtime::RegisteredToolProtocolVersion::new(
                            swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
                        )
                        .expect("protocol"),
                    ],
                )
                .expect("transport"),
            ],
            required_services: [].into_iter().collect(),
            credential_references: Vec::new(),
            executable_recipes: vec![executable],
            environment_recipes: vec![environment],
            bounds: swallowtail_runtime::RegisteredToolBounds::ceiling(),
            source: swallowtail_runtime::RegisteredToolSource::new(
                swallowtail_runtime::RegisteredToolSourceId::new("desktop.registration.1")
                    .expect("source"),
                swallowtail_runtime::MonotonicInstant::from_ticks(1),
            ),
        },
    )
    .expect("snapshot")
}

struct RegisteredFixtureInput {
    tool: swallowtail_runtime::RegisteredToolId,
    kind: swallowtail_runtime::RegisteredToolExecutionKind,
    mediated: bool,
}

impl Default for RegisteredFixtureInput {
    fn default() -> Self {
        Self {
            tool: registered_tool_id(),
            kind: swallowtail_runtime::RegisteredToolExecutionKind::Mcp,
            mediated: true,
        }
    }
}

fn registered_preparation(
    host: ExecutionHostId,
    admission: swallowtail_runtime::ConsumerAdmissionBinding,
    executable: swallowtail_runtime::ExecutableRef,
    environment: EnvironmentRef,
    input: RegisteredFixtureInput,
) -> swallowtail_runtime::RegisteredToolPreparation {
    let snapshot = Arc::new(registered_snapshot(
        &host,
        executable.clone(),
        environment.clone(),
        input.tool.clone(),
        input.kind,
    ));
    let transport = if input.mediated {
        swallowtail_runtime::RegisteredToolTransport::PrivateLoopbackHttp
    } else {
        swallowtail_runtime::RegisteredToolTransport::HostMediatedCallback
    };
    let selection = swallowtail_runtime::RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [input.tool],
        transport,
        swallowtail_runtime::RegisteredToolProtocolVersion::new(
            swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
        )
        .expect("protocol"),
    )
    .expect("selection");
    let selection = if input.mediated {
        selection
            .with_attachment(swallowtail_runtime::RegisteredToolAttachment::MediatedStdioProxy)
            .with_proxy_recipe(
                swallowtail_runtime::RegisteredToolProxyRecipe::new(
                    executable,
                    environment,
                    swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG,
                )
                .expect("recipe"),
            )
    } else {
        selection
    };
    swallowtail_runtime::RegisteredToolPreparation::new(
        snapshot,
        selection,
        admission,
        swallowtail_runtime::RegisteredToolLimits::ceiling(),
    )
}

struct OpenedRegisteredRoute {
    session: Box<dyn InteractiveSessionHandle>,
    fixture: FixtureHost,
    local: swallowtail_host_local::LocalHostServices,
    services: HostServices,
    courier: CourierClient,
}

fn registered_turn_id() -> RuntimeTurnId {
    RuntimeTurnId::new(REGISTERED_TURN).expect("turn")
}

fn registered_open_deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(REGISTERED_OPEN_DEADLINE_TICKS))
}

fn registered_route_services(
    host_id: &ExecutionHostId,
    fixture: &FixtureHost,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    courier_path: &std::path::Path,
    executable: &swallowtail_runtime::ExecutableRef,
    environment: &EnvironmentRef,
) -> (swallowtail_host_local::LocalHostServices, HostServices) {
    registered_route_services_with_budget(
        host_id,
        fixture,
        dispatcher,
        courier_path,
        executable,
        environment,
        None,
    )
}

fn registered_route_services_with_budget(
    host_id: &ExecutionHostId,
    fixture: &FixtureHost,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    courier_path: &std::path::Path,
    executable: &swallowtail_runtime::ExecutableRef,
    environment: &EnvironmentRef,
    cleanup_budget: Option<std::time::Duration>,
) -> (swallowtail_host_local::LocalHostServices, HostServices) {
    let mut builder = swallowtail_host_local::LocalProcessHost::builder(
        swallowtail_host_local::LocalProcessLimits::default(),
    )
    .approve_executable(executable.clone(), courier_path)
    .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
    .with_registered_tool_dispatcher(dispatcher)
    .with_registered_tool_clock(Arc::new(fixture.clone()));
    if let Some(budget) = cleanup_budget {
        builder = builder.with_registered_tool_cleanup_budget(budget);
    }
    let local = builder.build_services(host_id.clone());
    let services = local
        .services()
        .clone()
        .with_task(Arc::new(fixture.task_service()))
        .with_time(Arc::new(fixture.clone()))
        .with_process(Arc::new(fixture.clone()))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_working_resource_io(Arc::new(fixture.clone()));
    (local, services)
}

fn open_registered_route(
    host_name: &str,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
) -> OpenedRegisteredRoute {
    open_registered_route_for(host_name, Scenario::Success, dispatcher, admission, None)
}

fn open_registered_route_for(
    host_name: &str,
    scenario: Scenario,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
    cleanup_budget: Option<std::time::Duration>,
) -> OpenedRegisteredRoute {
    match try_open_registered_route(host_name, scenario, dispatcher, admission, cleanup_budget) {
        Ok(opened) => opened,
        Err(boxed) => panic!("registered session opens: {}", boxed.0.diagnostic().code()),
    }
}

fn try_open_registered_route(
    host_name: &str,
    scenario: Scenario,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
    cleanup_budget: Option<std::time::Duration>,
) -> Result<
    OpenedRegisteredRoute,
    Box<(RuntimeFailure, swallowtail_host_local::LocalHostServices)>,
> {
    try_open_registered_route_with_deadline(
        host_name,
        scenario,
        dispatcher,
        admission,
        cleanup_budget,
        registered_open_deadline(),
    )
}

fn try_open_registered_route_with_deadline(
    host_name: &str,
    scenario: Scenario,
    dispatcher: Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
    cleanup_budget: Option<std::time::Duration>,
    open_deadline: Deadline,
) -> Result<
    OpenedRegisteredRoute,
    Box<(RuntimeFailure, swallowtail_host_local::LocalHostServices)>,
> {
    let host_id = ExecutionHostId::new(host_name).expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(scenario);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services_with_budget(
        &host_id,
        &fixture,
        dispatcher,
        courier_binary(),
        &executable,
        &environment,
        cleanup_budget,
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(admission),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("mediated stdio selection qualifies")
            .with_host(local.clone())
            .with_open_deadline(open_deadline)
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let session = match block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services.clone(),
    )) {
        Ok(session) => session,
        Err(error) => return Err(Box::new((error, local))),
    };
    let courier = CourierClient {
        process: fixture
            .spawned_registered_courier()
            .expect("Grok spawned the declared courier child"),
    };
    Ok(OpenedRegisteredRoute {
        session,
        fixture,
        local,
        services,
        courier,
    })
}

fn registered_open_request(resource: WorkingResourceRef) -> OpenSessionRequest {
    OpenSessionRequest::new(
        RequestId::new("grok-registered-open").expect("request"),
        resource,
        None,
        SessionPlanAgreement::explicit(
            SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
            Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
            Some(HarnessConfigurationPosture::Ambient),
        ),
    )
}

fn close_registered_route(opened: OpenedRegisteredRoute) {
    let _ = block_on(close_session(opened.session, opened.services));
}

fn declared_registered_server(fixture: &FixtureHost) -> Value {
    fixture
        .writes()
        .into_iter()
        .find(|write| write["method"] == "session/new")
        .expect("session/new is on the wire")["params"]["mcpServers"][0]
        .clone()
}

#[test]
fn an_open_without_registered_tools_still_sends_an_empty_mcp_server_list() {
    let (host, services, session) = open(Scenario::Success);
    let session_new = host
        .writes()
        .into_iter()
        .find(|write| write["method"] == "session/new")
        .expect("session/new is on the wire");
    assert_eq!(
        session_new["params"]["mcpServers"],
        json!([]),
        "omission stays byte-identical: {session_new}"
    );
    let _ = block_on(close_session(session, services));
}

#[test]
fn a_registered_open_declares_the_reserved_courier_in_the_acp_mcp_server_list() {
    let opened = open_registered_route(
        "fixture.host.grok.registered-declare",
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
    );
    let declared = declared_registered_server(&opened.fixture);
    assert_eq!(
        declared["name"],
        swallowtail_adapter_grok::registered_tool::GROK_ACP_REGISTERED_TOOL_SERVER
    );
    assert_eq!(
        declared["command"].as_str().expect("declared command"),
        courier_binary().to_str().expect("courier path is UTF-8"),
        "the ACP command is the host-resolved filesystem path, not an ExecutableRef host value"
    );
    assert_eq!(
        declared["args"][0],
        swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG
    );
    let rendezvous = std::path::PathBuf::from(
        declared["args"][1].as_str().expect("rendezvous path"),
    );
    assert!(
        !rendezvous.exists(),
        "ready expires the one-shot rendezvous after the courier authenticates: {rendezvous:?}"
    );
    assert_eq!(
        declared["env"],
        json!([{"name": "PATH", "value": "/usr/bin"}]),
        "ACP env is the allowlisted recipe as a name/value list: {declared}"
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    close_registered_route(opened);
}

#[test]
fn one_registered_call_round_trips_through_the_courier_into_the_dispatcher() {
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let opened = open_registered_route(
        "fixture.host.grok.registered-round-trip",
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::clone(&admission),
    );
    opened.courier.handshake();
    let listed = opened.courier.list_tools();
    assert!(
        listed.contains(&registered_tool_id().to_string()),
        "tools/list carries the namespaced identity: {listed}"
    );
    assert!(
        listed.contains("\"type\":\"object\""),
        "tools/list carries the declared input schema: {listed}"
    );
    let called = opened
        .courier
        .call_tool(&registered_tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        called.contains("from-dispatcher"),
        "one result correlates back to its call: {called}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(
        admission
            .observed_phases()
            .contains(&swallowtail_runtime::AdmissionPhase::BeforeDispatch),
        "dispatch is kernel-admitted: {:?}",
        admission.observed_phases()
    );
    close_registered_route(opened);
}

#[test]
fn a_registered_call_revoked_before_dispatch_never_reaches_the_dispatcher() {
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    admission.revoke_from(swallowtail_runtime::AdmissionPhase::BeforeDispatch);
    let opened = open_registered_route(
        "fixture.host.grok.registered-deny",
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        admission,
    );
    opened.courier.handshake();
    let denied = opened
        .courier
        .call_tool(&registered_tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        !denied.contains("from-dispatcher"),
        "a denial delivers no result: {denied}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_registered_route(opened);
}

#[test]
fn an_unknown_courier_tool_name_fails_without_reaching_the_dispatcher() {
    let calls = Arc::new(AtomicUsize::new(0));
    let opened = open_registered_route(
        "fixture.host.grok.registered-unknown-tool",
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
    );
    opened.courier.handshake();
    let rejected = opened.courier.call_tool("desktop/not-registered", "{}");
    assert!(
        rejected.contains("error"),
        "an unregistered name fails closed: {rejected}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_registered_route(opened);
}

#[test]
fn closing_a_registered_session_joins_the_lease_and_the_shared_listener() {
    let opened = open_registered_route(
        "fixture.host.grok.registered-close",
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    assert_eq!(opened.local.operation_bridge_listener_count(), 1);
    let local = opened.local.clone();
    close_registered_route(opened);
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn a_cancelled_registered_session_closes_its_lease_before_the_route_leases() {
    let opened = open_registered_route(
        "fixture.host.grok.registered-cancel",
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
    );
    let local = opened.local.clone();
    let courier = Arc::clone(&opened.courier.process);
    block_on(opened.session.cancellation().request())
        .expect("session cancellation is acknowledged");
    let services = opened.services.clone();
    let _ = block_on(close_session(opened.session, services));
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
    // The courier is the provider's child: cancellation and close leave no
    // running Swallowtail-owned process behind.
    assert!(
        block_on(courier.wait()).is_ok(),
        "the provider-owned courier child terminated with the cancelled session"
    );
}

#[test]
fn the_registered_capability_projects_unqualified_with_the_real_route_gate_pending() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-projection").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::Success);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services(
        &host_id,
        &fixture,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        courier_binary(),
        &executable,
        &environment,
    );
    let _ = local;
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("mediated stdio selection qualifies");
    assert_eq!(
        swallowtail_adapter_grok::registered_tool::grok_build_acp_registered_tool_qualification(),
        swallowtail_runtime::RegisteredToolRouteQualification::Unqualified,
        "the route stays unqualified until the separately authorized live gate runs"
    );
    let readiness = swallowtail_runtime::RegisteredToolReadiness::evaluate(
        &services,
        binding.selection(),
    );
    let contribution =
        swallowtail_adapter_grok::registered_tool::project_grok_build_acp_registered_tool(
            &swallowtail_runtime::ConsumerRouteApplicability::from_plan(&selected.plan),
            binding.carrier(),
            &readiness,
        )
        .expect("registered capability projects");
    let mediation = contribution
        .selection_rows()
        .find(|row| {
            row.safe_reason().is_some_and(|reason| {
                reason.diagnostic().code()
                    == swallowtail_adapter_grok::registered_tool::GROK_ACP_REAL_ROUTE_GATE_PENDING_CODE
            })
        })
        .expect("the mediation-kind row publishes the pending real-route gate");
    assert_eq!(
        mediation.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Unavailable
    );
    assert_eq!(
        mediation.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Unknown
    );
    assert!(
        contribution.selection_rows().all(|row| {
            row.support() != swallowtail_runtime::ConsumerRouteSupportPosture::Supported
        }),
        "an unqualified registered capability is never supported by inference"
    );
}

#[test]
fn a_host_mediated_selection_is_refused_before_any_provider_work() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-transport").expect("host");
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe"),
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment"),
        RegisteredFixtureInput {
            mediated: false,
            ..RegisteredFixtureInput::default()
        },
    );
    let Err(error) =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
    else {
        panic!("Grok ACP carries registered tools over the mediated stdio courier only");
    };
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.grok.acp.registered_tool.transport_unsupported"
    );
}

#[test]
fn a_non_mcp_execution_kind_is_refused_before_any_provider_work() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-kind").expect("host");
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe"),
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment"),
        RegisteredFixtureInput {
            kind: swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
            ..RegisteredFixtureInput::default()
        },
    );
    let Err(error) =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
    else {
        panic!("Grok ACP registered mediation presents MCP-kind tools only");
    };
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.grok.acp.registered_tool.kind_unsupported"
    );
}

#[test]
fn an_identity_the_courier_cannot_spell_is_refused_before_any_provider_work() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-identity").expect("host");
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe"),
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment"),
        RegisteredFixtureInput {
            tool: named_registered_tool_id("desktop/nested", "reconcile"),
            ..RegisteredFixtureInput::default()
        },
    );
    let Err(error) =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
    else {
        panic!("an unspellable namespaced identity must fail closed");
    };
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.grok.acp.registered_tool.identity_unprojectable"
    );
}

#[test]
fn a_registered_open_without_a_host_composition_fails_typed() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-host-missing").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::Success);
    let preparation = registered_preparation(
        host_id.clone(),
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe"),
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment"),
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("qualify does not require the host composition")
            .with_open_deadline(registered_open_deadline())
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        fixture.services(host_id),
    )) else {
        panic!("registered open requires the local host composition");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.host_missing"
    );
    assert!(
        !host_started_a_process(&fixture),
        "a refused registered open never spawns Grok"
    );
}

#[test]
fn a_registered_open_without_a_deadline_fails_typed() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-deadline").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::Success);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services(
        &host_id,
        &fixture,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        courier_binary(),
        &executable,
        &environment,
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("qualify does not require a deadline")
            .with_host(local)
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    )) else {
        panic!("registered open requires an explicit registered-tool deadline");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.deadline_missing"
    );
}

#[test]
fn an_unspawnable_courier_command_fails_typed_before_grok_starts() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-unspawnable").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::Success);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let missing = std::env::temp_dir().join("swallowtail-card118-missing-courier");
    let (local, services) = registered_route_services(
        &host_id,
        &fixture,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        &missing,
        &executable,
        &environment,
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("an unspawnable command still qualifies")
            .with_host(local)
            .with_open_deadline(registered_open_deadline())
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    )) else {
        panic!("registered open requires a spawnable courier filesystem path");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.command_unspawnable"
    );
    assert!(
        !host_started_a_process(&fixture),
        "a refused registered open never spawns Grok"
    );
}

fn host_started_a_process(host: &FixtureHost) -> bool {
    host.process
        .lock()
        .expect("process lock poisoned")
        .is_some()
}

/// Drives one bounded courier `tools/call` on its own thread.
///
/// The dispatcher blocks, so the call stays outstanding while the test
/// observes the lifecycle around it.
struct OutstandingCall {
    handle: std::thread::JoinHandle<()>,
}

impl OutstandingCall {
    fn issue(courier: &CourierClient) -> Self {
        let process = Arc::clone(&courier.process);
        let name = registered_tool_id().to_string();
        Self {
            handle: std::thread::spawn(move || {
                let client = CourierClient { process };
                let request = format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{{}}}}}}"
                );
                let mut bytes = request.into_bytes();
                bytes.push(b'\n');
                if block_on(client.process.write_stdin(ProcessInputChunk::new(bytes))).is_ok() {
                    let _ = client.read_stdout_line();
                }
            }),
        }
    }

    fn join(self) {
        let _ = self.handle.join();
    }
}

fn wait_until(condition: impl Fn() -> bool, what: &str) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while !condition() {
        assert!(std::time::Instant::now() < deadline, "never observed {what}");
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

#[test]
fn the_bound_registered_turn_settles_its_lease_at_the_turn_terminal() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-turn-terminal",
        Scenario::RegisteredTurn,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    );
    opened.courier.handshake();
    // One real ACP turn, on the exact attempt the lease was opened for.
    let mut turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    let called = opened
        .courier
        .call_tool(&registered_tool_id().to_string(), r#"{"path":"in-turn"}"#);
    assert!(
        called.contains("from-dispatcher"),
        "the bound turn admits its registered call: {called}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    opened.fixture.complete_turn();
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("the turn publishes one terminal outcome"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    // Settlement completes before terminal is published, so this is asserted
    // the instant the consumer observes terminal, with no wait that could hide
    // a late close.
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        0,
        "the lease must already be settled when terminal is observed"
    );
    assert_eq!(opened.local.operation_bridge_listener_count(), 0);
    let after_terminal = opened
        .courier
        .try_call_tool(&registered_tool_id().to_string(), r#"{"path":"post-terminal"}"#);
    assert!(
        after_terminal
            .as_deref()
            .is_none_or(|answer| !answer.contains("from-dispatcher")),
        "a post-terminal call must not dispatch: {after_terminal:?}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    close_registered_route(opened);
}

#[test]
fn cancelling_the_bound_registered_turn_freezes_admission_and_settles_cancelled() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-turn-cancel",
        Scenario::Cancellation,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    );
    opened.courier.handshake();
    let mut turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    block_on(turn.cancellation().request()).expect("turn cancellation requested");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("the cancelled turn publishes one terminal outcome"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        0,
        "the lease must already be settled when the cancelled terminal is observed"
    );
    let after_cancel = opened
        .courier
        .try_call_tool(&registered_tool_id().to_string(), r#"{"path":"post-cancel"}"#);
    assert!(
        after_cancel
            .as_deref()
            .is_none_or(|answer| !answer.contains("from-dispatcher")),
        "a post-cancel call must not dispatch: {after_cancel:?}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_registered_route(opened);
}

#[test]
fn a_turn_other_than_the_bound_one_is_refused_while_the_lease_is_live() {
    let mut opened = open_registered_route(
        "fixture.host.grok.registered-turn-mismatch",
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
    );
    let request = TurnRequest::new(
        RuntimeTurnId::new("grok-unbound-turn").expect("turn"),
        OperationContent::new("private fixture prompt").expect("prompt"),
    );
    let Err(error) = block_on(
        opened
            .session
            .start_turn(request, opened.services.clone()),
    ) else {
        panic!("an unbound turn must not run beside a live registered lease");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.turn_mismatch"
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    close_registered_route(opened);
}

#[test]
fn a_registered_open_the_provider_never_answers_fails_on_its_own_deadline() {
    let Err(boxed) = try_open_registered_route(
        "fixture.host.grok.registered-open-deadline",
        Scenario::RegisteredOpenUnanswered,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    ) else {
        panic!("an unanswered session/new must not hang a registered open");
    };
    let (error, local) = *boxed;
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.open_deadline"
    );
    // The bounded open joined the partial resources it created.
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn a_registered_open_without_a_bound_turn_fails_typed() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-turn-missing").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::Success);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services(
        &host_id,
        &fixture,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        courier_binary(),
        &executable,
        &environment,
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("qualify does not require a turn")
            .with_host(local)
            .with_open_deadline(registered_open_deadline());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    )) else {
        panic!("registered open requires the exact turn its lease serves");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.turn_missing"
    );
    assert!(
        !host_started_a_process(&fixture),
        "a refused registered open never spawns Grok"
    );
}

#[test]
fn a_failed_registered_cleanup_is_never_a_clean_session_close() {
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        release: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: Arc::new(Mutex::new(None)),
    });
    let opened = open_registered_route_for(
        "fixture.host.grok.registered-cleanup-failed",
        Scenario::Success,
        Arc::clone(&blocking) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Some(std::time::Duration::from_millis(50)),
    );
    opened.courier.handshake();
    let call = OutstandingCall::issue(&opened.courier);
    wait_until(
        || blocking.entered.load(Ordering::SeqCst),
        "the dispatcher entering its blocking call",
    );
    let fixture = opened.fixture.clone();
    let local = opened.local.clone();
    let services = opened.services.clone();
    let outcome = block_on(close_session(opened.session, services));
    assert!(
        matches!(outcome, CleanupOutcome::Failed(_)),
        "an unjoined registered lease is never a clean session close: {outcome:?}"
    );
    // A retained bridge lease means the operation is not over: the working
    // resource and the credential stay held rather than returned for reuse.
    assert_eq!(fixture.resource_releases.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.credential_releases.load(Ordering::SeqCst), 0);
    assert_eq!(
        local.registered_tool_lease_count(),
        1,
        "the host retains the lease it could not join"
    );
    blocking.release();
    call.join();
}

/// A dispatcher that blocks until cancelled or released.
///
/// Unlike [`BlockingDispatcher`] it cooperates: it observes the kernel's
/// call-bound cancellation and settles, which is what a freeze must actually
/// deliver to an outstanding call.
/// Blocks forever, but reports the exact moment the kernel froze its lease.
///
/// The freeze is the first step of `close`, and the join that follows spins for
/// the cleanup budget. Observing it is therefore a precise signal that a
/// settlement is in flight, which is what a concurrency fixture needs instead
/// of a sleep.
struct FreezeObservingDispatcher {
    entered: Arc<std::sync::atomic::AtomicBool>,
    froze: Arc<std::sync::atomic::AtomicBool>,
}

impl swallowtail_runtime::RegisteredToolDispatcher for FreezeObservingDispatcher {
    fn dispatch(
        &self,
        _call: swallowtail_runtime::RegisteredToolCall,
        context: swallowtail_runtime::RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<swallowtail_runtime::RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let froze = Arc::clone(&self.froze);
        let cancellation = context.cancellation().clone();
        let started = Arc::new(std::sync::atomic::AtomicBool::new(false));
        Box::pin(std::future::poll_fn(move |context| {
            entered.store(true, Ordering::SeqCst);
            if cancellation.is_cancelled() {
                froze.store(true, Ordering::SeqCst);
            }
            if !started.swap(true, Ordering::SeqCst) {
                let waker = context.waker().clone();
                let cancellation = cancellation.clone();
                let froze = Arc::clone(&froze);
                std::thread::spawn(move || {
                    while !cancellation.is_cancelled() {
                        std::thread::sleep(std::time::Duration::from_millis(2));
                    }
                    froze.store(true, Ordering::SeqCst);
                    waker.wake();
                });
            }
            // Never settles: the close must time out and report failed cleanup.
            std::task::Poll::Pending
        }))
    }
}

struct CancellableDispatcher {
    entered: Arc<std::sync::atomic::AtomicBool>,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
}

impl swallowtail_runtime::RegisteredToolDispatcher for CancellableDispatcher {
    fn dispatch(
        &self,
        _call: swallowtail_runtime::RegisteredToolCall,
        context: swallowtail_runtime::RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<swallowtail_runtime::RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let observed = Arc::clone(&self.cancelled);
        let cancellation = context.cancellation().clone();
        let started = Arc::new(std::sync::atomic::AtomicBool::new(false));
        Box::pin(std::future::poll_fn(move |context| {
            use std::task::Poll;
            entered.store(true, Ordering::SeqCst);
            if cancellation.is_cancelled() {
                observed.store(true, Ordering::SeqCst);
                return Poll::Ready(Err(fixture_failure()));
            }
            if !started.swap(true, Ordering::SeqCst) {
                // The kernel sets the flag; nothing wakes this future, so the
                // fixture polls for it exactly as a real dispatcher would have
                // to observe an out-of-band cancellation.
                let waker = context.waker().clone();
                let cancellation = cancellation.clone();
                std::thread::spawn(move || {
                    while !cancellation.is_cancelled() {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                    }
                    waker.wake();
                });
            }
            Poll::Pending
        }))
    }
}

#[test]
fn cancelling_a_turn_with_an_outstanding_call_cancels_that_call() {
    let cancelled = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let dispatcher = Arc::new(CancellableDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        cancelled: Arc::clone(&cancelled),
    });
    let entered = Arc::clone(&dispatcher.entered);
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-cancel-outstanding",
        Scenario::RegisteredTurn,
        Arc::clone(&dispatcher) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    );
    opened.courier.handshake();
    let turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    let call = OutstandingCall::issue(&opened.courier);
    wait_until(
        || entered.load(Ordering::SeqCst),
        "the dispatcher entering its call",
    );
    // Cancellation must reach the call itself, not only future admission.
    block_on(turn.cancellation().request()).expect("turn cancellation requested");
    assert!(
        cancelled.load(Ordering::SeqCst),
        "an outstanding call must observe cancellation, not stay free to settle"
    );
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        0,
        "cancellation settles the lease it froze"
    );
    call.join();
    close_registered_route(opened);
}

#[test]
fn a_transport_failure_settles_the_lease_before_it_publishes_terminal() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-transport-terminal",
        Scenario::Disconnect,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    );
    opened.courier.handshake();
    let mut turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    // The pump is the terminal publisher on this path, not the prompt task.
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("the failed turn publishes one terminal outcome"),
    );
    assert!(
        matches!(terminal.status(), TerminalStatus::RuntimeFailed(_)),
        "a transport failure is a failed turn: {:?}",
        terminal.status()
    );
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        0,
        "the pump must settle the lease before it publishes terminal"
    );
    let after_terminal = opened
        .courier
        .try_call_tool(&registered_tool_id().to_string(), r#"{"path":"post-transport"}"#);
    assert!(
        after_terminal
            .as_deref()
            .is_none_or(|answer| !answer.contains("from-dispatcher")),
        "a call after transport terminal must not dispatch: {after_terminal:?}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_registered_route(opened);
}

#[test]
fn a_failed_settlement_refuses_every_later_turn_and_fails_its_own() {
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        release: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: Arc::new(Mutex::new(None)),
    });
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-settle-failed",
        Scenario::RegisteredTurn,
        Arc::clone(&blocking) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Some(std::time::Duration::from_millis(50)),
    );
    opened.courier.handshake();
    let mut turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    let call = OutstandingCall::issue(&opened.courier);
    wait_until(
        || blocking.entered.load(Ordering::SeqCst),
        "the dispatcher entering its blocking call",
    );
    opened.fixture.complete_turn();
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("the turn publishes one terminal outcome"),
    );
    // A lease the host could not join is not a completed turn.
    assert!(
        matches!(terminal.status(), TerminalStatus::RuntimeFailed(_)),
        "an unjoined registered lease must fail its turn, not complete it: {:?}",
        terminal.status()
    );
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        1,
        "the host retains the lease it could not join"
    );
    // The operation is not over, so no later turn may run beside it.
    let request = TurnRequest::new(
        RuntimeTurnId::new("grok-turn-after-failure").expect("turn"),
        OperationContent::new("private fixture prompt").expect("prompt"),
    );
    let Err(error) = block_on(opened.session.start_turn(request, opened.services.clone())) else {
        panic!("a retained registered lease must refuse every later turn");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.turn_retained"
    );
    blocking.release();
    call.join();
}

#[test]
fn a_failed_registered_cleanup_during_open_retains_the_route_leases() {
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        release: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: Arc::new(Mutex::new(None)),
    });
    let host_id =
        ExecutionHostId::new("fixture.host.grok.registered-open-retained").expect("host");
    let selected = selection(host_id.clone());
    // The provider spawns the courier, calls it, and then never answers
    // session setup, so the open expires with one call still executing.
    let fixture = FixtureHost::new(Scenario::RegisteredOpenBlockedCall);
    fixture.fire_deadline_when(Arc::clone(&blocking.entered));
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services_with_budget(
        &host_id,
        &fixture,
        Arc::clone(&blocking) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        courier_binary(),
        &executable,
        &environment,
        Some(std::time::Duration::from_millis(50)),
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("mediated stdio selection qualifies")
            .with_host(local.clone())
            .with_open_deadline(registered_open_deadline())
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    )) else {
        panic!("an unanswered registered open must fail");
    };
    assert!(
        matches!(error.diagnostic().code(), code if code.starts_with("swallowtail.registered_tool.")
            || code == "swallowtail.grok.acp.registered_tool.open_deadline"),
        "the failure surfaces the retained lease or the expired open: {}",
        error.diagnostic().code()
    );
    assert_eq!(
        local.registered_tool_lease_count(),
        1,
        "the host retains the lease it could not join"
    );
    // Ownership, not just the diagnostic: a credential returned for reuse
    // beside executing work is the defect this guards.
    assert_eq!(fixture.credential_releases.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.resource_releases.load(Ordering::SeqCst), 0);
    blocking.release();
}

#[test]
fn a_generous_caller_deadline_is_capped_at_the_ten_second_open_ceiling() {
    // Contract 063 bounds opening by ten seconds and the parent budget. A five
    // minute caller deadline must not keep a minted lease, its listener, and
    // the route's resources open for five minutes.
    let far = Deadline::at(MonotonicInstant::from_ticks(300_000_000_000));
    let Err(boxed) = try_open_registered_route_with_deadline(
        "fixture.host.grok.registered-open-ceiling",
        Scenario::RegisteredOpenUnanswered,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
        far,
    ) else {
        panic!("an unanswered registered open must fail");
    };
    let (error, local) = *boxed;
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.open_deadline"
    );
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn the_open_bound_uses_the_ten_second_ceiling_not_the_caller_budget() {
    let host_id = ExecutionHostId::new("fixture.host.grok.registered-ceiling-value").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::RegisteredOpenUnanswered);
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services(
        &host_id,
        &fixture,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        courier_binary(),
        &executable,
        &environment,
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("mediated stdio selection qualifies")
            .with_host(local)
            .with_open_deadline(Deadline::at(MonotonicInstant::from_ticks(300_000_000_000)))
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let _ = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    ));
    // The exact bound the route asked the host to wait on is the ceiling, not
    // the five minute caller budget.
    let observed = fixture.observed_deadlines();
    assert_eq!(
        observed.first().map(|deadline| deadline.instant().ticks()),
        Some(REGISTERED_OPEN_DEADLINE_TICKS),
        "open must be bounded by the ten second ceiling: {observed:?}"
    );
}

#[test]
fn a_failed_turn_start_settles_before_it_reports_the_failed_attempt() {
    let calls = Arc::new(AtomicUsize::new(0));
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-turn-start-failed",
        Scenario::RegisteredTurn,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
    );
    opened.courier.handshake();
    // One prompt past the ACP frame ceiling: turn validation admits it, the
    // request fails to encode, and the connection and courier both survive.
    let request = TurnRequest::new(
        registered_turn_id(),
        OperationContent::new("x".repeat(96 * 1024)).expect("oversized prompt"),
    );
    let Err(error) = block_on(opened.session.start_turn(request, opened.services.clone())) else {
        panic!("an unencodable prompt must fail the turn start");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.protocol_failed"
    );
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        0,
        "a failed turn start must settle before it reports the failed attempt"
    );
    let after_failure = opened
        .courier
        .try_call_tool(&registered_tool_id().to_string(), r#"{"path":"post-start-failure"}"#);
    assert!(
        after_failure
            .as_deref()
            .is_none_or(|answer| !answer.contains("from-dispatcher")),
        "a call after a failed turn start must not dispatch: {after_failure:?}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_registered_route(opened);
}

#[test]
fn concurrent_settlement_reports_one_shared_cleanup_truth() {
    // Cancellation begins closing a lease the host cannot join while the turn
    // reaches terminal. The second settler must await the first, not read a
    // not-yet-recorded outcome and publish a clean completion.
    //
    // The overlap is synchronized, not timed: the dispatcher reports the
    // kernel freeze, which is the first step of `close`, and the racing
    // settlement is released only after that freeze is observed. The close
    // then spins for its whole cleanup budget, so the second settler is
    // guaranteed to arrive while the settlement is still in flight.
    let dispatcher = Arc::new(FreezeObservingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        froze: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    });
    let entered = Arc::clone(&dispatcher.entered);
    let froze = Arc::clone(&dispatcher.froze);
    let mut opened = open_registered_route_for(
        "fixture.host.grok.registered-concurrent-settle",
        Scenario::RegisteredTurn,
        Arc::clone(&dispatcher) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Some(std::time::Duration::from_millis(500)),
    );
    opened.courier.handshake();
    let mut turn = start(
        opened.session.as_mut(),
        opened.services.clone(),
        REGISTERED_TURN,
    );
    let call = OutstandingCall::issue(&opened.courier);
    wait_until(
        || entered.load(Ordering::SeqCst),
        "the dispatcher entering its blocking call",
    );
    let fixture = opened.fixture.clone();
    let racer = std::thread::spawn(move || {
        // Only once the freeze is observed is a settlement provably in flight.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
        while !froze.load(Ordering::SeqCst) {
            assert!(
                std::time::Instant::now() < deadline,
                "never observed the kernel freeze that proves settlement started"
            );
            std::thread::sleep(std::time::Duration::from_millis(2));
        }
        fixture.complete_turn();
    });
    block_on(turn.cancellation().request()).expect("turn cancellation requested");
    racer.join().expect("racing thread joins");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("the turn publishes one terminal outcome"),
    );
    // Both settlers observe the same failed cleanup, so the turn is never
    // published as a clean completion beside a lease the host still holds.
    assert!(
        matches!(terminal.status(), TerminalStatus::RuntimeFailed(_)),
        "a concurrently settled failed cleanup must not publish a clean terminal: {:?}",
        terminal.status()
    );
    assert_eq!(
        opened.local.registered_tool_lease_count(),
        1,
        "the host retains the lease it could not join"
    );
    // Closing the session stops the provider child, which releases the reader
    // waiting on a call the dispatcher will never settle.
    close_registered_route(opened);
    call.join();
}

#[test]
fn the_courier_ready_barrier_runs_inside_the_opening_deadline() {
    // The provider answers `session/new` but never starts the declared server,
    // so the ready barrier is pending when the opening deadline expires.
    //
    // Before the barrier was bounded it ran outside the deadline and reported
    // its own `proxy_not_ready` after a further wall-clock wait; open could
    // therefore outlive its ceiling. The deadline must win instead.
    let Err(boxed) = try_open_registered_route_with_deadline(
        "fixture.host.grok.registered-ready-bounded",
        Scenario::RegisteredReadyUnreached,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        None,
        Deadline::at(MonotonicInstant::from_ticks(300_000_000_000)),
    ) else {
        panic!("an unreachable ready barrier must not open a session");
    };
    let (error, local) = *boxed;
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.acp.registered_tool.open_deadline",
        "the ready barrier must be bounded by the opening deadline, not its own wait"
    );
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn a_failed_cleanup_at_the_ready_barrier_retains_the_route_leases() {
    // The provider spawns and connects the courier, starts a call the
    // dispatcher never settles, and answers session setup. The open then
    // expires at the ready barrier, and that settlement fails because the host
    // cannot join the call. Its truth must survive into abandonment: losing it
    // releases the working resource and the credential beside retained work.
    //
    // Expiring during `session/new` instead would exercise a different
    // abandonment path, so the ordering is constructed, not raced. The
    // deadline is delivered only once the ready task's own scope is recorded,
    // and that task is held until the dispatcher observes the kernel freeze —
    // which only happens inside the close that this repair is about. The ready
    // step therefore cannot have completed when the deadline is delivered.
    let dispatcher = Arc::new(FreezeObservingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        froze: Arc::new(std::sync::atomic::AtomicBool::new(false)),
    });
    let host_id =
        ExecutionHostId::new("fixture.host.grok.registered-ready-retained").expect("host");
    let selected = selection(host_id.clone());
    let fixture = FixtureHost::new(Scenario::RegisteredReadyBlockedCall);
    fixture.fire_deadline_when(Arc::clone(&dispatcher.entered));
    fixture.hold_ready_barrier_until(Arc::clone(&dispatcher.froze));
    let executable =
        swallowtail_runtime::ExecutableRef::new("grok.fixture.registered-courier").expect("exe");
    let environment =
        EnvironmentRef::new("grok.fixture.registered-environment").expect("environment");
    let (local, services) = registered_route_services_with_budget(
        &host_id,
        &fixture,
        Arc::clone(&dispatcher) as Arc<dyn swallowtail_runtime::RegisteredToolDispatcher>,
        courier_binary(),
        &executable,
        &environment,
        Some(std::time::Duration::from_millis(50)),
    );
    let preparation = registered_preparation(
        host_id,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        executable,
        environment,
        RegisteredFixtureInput::default(),
    );
    let binding =
        swallowtail_adapter_grok::registered_tool::GrokRegisteredToolBinding::qualify(preparation)
            .expect("mediated stdio selection qualifies")
            .with_host(local.clone())
            .with_open_deadline(registered_open_deadline())
            .with_turn(registered_turn_id());
    let driver = GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient").expect("environment"),
        selected.credential,
    )
    .with_registered_tools(binding);
    let Err(error) = block_on(driver.open_session(
        selected.plan,
        registered_open_request(selected.resource),
        services,
    )) else {
        panic!("an open whose ready barrier expires must fail");
    };
    // The named branch was reached: session setup was answered and the ready
    // task was entered before the open failed.
    assert!(
        fixture
            .writes()
            .iter()
            .any(|write| write["method"] == "session/new"),
        "the provider must have received session setup"
    );
    assert!(
        fixture
            .spawned_scopes()
            .iter()
            .any(|scope| scope.contains("registered-ready")),
        "the open must have reached the ready barrier, not expired before it"
    );
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.registered_tool.teardown_failed",
        "the retained cleanup truth is what surfaces, not the bare expiry"
    );
    assert_eq!(
        local.registered_tool_lease_count(),
        1,
        "the host retains the lease it could not join"
    );
    assert_eq!(
        fixture.credential_releases.load(Ordering::SeqCst),
        0,
        "a failed registered cleanup at the ready barrier must not return the credential"
    );
    assert_eq!(fixture.resource_releases.load(Ordering::SeqCst), 0);
}
