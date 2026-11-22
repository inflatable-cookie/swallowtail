use crate::{
    AcpActivityConfig, AcpActivityProjection, AcpConnection, AcpDiagnostics, AcpHttpAdvertisement,
    AcpMcpDiagnostics, AcpRemoteMcpPlacement, AcpSessionConfig, AcpSessionOpenContext,
    AcpTransport, MessageClassification, PlanProjection, StandardAcpHooks, open_acp_session,
    production_mcp_servers, redacted_mcp_servers_debug,
};
use futures_executor::block_on;
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use swallowtail_core::{ExecutionHostId, ResourceAccess, ResourceRepresentation, SafeDiagnostic};
use swallowtail_protocol_acp::{
    ACP_PROTOCOL_VERSION, DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits,
    decode_session_update,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, BoxFuture, HostServices,
    InteractiveSessionHandle, JoinedTask, MaterializedResourceRef, OperationContent, ProcessExit,
    ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RequestId, ResourceLease, RuntimeFailure,
    RuntimeTurnId, ScopeId, ScopedTaskService, TerminalStatus, TurnRequest, WorkingResourceRef,
};

fn test_failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

fn test_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "test.acp",
        activity: AcpActivityConfig {
            id_prefix: "test-acp",
            namespace_prefix: "test.acp",
            plan_projection: PlanProjection::TaskList,
            message_classification: MessageClassification::ModeAndWarning,
            malformed: || test_failure("swallowtail.test.acp.malformed", "malformed"),
        },
        malformed: || test_failure("swallowtail.test.acp.malformed", "malformed"),
        protocol_failure: || test_failure("swallowtail.test.acp.protocol", "protocol"),
        version_mismatch: || test_failure("swallowtail.test.acp.version", "version"),
        connection_closed: || test_failure("swallowtail.test.acp.closed", "closed"),
        correlation_limit: || test_failure("swallowtail.test.acp.limit", "limit"),
        session_duplicate: || test_failure("swallowtail.test.acp.dup", "dup"),
        turn_active: || test_failure("swallowtail.test.acp.turn_active", "turn active"),
        unknown_response: || test_failure("swallowtail.test.acp.unknown", "unknown"),
        update_without_turn: || test_failure("swallowtail.test.acp.no_turn", "no turn"),
        agent_cancel_unsupported: || test_failure("swallowtail.test.acp.cancel", "cancel"),
        notification_unsupported: || test_failure("swallowtail.test.acp.notif", "notif"),
        callback_unsupported: || test_failure("swallowtail.test.acp.callback", "callback"),
        permission_without_turn: || test_failure("swallowtail.test.acp.perm", "perm"),
        session_mismatch_callback: || test_failure("swallowtail.test.acp.mismatch", "mismatch"),
        session_mismatch_update: || test_failure("swallowtail.test.acp.mismatch", "mismatch"),
        process_cleanup_failed: || SafeDiagnostic::new("swallowtail.test.acp.cleanup", "cleanup"),
        cleanup_missing: || SafeDiagnostic::new("swallowtail.test.acp.cleanup_missing", "missing"),
        connection_ended: || test_failure("swallowtail.test.acp.ended", "ended"),
        output_limit_exceeded: || test_failure("swallowtail.test.acp.output", "output"),
        prompt_stopped: || SafeDiagnostic::new("swallowtail.test.acp.stopped", "stopped"),
        stop_reason_unsupported: || test_failure("swallowtail.test.acp.stop", "stop"),
        task_service_missing: || {
            test_failure("swallowtail.test.acp.task_missing", "task service missing")
        },
        task_join_failed: || SafeDiagnostic::new("swallowtail.test.acp.task_join", "task join"),
        turn_join_failed: || SafeDiagnostic::new("swallowtail.test.acp.turn_join", "turn join"),
        resource_release_failed: || {
            SafeDiagnostic::new("swallowtail.test.acp.resource", "resource")
        },
        classify_rpc_error: |_| test_failure("swallowtail.test.acp.rejected", "rejected"),
        permission_namespace: crate::diagnostics::default_permission_namespace,
        unsupported: |_| test_failure("swallowtail.test.acp.unsupported", "unsupported"),
    }
}

fn test_resource_lease() -> ResourceLease {
    ResourceLease::consumer_owned(
        ScopeId::new("test:session").expect("valid scope"),
        WorkingResourceRef::new("workspace").expect("valid ref"),
        ResourceAccess::Read,
        ResourceRepresentation::Filesystem,
    )
    .with_filesystem(MaterializedResourceRef::new("/tmp/workspace").expect("valid path"))
}

struct EofProcess {
    stdin_closed: AtomicBool,
    stop_requested: AtomicBool,
    force_stopped: AtomicBool,
    waited: AtomicBool,
}

impl EofProcess {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            stdin_closed: AtomicBool::new(false),
            stop_requested: AtomicBool::new(false),
            force_stopped: AtomicBool::new(false),
            waited: AtomicBool::new(false),
        })
    }
}

impl ProcessHandle for EofProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop_requested.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.force_stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.waited.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

struct NoopTask;

impl JoinedTask for NoopTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }
}

struct InlineTestTaskService;

impl ScopedTaskService for InlineTestTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        future: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        drop(future);
        Ok(Box::new(NoopTask))
    }
}

#[test]
fn closed_connection_rejects_new_requests_atomically() {
    block_on(async {
        let process: Arc<dyn ProcessHandle> = EofProcess::new();
        let transport = AcpTransport::stdio(
            process,
            FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES),
        );
        let host = ExecutionHostId::new("local").expect("valid host");
        let connection = AcpConnection::new(
            transport,
            HostServices::new(host),
            test_diagnostics(),
            Arc::new(StandardAcpHooks),
        );
        Arc::clone(&connection).pump().await;
        let error = connection
            .begin_request(
                "initialize",
                json!({"protocolVersion": ACP_PROTOCOL_VERSION}),
            )
            .await
            .err()
            .expect("closed connection must reject new requests");
        assert_eq!(error.diagnostic().code(), "swallowtail.test.acp.closed");
    });
}

#[test]
fn open_acp_session_closes_process_when_task_service_is_missing() {
    block_on(async {
        let process = EofProcess::new();
        let transport = AcpTransport::stdio(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES),
        );
        let host = ExecutionHostId::new("local").expect("valid host");
        let error = open_acp_session(
            transport,
            AcpSessionOpenContext {
                scope: ScopeId::new("test:session").expect("valid scope"),
                request_id: RequestId::new("req-1").expect("valid request id"),
                execution_host_id: host.clone(),
                resource: test_resource_lease(),
                services: HostServices::new(host),
                config: AcpSessionConfig::new("test-acp", test_diagnostics()),
            },
            |_| async { Ok("sess-1".to_owned()) },
        )
        .await
        .err()
        .expect("missing task service must fail");
        assert_eq!(
            error.0.diagnostic().code(),
            "swallowtail.test.acp.task_missing"
        );
        assert!(process.stdin_closed.load(Ordering::SeqCst));
        assert!(process.stop_requested.load(Ordering::SeqCst));
        assert!(
            process.force_stopped.load(Ordering::SeqCst),
            "an early validation failure must force-stop the spawned transport"
        );
        assert!(
            process.waited.load(Ordering::SeqCst),
            "an early validation failure must reap the spawned process"
        );
    });
}

#[test]
fn start_turn_missing_task_service_does_not_wedge_session() {
    block_on(async {
        let process = EofProcess::new();
        let transport = AcpTransport::stdio(
            Arc::clone(&process) as Arc<dyn ProcessHandle>,
            FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES),
        );
        let host = ExecutionHostId::new("local").expect("valid host");
        let services = HostServices::new(host.clone()).with_task(Arc::new(InlineTestTaskService));
        let mut session = open_acp_session(
            transport,
            AcpSessionOpenContext {
                scope: ScopeId::new("test:session").expect("valid scope"),
                request_id: RequestId::new("req-1").expect("valid request id"),
                execution_host_id: host.clone(),
                resource: test_resource_lease(),
                services,
                config: AcpSessionConfig::new("test-acp", test_diagnostics()),
            },
            |_| async { Ok("sess-1".to_owned()) },
        )
        .await
        .expect("session opens");

        // Call start_turn with a HostServices missing the task service.
        let turn_req = TurnRequest::new(
            RuntimeTurnId::new("turn-1").expect("valid turn id"),
            OperationContent::new("hello").expect("valid content"),
        );
        let err = session
            .start_turn(turn_req.clone(), HostServices::new(host.clone()))
            .await
            .err()
            .expect("missing task service must fail");
        assert_eq!(err.diagnostic().code(), "swallowtail.test.acp.task_missing");

        // Calling again must STILL report task_missing, NOT turn_active (no wedging!).
        let err2 = session
            .start_turn(turn_req, HostServices::new(host))
            .await
            .err()
            .expect("second call must not be wedged");
        assert_eq!(
            err2.diagnostic().code(),
            "swallowtail.test.acp.task_missing"
        );
    });
}

#[test]
fn activity_projection_covers_messages_tools_plans_and_mode_updates() {
    let mut projection = AcpActivityProjection::new(
        RuntimeTurnId::new("turn-1").expect("valid turn"),
        test_diagnostics().activity,
    );
    let mode_update = decode_session_update(&json!({
        "sessionId": "s1",
        "update": {
            "sessionUpdate": "agent_message_chunk",
            "content": {"type": "text", "text": "[MODE_UPDATE] plan"}
        }
    }))
    .expect("valid update");
    let obs = projection.project(&mode_update.update).expect("projected");
    assert_eq!(obs.len(), 1);
    assert_eq!(obs[0].kind(), &ActivityKind::AssistantMessage);

    let plan_update = decode_session_update(&json!({
        "sessionId": "s1",
        "update": {
            "sessionUpdate": "plan",
            "entries": [{"content": "Step 1", "priority": "high", "status": "in_progress"}]
        }
    }))
    .expect("valid plan");
    let obs = projection.project(&plan_update.update).expect("projected");
    assert_eq!(obs.len(), 1);
    assert_eq!(obs[0].kind(), &ActivityKind::Plan);
    assert!(obs[0].task_list().is_some());

    let completed = projection
        .complete(&TerminalStatus::Completed)
        .expect("completed");
    assert_eq!(completed.len(), 2);
    assert!(completed.iter().all(|o| {
        o.phase() == ActivityLifecyclePhase::Completed && o.status() == ActivityStatus::Completed
    }));
}

#[test]
fn mcp_placement_validates_and_redacts_secrets() {
    let diag = AcpMcpDiagnostics {
        reserved_name: "swallowtail-test-acp",
        name_collision: || test_failure("swallowtail.test.acp.collision", "collision"),
        http_invalid: || test_failure("swallowtail.test.acp.invalid", "invalid"),
    };
    let placement = AcpRemoteMcpPlacement::new(
        "swallowtail-test-acp",
        "https://example.invalid/mcp-secret-url",
        vec![("Authorization".to_owned(), "Bearer top-secret".to_owned())],
    );
    let debug = format!("{placement:?}");
    assert!(!debug.contains("mcp-secret-url"));
    assert!(!debug.contains("top-secret"));
    let value = production_mcp_servers(Some(&placement), &diag).expect("valid");
    let redacted = redacted_mcp_servers_debug(&value);
    let rendered = format!("{redacted}");
    assert!(!rendered.contains("mcp-secret-url"));
    assert!(!rendered.contains("top-secret"));
    assert_eq!(
        AcpHttpAdvertisement::from_initialize(&json!({
            "agentCapabilities": {"mcpCapabilities": {"http": true}}
        })),
        AcpHttpAdvertisement::Advertised
    );
}

#[test]
fn active_turn_uses_content_unsupported_for_non_text_message_chunks() {
    use crate::{AcpTurnSink, ActiveTurn, ActiveTurnConfig};

    let config = ActiveTurnConfig::new(test_diagnostics()).with_content_unsupported(|| {
        test_failure(
            "swallowtail.test.acp.content_unsupported",
            "unsupported content",
        )
    });
    let (turn, _events, _terminal) = ActiveTurn::new(
        RuntimeTurnId::new("turn-content").expect("valid turn id"),
        "sess-1".to_owned(),
        config,
    )
    .expect("turn creates");

    let non_text_agent = json!({
        "sessionId": "sess-1",
        "update": {
            "sessionUpdate": "agent_message_chunk",
            "content": {"type": "image", "mimeType": "image/png", "data": "AAAA"}
        }
    });
    let err = turn
        .handle_update(&non_text_agent)
        .expect_err("non-text agent chunk must fail");
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.test.acp.content_unsupported"
    );

    let non_text_thought = json!({
        "sessionId": "sess-1",
        "update": {
            "sessionUpdate": "agent_thought_chunk",
            "content": {"type": "image", "mimeType": "image/png", "data": "AAAA"}
        }
    });
    let err = turn
        .handle_update(&non_text_thought)
        .expect_err("non-text thought chunk must fail");
    assert_eq!(
        err.diagnostic().code(),
        "swallowtail.test.acp.content_unsupported"
    );
}

#[test]
fn active_turn_maps_auth_required_and_extended_stop_reasons_to_provider_failed() {
    use crate::{AcpTurnSink, ActiveTurn, ActiveTurnConfig, StopReasonPolicy};

    let config = ActiveTurnConfig::new(test_diagnostics())
        .with_stop_reason_policy(StopReasonPolicy::ExtendedProviderStops)
        .with_auth_required_stop(|| {
            SafeDiagnostic::new("swallowtail.test.acp.auth_required", "auth required")
        });

    for (stop_reason, expected_code) in [
        ("auth_required", "swallowtail.test.acp.auth_required"),
        ("max_turn_requests", "swallowtail.test.acp.stopped"),
        ("refusal", "swallowtail.test.acp.stopped"),
    ] {
        let (turn, _events, terminal) = ActiveTurn::new(
            RuntimeTurnId::new("turn-stop").expect("valid turn id"),
            "sess-1".to_owned(),
            config,
        )
        .expect("turn creates");
        turn.finish_prompt(stop_reason);
        let outcome = block_on(terminal);
        match outcome.status() {
            TerminalStatus::ProviderFailed(diag) => {
                assert_eq!(diag.code(), expected_code, "stop_reason={stop_reason}");
            }
            other => panic!("expected ProviderFailed for {stop_reason}, got {other:?}"),
        }
    }
}

#[test]
fn mcp_placement_rejects_line_breaks_in_header_names_and_values() {
    let diag = AcpMcpDiagnostics {
        reserved_name: "swallowtail-test-acp",
        name_collision: || test_failure("swallowtail.test.acp.collision", "collision"),
        http_invalid: || test_failure("swallowtail.test.acp.invalid", "invalid"),
    };
    let well_formed = AcpRemoteMcpPlacement::new(
        "swallowtail-test-acp",
        "https://example.invalid/mcp",
        vec![("authorization".to_owned(), "Bearer token".to_owned())],
    );
    assert!(production_mcp_servers(Some(&well_formed), &diag).is_ok());
    for (name, value) in [
        ("x-injected\r\nset-cookie", "value"),
        ("x-injected", "value\r\nset-cookie: stolen"),
        ("x-injected", "value\nset-cookie: stolen"),
        ("x-injected\n", "value"),
    ] {
        let placement = AcpRemoteMcpPlacement::new(
            "swallowtail-test-acp",
            "https://example.invalid/mcp",
            vec![(name.to_owned(), value.to_owned())],
        );
        let failure = production_mcp_servers(Some(&placement), &diag)
            .expect_err("a header with a line break is rejected");
        assert_eq!(failure.diagnostic().code(), "swallowtail.test.acp.invalid");
    }
}

#[test]
fn mcp_placement_zeroizes_url_and_header_values_on_wipe() {
    let mut placement = AcpRemoteMcpPlacement::new(
        "swallowtail-test-acp",
        "https://example.invalid/mcp-secret",
        vec![("Authorization".to_owned(), "Bearer secret-token".to_owned())],
    );
    let cloned = placement.clone();
    assert_eq!(placement, cloned);
    assert_eq!(placement.name(), "swallowtail-test-acp");
    assert_eq!(placement.url(), "https://example.invalid/mcp-secret");
    assert_eq!(
        placement.headers(),
        &[("Authorization".to_owned(), "Bearer secret-token".to_owned())]
    );
    placement.wipe_secrets_for_test();
    assert!(
        placement.url().is_empty(),
        "url must be zeroized on wipe/drop"
    );
    assert!(
        placement
            .headers()
            .iter()
            .all(|(_, value)| value.is_empty()),
        "header values must be zeroized on wipe/drop"
    );
    drop(placement);
    drop(cloned);
}

#[test]
fn duplicate_correlation_id_does_not_clobber_live_waiter() {
    use std::sync::Mutex;

    struct SingleResponseProcess {
        output: Mutex<Option<Vec<u8>>>,
    }

    impl ProcessHandle for SingleResponseProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            let chunk = self
                .output
                .lock()
                .expect("output lock")
                .take()
                .map(|bytes| {
                    ProcessOutputChunk::new(swallowtail_runtime::ProcessOutputStream::Stdout, bytes)
                });
            Box::pin(async move { Ok(chunk) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    block_on(async {
        let process: Arc<dyn ProcessHandle> = Arc::new(SingleResponseProcess {
            output: Mutex::new(Some(
                b"{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":{\"ok\":true}}\n".to_vec(),
            )),
        });
        let transport = AcpTransport::stdio(
            process,
            FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES),
        );
        let host = ExecutionHostId::new("local").expect("valid host");
        let connection = AcpConnection::new(
            transport,
            HostServices::new(host),
            test_diagnostics(),
            Arc::new(StandardAcpHooks),
        );

        let live_waiter = connection
            .begin_request_with_id(42, "test/method", json!({}))
            .await
            .expect("first request with id 42 succeeds");
        let duplicate_err = connection
            .begin_request_with_id(42, "test/method", json!({}))
            .await
            .err()
            .expect("duplicate correlation id must be rejected");
        assert_eq!(
            duplicate_err.diagnostic().code(),
            "swallowtail.test.acp.limit"
        );

        Arc::clone(&connection).pump().await;
        let response = live_waiter
            .await
            .expect("duplicate id rejection must not clobber the live waiter");
        assert_eq!(response, json!({"ok": true}));
    });
}
