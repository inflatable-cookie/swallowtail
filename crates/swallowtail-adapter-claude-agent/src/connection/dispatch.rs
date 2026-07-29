use super::*;

impl AcpConnection {
    pub(super) async fn dispatch(&self, message: Message) -> Result<(), RuntimeFailure> {
        match message {
            Message::Response { id, result } => self.dispatch_response(id, result),
            Message::Notification { method, params } => {
                self.dispatch_notification(&method, &params)
            }
            Message::Request { id, method, params } => {
                self.dispatch_request(id, &method, &params).await
            }
        }
    }

    fn dispatch_response(
        &self,
        id: Value,
        result: Result<Value, swallowtail_protocol_acp::RpcError>,
    ) -> Result<(), RuntimeFailure> {
        let id = id.as_u64().ok_or_else(malformed)?;
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
        let sender = self
            .pending
            .lock()
            .expect("ACP pending lock poisoned")
            .remove(&id)
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude_agent.acp.unknown_response",
                    "Claude Agent returned an unknown ACP response",
                )
            })?;
        sender.complete(result.map_err(|_| {
            failure(
                "swallowtail.claude_agent.acp.request_rejected",
                "Claude Agent rejected an ACP request",
            )
        }));
        Ok(())
    }

    fn dispatch_notification(&self, method: &str, params: &Value) -> Result<(), RuntimeFailure> {
        match method {
            "session/update" => self.dispatch_session_update(params),
            method if method.starts_with('_') => Ok(()),
            _ => Err(failure(
                "swallowtail.claude_agent.acp.notification_unsupported",
                "Claude Agent sent an unsupported ACP notification",
            )),
        }
    }

    fn dispatch_session_update(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let session_id = params
            .get("sessionId")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        let update = params.get("update").ok_or_else(malformed)?;
        let kind = update
            .get("sessionUpdate")
            .and_then(Value::as_str)
            .ok_or_else(malformed)?;
        let mut phase = self.phase.lock().expect("ACP phase lock poisoned");
        if let Some(phase) = phase.as_mut() {
            return match phase {
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
                        return passive_update(kind);
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
                    Ok(())
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
                        passive_update(kind)
                    } else {
                        Err(failure(
                            "swallowtail.claude_agent.acp.resume_replay_rejected",
                            "Claude Agent emitted historical replay while resuming",
                        ))
                    }
                }
            };
        }
        drop(phase);
        if let Some(turn) = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
        {
            turn.handle_update(params)
        } else {
            passive_update(kind)
        }
    }

    async fn dispatch_request(
        &self,
        id: Value,
        method: &str,
        params: &Value,
    ) -> Result<(), RuntimeFailure> {
        match method {
            "fs/read_text_file" => self.read_text(id, params).await,
            "session/request_permission" => self.handle_permission(id, params).await,
            method if method.starts_with('_') => {
                self.write(
                    encode_error(id, -32601, "Method not found").map_err(|_| protocol_failure())?,
                )
                .await
            }
            _ => {
                self.write(
                    encode_error(id, -32601, "Method not found").map_err(|_| protocol_failure())?,
                )
                .await?;
                Err(failure(
                    "swallowtail.claude_agent.acp.callback_unsupported",
                    "Claude Agent requested an unsupported ACP client callback",
                ))
            }
        }
    }

    async fn read_text(&self, id: Value, params: &Value) -> Result<(), RuntimeFailure> {
        self.verify_session(params)?;
        let locator = params
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(malformed)
            .and_then(|value| WorkingResourceLocator::new(value).map_err(|_| malformed()))?;
        let request = WorkingResourceReadRequest::new(
            locator,
            NonZeroUsize::new(MAXIMUM_READ_BYTES).expect("static limit is non-zero"),
        )
        .with_lines(
            optional_usize(params, "line")?,
            optional_usize(params, "limit")?,
        );
        let content = self.resource_io.read_text(&self.resource, request).await?;
        self.write(
            encode_result(id, json!({"content": content.as_driver_value()}))
                .map_err(|_| protocol_failure())?,
        )
        .await
    }

    async fn handle_permission(&self, id: Value, params: &Value) -> Result<(), RuntimeFailure> {
        self.verify_session(params)?;
        let turn = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude_agent.acp.permission_without_turn",
                    "Claude Agent requested permission without an active turn",
                )
            })?;
        if turn.exchanges_permissions() {
            return turn.exchange_permission(&id, params);
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
            .find(|option| option.get("kind").and_then(Value::as_str) == Some("reject_once"))
            .and_then(|option| option.get("optionId"))
            .and_then(Value::as_str)
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude_agent.acp.permission_rejection_unavailable",
                    "Claude Agent permission request offered no one-shot rejection",
                )
            })?;
        turn.observe_permission(&id)?;
        self.write(
            encode_result(
                id,
                json!({"outcome": {"outcome": "selected", "optionId": reject}}),
            )
            .map_err(|_| protocol_failure())?,
        )
        .await?;
        self.notify("session/cancel", json!({"sessionId": turn.session_id()}))
            .await
    }

    fn verify_session(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let current = self.session_id.lock().expect("ACP session lock poisoned");
        if params.get("sessionId").and_then(Value::as_str) == current.as_deref() {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.claude_agent.acp.session_mismatch",
                "Claude Agent callback does not match the active session",
            ))
        }
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

fn passive_update(kind: &str) -> Result<(), RuntimeFailure> {
    if is_session_scoped_metadata_update_kind(kind) {
        Ok(())
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
