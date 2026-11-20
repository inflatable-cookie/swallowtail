use crate::activity::CURSOR_ACTIVITY_CONFIG;
use crate::failure::{failure, malformed, protocol_failure, unsupported};
use crate::turn::ActiveTurn;
use futures_channel::oneshot;
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use swallowtail_acp_client::{
    AcpConnection as SharedConnection, AcpConnectionHooks, AcpDiagnostics, AcpTransport,
    AcpTurnSink, PendingResponse, RequestDisposition, SessionUpdateDisposition,
    default_permission_namespace,
};
use swallowtail_core::{SafeDiagnostic, SessionRef};
use swallowtail_protocol_acp::{FramingLimits, RpcError};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProcessHandle, ResourceLease, RuntimeFailure,
    WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
};

const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_RECEIVE_FRAME_BYTES: usize = 16 * 1024 * 1024;
const MAXIMUM_RECEIVE_BUFFER_BYTES: usize = 32 * 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(MAXIMUM_RECEIVE_FRAME_BYTES, MAXIMUM_RECEIVE_BUFFER_BYTES);

fn cursor_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "cursor-agent.acp",
        activity: CURSOR_ACTIVITY_CONFIG,
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.cursor.acp.version_mismatch",
                "Cursor Agent negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.cursor.acp.connection_closed",
                "Cursor Agent ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.cursor.acp.correlation_limit",
                "Cursor Agent ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.cursor.acp.session_duplicate",
                "Cursor Agent returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.cursor.acp.turn_active",
                "Cursor Agent session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.cursor.acp.unknown_response",
                "Cursor Agent returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.cursor.acp.update_without_turn",
                "Cursor Agent updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.cursor.acp.notification_unsupported",
                "Cursor Agent sent an unsupported ACP notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.cursor.acp.notification_unsupported",
                "Cursor Agent sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.cursor.acp.callback_unsupported",
                "Cursor Agent requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.cursor.acp.permission_without_turn",
                "Cursor Agent requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.cursor.acp.session_mismatch",
                "Cursor Agent callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.cursor.acp.session_mismatch",
                "Cursor Agent update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.process_cleanup_failed",
                "Cursor Agent process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.cleanup_missing",
                "Cursor Agent process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.cursor.acp.connection_ended",
                "Cursor Agent ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.cursor.acp.output_limit_exceeded",
                "Cursor Agent output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.prompt_stopped",
                "Cursor Agent stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.cursor.acp.stop_reason_unsupported",
                "Cursor Agent returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.cursor.acp.task_service_missing",
                "Cursor Agent ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.task_join_failed",
                "Cursor Agent ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.turn_join_failed",
                "Cursor Agent ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.cursor.acp.resource_release_failed",
                "Cursor Agent ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |_| {
            failure(
                "swallowtail.cursor.acp.request_rejected",
                "Cursor Agent rejected an ACP request",
            )
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

struct AttachmentRecoveryPhase {
    response_id: u64,
    session: SessionRef,
    response_seen: bool,
    updates: usize,
    bytes: usize,
    batch_completion: Option<oneshot::Sender<Result<(), RuntimeFailure>>>,
}

struct CursorHooks {
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    attachment_recovery: Mutex<Option<AttachmentRecoveryPhase>>,
}

impl AcpConnectionHooks for CursorHooks {
    fn on_response(&self, id: u64, _result: &Result<Value, RpcError>) {
        if let Some(phase) = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .as_mut()
            && phase.response_id == id
        {
            phase.response_seen = true;
        }
    }

    fn on_session_update(
        &self,
        params: &Value,
    ) -> Result<SessionUpdateDisposition, RuntimeFailure> {
        let mut phase = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned");
        let Some(phase) = phase.as_mut() else {
            return Ok(SessionUpdateDisposition::RouteToActiveTurn);
        };
        let session_id = params
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        if session_id != phase.session.as_provider_value() {
            return Err(failure(
                "swallowtail.cursor.acp.attachment_recovery_session_mismatch",
                "Cursor Agent recovery update belongs to another provider session",
            ));
        }
        if !params.get("update").is_some_and(Value::is_object) {
            return Err(malformed());
        }
        if phase.response_seen {
            return Err(failure(
                "swallowtail.cursor.acp.attachment_recovery_late_update",
                "Cursor Agent emitted recovery replay after attachment readiness",
            ));
        }
        let bytes = serde_json::to_vec(params).map_err(|_| malformed())?.len();
        if phase.updates >= crate::MAXIMUM_ATTACHMENT_RECOVERY_UPDATES
            || phase.bytes.saturating_add(bytes) > crate::MAXIMUM_ATTACHMENT_RECOVERY_BYTES
        {
            return Err(failure(
                "swallowtail.cursor.acp.attachment_recovery_limit_exceeded",
                "Cursor Agent attachment recovery exceeded its bounded discard limit",
            ));
        }
        phase.updates += 1;
        phase.bytes += bytes;
        Ok(SessionUpdateDisposition::Handled)
    }

    fn on_batch_complete(&self) {
        let sender = self
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .as_mut()
            .filter(|phase| phase.response_seen)
            .and_then(|phase| phase.batch_completion.take());
        if let Some(sender) = sender {
            let _ = sender.send(Ok(()));
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
            if self
                .attachment_recovery
                .lock()
                .expect("ACP attachment-recovery lock poisoned")
                .is_some()
            {
                connection
                    .send_error(id, -32600, "Request unavailable during session recovery")
                    .await?;
                return Err(failure(
                    "swallowtail.cursor.acp.attachment_recovery_callback_rejected",
                    "Cursor Agent requested a callback during session attachment recovery",
                ));
            }
            match method {
                "fs/read_text_file" => {
                    connection.verify_session(params)?;
                    let locator = params
                        .get("path")
                        .and_then(Value::as_str)
                        .ok_or_else(malformed)
                        .and_then(|value| {
                            WorkingResourceLocator::new(value).map_err(|_| malformed())
                        })?;
                    let request = WorkingResourceReadRequest::new(
                        locator,
                        NonZeroUsize::new(MAXIMUM_READ_BYTES).expect("static limit is non-zero"),
                    )
                    .with_lines(
                        optional_usize(params, "line")?,
                        optional_usize(params, "limit")?,
                    );
                    let content = self.resource_io.read_text(&self.resource, request).await?;
                    connection
                        .send_result(id, json!({"content": content.as_driver_value()}))
                        .await?;
                    Ok(RequestDisposition::Handled)
                }
                "session/request_permission" => Ok(RequestDisposition::RejectPermission),
                _ => Ok(RequestDisposition::Unsupported),
            }
        })
    }

    fn on_transport_ended<'a>(&'a self, error: &'a RuntimeFailure) -> BoxFuture<'a, ()> {
        let error = error.clone();
        Box::pin(async move {
            let sender = self
                .attachment_recovery
                .lock()
                .expect("ACP attachment-recovery lock poisoned")
                .as_mut()
                .and_then(|phase| phase.batch_completion.take());
            if let Some(sender) = sender {
                let _ = sender.send(Err(error));
            }
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    hooks: Arc<CursorHooks>,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Arc<dyn WorkingResourceIoService>,
        services: HostServices,
    ) -> Arc<Self> {
        let hooks = Arc::new(CursorHooks {
            resource,
            resource_io,
            attachment_recovery: Mutex::new(None),
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            cursor_diagnostics(),
            Arc::clone(&hooks) as Arc<dyn AcpConnectionHooks>,
        );
        Arc::new(Self { inner, hooks })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.inner
            .initialize_with(
                json!({
                    "fs": {"readTextFile": true, "writeTextFile": false}
                }),
                json!({
                    "name": "swallowtail",
                    "title": "Swallowtail",
                    "version": env!("CARGO_PKG_VERSION")
                }),
            )
            .await
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

    pub(crate) async fn recover_session_attachment(
        &self,
        session: SessionRef,
        cwd: String,
    ) -> Result<Value, RuntimeFailure> {
        let id = self.inner.allocate_request_id();
        let (batch_sender, batch_completion) = oneshot::channel();
        self.set_session_id(session.as_provider_value().to_owned())?;
        *self
            .hooks
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned") = Some(AttachmentRecoveryPhase {
            response_id: id,
            session: session.clone(),
            response_seen: false,
            updates: 0,
            bytes: 0,
            batch_completion: Some(batch_sender),
        });
        let pending = self
            .inner
            .begin_request_with_id(
                id,
                "session/load",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await;
        let response = match pending {
            Ok(response) => {
                let response = response.await;
                let batch = batch_completion.await.unwrap_or_else(|_| {
                    Err(failure(
                        "swallowtail.cursor.acp.connection_ended",
                        "Cursor Agent ACP connection ended",
                    ))
                });
                match (response, batch) {
                    (Ok(response), Ok(())) => Ok(response),
                    (Err(error), _) | (_, Err(error)) => Err(error),
                }
            }
            Err(error) => Err(error),
        };
        let phase = self
            .hooks
            .attachment_recovery
            .lock()
            .expect("ACP attachment-recovery lock poisoned")
            .take();
        match (response, phase) {
            (
                Ok(response),
                Some(AttachmentRecoveryPhase {
                    response_seen: true,
                    ..
                }),
            ) => Ok(response),
            (Err(error), _) => Err(error),
            _ => Err(protocol_failure()),
        }
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

fn optional_usize(params: &Value, field: &str) -> Result<Option<usize>, RuntimeFailure> {
    params
        .get(field)
        .map(|value| {
            value
                .as_u64()
                .and_then(|value| usize::try_from(value).ok())
                .ok_or_else(malformed)
        })
        .transpose()
}
