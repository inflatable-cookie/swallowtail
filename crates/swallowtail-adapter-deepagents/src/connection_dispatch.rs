impl AcpConnection {
    async fn dispatch(&self, message: Message) -> Result<(), RuntimeFailure> {
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
                    "swallowtail.deepagents.acp.unknown_response",
                    "Deep Agents returned an unknown ACP response",
                )
            })?;
        sender.complete(result.map_err(|error| {
            if error.message().contains("not authenticated")
                || error.message().contains("sign in")
                || error.message().contains("API key")
            {
                failure(
                    "swallowtail.deepagents.acp.host_auth_required",
                    "Deep Agents ACP requires host-owned login or API key before session work",
                )
            } else {
                failure(
                    "swallowtail.deepagents.acp.request_rejected",
                    "Deep Agents rejected an ACP request",
                )
            }
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
                    None if is_session_scoped_metadata_update(params) => Ok(()),
                    None => Err(failure(
                        "swallowtail.deepagents.acp.update_without_turn",
                        "Deep Agents updated a session without an active turn",
                    )),
                }
            }
            "session/cancel" => Err(failure(
                "swallowtail.deepagents.acp.agent_cancel_unsupported",
                "Deep Agents sent an unsupported client cancellation notification",
            )),
            method if method.starts_with('_') => Ok(()),
            _ => Err(failure(
                "swallowtail.deepagents.acp.notification_unsupported",
                "Deep Agents sent an unsupported ACP notification",
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
                    "swallowtail.deepagents.acp.callback_unsupported",
                    "Deep Agents requested an unsupported ACP client callback",
                ))
            }
        }
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
        let turn = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.deepagents.acp.permission_without_turn",
                    "Deep Agents requested permission without an active turn",
                )
            })?;
        turn.observe_permission(&id)?;
        self.notify("session/cancel", json!({"sessionId": turn.session_id()}))
            .await?;
        self.write(
            encode_result(id, json!({"outcome": {"outcome": "cancelled"}}))
                .map_err(|_| protocol_failure())?,
        )
        .await
    }

    fn verify_session(&self, params: &Value) -> Result<(), RuntimeFailure> {
        let current = self.session_id.lock().expect("ACP session lock poisoned");
        if params.get("sessionId").and_then(Value::as_str) == current.as_deref() {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.deepagents.acp.session_mismatch",
                "Deep Agents callback does not match the active session",
            ))
        }
    }
}
