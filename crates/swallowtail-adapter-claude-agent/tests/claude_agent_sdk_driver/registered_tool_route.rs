//! Provider-free route-binding proofs for registered tools on `claude-agent.sdk`.
//!
//! The fake sidecar observes the reserved courier declaration. The real Card
//! 116 proxy and courier carry one mediated call into
//! `RegisteredToolDispatcher::dispatch`. No live Claude, Node SDK, or
//! credential is used.

use crate::host_id;
use crate::sdk_support::{SdkFixtureHost, SdkScenario, cleanup_request, prepared_session};
use futures_executor::block_on;
use std::future::ready;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Poll, Waker};
use swallowtail_adapter_claude_agent::sdk::registered_tool::{
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER, ClaudeAgentSdkRegisteredToolBinding,
};
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkSessionPreparation, ClaudeAgentSdkSessionProfile,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    AdmissionPhase, BoxFuture, EnvironmentRef, ExecutableRef, HostServices,
    InteractiveSessionHandle, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService,
    REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, REGISTERED_TOOL_PROXY_WIRE_TAG,
    RegisteredServerId, RegisteredServerRevision, RegisteredToolAttachment, RegisteredToolBounds,
    RegisteredToolCall, RegisteredToolDispatchContext, RegisteredToolDispatcher,
    RegisteredToolEffectPosture, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolLimits, RegisteredToolLocalName, RegisteredToolNamespace, RegisteredToolOutcome,
    RegisteredToolPayload, RegisteredToolPreparation, RegisteredToolProtocolVersion,
    RegisteredToolProxyRecipe, RegisteredToolResult, RegisteredToolRetryPosture,
    RegisteredToolSchema, RegisteredToolSchemaDialect, RegisteredToolSchemaDigest,
    RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace,
    RegisteredToolSelection, RegisteredToolSnapshot, RegisteredToolSnapshotInput,
    RegisteredToolSource, RegisteredToolSourceId, RegisteredToolTransport,
    RegisteredToolTransportSupport, RuntimeFailure, ScopeId,
};
use swallowtail_testkit::{ScriptedAdmissionPort, fixture_admission};

const NAMESPACE: &str = "desktop";
const RECONCILE: &str = "reconcile";

struct CountingDispatcher {
    calls: Arc<AtomicUsize>,
}

impl RegisteredToolDispatcher for CountingDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let payload = RegisteredToolPayload::new(
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            br#"{"answer":"from-dispatcher"}"#.to_vec(),
            call.binding().effective_bounds().max_result_bytes(),
        )
        .expect("bounded dispatcher result");
        let result = RegisteredToolResult::new(
            payload,
            RegisteredToolSchemaDigest::new("sha256:output").expect("output digest"),
        );
        Box::pin(ready(Ok(RegisteredToolOutcome::completed(&call, result))))
    }
}

struct BlockingDispatcher {
    entered: Arc<AtomicBool>,
    release: Arc<AtomicBool>,
    waker: Arc<Mutex<Option<Waker>>>,
}

impl RegisteredToolDispatcher for BlockingDispatcher {
    fn dispatch(
        &self,
        _call: RegisteredToolCall,
        context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let release = Arc::clone(&self.release);
        let waker = Arc::clone(&self.waker);
        let cancellation = context.cancellation().clone();
        Box::pin(std::future::poll_fn(move |cx| {
            entered.store(true, Ordering::SeqCst);
            if release.load(Ordering::SeqCst) || cancellation.is_cancelled() {
                *waker.lock().expect("blocking dispatcher waker lock") = None;
                return Poll::Ready(Err(RuntimeFailure::new(
                    swallowtail_core::SafeDiagnostic::new(
                        "fixture.blocking_dispatcher.released",
                        "fixture blocking dispatcher released",
                    ),
                )));
            }
            let mut slot = waker.lock().expect("blocking dispatcher waker lock");
            let start_ticker = slot.is_none();
            *slot = Some(cx.waker().clone());
            drop(slot);
            if start_ticker {
                let waker = Arc::clone(&waker);
                let release = Arc::clone(&release);
                let cancellation = cancellation.clone();
                std::thread::spawn(move || {
                    while !release.load(Ordering::SeqCst) && !cancellation.is_cancelled() {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                        if let Some(waker) = waker
                            .lock()
                            .expect("blocking dispatcher waker lock")
                            .clone()
                        {
                            waker.wake();
                        }
                    }
                    if let Some(waker) =
                        waker.lock().expect("blocking dispatcher waker lock").take()
                    {
                        waker.wake();
                    }
                });
            }
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

struct SharedProcess(Arc<dyn ProcessHandle>);

impl ProcessHandle for SharedProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.write_stdin(chunk)
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.close_stdin()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        self.0.read_output()
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.request_stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.force_stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.0.wait()
    }
}

struct RouteProcessService {
    sidecar: SdkFixtureHost,
    local: LocalHostServices,
    courier: ExecutableRef,
    captured: Arc<Mutex<Option<Arc<dyn ProcessHandle>>>>,
}

impl ProcessService for RouteProcessService {
    fn start(
        &self,
        scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        if request.executable() == &self.courier {
            let local = Arc::clone(self.local.process_host());
            let captured = Arc::clone(&self.captured);
            Box::pin(async move {
                let handle = ProcessService::start(local.as_ref(), scope, request).await?;
                let shared: Arc<dyn ProcessHandle> = Arc::from(handle);
                *captured.lock().expect("courier capture lock") = Some(Arc::clone(&shared));
                Ok(Box::new(SharedProcess(shared)) as Box<dyn ProcessHandle>)
            })
        } else {
            self.sidecar.start(scope, request)
        }
    }
}

struct CourierClient {
    process: Arc<dyn ProcessHandle>,
}

impl CourierClient {
    fn request(&self, request: &[u8]) -> String {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one JSONL request");
        self.read_stdout_line()
            .expect("SDK reads one JSONL response")
    }

    fn notify(&self, request: &[u8]) {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one notification");
    }

    fn call_tool(&self, name: &str, arguments: &str) -> String {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        self.request(request.as_bytes())
    }

    fn try_call_tool(&self, name: &str, arguments: &str) -> Option<String> {
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        let mut bytes = request.into_bytes();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes))).ok()?;
        self.read_stdout_line()
    }

    fn handshake(&self) {
        let initialized = self.request(
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        );
        assert!(
            initialized.contains("2025-11-25"),
            "courier handshake: {initialized}"
        );
        self.notify(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    }

    fn read_stdout_line(&self) -> Option<String> {
        let mut output = Vec::new();
        loop {
            let chunk = block_on(self.process.read_output()).ok()??;
            if chunk.stream() == ProcessOutputStream::Stdout {
                output.extend_from_slice(chunk.bytes());
                if output.contains(&b'\n') {
                    return Some(String::from_utf8(output).expect("JSON response is UTF-8"));
                }
            }
        }
    }
}

fn courier_binary() -> &'static Path {
    static PATH: OnceLock<PathBuf> = OnceLock::new();
    PATH.get_or_init(|| {
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("crate dir")
            .parent()
            .expect("workspace")
            .to_path_buf();
        // Nested cargo must not share the outer `cargo test` target lock.
        let nested_target = workspace.join("target").join("card125-courier");
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

fn tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new(NAMESPACE).expect("namespace"),
        RegisteredToolLocalName::new(RECONCILE).expect("local name"),
    )
}

fn schema(digest: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.registered-tools.schema").expect("namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new("1").expect("revision"),
        RegisteredToolSchemaDigest::new(digest).expect("digest"),
        RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document"),
    )
}

fn snapshot(
    host: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("desktop.registered-tools").expect("server"),
        revision: RegisteredServerRevision::new("2026-09-07.1").expect("revision"),
        execution_host_id: host.clone(),
        declarations: vec![
            swallowtail_runtime::RegisteredToolDeclaration::new(
                tool_id(),
                RegisteredToolExecutionKind::Mcp,
                schema("sha256:input"),
                schema("sha256:output"),
                RegisteredToolEffectPosture::Mutating,
                RegisteredToolRetryPosture::ConsumerRetryable,
                RegisteredToolBounds::ceiling(),
            )
            .expect("declaration"),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::PrivateLoopbackHttp,
                [
                    RegisteredToolProtocolVersion::new(
                        REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
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
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("desktop.registration.1").expect("source"),
            swallowtail_runtime::MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot")
}

fn preparation_for(
    host: ExecutionHostId,
    admission: swallowtail_runtime::ConsumerAdmissionBinding,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolPreparation {
    let snapshot = Arc::new(snapshot(&host, executable.clone(), environment.clone()));
    let recipe =
        RegisteredToolProxyRecipe::new(executable, environment, REGISTERED_TOOL_PROXY_WIRE_TAG)
            .expect("recipe");
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [tool_id()],
        RegisteredToolTransport::PrivateLoopbackHttp,
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("protocol"),
    )
    .expect("selection")
    .with_attachment(RegisteredToolAttachment::MediatedStdioProxy)
    .with_proxy_recipe(recipe);
    RegisteredToolPreparation::new(
        snapshot,
        selection,
        admission,
        RegisteredToolLimits::ceiling(),
    )
}

struct OpenedRoute {
    session: swallowtail_adapter_claude_agent::sdk::ClaudeAgentSdkSessionHandle,
    fixture: SdkFixtureHost,
    local: LocalHostServices,
    services: HostServices,
    courier: CourierClient,
}

fn open_route(
    host: ExecutionHostId,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    admission: Arc<ScriptedAdmissionPort>,
) -> OpenedRoute {
    let fixture = SdkFixtureHost::new(SdkScenario::McpConnected);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let captured: Arc<Mutex<Option<Arc<dyn ProcessHandle>>>> = Arc::new(Mutex::new(None));
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable.clone(), courier_binary())
        .approve_environment(environment.clone(), [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_clock(Arc::new(fixture.clone()))
        .build_services(host.clone());
    let process = RouteProcessService {
        sidecar: fixture.clone(),
        local: local.clone(),
        courier: executable.clone(),
        captured: Arc::clone(&captured),
    };
    let services = local
        .services()
        .clone()
        .with_process(Arc::new(process))
        .with_credential(Arc::new(fixture.clone()))
        .with_working_resource(Arc::new(fixture.clone()))
        .with_time(Arc::new(fixture.clone()));
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::clone(&admission)),
        executable,
        environment,
    );
    let binding = ClaudeAgentSdkSessionProfile::read_only()
        .with_registered_tools(preparation)
        .expect("profile qualifies the mediated stdio preparation")
        .with_host(local.clone());
    let prepared = prepare_claude_agent_sdk_session(
        ClaudeAgentSdkSessionPreparation::new(
            ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("instance"),
            swallowtail_core::InstanceRevision::new("fixture-revision").expect("revision"),
            host,
            swallowtail_core::InstanceTargetRef::new("claude-agent-sdk.fixture.launch-recipe")
                .expect("target"),
            swallowtail_runtime::EnvironmentRef::new("claude-agent-sdk.fixture.environment")
                .expect("environment"),
            swallowtail_core::CredentialRef::new("claude-agent-sdk.fixture.delegated-subscription")
                .expect("credential"),
            swallowtail_core::AccessProfileId::new("claude-agent-sdk.fixture.subscription")
                .expect("access"),
            swallowtail_core::ModelRouteId::new("claude-agent-sdk.fixture.route").expect("route"),
            swallowtail_core::ModelRouteRevision::new("fixture-route-revision").expect("revision"),
            swallowtail_core::ModelId::new("claude-sonnet-5").expect("model"),
            swallowtail_runtime::WorkingResourceRef::new("claude-agent-sdk.fixture.workspace")
                .expect("workspace"),
            swallowtail_runtime::RequestId::new("request-1").expect("request"),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                10_000,
            )),
        )
        .with_registered_tool_binding(binding),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("registered-tool preparation succeeds");
    let session = block_on(prepared.open_route_session(services.clone())).expect("session opens");
    let courier = CourierClient {
        process: captured
            .lock()
            .expect("courier capture lock")
            .clone()
            .expect("driver spawned the courier through the host process port"),
    };
    OpenedRoute {
        session,
        fixture,
        local,
        services,
        courier,
    }
}

fn close_route(opened: OpenedRoute) {
    let _ = block_on(Box::new(opened.session).close(cleanup_request(), opened.services));
}

#[test]
fn the_default_open_without_registered_tools_omits_mcp_servers() {
    let host = host_id("claude-agent-sdk.fixture.registered-omit");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    assert!(prepared.registered_tools().is_none());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("default session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("default open is on the wire");
    assert!(
        open["params"].get("mcpServers").is_none(),
        "omission stays byte-identical: {open}"
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn open_without_a_host_composition_fails_typed() {
    let host = host_id("claude-agent-sdk.fixture.registered-host-missing");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let preparation = preparation_for(
        host.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        executable,
        environment,
    );
    let binding = ClaudeAgentSdkRegisteredToolBinding::qualify(preparation)
        .expect("qualify does not require the host composition");
    let prepared = prepare_claude_agent_sdk_session(
        crate::sdk_support::preparation(host.clone()).with_registered_tool_binding(binding),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("prepared without host");
    let Err(error) = block_on(prepared.open_session(fixture.services(host))) else {
        panic!("open requires the local host composition");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.registered_tool.host_missing"
    );
}

#[test]
fn open_declares_the_reserved_courier_and_round_trips_one_mediated_call() {
    let host = host_id("claude-agent-sdk.fixture.registered-round-trip");
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        Arc::clone(&admission),
    );
    let open = opened
        .fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("registered open is on the wire");
    let servers = open["params"]["mcpServers"]
        .as_array()
        .expect("courier is declared");
    assert_eq!(servers.len(), 1);
    assert_eq!(servers[0]["name"], CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER);
    assert_eq!(servers[0]["args"][0], REGISTERED_TOOL_PROXY_WIRE_TAG);
    assert!(
        open["params"]["tools"]
            .as_array()
            .expect("tools")
            .iter()
            .any(|tool| tool.as_str()
                == Some("mcp__swallowtail-registered-tools__desktop_reconcile")),
        "admitted tools include the carrier spelling: {}",
        open["params"]["tools"]
    );
    assert_eq!(
        opened.session.mcp_server_status()[0].name(),
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER
    );
    opened.courier.handshake();
    let called = opened
        .courier
        .call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        called.contains("from-dispatcher"),
        "result correlates back: {called}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(
        admission
            .observed_phases()
            .contains(&AdmissionPhase::BeforeDispatch),
        "dispatch is kernel-admitted: {:?}",
        admission.observed_phases()
    );
    close_route(opened);
}

#[test]
fn a_revoked_before_dispatch_never_reaches_the_dispatcher() {
    let host = host_id("claude-agent-sdk.fixture.registered-deny");
    let calls = Arc::new(AtomicUsize::new(0));
    let admission = Arc::new(ScriptedAdmissionPort::current());
    admission.revoke_from(AdmissionPhase::BeforeDispatch);
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::clone(&calls),
        }),
        admission,
    );
    opened.courier.handshake();
    let denied = opened
        .courier
        .call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#);
    assert!(
        !denied.contains("from-dispatcher"),
        "deny does not deliver a result: {denied}"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    close_route(opened);
}

#[test]
fn cancellation_during_a_registered_call_joins_the_lease() {
    let host = host_id("claude-agent-sdk.fixture.registered-cancel");
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(AtomicBool::new(false)),
        release: Arc::new(AtomicBool::new(false)),
        waker: Arc::new(Mutex::new(None)),
    });
    let opened = open_route(
        host,
        Arc::clone(&blocking) as Arc<dyn RegisteredToolDispatcher>,
        Arc::new(ScriptedAdmissionPort::current()),
    );
    opened.courier.handshake();
    let courier = Arc::clone(&opened.courier.process);
    let client = CourierClient {
        process: Arc::clone(&courier),
    };
    let call = std::thread::spawn(move || {
        client.try_call_tool(&tool_id().to_string(), r#"{"path":"workspace/file"}"#)
    });
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while !blocking.entered.load(Ordering::SeqCst) {
        assert!(
            std::time::Instant::now() < deadline,
            "dispatcher never entered"
        );
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    let local = opened.local.clone();
    close_route(opened);
    blocking.release();
    let _ = call.join();
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn close_joins_the_registered_listener() {
    let host = host_id("claude-agent-sdk.fixture.registered-close");
    let opened = open_route(
        host,
        Arc::new(CountingDispatcher {
            calls: Arc::new(AtomicUsize::new(0)),
        }),
        Arc::new(ScriptedAdmissionPort::current()),
    );
    assert_eq!(opened.local.registered_tool_lease_count(), 1);
    let local = opened.local.clone();
    close_route(opened);
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
}
