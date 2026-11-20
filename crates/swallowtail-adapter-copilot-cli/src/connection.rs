use crate::{
    failure::{failure, malformed, protocol_failure, unsupported},
    turn::ActiveTurn,
};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use swallowtail_acp_client::{
    AcpActivityConfig, AcpConnection as SharedConnection, AcpConnectionHooks, AcpDiagnostics,
    AcpTransport, AcpTurnSink, MessageClassification, PendingResponse, PlanProjection,
    RequestDisposition, default_permission_namespace,
};
use swallowtail_core::SafeDiagnostic;
use swallowtail_protocol_acp::{DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits};
use swallowtail_runtime::{BoxFuture, CleanupOutcome, HostServices, ProcessHandle, RuntimeFailure};

const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn copilot_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "copilot-cli.acp",
        activity: AcpActivityConfig {
            id_prefix: "copilot-cli-acp",
            namespace_prefix: "copilot-cli.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::Standard,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.copilot-cli.acp.version_mismatch",
                "Copilot CLI negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.copilot-cli.acp.connection_closed",
                "Copilot CLI ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.copilot-cli.acp.correlation_limit",
                "Copilot CLI ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.copilot-cli.acp.session_duplicate",
                "Copilot CLI returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.copilot-cli.acp.turn_active",
                "Copilot CLI session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.copilot-cli.acp.unknown_response",
                "Copilot CLI returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.copilot-cli.acp.update_without_turn",
                "Copilot CLI updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.copilot-cli.acp.agent_cancel_unsupported",
                "Copilot CLI sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.copilot-cli.acp.notification_unsupported",
                "Copilot CLI sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.copilot-cli.acp.callback_unsupported",
                "Copilot CLI requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.copilot-cli.acp.permission_without_turn",
                "Copilot CLI requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.copilot-cli.acp.session_mismatch",
                "Copilot CLI callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.copilot-cli.acp.session_mismatch",
                "Copilot CLI update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.process_cleanup_failed",
                "Copilot CLI process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.cleanup_missing",
                "Copilot CLI process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.copilot-cli.acp.connection_ended",
                "Copilot CLI ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.copilot-cli.acp.output_limit_exceeded",
                "Copilot CLI output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.prompt_stopped",
                "Copilot CLI stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.copilot-cli.acp.stop_reason_unsupported",
                "Copilot CLI returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.copilot-cli.acp.task_service_missing",
                "Copilot CLI ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.task_join_failed",
                "Copilot CLI ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.turn_join_failed",
                "Copilot CLI ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.copilot-cli.acp.resource_release_failed",
                "Copilot CLI ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            let message = error.message().to_ascii_lowercase();
            if message.contains("unauthorized")
                || message.contains("unauthenticated")
                || message.contains("not logged in")
            {
                failure(
                    "swallowtail.copilot-cli.acp.host_auth_required",
                    "Copilot CLI ACP requires host-owned GitHub login or BYOK before session work",
                )
            } else {
                failure(
                    "swallowtail.copilot-cli.acp.request_rejected",
                    "Copilot CLI rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

struct CopilotHooks {
    turn: Mutex<Option<Arc<ActiveTurn>>>,
}

impl AcpConnectionHooks for CopilotHooks {
    fn handle_request<'a>(
        &'a self,
        connection: &'a SharedConnection,
        id: Value,
        method: &'a str,
        params: &'a Value,
    ) -> BoxFuture<'a, Result<RequestDisposition, RuntimeFailure>> {
        Box::pin(async move {
            if method != "session/request_permission" {
                return Ok(RequestDisposition::Unsupported);
            }
            connection.verify_session(params)?;
            let options = params
                .get("options")
                .and_then(Value::as_array)
                .ok_or_else(malformed)?;
            let tool_call_id = params
                .get("toolCall")
                .and_then(|tool| tool.get("toolCallId"))
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            if options.len() > 32 {
                return Err(malformed());
            }
            let turn = self
                .turn
                .lock()
                .expect("ACP active lock poisoned")
                .clone()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.copilot-cli.acp.permission_without_turn",
                        "Copilot CLI requested permission without an active turn",
                    )
                })?;
            turn.observe_permission(&id, tool_call_id)?;
            connection
                .notify("session/cancel", json!({"sessionId": turn.session_id()}))
                .await?;
            connection
                .send_result(id, json!({"outcome": {"outcome": "cancelled"}}))
                .await?;
            Ok(RequestDisposition::Handled)
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    hooks: Arc<CopilotHooks>,
}

impl AcpConnection {
    pub(crate) fn new(process: Arc<dyn ProcessHandle>, services: HostServices) -> Arc<Self> {
        let hooks = Arc::new(CopilotHooks {
            turn: Mutex::new(None),
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            copilot_diagnostics(),
            Arc::clone(&hooks) as Arc<dyn AcpConnectionHooks>,
        );
        Arc::new(Self { inner, hooks })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.inner.initialize().await
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
        let sink: Arc<dyn AcpTurnSink> = Arc::clone(&turn) as Arc<dyn AcpTurnSink>;
        self.inner.set_active_turn(sink)?;
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

    pub(crate) async fn force_stop_owned_process(&self) -> Result<(), RuntimeFailure> {
        self.inner.cancel_session().await
    }

    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.inner.cleanup_outcome()
    }

    pub(crate) async fn pump(self: Arc<Self>) {
        Arc::clone(&self.inner).pump().await;
    }
}
