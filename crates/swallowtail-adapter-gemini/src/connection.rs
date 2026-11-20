use crate::failure::{failure, malformed, protocol_failure, unsupported};
use serde_json::{Value, json};
use std::num::NonZeroUsize;
use std::sync::Arc;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpConnection as SharedConnection, AcpConnectionHooks, AcpDiagnostics,
    AcpTransport, AcpTurnSink, MessageClassification, PendingResponse, PlanProjection,
    RequestDisposition, default_permission_namespace,
};
use swallowtail_core::SafeDiagnostic;
use swallowtail_protocol_acp::{DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProcessHandle, ResourceLease, RuntimeFailure,
    WorkingResourceIoService, WorkingResourceLocator, WorkingResourceReadRequest,
    WorkingResourceText, WorkingResourceWriteRequest,
};

const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

pub(crate) fn gemini_acp_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "gemini-cli.acp",
        activity: AcpActivityConfig {
            id_prefix: "gemini-cli-acp",
            namespace_prefix: "gemini-cli.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::ModeAndWarning,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.gemini.acp.version_mismatch",
                "Gemini CLI negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.gemini.acp.connection_closed",
                "Gemini CLI ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.gemini.acp.correlation_limit",
                "Gemini CLI ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.gemini.acp.session_duplicate",
                "Gemini CLI returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.gemini.acp.turn_active",
                "Gemini CLI session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.gemini.acp.unknown_response",
                "Gemini CLI returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.gemini.acp.update_without_turn",
                "Gemini CLI updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.gemini.acp.agent_cancel_unsupported",
                "Gemini CLI sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.gemini.acp.notification_unsupported",
                "Gemini CLI sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.gemini.acp.callback_unsupported",
                "Gemini CLI requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.gemini.acp.permission_without_turn",
                "Gemini CLI requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.gemini.acp.session_mismatch",
                "Gemini CLI callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.gemini.acp.session_mismatch",
                "Gemini CLI update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.process_cleanup_failed",
                "Gemini CLI process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.cleanup_missing",
                "Gemini CLI process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.gemini.acp.connection_ended",
                "Gemini CLI ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.gemini.acp.output_limit_exceeded",
                "Gemini CLI output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.prompt_stopped",
                "Gemini CLI stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.gemini.acp.stop_reason_unsupported",
                "Gemini CLI returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.gemini.acp.task_service_missing",
                "Gemini CLI ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.task_join_failed",
                "Gemini CLI ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.turn_join_failed",
                "Gemini CLI ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.gemini.acp.resource_release_failed",
                "Gemini CLI ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            let message = error.message().to_ascii_lowercase();
            if message.contains("authentication required")
                || message.contains("api key is missing or not configured")
            {
                failure(
                    "swallowtail.gemini.acp.auth_required",
                    "Gemini CLI requires completed authentication before session work",
                )
            } else {
                failure(
                    "swallowtail.gemini.acp.request_rejected",
                    "Gemini CLI rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

struct GeminiAcpHooks {
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    write_enabled: bool,
    permission_allow_once: bool,
}

impl AcpConnectionHooks for GeminiAcpHooks {
    fn handle_request<'a>(
        &'a self,
        connection: &'a SharedConnection,
        id: Value,
        method: &'a str,
        params: &'a Value,
    ) -> BoxFuture<'a, Result<RequestDisposition, RuntimeFailure>> {
        Box::pin(async move {
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
                "fs/write_text_file" if self.write_enabled => {
                    connection.verify_session(params)?;
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
                        NonZeroUsize::new(MAXIMUM_WRITE_BYTES).expect("static limit is non-zero"),
                    )
                    .map_err(|_| malformed())?;
                    self.resource_io
                        .write_text(
                            &self.resource,
                            WorkingResourceWriteRequest::new(locator, content),
                        )
                        .await?;
                    connection.send_result(id, Value::Null).await?;
                    Ok(RequestDisposition::Handled)
                }
                "session/request_permission" => {
                    if !self.permission_allow_once {
                        return Ok(RequestDisposition::RejectPermission);
                    }
                    connection.verify_session(params)?;
                    let options = params
                        .get("options")
                        .and_then(Value::as_array)
                        .ok_or_else(malformed)?;
                    if options.len() > 32
                        || params
                            .get("toolCall")
                            .and_then(|tool| tool.get("toolCallId"))
                            .and_then(Value::as_str)
                            .is_none()
                    {
                        return Err(malformed());
                    }
                    let has_proceed_once = options.iter().any(|option| {
                        option.get("optionId").and_then(Value::as_str) == Some("proceed_once")
                    });
                    if !has_proceed_once {
                        return Err(malformed());
                    }
                    connection
                        .send_result(
                            id,
                            json!({"outcome": {"outcome": "selected", "optionId": "proceed_once"}}),
                        )
                        .await?;
                    Ok(RequestDisposition::Handled)
                }
                _ => Ok(RequestDisposition::Unsupported),
            }
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    write_enabled: bool,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Arc<dyn WorkingResourceIoService>,
        write_enabled: bool,
        permission_allow_once: bool,
        services: HostServices,
    ) -> Arc<Self> {
        let hooks = Arc::new(GeminiAcpHooks {
            resource,
            resource_io,
            write_enabled,
            permission_allow_once,
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            gemini_acp_diagnostics(),
            hooks,
        );
        Arc::new(Self {
            inner,
            write_enabled,
        })
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.inner
            .initialize_with(
                json!({
                    "fs": {
                        "readTextFile": true,
                        "writeTextFile": self.write_enabled
                    }
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

    pub(crate) fn set_active_turn(
        &self,
        turn: Arc<crate::turn::ActiveTurn>,
    ) -> Result<(), RuntimeFailure> {
        self.inner
            .set_active_turn(Arc::clone(&turn) as Arc<dyn AcpTurnSink>)
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<crate::turn::ActiveTurn>) {
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

fn optional_usize(params: &Value, key: &str) -> Result<Option<usize>, RuntimeFailure> {
    match params.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_u64()
            .and_then(|number| usize::try_from(number).ok())
            .map(Some)
            .ok_or_else(malformed),
    }
}
