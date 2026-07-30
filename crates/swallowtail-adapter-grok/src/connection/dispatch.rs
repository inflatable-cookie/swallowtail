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
                    "swallowtail.grok.acp.unknown_response",
                    "Grok Build returned an unknown ACP response",
                )
            })?;
        sender.complete(result.map_err(|_| {
            failure(
                "swallowtail.grok.acp.request_rejected",
                "Grok Build rejected an ACP request",
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
                    None if is_session_scoped_metadata_update(params) => Ok(()),
                    None => Err(failure(
                        "swallowtail.grok.acp.update_without_turn",
                        "Grok Build updated a session without an active turn",
                    )),
                }
            }
            method if method.starts_with('_') => Ok(()),
            _ => Err(failure(
                "swallowtail.grok.acp.notification_unsupported",
                "Grok Build sent an unsupported ACP notification",
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
                    "swallowtail.grok.acp.callback_unsupported",
                    "Grok Build requested an unsupported ACP client callback",
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
        let turn = self
            .active_turn
            .lock()
            .expect("ACP active lock poisoned")
            .clone()
            .ok_or_else(|| {
                failure(
                    "swallowtail.grok.acp.permission_without_turn",
                    "Grok Build requested permission without an active turn",
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
                "swallowtail.grok.acp.session_mismatch",
                "Grok Build callback does not match the active session",
            ))
        }
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
