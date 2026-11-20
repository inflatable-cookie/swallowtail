use crate::acp_activity::KIMI_ACTIVITY_CONFIG;
use crate::failure::{failure, malformed, protocol_failure, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use swallowtail_acp_client::{
    AcpConnection as SharedConnection, AcpConnectionHooks, AcpDiagnostics, AcpTransport,
    AcpTurnSink, PendingResponse, RequestDisposition, SessionUpdateDisposition,
    default_permission_namespace,
};
use swallowtail_core::{SafeDiagnostic, SessionRef};
use swallowtail_protocol_acp::{
    DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits, RpcError,
    is_session_scoped_metadata_update_kind,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, OperationContent, ProcessHandle, ResourceLease,
    RuntimeFailure, SessionReplayItem, SessionReplayKind, WorkingResourceIoService,
    WorkingResourceLocator, WorkingResourceText, WorkingResourceWriteRequest,
};

const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn kimi_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "kimi-code.acp",
        activity: KIMI_ACTIVITY_CONFIG,
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.kimi.acp.version_mismatch",
                "Kimi Code negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.kimi.acp.connection_closed",
                "Kimi Code ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.kimi.acp.correlation_limit",
                "Kimi Code ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.kimi.acp.session_duplicate",
                "Kimi Code returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.kimi.acp.turn_active",
                "Kimi Code session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.kimi.acp.unknown_response",
                "Kimi Code returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.kimi.acp.update_without_turn",
                "Kimi Code updated a session outside an allowed lifecycle phase",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.kimi.acp.notification_unsupported",
                "Kimi Code sent an unsupported ACP notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.kimi.acp.notification_unsupported",
                "Kimi Code sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.kimi.acp.callback_unsupported",
                "Kimi Code requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.kimi.acp.callback_unsupported",
                "Kimi Code requested an unsupported ACP client callback",
            )
        },
        session_mismatch_callback: session_mismatch,
        session_mismatch_update: session_mismatch,
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.process_cleanup_failed",
                "Kimi Code process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.cleanup_missing",
                "Kimi Code process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.kimi.acp.connection_ended",
                "Kimi Code ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.kimi.acp.output_limit_exceeded",
                "Kimi Code output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.prompt_stopped",
                "Kimi Code stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.kimi.acp.stop_reason_unsupported",
                "Kimi Code returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.kimi.acp.task_service_missing",
                "Kimi Code ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.task_join_failed",
                "Kimi Code ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.turn_join_failed",
                "Kimi Code ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kimi.acp.resource_release_failed",
                "Kimi Code ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |_| {
            failure(
                "swallowtail.kimi.acp.request_rejected",
                "Kimi Code rejected an ACP request",
            )
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

enum AttachPhase {
    Loading {
        response_id: u64,
        session: SessionRef,
        response_seen: bool,
        bytes: usize,
        replay: Vec<SessionReplayItem>,
    },
    Resuming {
        response_id: u64,
        session: SessionRef,
        response_seen: bool,
    },
}

struct KimiHooks {
    resource: ResourceLease,
    resource_io: Option<Arc<dyn WorkingResourceIoService>>,
    phase: Mutex<Option<AttachPhase>>,
}

impl AcpConnectionHooks for KimiHooks {
    fn on_response(&self, id: u64, _result: &Result<Value, RpcError>) {
        if let Some(phase) = self.phase.lock().expect("ACP phase lock poisoned").as_mut() {
            match phase {
                AttachPhase::Loading {
                    response_id,
                    response_seen,
                    ..
                }
                | AttachPhase::Resuming {
                    response_id,
                    response_seen,
                    ..
                } if *response_id == id => *response_seen = true,
                _ => {}
            }
        }
    }

    fn on_session_update(
        &self,
        params: &Value,
    ) -> Result<SessionUpdateDisposition, RuntimeFailure> {
        let mut phase = self.phase.lock().expect("ACP phase lock poisoned");
        let Some(phase) = phase.as_mut() else {
            return Ok(SessionUpdateDisposition::RouteToActiveTurn);
        };
        let session_id = params
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        let update = params.get("update").ok_or_else(malformed)?;
        let kind = update
            .get("sessionUpdate")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        match phase {
            AttachPhase::Loading {
                session,
                response_seen,
                bytes,
                replay,
                ..
            } => {
                if session.as_provider_value() != session_id {
                    return Err(session_mismatch());
                }
                if *response_seen {
                    passive_update(kind)?;
                    return Ok(SessionUpdateDisposition::Handled);
                }
                let item = replay_item(session.clone(), replay.len() as u64, kind, update)?;
                let item_bytes = item.content().map_or(0, OperationContent::byte_len);
                if replay.len() >= crate::MAXIMUM_REPLAY_ITEMS
                    || bytes.saturating_add(item_bytes) > crate::MAXIMUM_REPLAY_BYTES
                {
                    return Err(failure(
                        "swallowtail.kimi.acp.replay_limit_exceeded",
                        "Kimi Code session replay exceeded the adapter limit",
                    ));
                }
                *bytes += item_bytes;
                replay.push(item);
                Ok(SessionUpdateDisposition::Handled)
            }
            AttachPhase::Resuming {
                session,
                response_seen,
                ..
            } => {
                if session.as_provider_value() != session_id {
                    return Err(session_mismatch());
                }
                if *response_seen {
                    passive_update(kind)?;
                    Ok(SessionUpdateDisposition::Handled)
                } else {
                    Err(failure(
                        "swallowtail.kimi.acp.resume_replay_rejected",
                        "Kimi Code emitted historical replay while resuming",
                    ))
                }
            }
        }
    }

    fn handle_request<'a>(
        &'a self,
        connection: &'a SharedConnection,
        id: Value,
        method: &'a str,
        params: &'a Value,
    ) -> BoxFuture<'a, Result<RequestDisposition, RuntimeFailure>> {
        Box::pin(async move {
            match method {
                "fs/write_text_file" => {
                    if connection.verify_session(params).is_err() {
                        connection.send_error(id, -32602, "Invalid params").await?;
                        return Ok(RequestDisposition::Handled);
                    }
                    let request: Result<WorkingResourceWriteRequest, RuntimeFailure> = (|| {
                        let locator = params
                            .get("path")
                            .and_then(Value::as_str)
                            .ok_or_else(malformed)
                            .and_then(|value| {
                                WorkingResourceLocator::new(value).map_err(|_| malformed())
                            })?;
                        let content = params
                            .get("content")
                            .and_then(Value::as_str)
                            .ok_or_else(malformed)?;
                        let content = WorkingResourceText::new(
                            content.to_owned(),
                            NonZeroUsize::new(crate::MAXIMUM_WRITE_BYTES)
                                .expect("static limit is non-zero"),
                        )
                        .map_err(|_| malformed())?;
                        Ok(WorkingResourceWriteRequest::new(locator, content))
                    })(
                    );
                    let Ok(request) = request else {
                        connection.send_error(id, -32602, "Invalid params").await?;
                        return Ok(RequestDisposition::Handled);
                    };
                    let Some(resource_io) = &self.resource_io else {
                        connection.send_error(id, -32602, "Invalid params").await?;
                        return Ok(RequestDisposition::Handled);
                    };
                    if resource_io
                        .write_text(&self.resource, request)
                        .await
                        .is_err()
                    {
                        connection.send_error(id, -32602, "Invalid params").await?;
                        return Ok(RequestDisposition::Handled);
                    }
                    connection.send_result(id, Value::Null).await?;
                    Ok(RequestDisposition::Handled)
                }
                _ => Ok(RequestDisposition::Unsupported),
            }
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    hooks: Arc<KimiHooks>,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Option<Arc<dyn WorkingResourceIoService>>,
        services: HostServices,
    ) -> Arc<Self> {
        let hooks = Arc::new(KimiHooks {
            resource,
            resource_io,
            phase: Mutex::new(None),
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            kimi_diagnostics(),
            Arc::clone(&hooks) as Arc<dyn AcpConnectionHooks>,
        );
        Arc::new(Self { inner, hooks })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.initialize_with_write_capability(true).await
    }

    pub(crate) async fn initialize_catalogue(&self) -> Result<Value, RuntimeFailure> {
        self.initialize_with_write_capability(false).await
    }

    async fn initialize_with_write_capability(
        &self,
        write_text_file: bool,
    ) -> Result<Value, RuntimeFailure> {
        self.inner
            .initialize_with(
                json!({
                    "fs": {"readTextFile": false, "writeTextFile": write_text_file},
                    "terminal": false,
                    "auth": {"terminal": false}
                }),
                json!({
                    "name": "swallowtail",
                    "title": "Swallowtail",
                    "version": env!("CARGO_PKG_VERSION")
                }),
            )
            .await
    }

    pub(crate) async fn new_session(&self, cwd: String) -> Result<Value, RuntimeFailure> {
        self.request("session/new", json!({"cwd": cwd, "mcpServers": []}))
            .await
    }

    pub(crate) async fn set_config_option(
        &self,
        session_id: &str,
        config_id: &str,
        value: &str,
    ) -> Result<Value, RuntimeFailure> {
        self.request(
            "session/set_config_option",
            json!({"sessionId": session_id, "configId": config_id, "value": value}),
        )
        .await
    }

    pub(crate) async fn load_session(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<(Value, Vec<SessionReplayItem>), RuntimeFailure> {
        let id = self.inner.allocate_request_id();
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self.hooks.phase.lock().expect("ACP phase lock poisoned") = Some(AttachPhase::Loading {
            response_id: id,
            session: session.clone(),
            response_seen: false,
            bytes: 0,
            replay: Vec::new(),
        });
        let response = self
            .inner
            .begin_request_with_id(
                id,
                "session/load",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        match self
            .hooks
            .phase
            .lock()
            .expect("ACP phase lock poisoned")
            .take()
        {
            Some(AttachPhase::Loading {
                response_seen: true,
                replay,
                ..
            }) => Ok((response, replay)),
            _ => Err(protocol_failure()),
        }
    }

    pub(crate) async fn resume_session(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<Value, RuntimeFailure> {
        let id = self.inner.allocate_request_id();
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self.hooks.phase.lock().expect("ACP phase lock poisoned") = Some(AttachPhase::Resuming {
            response_id: id,
            session: session.clone(),
            response_seen: false,
        });
        let response = self
            .inner
            .begin_request_with_id(
                id,
                "session/resume",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        match self
            .hooks
            .phase
            .lock()
            .expect("ACP phase lock poisoned")
            .take()
        {
            Some(AttachPhase::Resuming {
                response_seen: true,
                ..
            }) => Ok(response),
            _ => Err(protocol_failure()),
        }
    }

    pub(crate) async fn list_sessions(
        &self,
        capabilities: swallowtail_protocol_acp::AcpSessionListCapabilities,
        cwd: String,
        cursor: Option<String>,
        limits: swallowtail_protocol_acp::AcpSessionListLimits,
    ) -> Result<swallowtail_protocol_acp::AcpSessionListPage, RuntimeFailure> {
        let id = self.inner.allocate_request_id();
        let request = swallowtail_protocol_acp::AcpSessionListRequest::new(
            json!(id),
            capabilities,
            Some(cwd),
            cursor,
            limits,
        )
        .map_err(|_| {
            failure(
                "swallowtail.kimi.acp.session_list_request_invalid",
                "Kimi Code ACP session-list request is invalid",
            )
        })?;
        let result = self
            .inner
            .begin_request_with_id(
                id,
                swallowtail_protocol_acp::ACP_SESSION_LIST_METHOD,
                json!({"cwd": request.cwd(), "cursor": request.cursor()}),
            )
            .await?
            .await?;
        request.decode_result(&result).map_err(|_| {
            failure(
                "swallowtail.kimi.acp.session_list_response_invalid",
                "Kimi Code returned invalid ACP session-list evidence",
            )
        })
    }

    pub(crate) async fn request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<Value, RuntimeFailure> {
        self.inner.request(method, params).await
    }

    pub(crate) async fn begin_request(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<PendingResponse, RuntimeFailure> {
        self.inner.begin_request(method, params).await
    }

    pub(crate) async fn notify(
        &self,
        method: &'static str,
        params: Value,
    ) -> Result<(), RuntimeFailure> {
        self.inner.notify(method, params).await
    }

    pub(crate) fn set_session_id(&self, session_id: String) -> Result<(), RuntimeFailure> {
        self.inner.set_session_id(session_id)
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        self.inner
            .set_active_turn(Arc::clone(&turn) as Arc<dyn AcpTurnSink>)
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        let sink: Arc<dyn AcpTurnSink> = Arc::clone(turn) as Arc<dyn AcpTurnSink>;
        self.inner.clear_active_turn(&sink);
    }

    pub(crate) async fn begin_close(&self) {
        self.inner.begin_close().await;
    }

    pub(crate) async fn cancel_session(&self) -> Result<(), RuntimeFailure> {
        self.inner.cancel_session().await
    }

    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.inner.cleanup_outcome()
    }

    pub(crate) async fn pump(self: Arc<Self>) {
        Arc::clone(&self.inner).pump().await;
    }
}

include!("connection/dispatch/replay.rs");
