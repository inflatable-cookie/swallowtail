use futures_executor::block_on;
use std::future::ready;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
use swallowtail_host_local::wire::RegisteredToolProxyRendezvousDocument;
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits, RegisteredToolProxyLaunch};
use swallowtail_runtime::{
    BoxFuture, Deadline, EnvironmentRef, ExecutableRef, ProcessHandle, ProcessInputChunk,
    ProcessOutputStream, ProcessService, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    RegisteredServerId, RegisteredServerRevision, RegisteredToolAttachment, RegisteredToolBounds,
    RegisteredToolCall, RegisteredToolCallId, RegisteredToolCallRequest, RegisteredToolDeclaration,
    RegisteredToolDispatchContext, RegisteredToolDispatcher, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolPreparation, RegisteredToolProtocolVersion, RegisteredToolProxyRecipe,
    RegisteredToolResult, RegisteredToolRetryPosture, RegisteredToolSchemaDialect,
    RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
    RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RuntimeFailure, RuntimeTurnId,
    ScopeId,
};

const COURIER: Option<&str> = option_env!("CARGO_BIN_EXE_swallowtail-registered-tool-courier");

struct Dispatcher {
    calls: Arc<AtomicUsize>,
}

impl RegisteredToolDispatcher for Dispatcher {
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

#[test]
fn real_courier_and_sdk_shaped_process_reach_the_kernel_dispatcher() {
    let courier = COURIER.expect("feature-gated courier binary is built");
    let host_id = ExecutionHostId::new("fixture.host.mediated-stdio").expect("host id");
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let snapshot = Arc::new(snapshot(&host_id, executable.clone(), environment.clone()));
    let recipe = RegisteredToolProxyRecipe::new(
        executable.clone(),
        environment.clone(),
        swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG,
    )
    .expect("fixed recipe wire");
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
    let calls = Arc::new(AtomicUsize::new(0));
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable, courier)
        .approve_environment(environment, [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(Dispatcher {
            calls: Arc::clone(&calls),
        }))
        .build_services(host_id.clone());
    let admission = swallowtail_testkit::fixture_admission(Arc::new(
        swallowtail_testkit::ScriptedAdmissionPort::current(),
    ));
    let preparation = RegisteredToolPreparation::new(
        Arc::clone(&snapshot),
        selection,
        admission,
        swallowtail_runtime::RegisteredToolLimits::ceiling(),
    );
    let prepared = preparation
        .prepare(
            local.services(),
            ConfiguredInstanceId::new("fixture.instance").expect("instance"),
            ScopeId::new("fixture.scope").expect("scope"),
            RuntimeTurnId::new("fixture.turn").expect("turn"),
            Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                1_000_000_000_000,
            )),
        )
        .expect("mediated selection is ready");
    let lease = block_on(prepared.open()).expect("open binds the real listener and kernel");
    let pre_ready_payload = RegisteredToolPayload::new(
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        br#"{}"#.to_vec(),
        lease.selection().effective_bounds().max_argument_bytes(),
    )
    .expect("bounded pre-ready arguments");
    let pre_ready = block_on(lease.call(RegisteredToolCallRequest::new(
        RegisteredToolCallId::new("pre-ready").expect("call id"),
        tool_id(),
        pre_ready_payload,
        lease.deadline(),
    )));
    assert!(pre_ready.is_err(), "calls must wait for courier readiness");
    let mut launch = local
        .registered_tool_proxy_launch(&lease)
        .expect("host materializes one proxy launch");
    assert!(local.registered_tool_proxy_launch(&lease).is_err());
    let rendezvous_path = launch.rendezvous_path().to_path_buf();
    let rendezvous = RegisteredToolProxyRendezvousDocument::decode(
        &std::fs::read(&rendezvous_path).expect("rendezvous is materialized before spawn"),
    )
    .expect("rendezvous uses the frozen wire shape");
    assert!(rendezvous_path.exists());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        assert_eq!(
            std::fs::metadata(&rendezvous_path)
                .expect("rendezvous metadata")
                .permissions()
                .mode()
                & 0o777,
            0o600
        );
    }
    assert_eq!(launch.process_request().arguments().count(), 2);
    assert!(
        launch
            .process_request()
            .arguments()
            .all(|argument| !argument.contains("bearer") && !argument.contains("generation"))
    );
    let launch_debug = format!("{:?}", launch.process_request());
    assert!(!launch_debug.contains(&rendezvous.endpoint));
    assert!(!launch_debug.contains(&rendezvous.bearer));
    let rendezvous_debug = format!("{rendezvous:?}");
    assert!(!rendezvous_debug.contains(&rendezvous.endpoint));
    assert!(!rendezvous_debug.contains(&rendezvous.bearer));
    let mut sdk = FakeSdk::spawn(&local, &launch, calls);
    launch
        .wait_until_ready()
        .expect("courier authenticates and completes the ready barrier");
    assert!(!rendezvous_path.exists());
    let initialized = sdk.request(
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert!(initialized.contains("2025-11-25"));
    sdk.notify(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);
    let listed = sdk.request(br#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
    assert!(listed.contains("swallowtail.conformance/reconcile"));
    assert!(
        sdk.call_tool("swallowtail.conformance/denied", "{}")
            .is_none()
    );
    assert_eq!(sdk.dispatch_count(), 0);
    let called = sdk
        .call_tool(
            "swallowtail.conformance/reconcile",
            r#"{"path":"workspace/file"}"#,
        )
        .expect("fake SDK canUseTool allows the declared tool");
    assert!(called.contains("from-dispatcher"));
    assert!(called.contains("\"id\":3"));
    assert_eq!(sdk.dispatch_count(), 1);
    assert!(!called.contains("fixture-local-tool-response"));
    sdk.close();
    block_on(
        local
            .services()
            .registered_tool_bridge()
            .expect("registered bridge")
            .close(
                lease,
                swallowtail_runtime::RegisteredToolCleanupCause::Completion,
            ),
    )
    .expect("close joins listener and kernel");
}

struct FakeSdk {
    process: Box<dyn ProcessHandle>,
    calls: Arc<AtomicUsize>,
}

impl FakeSdk {
    fn spawn(
        local: &swallowtail_host_local::LocalHostServices,
        launch: &RegisteredToolProxyLaunch,
        calls: Arc<AtomicUsize>,
    ) -> Self {
        let process = block_on(local.process_host().start(
            ScopeId::new("fixture.scope").expect("scope"),
            launch.process_request().clone(),
        ));
        let process = process.expect("courier process starts through the host launcher");
        Self { process, calls }
    }

    fn request(&mut self, request: &[u8]) -> String {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one JSONL request");
        self.read_stdout_line()
    }

    fn can_use_tool(&self, name: &str) -> bool {
        name == "swallowtail.conformance/reconcile"
    }

    fn call_tool(&mut self, name: &str, arguments: &str) -> Option<String> {
        if !self.can_use_tool(name) {
            return None;
        }
        let request = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{name:?},\"arguments\":{arguments}}}}}"
        );
        Some(self.request(request.as_bytes()))
    }

    fn dispatch_count(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }

    fn notify(&mut self, request: &[u8]) {
        let mut bytes = request.to_vec();
        bytes.push(b'\n');
        block_on(self.process.write_stdin(ProcessInputChunk::new(bytes)))
            .expect("SDK writes one notification");
    }

    fn read_stdout_line(&mut self) -> String {
        let mut output = Vec::new();
        loop {
            let chunk = block_on(self.process.read_output())
                .expect("SDK reads courier output")
                .expect("courier remains alive");
            if chunk.stream() == ProcessOutputStream::Stdout {
                output.extend_from_slice(chunk.bytes());
                if output.contains(&b'\n') {
                    return String::from_utf8(output).expect("JSON response is UTF-8");
                }
            }
        }
    }

    fn close(&mut self) {
        block_on(self.process.close_stdin()).expect("SDK closes courier input");
        let _ = block_on(self.process.wait());
    }
}

fn tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("swallowtail.conformance").expect("namespace"),
        RegisteredToolLocalName::new("reconcile").expect("local name"),
    )
}

fn snapshot(
    host_id: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("swallowtail-registered-tools").expect("server"),
        revision: RegisteredServerRevision::new("fixture-1").expect("revision"),
        execution_host_id: host_id.clone(),
        declarations: vec![
            RegisteredToolDeclaration::new(
                tool_id(),
                RegisteredToolExecutionKind::Mcp,
                schema("input", "sha256:input"),
                schema("output", "sha256:output"),
                RegisteredToolEffectPosture::ReadOnly,
                RegisteredToolRetryPosture::NeverRetry,
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
            RegisteredToolSourceId::new("fixture.source").expect("source"),
            swallowtail_runtime::MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot")
}

fn schema(label: &str, digest: &str) -> swallowtail_runtime::RegisteredToolSchema {
    swallowtail_runtime::RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new(format!("fixture.{label}")).expect("schema namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("schema media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("schema dialect"),
        RegisteredServerRevision::new("fixture-1").expect("schema revision"),
        RegisteredToolSchemaDigest::new(digest).expect("schema digest"),
        RegisteredToolSchemaDocument::new(r#"{"type":"object"}"#).expect("schema document"),
    )
}
