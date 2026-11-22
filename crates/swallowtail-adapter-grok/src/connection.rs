use crate::activity::GROK_ACTIVITY_CONFIG;
use crate::failure::{failure, malformed, protocol_failure, unsupported};
use crate::turn::ActiveTurn;
use futures_channel::oneshot;
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use swallowtail_acp_client::{
    AcpConnection as SharedConnection, AcpConnectionHooks, AcpDiagnostics, AcpTransport,
    AcpTurnSink, PendingResponse, RequestDisposition, SessionUpdateDisposition,
};
use swallowtail_core::{SafeDiagnostic, SessionRef};
use swallowtail_protocol_acp::{FramingLimits, RpcError};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProcessHandle, RegisteredToolCleanupCause,
    ResourceLease, RuntimeFailure, WorkingResourceIoService, WorkingResourceLocator,
    WorkingResourceReadRequest,
};

const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_RECEIVE_FRAME_BYTES: usize = 16 * 1024 * 1024;
const MAXIMUM_RECEIVE_BUFFER_BYTES: usize = 32 * 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(MAXIMUM_RECEIVE_FRAME_BYTES, MAXIMUM_RECEIVE_BUFFER_BYTES);

pub(crate) fn grok_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "grok-build.acp",
        activity: GROK_ACTIVITY_CONFIG,
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.grok.acp.version_mismatch",
                "Grok Build negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.grok.acp.connection_closed",
                "Grok Build ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.grok.acp.correlation_limit",
                "Grok Build ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.grok.acp.session_duplicate",
                "Grok Build returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.grok.acp.turn_active",
                "Grok Build session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.grok.acp.unknown_response",
                "Grok Build returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.grok.acp.update_without_turn",
                "Grok Build updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.grok.acp.notification_unsupported",
                "Grok Build sent an unsupported ACP notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.grok.acp.notification_unsupported",
                "Grok Build sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.grok.acp.callback_unsupported",
                "Grok Build requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.grok.acp.permission_without_turn",
                "Grok Build requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.grok.acp.session_mismatch",
                "Grok Build callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.grok.acp.session_mismatch",
                "Grok Build update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.process_cleanup_failed",
                "Grok Build process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.cleanup_missing",
                "Grok Build process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.grok.acp.connection_ended",
                "Grok Build ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.grok.acp.output_limit_exceeded",
                "Grok Build output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.prompt_stopped",
                "Grok Build stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.grok.acp.stop_reason_unsupported",
                "Grok Build returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.grok.acp.task_service_missing",
                "Grok Build ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.task_join_failed",
                "Grok Build ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.turn_join_failed",
                "Grok Build ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.grok.acp.resource_release_failed",
                "Grok Build ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |_| {
            failure(
                "swallowtail.grok.acp.request_rejected",
                "Grok Build rejected an ACP request",
            )
        },
        permission_namespace: crate::grok_build_permission_namespace,
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

struct GrokHooks {
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    services: HostServices,
    turn: Mutex<Option<Arc<ActiveTurn>>>,
    registered: Mutex<Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>>,
    attachment_recovery: Mutex<Option<AttachmentRecoveryPhase>>,
}

impl AcpConnectionHooks for GrokHooks {
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
                "swallowtail.grok.acp.attachment_recovery_session_mismatch",
                "Grok Build recovery update belongs to another provider session",
            ));
        }
        if !params.get("update").is_some_and(Value::is_object) {
            return Err(malformed());
        }
        if phase.response_seen {
            return Err(failure(
                "swallowtail.grok.acp.attachment_recovery_late_update",
                "Grok Build emitted recovery replay after attachment readiness",
            ));
        }
        let bytes = serde_json::to_vec(params).map_err(|_| malformed())?.len();
        if phase.updates >= crate::MAXIMUM_ATTACHMENT_RECOVERY_UPDATES
            || phase.bytes.saturating_add(bytes) > crate::MAXIMUM_ATTACHMENT_RECOVERY_BYTES
        {
            return Err(failure(
                "swallowtail.grok.acp.attachment_recovery_limit_exceeded",
                "Grok Build attachment recovery exceeded its bounded discard limit",
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
                    "swallowtail.grok.acp.attachment_recovery_callback_rejected",
                    "Grok Build requested a callback during session attachment recovery",
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
                "session/request_permission" => {
                    connection.verify_session(params)?;
                    let turn = self.turn.lock().expect("ACP active lock poisoned").clone();
                    let Some(turn) = turn else {
                        connection
                            .send_error(id, -32600, "Permission request requires an active turn")
                            .await?;
                        return Err(failure(
                            "swallowtail.grok.acp.permission_without_turn",
                            "Grok Build requested permission without an active turn",
                        ));
                    };
                    if turn.exchanges_permissions() {
                        turn.exchange_permission(&id, params)?;
                        return Ok(RequestDisposition::Handled);
                    }
                    Ok(RequestDisposition::RejectPermission)
                }
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
            let registered = self
                .registered
                .lock()
                .expect("ACP registered lock poisoned")
                .clone();
            if let Some(registered) = registered {
                let _ = registered
                    .settle(&self.services, RegisteredToolCleanupCause::TransportFailure)
                    .await;
            }
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    hooks: Arc<GrokHooks>,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Arc<dyn WorkingResourceIoService>,
        services: HostServices,
    ) -> Arc<Self> {
        let hooks = Arc::new(GrokHooks {
            resource,
            resource_io,
            services: services.clone(),
            turn: Mutex::new(None),
            registered: Mutex::new(None),
            attachment_recovery: Mutex::new(None),
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            grok_diagnostics(),
            Arc::clone(&hooks) as Arc<dyn AcpConnectionHooks>,
        );
        Arc::new(Self { inner, hooks })
    }

    pub(crate) fn set_registered_session(
        &self,
        registered: Option<Arc<crate::registered_tool::GrokRegisteredToolSession>>,
    ) {
        *self
            .hooks
            .registered
            .lock()
            .expect("ACP registered lock poisoned") = registered;
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.inner
            .initialize_with(
                json!({
                    "fs": {"readTextFile": true, "writeTextFile": false},
                    "terminal": false
                }),
                json!({
                    "name": "swallowtail",
                    "title": "Swallowtail",
                    "version": env!("CARGO_PKG_VERSION")
                }),
            )
            .await
    }

    pub(crate) async fn activate_cached_token(&self) -> Result<(), RuntimeFailure> {
        let _private_response = self
            .request(
                "authenticate",
                json!({"methodId": "cached_token", "_meta": {"headless": true}}),
            )
            .await?;
        Ok(())
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
                        "swallowtail.grok.acp.connection_ended",
                        "Grok Build ACP connection ended",
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

    pub(crate) async fn respond_permission(
        &self,
        id: Value,
        option_id: &str,
    ) -> Result<(), RuntimeFailure> {
        self.inner
            .send_result(
                id,
                json!({"outcome": {"outcome": "selected", "optionId": option_id}}),
            )
            .await
    }

    pub(crate) fn set_session_id(&self, session_id: String) -> Result<(), RuntimeFailure> {
        self.inner.set_session_id(session_id)
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        self.inner
            .set_active_turn(Arc::clone(&turn) as Arc<dyn AcpTurnSink>)?;
        *self.hooks.turn.lock().expect("ACP active lock poisoned") = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        let sink: Arc<dyn AcpTurnSink> = Arc::clone(turn) as Arc<dyn AcpTurnSink>;
        self.inner.clear_active_turn(&sink);
        let mut slot = self.hooks.turn.lock().expect("ACP active lock poisoned");
        if slot
            .as_ref()
            .is_some_and(|current| Arc::ptr_eq(current, turn))
        {
            *slot = None;
        }
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
