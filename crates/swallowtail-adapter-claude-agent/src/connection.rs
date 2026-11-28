use crate::acp_activity::CLAUDE_AGENT_ACTIVITY_CONFIG;
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
use swallowtail_core::{ResourceAccess, SafeDiagnostic, SessionRef};
use swallowtail_protocol_acp::{
    AcpSessionUpdate, FramingLimits, RpcError, decode_session_update,
    is_session_scoped_metadata_update_kind,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, OperationContent, ProcessHandle, ResourceLease,
    RuntimeFailure, SessionReplayItem, SessionReplayKind, WorkingResourceIoService,
    WorkingResourceLocator, WorkingResourceReadRequest,
};

const MAXIMUM_READ_BYTES: usize = 1024 * 1024;
const MAXIMUM_RECEIVED_FRAME_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_RECEIVE_BUFFER_BYTES: usize = 8 * 1024 * 1024;
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(MAXIMUM_RECEIVED_FRAME_BYTES, MAXIMUM_RECEIVE_BUFFER_BYTES);

fn claude_agent_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "claude-agent.acp",
        activity: CLAUDE_AGENT_ACTIVITY_CONFIG,
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.claude_agent.acp.version_mismatch",
                "Claude Agent negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.claude_agent.acp.connection_closed",
                "Claude Agent ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.claude_agent.acp.correlation_limit",
                "Claude Agent ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.claude_agent.acp.session_duplicate",
                "Claude Agent returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.claude_agent.acp.turn_active",
                "Claude Agent session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.claude_agent.acp.unknown_response",
                "Claude Agent returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.claude_agent.acp.update_without_turn",
                "Claude Agent updated a session outside an allowed lifecycle phase",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.claude_agent.acp.notification_unsupported",
                "Claude Agent sent an unsupported ACP notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.claude_agent.acp.notification_unsupported",
                "Claude Agent sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.claude_agent.acp.callback_unsupported",
                "Claude Agent requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.claude_agent.acp.permission_without_turn",
                "Claude Agent requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.claude_agent.acp.session_mismatch",
                "Claude Agent callback does not match the active session",
            )
        },
        session_mismatch_update: session_mismatch,
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.process_cleanup_failed",
                "Claude Agent process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.cleanup_missing",
                "Claude Agent process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.claude_agent.acp.connection_ended",
                "Claude Agent ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.claude_agent.acp.output_limit_exceeded",
                "Claude Agent output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.prompt_stopped",
                "Claude Agent stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.claude_agent.acp.stop_reason_unsupported",
                "Claude Agent returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.claude_agent.acp.task_service_missing",
                "Claude Agent ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.task_join_failed",
                "Claude Agent ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.turn_join_failed",
                "Claude Agent ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.claude_agent.acp.resource_release_failed",
                "Claude Agent ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |_| {
            failure(
                "swallowtail.claude_agent.acp.request_rejected",
                "Claude Agent rejected an ACP request",
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

struct ClaudeAgentHooks {
    resource: ResourceLease,
    resource_io: Arc<dyn WorkingResourceIoService>,
    session_id: Mutex<Option<String>>,
    phase: Mutex<Option<AttachPhase>>,
    turn: Mutex<Option<Arc<ActiveTurn>>>,
}

impl AcpConnectionHooks for ClaudeAgentHooks {
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

    fn on_notification(&self, method: &str, params: &Value) -> Result<bool, RuntimeFailure> {
        if method == "auth/status_update" {
            params
                .get("state")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            #[cfg(test)]
            eprintln!("ACP_TRUSTED_PASSIVE_AUTH_STATUS");
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn on_session_update(
        &self,
        params: &Value,
    ) -> Result<SessionUpdateDisposition, RuntimeFailure> {
        let mut phase = self.phase.lock().expect("ACP phase lock poisoned");
        let Some(phase) = phase.as_mut() else {
            let has_active_turn = self
                .turn
                .lock()
                .expect("ACP active lock poisoned")
                .is_some();
            if !has_active_turn
                && params
                    .get("update")
                    .and_then(|update| update.get("sessionUpdate"))
                    .and_then(Value::as_str)
                    == Some("session_info_update")
            {
                let bound = self
                    .session_id
                    .lock()
                    .expect("ACP session lock poisoned")
                    .clone();
                let session_id = params
                    .get("sessionId")
                    .and_then(Value::as_str)
                    .ok_or_else(malformed)?;
                if bound.as_deref() != Some(session_id) {
                    return Err(session_mismatch());
                }
                let decoded = decode_session_update(params).map_err(|_| malformed())?;
                if !matches!(decoded.update, AcpSessionUpdate::SessionInfo { .. }) {
                    return Err(malformed());
                }
                return Ok(SessionUpdateDisposition::Handled);
            }
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
                    passive_update(kind, params)?;
                    return Ok(SessionUpdateDisposition::Handled);
                }
                let item = replay_item(session.clone(), replay.len() as u64, kind, update)?;
                let item_bytes = item.content().map_or(0, OperationContent::byte_len);
                if replay.len() >= crate::MAXIMUM_REPLAY_ITEMS
                    || bytes.saturating_add(item_bytes) > crate::MAXIMUM_REPLAY_BYTES
                {
                    return Err(failure(
                        "swallowtail.claude_agent.acp.replay_limit_exceeded",
                        "Claude Agent session replay exceeded the adapter limit",
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
                    passive_update(kind, params)?;
                    Ok(SessionUpdateDisposition::Handled)
                } else {
                    Err(failure(
                        "swallowtail.claude_agent.acp.resume_replay_rejected",
                        "Claude Agent emitted historical replay while resuming",
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
                    let turn = self
                        .turn
                        .lock()
                        .expect("ACP active lock poisoned")
                        .clone()
                        .filter(|turn| !turn.is_finished())
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.claude_agent.acp.permission_without_turn",
                                "Claude Agent requested permission without an active turn",
                            )
                        })?;
                    if turn.exchanges_permissions() {
                        turn.exchange_permission(&id, params)?;
                        return Ok(RequestDisposition::Handled);
                    }
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
                    let reject = options
                        .iter()
                        .find(|option| {
                            option.get("kind").and_then(Value::as_str) == Some("reject_once")
                        })
                        .and_then(|option| option.get("optionId"))
                        .and_then(Value::as_str)
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.claude_agent.acp.permission_rejection_unavailable",
                                "Claude Agent permission request offered no one-shot rejection",
                            )
                        })?;
                    turn.observe_permission(&id)?;
                    connection
                        .send_result(
                            id,
                            json!({"outcome": {"outcome": "selected", "optionId": reject}}),
                        )
                        .await?;
                    connection
                        .notify("session/cancel", json!({"sessionId": turn.session_id()}))
                        .await?;
                    Ok(RequestDisposition::Handled)
                }
                "elicitation/create" => {
                    connection.verify_session(params)?;
                    let turn = self
                        .turn
                        .lock()
                        .expect("ACP active lock poisoned")
                        .clone()
                        .filter(|turn| !turn.is_finished())
                        .ok_or_else(|| {
                            failure(
                                "swallowtail.claude_agent.acp.elicitation_without_turn",
                                "Claude Agent requested user input without an active turn",
                            )
                        })?;
                    match crate::elicitation::request(params)? {
                        Some(request) => turn.exchange_user_input(&id, request)?,
                        None => {
                            connection
                                .send_result(id, crate::elicitation::declined_response())
                                .await?;
                        }
                    }
                    Ok(RequestDisposition::Handled)
                }
                _ => Ok(RequestDisposition::Unsupported),
            }
        })
    }
}

pub(crate) struct AcpConnection {
    inner: Arc<SharedConnection>,
    hooks: Arc<ClaudeAgentHooks>,
    resource: ResourceLease,
}

impl AcpConnection {
    pub(crate) fn new(
        process: Arc<dyn ProcessHandle>,
        resource: ResourceLease,
        resource_io: Arc<dyn WorkingResourceIoService>,
        services: HostServices,
    ) -> Arc<Self> {
        let hooks = Arc::new(ClaudeAgentHooks {
            resource: resource.clone(),
            resource_io,
            session_id: Mutex::new(None),
            phase: Mutex::new(None),
            turn: Mutex::new(None),
        });
        let inner = SharedConnection::new(
            AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS),
            services,
            claude_agent_diagnostics(),
            Arc::clone(&hooks) as Arc<dyn AcpConnectionHooks>,
        );
        Arc::new(Self {
            inner,
            hooks,
            resource,
        })
    }

    pub(crate) fn emit_protocol_debug(&self, error: &RuntimeFailure, stage: &'static str) {
        self.inner.emit_protocol_debug(error, stage);
    }

    pub(crate) async fn initialize(&self) -> Result<Value, RuntimeFailure> {
        self.inner
            .initialize_with(
                json!({
                    "fs": {"readTextFile": true, "writeTextFile": false},
                    "elicitation": {"form": {}}
                }),
                json!({
                    "name": "swallowtail",
                    "title": "Swallowtail",
                    "version": env!("CARGO_PKG_VERSION")
                }),
            )
            .await
    }

    pub(crate) async fn new_session(
        &self,
        cwd: String,
        model: &str,
        mcp_servers: Value,
    ) -> Result<Value, RuntimeFailure> {
        let tools = match self.resource.access() {
            ResourceAccess::Read => json!(["Read", "Glob", "Grep"]),
            ResourceAccess::ReadWrite => json!(["Read", "Glob", "Grep", "Edit", "Write"]),
        };
        self.request(
            "session/new",
            json!({
                "cwd": cwd,
                "mcpServers": mcp_servers,
                "_meta": {
                    "claudeCode": {
                        "options": {
                            "tools": tools,
                            "settings": {
                                "model": model,
                                "availableModels": [model]
                            }
                        }
                    }
                }
            }),
        )
        .await
    }

    pub(crate) async fn set_session_mode(
        &self,
        session_id: &str,
        mode_id: &'static str,
    ) -> Result<(), RuntimeFailure> {
        let response = self
            .request(
                "session/set_mode",
                json!({"sessionId": session_id, "modeId": mode_id}),
            )
            .await?;
        if response.as_object().is_some_and(serde_json::Map::is_empty) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.claude_agent.acp.mode_malformed",
                "Claude Agent returned a malformed session-mode response",
            ))
        }
    }

    pub(crate) async fn close_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        let response = self
            .request("session/close", json!({"sessionId": session_id}))
            .await?;
        if response.as_object().is_some_and(serde_json::Map::is_empty) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.claude_agent.acp.close_malformed",
                "Claude Agent returned a malformed ACP session-close response",
            ))
        }
    }

    pub(crate) async fn delete_session(&self, session_id: &str) -> Result<(), RuntimeFailure> {
        let response = self
            .request("session/delete", json!({"sessionId": session_id}))
            .await?;
        if response.as_object().is_some_and(serde_json::Map::is_empty) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.claude_agent.acp.delete_malformed",
                "Claude Agent returned a malformed ACP session-delete response",
            ))
        }
    }

    pub(crate) async fn set_config_option(
        &self,
        session_id: &str,
        config_id: &'static str,
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
        let phase_guard = AttachPhaseGuard::enter(
            &self.hooks.phase,
            AttachPhase::Loading {
                response_id: id,
                session: session.clone(),
                response_seen: false,
                bytes: 0,
                replay: Vec::new(),
            },
        );
        let response = self
            .inner
            .begin_request_with_id(
                id,
                "session/load",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        match phase_guard.take() {
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
        let phase_guard = AttachPhaseGuard::enter(
            &self.hooks.phase,
            AttachPhase::Resuming {
                response_id: id,
                session: session.clone(),
                response_seen: false,
            },
        );
        let response = self
            .inner
            .begin_request_with_id(
                id,
                "session/resume",
                json!({"sessionId": session.as_provider_value(), "cwd": cwd, "mcpServers": []}),
            )
            .await?
            .await?;
        match phase_guard.take() {
            Some(AttachPhase::Resuming {
                response_seen: true,
                ..
            }) => Ok(response),
            _ => Err(protocol_failure()),
        }
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

    pub(crate) async fn respond_elicitation(
        &self,
        id: Value,
        response: Value,
    ) -> Result<(), RuntimeFailure> {
        self.inner.send_result(id, response).await
    }

    pub(crate) fn set_session_id(&self, session_id: String) -> Result<(), RuntimeFailure> {
        self.inner.set_session_id(session_id.clone())?;
        *self
            .hooks
            .session_id
            .lock()
            .expect("ACP session lock poisoned") = Some(session_id);
        Ok(())
    }

    pub(crate) fn set_active_turn(&self, turn: Arc<ActiveTurn>) -> Result<(), RuntimeFailure> {
        self.inner
            .set_active_turn(Arc::clone(&turn) as Arc<dyn AcpTurnSink>)?;
        *self.hooks.turn.lock().expect("ACP active lock poisoned") = Some(turn);
        Ok(())
    }

    pub(crate) fn clear_active_turn(&self, turn: &Arc<ActiveTurn>) {
        if !turn.retains_terminal_tail() {
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
    }

    pub(crate) async fn begin_close(&self) {
        self.inner.begin_close().await;
    }

    pub(crate) fn is_closed(&self) -> bool {
        self.inner.is_closed()
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

fn replay_item(
    session: SessionRef,
    sequence: u64,
    kind: &str,
    update: &Value,
) -> Result<SessionReplayItem, RuntimeFailure> {
    let replay_kind = match kind {
        "user_message_chunk" => SessionReplayKind::UserMessage,
        "agent_message_chunk" => SessionReplayKind::AgentMessage,
        "agent_thought_chunk" => SessionReplayKind::AgentReasoning,
        "tool_call" => SessionReplayKind::ToolCall,
        "tool_call_update" => SessionReplayKind::ToolCallUpdate,
        "plan" => SessionReplayKind::Plan,
        "available_commands_update" | "config_option_update" | "current_mode_update" => {
            SessionReplayKind::Configuration
        }
        _ => {
            return Err(failure(
                "swallowtail.claude_agent.acp.replay_unsupported",
                "Claude Agent returned unsupported replay content",
            ));
        }
    };
    match kind {
        "user_message_chunk" | "agent_message_chunk" | "agent_thought_chunk" => {
            let text = update
                .get("content")
                .filter(|content| content.get("type").and_then(Value::as_str) == Some("text"))
                .and_then(|content| content.get("text"))
                .and_then(Value::as_str)
                .ok_or_else(malformed)?;
            Ok(SessionReplayItem::with_content(
                session,
                sequence,
                replay_kind,
                OperationContent::new(text).map_err(|_| malformed())?,
            ))
        }
        _ => Ok(SessionReplayItem::new(session, sequence, replay_kind)),
    }
}

fn passive_update(kind: &str, params: &Value) -> Result<(), RuntimeFailure> {
    if is_session_scoped_metadata_update_kind(kind) {
        Ok(())
    } else if kind == "session_info_update" {
        let decoded = decode_session_update(params).map_err(|_| malformed())?;
        if matches!(decoded.update, AcpSessionUpdate::SessionInfo { .. }) {
            Ok(())
        } else {
            Err(malformed())
        }
    } else {
        Err(failure(
            "swallowtail.claude_agent.acp.update_without_turn",
            "Claude Agent updated a session outside an allowed lifecycle phase",
        ))
    }
}

fn session_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.session_mismatch",
        "Claude Agent message does not match the bound session",
    )
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

/// Ensures `hooks.phase` is cleared if `load_session` or `resume_session`
/// returns early on error or is dropped before completion.
struct AttachPhaseGuard<'a> {
    phase: &'a Mutex<Option<AttachPhase>>,
    armed: bool,
}

impl<'a> AttachPhaseGuard<'a> {
    fn enter(phase: &'a Mutex<Option<AttachPhase>>, state: AttachPhase) -> Self {
        *phase.lock().expect("ACP phase lock poisoned") = Some(state);
        Self { phase, armed: true }
    }

    fn take(mut self) -> Option<AttachPhase> {
        self.armed = false;
        self.phase.lock().expect("ACP phase lock poisoned").take()
    }
}

impl Drop for AttachPhaseGuard<'_> {
    fn drop(&mut self) {
        if self.armed {
            *self.phase.lock().expect("ACP phase lock poisoned") = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_core::SafeDiagnostic;
    use swallowtail_runtime::{BoxFuture, ProcessExit, ProcessInputChunk, ProcessOutputChunk};

    struct RefusingStdinProcess;

    impl ProcessHandle for RefusingStdinProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.stdin_refused",
                    "stdin refused",
                )))
            })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
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

    #[test]
    fn receive_framing_profile_covers_observed_large_tool_updates() {
        assert_eq!(
            RECEIVE_FRAMING_LIMITS.maximum_frame_bytes(),
            4 * 1024 * 1024
        );
        assert_eq!(
            RECEIVE_FRAMING_LIMITS.maximum_buffer_bytes(),
            8 * 1024 * 1024
        );
        assert!(RECEIVE_FRAMING_LIMITS.maximum_frame_bytes() > 135_553);
    }

    struct DummyResourceIo;

    impl WorkingResourceIoService for DummyResourceIo {
        fn read_text(
            &self,
            _lease: &ResourceLease,
            _request: WorkingResourceReadRequest,
        ) -> BoxFuture<'static, Result<swallowtail_runtime::WorkingResourceText, RuntimeFailure>>
        {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.unused",
                    "unused",
                )))
            })
        }

        fn write_text(
            &self,
            _lease: &ResourceLease,
            _request: swallowtail_runtime::WorkingResourceWriteRequest,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "test.unused",
                    "unused",
                )))
            })
        }
    }

    #[test]
    fn load_and_resume_session_clear_attach_phase_on_early_error() {
        let host = swallowtail_core::ExecutionHostId::new("claude-agent.phase-test")
            .expect("host id is valid");
        let lease = ResourceLease::consumer_owned(
            swallowtail_runtime::ScopeId::new("claude-agent.phase-test").expect("scope is valid"),
            swallowtail_runtime::WorkingResourceRef::new("workspace:phase-test")
                .expect("resource ref is valid"),
            swallowtail_core::ResourceAccess::Read,
            swallowtail_core::ResourceRepresentation::Filesystem,
        );
        let connection = AcpConnection::new(
            Arc::new(RefusingStdinProcess),
            lease,
            Arc::new(DummyResourceIo),
            HostServices::new(host),
        );
        let session = SessionRef::new("session-1").expect("session ref is valid");

        let load_err = futures_executor::block_on(
            connection.load_session(session.clone(), "/workspace".to_owned()),
        );
        assert!(load_err.is_err());
        assert!(
            connection
                .hooks
                .phase
                .lock()
                .expect("ACP phase lock poisoned")
                .is_none(),
            "load_session must clear hooks.phase on early error"
        );

        let resume_err =
            futures_executor::block_on(connection.resume_session(session, "/workspace".to_owned()));
        assert!(resume_err.is_err());
        assert!(
            connection
                .hooks
                .phase
                .lock()
                .expect("ACP phase lock poisoned")
                .is_none(),
            "resume_session must clear hooks.phase on early error"
        );
    }

    #[test]
    fn passive_session_info_update_validates_session_and_payload_without_admitting_other_idle_updates()
     {
        let host = swallowtail_core::ExecutionHostId::new("claude-agent.session-info-test")
            .expect("host id is valid");
        let lease = ResourceLease::consumer_owned(
            swallowtail_runtime::ScopeId::new("claude-agent.session-info-test")
                .expect("scope is valid"),
            swallowtail_runtime::WorkingResourceRef::new("workspace:session-info-test")
                .expect("resource ref is valid"),
            swallowtail_core::ResourceAccess::Read,
            swallowtail_core::ResourceRepresentation::Filesystem,
        );
        let connection = AcpConnection::new(
            Arc::new(RefusingStdinProcess),
            lease,
            Arc::new(DummyResourceIo),
            HostServices::new(host),
        );
        let valid = json!({
            "sessionId": "session-1",
            "update": {
                "sessionUpdate": "session_info_update",
                "title": "Generated title",
                "updatedAt": "2026-11-28T16:50:00Z"
            }
        });
        assert_eq!(
            connection
                .hooks
                .on_session_update(&valid)
                .expect_err("unbound session must fail closed")
                .diagnostic()
                .code(),
            "swallowtail.claude_agent.acp.session_mismatch"
        );
        connection
            .set_session_id("session-1".to_owned())
            .expect("session binds");
        assert_eq!(
            connection
                .hooks
                .on_session_update(&valid)
                .expect("bound session_info_update is passive"),
            SessionUpdateDisposition::Handled
        );
        let wrong_session = json!({
            "sessionId": "session-other",
            "update": { "sessionUpdate": "session_info_update", "title": "Other" }
        });
        assert_eq!(
            connection
                .hooks
                .on_session_update(&wrong_session)
                .expect_err("mismatched session must fail closed")
                .diagnostic()
                .code(),
            "swallowtail.claude_agent.acp.session_mismatch"
        );
        let malformed_payload = json!({
            "sessionId": "session-1",
            "update": { "sessionUpdate": "session_info_update", "title": 123 }
        });
        assert!(
            connection
                .hooks
                .on_session_update(&malformed_payload)
                .is_err(),
            "malformed session_info_update must fail closed"
        );
        let non_metadata = json!({
            "sessionId": "session-1",
            "update": {
                "sessionUpdate": "agent_message_chunk",
                "content": { "type": "text", "text": "unsolicited" }
            }
        });
        assert_eq!(
            connection
                .hooks
                .on_session_update(&non_metadata)
                .expect("non-metadata routes to active-turn gate"),
            SessionUpdateDisposition::RouteToActiveTurn
        );
    }
}
