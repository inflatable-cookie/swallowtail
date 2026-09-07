use futures_executor::block_on;
use std::future::ready;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Poll, Waker};
use std::time::Duration;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId, WatcherCleanupCause};
use swallowtail_host_local::wire::{
    REGISTERED_TOOL_PROXY_HTTP_PATH, RegisteredToolProxyRendezvousDocument,
};
use swallowtail_host_local::{
    LocalHostServices, LocalProcessHost, LocalProcessLimits, RegisteredToolProxyLaunch,
};
use swallowtail_runtime::{
    BoxFuture, Deadline, EnvironmentRef, ExecutableRef, ProcessHandle, ProcessInputChunk,
    ProcessOutputStream, ProcessService, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    RegisteredServerId, RegisteredServerRevision, RegisteredToolAttachment, RegisteredToolBounds,
    RegisteredToolBridgeLease, RegisteredToolCall, RegisteredToolCallId, RegisteredToolCallRequest,
    RegisteredToolDeclaration, RegisteredToolDispatchContext, RegisteredToolDispatcher,
    RegisteredToolEffectPosture, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolLocalName, RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolPreparation, RegisteredToolProtocolVersion, RegisteredToolProxyRecipe,
    RegisteredToolResult, RegisteredToolRetryPosture, RegisteredToolSchemaDialect,
    RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
    RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RuntimeFailure, RuntimeTurnId,
    ScopeId, WatcherBridgeOpenRequest,
};

const COURIER: Option<&str> = option_env!("CARGO_BIN_EXE_swallowtail-registered-tool-courier");

struct Dispatcher {
    calls: Arc<AtomicUsize>,
}

struct BlockingDispatcher {
    entered: Arc<std::sync::atomic::AtomicBool>,
    release: Arc<std::sync::atomic::AtomicBool>,
    waker: Arc<std::sync::Mutex<Option<Waker>>>,
}

impl RegisteredToolDispatcher for BlockingDispatcher {
    fn dispatch(
        &self,
        _call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        let entered = Arc::clone(&self.entered);
        let release = Arc::clone(&self.release);
        let waker = Arc::clone(&self.waker);
        Box::pin(std::future::poll_fn(move |context| {
            entered.store(true, Ordering::SeqCst);
            if release.load(Ordering::SeqCst) {
                Poll::Ready(Err(RuntimeFailure::new(
                    swallowtail_core::SafeDiagnostic::new(
                        "fixture.blocking_dispatcher.released",
                        "fixture blocking dispatcher released",
                    ),
                )))
            } else {
                *waker.lock().expect("blocking dispatcher waker lock") =
                    Some(context.waker().clone());
                Poll::Pending
            }
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

#[test]
fn mounted_mediated_attachment_rejects_wrong_kind_snapshot_before_listener() {
    let host_id = ExecutionHostId::new("fixture.host.mediated-stdio-wrong-kind").expect("host id");
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let snapshot = Arc::new(snapshot_with_kind(
        &host_id,
        executable.clone(),
        environment.clone(),
        RegisteredToolExecutionKind::NativeClient,
    ));
    let recipe = RegisteredToolProxyRecipe::new(
        executable.clone(),
        environment.clone(),
        swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG,
    )
    .expect("recipe");
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [tool_id()],
        RegisteredToolTransport::PrivateLoopbackHttp,
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("protocol"),
    )
    .expect("selection construction remains provider-neutral")
    .with_attachment(RegisteredToolAttachment::MediatedStdioProxy)
    .with_proxy_recipe(recipe);
    let calls = Arc::new(AtomicUsize::new(0));
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable, COURIER.expect("courier"))
        .approve_environment(environment, [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(Dispatcher {
            calls: Arc::clone(&calls),
        }))
        .build_services(host_id.clone());
    let preparation = RegisteredToolPreparation::new(
        snapshot,
        selection,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::RegisteredToolLimits::ceiling(),
    );
    let error = preparation
        .prepare(
            local.services(),
            ConfiguredInstanceId::new("fixture.instance.wrong-kind").expect("instance"),
            ScopeId::new("fixture.scope.wrong-kind").expect("scope"),
            RuntimeTurnId::new("fixture.turn.wrong-kind").expect("turn"),
            Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                1_000_000_000_000,
            )),
        )
        .expect_err("mediated stdio only accepts MCP declarations");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.registered_tool.unsupported_tool"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(local.registered_tool_lease_count(), 0);
    assert_eq!(local.operation_bridge_listener_count(), 0);
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
fn mounted_proxy_filters_native_declaration_before_host_work() {
    let courier = COURIER.expect("feature-gated courier binary is built");
    let host_id = ExecutionHostId::new("fixture.host.mediated-stdio-mixed-kinds").expect("host id");
    let executable = ExecutableRef::new("fixture.registered-tool.courier").expect("executable");
    let environment = EnvironmentRef::new("fixture.registered-tool.environment").expect("env");
    let snapshot = Arc::new(snapshot_with_declarations(
        &host_id,
        executable.clone(),
        environment.clone(),
        [
            (tool_id(), RegisteredToolExecutionKind::Mcp),
            (native_tool_id(), RegisteredToolExecutionKind::NativeClient),
        ],
    ));
    let recipe = RegisteredToolProxyRecipe::new(
        executable.clone(),
        environment.clone(),
        swallowtail_runtime::REGISTERED_TOOL_PROXY_WIRE_TAG,
    )
    .expect("recipe");
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [tool_id(), native_tool_id()],
        RegisteredToolTransport::PrivateLoopbackHttp,
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("protocol"),
    )
    .expect("mixed selection is constructed")
    .with_attachment(RegisteredToolAttachment::MediatedStdioProxy)
    .with_proxy_recipe(recipe);
    let calls = Arc::new(AtomicUsize::new(0));
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable, courier)
        .approve_environment(environment, [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(Arc::new(Dispatcher {
            calls: Arc::clone(&calls),
        }))
        .build_services(host_id);
    let preparation = RegisteredToolPreparation::new(
        Arc::clone(&snapshot),
        selection,
        swallowtail_testkit::fixture_admission(Arc::new(
            swallowtail_testkit::ScriptedAdmissionPort::current(),
        )),
        swallowtail_runtime::RegisteredToolLimits::ceiling(),
    );
    let prepared = preparation
        .prepare(
            local.services(),
            ConfiguredInstanceId::new("fixture.instance.mixed-kinds").expect("instance"),
            ScopeId::new("fixture.scope.mixed-kinds").expect("scope"),
            RuntimeTurnId::new("fixture.turn.mixed-kinds").expect("turn"),
            local.deadline_after(Duration::from_secs(10)),
        )
        .expect("mixed selection is ready through the mounted gate");
    let lease = block_on(prepared.open()).expect("open mounts the real listener");
    let direct_native_payload = RegisteredToolPayload::new(
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        br#"{}"#.to_vec(),
        lease.selection().effective_bounds().max_argument_bytes(),
    )
    .expect("bounded direct native arguments");
    let direct_native = block_on(lease.call(RegisteredToolCallRequest::new(
        RegisteredToolCallId::new("direct-native").expect("call id"),
        native_tool_id(),
        direct_native_payload,
        lease.deadline(),
    )))
    .expect_err("mediated lease.call rejects NativeClient before dispatch");
    assert_eq!(
        direct_native.diagnostic().code(),
        "swallowtail.registered_tool.unsupported_tool"
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let mut launch = local
        .registered_tool_proxy_launch(&lease)
        .expect("host materializes the real courier launch");
    let mut sdk = FakeSdk::spawn(&local, &launch, Arc::clone(&calls));
    launch
        .wait_until_ready()
        .expect("real courier reaches the ready barrier");
    let initialized = sdk.request(
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert!(initialized.contains("2025-11-25"));
    sdk.notify(br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#);

    let listed = sdk.request(br#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#);
    assert!(listed.contains("swallowtail.conformance/reconcile"));
    assert!(!listed.contains("swallowtail.conformance/native-client"));
    assert_eq!(sdk.dispatch_count(), 0);

    let native_call = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{{\"name\":{:?},\"arguments\":{{}}}}}}",
        native_tool_id().to_string()
    );
    let rejected = sdk.request(native_call.as_bytes());
    assert!(rejected.contains("Unknown registered tool"));
    assert_eq!(sdk.dispatch_count(), 0);

    let called = sdk
        .call_tool(
            "swallowtail.conformance/reconcile",
            r#"{"path":"workspace/file"}"#,
        )
        .expect("MCP declaration remains callable");
    assert!(called.contains("from-dispatcher"));
    assert_eq!(sdk.dispatch_count(), 1);
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

/// The raw mounted fixture drives the same listener without a fixture-local
/// response. It covers adverse wire cases that a provider SDK would hide.
struct RawMountedProxy {
    local: LocalHostServices,
    lease: RegisteredToolBridgeLease,
    launch: RegisteredToolProxyLaunch,
    document: RegisteredToolProxyRendezvousDocument,
    calls: Arc<AtomicUsize>,
}

fn mount_raw_proxy() -> RawMountedProxy {
    mount_raw_proxy_with_admission(Arc::new(
        swallowtail_testkit::ScriptedAdmissionPort::current(),
    ))
}

fn mount_raw_proxy_with_admission(
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
) -> RawMountedProxy {
    let calls = Arc::new(AtomicUsize::new(0));
    mount_raw_proxy_with_dispatcher(
        admission,
        Arc::new(Dispatcher {
            calls: Arc::clone(&calls),
        }),
        Duration::from_secs(10),
        Arc::clone(&calls),
    )
}

fn mount_raw_proxy_with_dispatcher(
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    cleanup_budget: Duration,
    calls: Arc<AtomicUsize>,
) -> RawMountedProxy {
    mount_raw_proxy_with_deadline(
        admission,
        dispatcher,
        cleanup_budget,
        calls,
        Duration::from_secs(10),
    )
}

fn mount_raw_proxy_with_deadline(
    admission: Arc<swallowtail_testkit::ScriptedAdmissionPort>,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    cleanup_budget: Duration,
    calls: Arc<AtomicUsize>,
    open_duration: Duration,
) -> RawMountedProxy {
    let courier = COURIER.expect("feature-gated courier binary is built");
    let host_id = ExecutionHostId::new("fixture.host.mediated-stdio-raw").expect("host id");
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
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_executable(executable, courier)
        .approve_environment(environment, [("PATH".into(), "/usr/bin".into())])
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_cleanup_budget(cleanup_budget)
        .build_services(host_id.clone());
    let admission_binding = swallowtail_testkit::fixture_admission(Arc::clone(&admission));
    let preparation = RegisteredToolPreparation::new(
        Arc::clone(&snapshot),
        selection,
        admission_binding,
        swallowtail_runtime::RegisteredToolLimits::ceiling(),
    );
    let deadline = local.deadline_after(open_duration);
    let lease = block_on(
        preparation
            .prepare(
                local.services(),
                ConfiguredInstanceId::new("fixture.instance.raw").expect("instance"),
                ScopeId::new("fixture.scope.raw").expect("scope"),
                RuntimeTurnId::new("fixture.turn.raw").expect("turn"),
                deadline,
            )
            .expect("prepared")
            .open(),
    )
    .expect("open binds the mounted listener");
    let launch = local
        .registered_tool_proxy_launch(&lease)
        .expect("host materializes the rendezvous");
    let document = RegisteredToolProxyRendezvousDocument::decode(
        &std::fs::read(launch.rendezvous_path()).expect("rendezvous body"),
    )
    .expect("rendezvous wire");
    RawMountedProxy {
        local,
        lease,
        launch,
        document,
        calls,
    }
}

impl RawMountedProxy {
    fn close(self) {
        block_on(
            self.local
                .services()
                .registered_tool_bridge()
                .expect("registered bridge")
                .close(
                    self.lease,
                    swallowtail_runtime::RegisteredToolCleanupCause::Completion,
                ),
        )
        .expect("mounted proxy closes cleanly");
    }
}

#[test]
fn mounted_proxy_rejects_foreign_auth_and_tool_identity() {
    let fixture = mount_raw_proxy();
    assert!(fixture.launch.rendezvous_path().exists());
    let mut stream = connect_raw(&fixture.document);
    let not_ready = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    );
    assert_eq!(not_ready.0, 200);
    assert!(String::from_utf8_lossy(&not_ready.1).contains("-32006"));
    let initialized = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(initialized.0, 200);
    assert!(String::from_utf8_lossy(&initialized.1).contains("2025-11-25"));
    assert!(!fixture.launch.rendezvous_path().exists());

    let foreign = raw_post(
        &mut stream,
        "foreign-bearer",
        br#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    );
    assert_eq!(foreign.0, 401);
    let unknown = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"foreign.tool","arguments":{}}}"#,
    );
    assert_eq!(unknown.0, 200);
    assert!(String::from_utf8_lossy(&unknown.1).contains("-32602"));
    let first = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{"path":"workspace/file"}}}"#,
    );
    assert_eq!(first.0, 200);
    assert!(String::from_utf8_lossy(&first.1).contains("from-dispatcher"));
    let duplicate = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{"path":"workspace/file"}}}"#,
    );
    assert_eq!(duplicate.0, 200);
    assert!(String::from_utf8_lossy(&duplicate.1).contains("-32000"));
    assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
    drop(stream);
    fixture.close();
}

#[test]
fn mounted_proxy_denies_through_kernel_admission_before_dispatch() {
    let admission = Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let fixture = mount_raw_proxy_with_admission(Arc::clone(&admission));
    let mut stream = connect_raw(&fixture.document);
    let initialized = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(initialized.0, 200);
    admission.revoke_now();
    let denied = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{}}}"#,
    );
    assert_eq!(denied.0, 200);
    assert!(String::from_utf8_lossy(&denied.1).contains("-32000"));
    assert_eq!(fixture.calls.load(Ordering::SeqCst), 0);
    drop(stream);
    fixture.close();
}

#[test]
fn mounted_proxy_revocation_between_dispatch_and_delivery_is_reported_without_replay() {
    let admission = Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    admission.revoke_after_validations(1);
    let fixture = mount_raw_proxy_with_admission(Arc::clone(&admission));
    let mut stream = connect_raw(&fixture.document);
    assert_eq!(
        raw_post(
            &mut stream,
            &fixture.document.bearer,
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        )
        .0,
        200
    );
    let result = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{}}}"#,
    );
    assert_eq!(result.0, 200);
    assert!(String::from_utf8_lossy(&result.1).contains("isError"));
    assert_eq!(fixture.calls.load(Ordering::SeqCst), 1);
    drop(stream);
    fixture.close();
}

#[test]
fn mounted_proxy_expired_deadline_rejects_the_callable_frame_before_dispatch() {
    let calls = Arc::new(AtomicUsize::new(0));
    let fixture = mount_raw_proxy_with_deadline(
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Arc::new(Dispatcher {
            calls: Arc::clone(&calls),
        }),
        Duration::from_secs(10),
        Arc::clone(&calls),
        Duration::ZERO,
    );
    let mut stream = connect_raw(&fixture.document);
    assert_eq!(
        raw_post(
            &mut stream,
            &fixture.document.bearer,
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        )
        .0,
        200
    );
    let result = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{}}}"#,
    );
    assert_eq!(result.0, 200);
    assert!(String::from_utf8_lossy(&result.1).contains("-32000"));
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    drop(stream);
    fixture.close();
}

#[test]
fn mounted_proxy_cancellation_closes_the_shared_listener_before_late_transport() {
    let fixture = mount_raw_proxy();
    let document = fixture.document.clone();
    let local = fixture.local.clone();
    let cleanup = block_on(
        local
            .services()
            .registered_tool_bridge()
            .expect("registered bridge")
            .close(
                fixture.lease,
                swallowtail_runtime::RegisteredToolCleanupCause::Cancellation,
            ),
    )
    .expect("cancellation joins the operation listener");
    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::Clean);
    assert_eq!(local.operation_bridge_listener_count(), 0);
    let close_deadline = std::time::Instant::now() + Duration::from_secs(1);
    let mut listener_closed = false;
    while std::time::Instant::now() < close_deadline {
        if TcpStream::connect_timeout(&socket_addr(&document), Duration::from_millis(50)).is_err() {
            listener_closed = true;
            break;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(listener_closed);
    drop(fixture.launch);
}

#[test]
fn mounted_proxy_rejects_stale_foreign_and_late_generations_without_reauth() {
    let fixture = mount_raw_proxy();
    assert!((1..=10_000).contains(&fixture.document.connect_timeout_ms));
    let mut stream = connect_raw(&fixture.document);
    let stale = raw_post_with_identity(
        &mut stream,
        &fixture.document.bearer,
        99,
        1,
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(stale.0, 409);
    let foreign = raw_post_with_identity(
        &mut stream,
        &fixture.document.bearer,
        1,
        99,
        br#"{"jsonrpc":"2.0","id":2,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(foreign.0, 409);
    let initialized = raw_post(
        &mut stream,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":3,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(initialized.0, 200);
    drop(stream);

    let mut second_connection = connect_raw(&fixture.document);
    let late = raw_post(
        &mut second_connection,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":4,"method":"tools/list"}"#,
    );
    assert_eq!(
        late.0, 409,
        "transport loss does not re-authenticate a courier"
    );
    drop(second_connection);
    let document = fixture.document.clone();
    fixture.close();
    assert!(
        TcpStream::connect_timeout(&socket_addr(&document), Duration::from_millis(100)).is_err()
    );
}

#[test]
fn mounted_proxy_second_real_courier_cannot_claim_one_rendezvous_or_connection() {
    let fixture = mount_raw_proxy();
    let mut first = FakeSdk::spawn(&fixture.local, &fixture.launch, Arc::clone(&fixture.calls));
    let mut launch = fixture.launch;
    launch
        .wait_until_ready()
        .expect("first real courier reaches ready");
    let second = block_on(fixture.local.process_host().start(
        ScopeId::new("fixture.scope.second-courier").expect("scope"),
        launch.process_request().clone(),
    ))
    .expect("second real courier process starts");
    assert!(
        !block_on(second.wait())
            .expect("second courier joins")
            .success()
    );
    first.close();
    let local = fixture.local;
    let lease = fixture.lease;
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
    .expect("one listener joins after both courier attempts");
}

#[test]
fn mounted_proxy_concurrency_rejects_a_second_connection_while_dispatch_is_in_flight() {
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        release: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: Arc::new(std::sync::Mutex::new(None)),
    });
    let fixture = mount_raw_proxy_with_dispatcher(
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Arc::clone(&blocking) as Arc<dyn RegisteredToolDispatcher>,
        Duration::from_secs(10),
        Arc::new(AtomicUsize::new(0)),
    );
    let mut first = connect_raw(&fixture.document);
    assert_eq!(
        raw_post(
            &mut first,
            &fixture.document.bearer,
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        )
        .0,
        200
    );
    let call_body = br#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{}}}"#;
    let (sender, receiver) = std::sync::mpsc::channel();
    let bearer = fixture.document.bearer.clone();
    std::thread::spawn(move || {
        sender
            .send(raw_post(&mut first, &bearer, call_body))
            .expect("call result")
    });
    let wait_until = std::time::Instant::now() + Duration::from_secs(2);
    while !blocking.entered.load(Ordering::SeqCst) && std::time::Instant::now() < wait_until {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(blocking.entered.load(Ordering::SeqCst));
    let mut second = connect_raw(&fixture.document);
    let second_result = raw_post(
        &mut second,
        &fixture.document.bearer,
        br#"{"jsonrpc":"2.0","id":3,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
    );
    assert_eq!(second_result.0, 409);
    blocking.release();
    assert_eq!(receiver.recv().expect("first call result").0, 200);
    drop(second);
    fixture.close();
}

#[test]
fn mounted_proxy_uncooperative_teardown_reports_failure_and_retains_resources() {
    let blocking = Arc::new(BlockingDispatcher {
        entered: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        release: Arc::new(std::sync::atomic::AtomicBool::new(false)),
        waker: Arc::new(std::sync::Mutex::new(None)),
    });
    let fixture = mount_raw_proxy_with_dispatcher(
        Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current()),
        Arc::clone(&blocking) as Arc<dyn RegisteredToolDispatcher>,
        Duration::from_millis(20),
        Arc::new(AtomicUsize::new(0)),
    );
    let mut stream = connect_raw(&fixture.document);
    assert_eq!(
        raw_post(
            &mut stream,
            &fixture.document.bearer,
            br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#,
        )
        .0,
        200
    );
    let (sender, receiver) = std::sync::mpsc::channel();
    let bearer = fixture.document.bearer.clone();
    std::thread::spawn(move || {
        sender
            .send(raw_post(
                &mut stream,
                &bearer,
                br#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"swallowtail.conformance/reconcile","arguments":{}}}"#,
            ))
            .expect("blocked call result");
    });
    let wait_until = std::time::Instant::now() + Duration::from_secs(2);
    while !blocking.entered.load(Ordering::SeqCst) && std::time::Instant::now() < wait_until {
        std::thread::sleep(Duration::from_millis(1));
    }
    assert!(blocking.entered.load(Ordering::SeqCst));
    let turn = fixture.lease.turn().clone();
    let cleanup = block_on(
        fixture
            .local
            .services()
            .registered_tool_bridge()
            .expect("registered bridge")
            .close(
                fixture.lease,
                swallowtail_runtime::RegisteredToolCleanupCause::Deadline,
            ),
    )
    .expect("teardown reports its bounded result");
    assert!(matches!(
        cleanup,
        swallowtail_runtime::CleanupOutcome::Failed(_)
    ));
    assert_eq!(fixture.local.registered_tool_lease_count(), 1);
    assert_eq!(fixture.local.operation_bridge_listener_count(), 1);
    assert_eq!(
        fixture.local.operation_bridge_generations(&turn),
        vec![("registered-tool", 1)]
    );
    blocking.release();
    assert_eq!(receiver.recv().expect("released call result").0, 200);
    drop(fixture.launch);
}

#[test]
fn mounted_both_profiles_demux_through_one_listener_and_one_resource_owner() {
    let fixture = mount_raw_proxy();
    let local = fixture.local.clone();
    let turn = fixture.lease.turn().clone();
    let watcher = block_on(
        fixture
            .local
            .services()
            .watcher_bridge()
            .expect("watcher bridge")
            .open(WatcherBridgeOpenRequest::new(
                ScopeId::new("fixture.scope.watcher").expect("scope"),
                turn.clone(),
            )),
    )
    .expect("watcher route registers on the existing operation listener");
    assert_eq!(fixture.local.operation_bridge_listener_count(), 1);
    assert_eq!(fixture.local.operation_bridge_lease_count(), 2);
    assert_eq!(
        fixture.local.operation_bridge_generations(&turn),
        vec![("watcher", 2), ("registered-tool", 1)]
    );
    let watcher_endpoint = watcher.endpoint().expose().to_owned();
    let watcher_bearer = watcher.bearer().expose().to_owned();
    let mut watcher_stream = connect_endpoint(&watcher_endpoint);
    let watcher_initialized = watcher_post(
        &mut watcher_stream,
        &watcher_bearer,
        br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-03-26"}}"#,
    );
    assert_eq!(watcher_initialized.0, 200);
    assert!(String::from_utf8_lossy(&watcher_initialized.1).contains("swallowtail-watcher-bridge"));
    drop(watcher_stream);
    let watcher_port = fixture
        .local
        .services()
        .watcher_bridge()
        .expect("watcher bridge")
        .clone();
    block_on(watcher_port.close(watcher, WatcherCleanupCause::Stopped))
        .expect("watcher route teardown joins without closing registered route");
    assert_eq!(fixture.local.operation_bridge_listener_count(), 1);
    fixture.close();
    assert_eq!(local.operation_bridge_listener_count(), 0);
}

#[test]
fn mounted_proxy_drops_oversized_and_partial_transport_without_dispatch() {
    let fixture = mount_raw_proxy();
    let mut partial = connect_raw(&fixture.document);
    partial
        .write_all(
            format!("POST {REGISTERED_TOOL_PROXY_HTTP_PATH} HTTP/1.1\r\nContent-Length: 2\r\n")
                .as_bytes(),
        )
        .expect("partial request");
    partial.flush().expect("partial request flush");
    drop(partial);
    assert_eq!(fixture.calls.load(Ordering::SeqCst), 0);
    fixture.close();

    let fixture = mount_raw_proxy();
    let mut stream = connect_raw(&fixture.document);
    stream
        .set_read_timeout(Some(Duration::from_millis(250)))
        .expect("read timeout");
    let oversized =
        vec![b'x'; swallowtail_host_local::wire::REGISTERED_TOOL_PROXY_MAX_RECORD_BYTES + 1];
    let header = format!(
        "POST {REGISTERED_TOOL_PROXY_HTTP_PATH} HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n",
        fixture.document.bearer,
        oversized.len()
    );
    stream
        .write_all(header.as_bytes())
        .expect("oversized header");
    stream.write_all(&oversized).expect("oversized body");
    stream.flush().expect("oversized request");
    let mut discarded = [0_u8; 1];
    let _ = stream.read(&mut discarded);
    drop(stream);
    assert_eq!(fixture.calls.load(Ordering::SeqCst), 0);
    fixture.close();
}

fn connect_raw(document: &RegisteredToolProxyRendezvousDocument) -> TcpStream {
    TcpStream::connect_timeout(&socket_addr(document), Duration::from_secs(1))
        .expect("mounted listener accepts")
}

fn connect_endpoint(endpoint: &str) -> TcpStream {
    let authority = endpoint
        .strip_prefix("http://127.0.0.1:")
        .expect("loopback endpoint");
    let port = authority
        .split_once('/')
        .expect("endpoint path")
        .0
        .parse::<u16>()
        .expect("endpoint port");
    TcpStream::connect_timeout(
        &format!("127.0.0.1:{port}").parse().expect("socket address"),
        Duration::from_secs(1),
    )
    .expect("mounted listener accepts")
}

fn socket_addr(document: &RegisteredToolProxyRendezvousDocument) -> std::net::SocketAddr {
    let authority = document
        .endpoint
        .strip_prefix("http://127.0.0.1:")
        .expect("loopback endpoint");
    let port = authority
        .split_once('/')
        .expect("endpoint path")
        .0
        .parse::<u16>()
        .expect("endpoint port");
    format!("127.0.0.1:{port}").parse().expect("socket address")
}

fn raw_post(stream: &mut TcpStream, bearer: &str, body: &[u8]) -> (u16, Vec<u8>) {
    raw_post_with_identity(stream, bearer, 1, 1, body)
}

fn raw_post_with_identity(
    stream: &mut TcpStream,
    bearer: &str,
    lease_generation: u64,
    transport_generation: u64,
    body: &[u8],
) -> (u16, Vec<u8>) {
    let header = format!(
        "POST {REGISTERED_TOOL_PROXY_HTTP_PATH} HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {bearer}\r\nX-Swallowtail-Lease-Generation: {lease_generation}\r\nX-Swallowtail-Transport-Generation: {transport_generation}\r\nX-Swallowtail-Server-Name: swallowtail-registered-tools\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        body.len(),
    );
    stream.write_all(header.as_bytes()).expect("request header");
    stream.write_all(body).expect("request body");
    stream.flush().expect("request flush");
    read_response(stream)
}

fn watcher_post(stream: &mut TcpStream, bearer: &str, body: &[u8]) -> (u16, Vec<u8>) {
    let header = format!(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nAuthorization: Bearer {bearer}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len(),
    );
    stream.write_all(header.as_bytes()).expect("request header");
    stream.write_all(body).expect("request body");
    stream.flush().expect("request flush");
    read_response(stream)
}

fn read_response(stream: &mut TcpStream) -> (u16, Vec<u8>) {
    let mut header = Vec::new();
    let mut byte = [0_u8; 1];
    loop {
        stream.read_exact(&mut byte).expect("response header");
        header.push(byte[0]);
        if header.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let text = std::str::from_utf8(&header[..header.len() - 4]).expect("response headers");
    let mut lines = text.split("\r\n");
    let status = lines
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|value| value.parse::<u16>().ok())
        .expect("response status");
    let length = lines
        .filter_map(|line| line.split_once(':'))
        .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
        .and_then(|(_, value)| value.trim().parse::<usize>().ok())
        .expect("response length");
    let mut response = vec![0_u8; length];
    stream.read_exact(&mut response).expect("response body");
    (status, response)
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

fn native_tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("swallowtail.conformance").expect("namespace"),
        RegisteredToolLocalName::new("native-client").expect("local name"),
    )
}

fn snapshot(
    host_id: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
) -> RegisteredToolSnapshot {
    snapshot_with_kind(
        host_id,
        executable,
        environment,
        RegisteredToolExecutionKind::Mcp,
    )
}

fn snapshot_with_kind(
    host_id: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
    kind: RegisteredToolExecutionKind,
) -> RegisteredToolSnapshot {
    snapshot_with_declarations(host_id, executable, environment, [(tool_id(), kind)])
}

fn snapshot_with_declarations(
    host_id: &ExecutionHostId,
    executable: ExecutableRef,
    environment: EnvironmentRef,
    declarations: impl IntoIterator<Item = (RegisteredToolId, RegisteredToolExecutionKind)>,
) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("swallowtail-registered-tools").expect("server"),
        revision: RegisteredServerRevision::new("fixture-1").expect("revision"),
        execution_host_id: host_id.clone(),
        declarations: declarations
            .into_iter()
            .enumerate()
            .map(|(index, (id, kind))| {
                RegisteredToolDeclaration::new(
                    id,
                    kind,
                    schema(&format!("input-{index}"), &format!("sha256:input-{index}")),
                    schema(
                        &format!("output-{index}"),
                        &format!("sha256:output-{index}"),
                    ),
                    RegisteredToolEffectPosture::ReadOnly,
                    RegisteredToolRetryPosture::NeverRetry,
                    RegisteredToolBounds::ceiling(),
                )
                .expect("declaration")
            })
            .collect(),
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
