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
            "session/update" => {
                let active = self
                    .active_turn
                    .lock()
                    .expect("ACP active lock poisoned")
                    .clone();
                match active {
                    Some(active) => active.handle_update(params),
                    None if session_update_kind(params) == Some("available_commands_update") => {
                        Ok(())
                    }
                    None => Err(failure(
                        "swallowtail.claude_agent.acp.update_without_turn",
                        "Claude Agent updated a session without an active turn",
                    )),
                }
            }
            method if method.starts_with('_') => Ok(()),
            _ => Err(failure(
                "swallowtail.claude_agent.acp.notification_unsupported",
                "Claude Agent sent an unsupported ACP notification",
            )),
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
            "session/request_permission" => self.reject_permission(id, params).await,
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

    async fn reject_permission(&self, id: Value, params: &Value) -> Result<(), RuntimeFailure> {
        self.verify_session(params)?;
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

fn session_update_kind(params: &Value) -> Option<&str> {
    params
        .get("update")?
        .get("sessionUpdate")
        .and_then(Value::as_str)
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
