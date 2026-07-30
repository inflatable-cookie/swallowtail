impl ActiveTurn {
    pub(crate) fn handle_tool_call(
        &self,
        provider_request_id: Value,
        params: &Value,
        callback_id: CallbackId,
    ) -> Result<(), RuntimeFailure> {
        self.verify_turn(params)?;
        if params
            .get("namespace")
            .is_some_and(|value| !value.is_null())
        {
            return Err(failure(
                "swallowtail.codex.app_server.callback_namespace_unsupported",
                "Codex namespaced dynamic tool callbacks are unsupported",
            ));
        }
        let provider_call_id = required_text(params, "callId")?.to_owned();
        let tool_name = required_text(params, "tool")?;
        if !self.declared_tools.contains(tool_name) {
            return Err(failure(
                "swallowtail.codex.app_server.callback_tool_undeclared",
                "Codex app-server requested an undeclared dynamic tool",
            ));
        }
        let arguments = params.get("arguments").ok_or_else(malformed_notification)?;
        let argument_bytes = serde_json::to_vec(arguments).map_err(|_| malformed_notification())?;
        let payload =
            CallbackPayload::new(argument_bytes, MAX_CALLBACK_ARGUMENT_BYTES).map_err(|_| {
                failure(
                    "swallowtail.codex.app_server.callback_arguments_too_large",
                    "Codex dynamic tool arguments exceeded the adapter limit",
                )
            })?;
        let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
        let request = CallbackRequest::tool_call(
            callback_id.clone(),
            self.runtime_id.clone(),
            sequence,
            self.deadline,
            tool_name,
            payload,
        )
        .map_err(|_| malformed_notification())?;
        let provider_request_ref =
            ProviderRequestRef::new(provider_request_value(&provider_request_id))
                .map_err(|_| malformed_notification())?;
        self.callbacks
            .enqueue_tool(request, provider_request_id, provider_call_id.clone())?;
        let request_activity = {
            let mut activity = self.activity.lock().expect("activity lock poisoned");
            activity.register_callback(&provider_call_id, callback_id.clone());
            activity.provider_request_started(
                provider_request_ref,
                Some(&provider_call_id),
                "dynamicTool",
            )?
        };
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id),
        ))?;
        self.emit(RuntimeEventKind::Activity(request_activity), None)
    }

    pub(crate) fn handle_provider_request(
        &self,
        provider_request_id: &Value,
        method: &str,
        params: &Value,
        callback_id: CallbackId,
    ) -> Result<ProviderRequestDisposition, RuntimeFailure> {
        let namespace = provider_request_namespace(method).ok_or_else(|| {
            failure(
                "swallowtail.codex.app_server.callback_unsupported",
                "Codex app-server requested an unsupported client callback",
            )
        })?;
        let handling = self.provider_requests.handling_for(&namespace);
        if handling == ProviderRequestHandling::Reject {
            return Err(failure(
                "swallowtail.codex.app_server.callback_unsupported",
                "Codex app-server requested an undeclared provider callback",
            ));
        }
        self.verify_provider_request(params)?;
        let provider_request_ref =
            ProviderRequestRef::new(provider_request_value(provider_request_id))
                .map_err(|_| malformed_notification())?;
        if handling == ProviderRequestHandling::Exchange {
            if namespace != crate::session_access::codex_user_input_request_extension() {
                return Err(failure(
                    "swallowtail.codex.app_server.callback_unsupported",
                    "Codex provider callback is not qualified for exchange",
                ));
            }
            let user_input = crate::user_input::request(params)?;
            let sequence = self.sequence.fetch_add(1, Ordering::SeqCst);
            let request = CallbackRequest::harness_user_input(
                callback_id.clone(),
                self.runtime_id.clone(),
                sequence,
                self.deadline,
                user_input.clone(),
            )
            .with_provider_request_ref(provider_request_ref.clone());
            let provider_call_id = required_text(params, "itemId")?.to_owned();
            self.callbacks.enqueue_user_input(
                request,
                provider_request_id.clone(),
                provider_call_id.clone(),
                user_input,
            )?;
            let request_activity = {
                let mut activity = self.activity.lock().expect("activity lock poisoned");
                activity.register_callback(&provider_call_id, callback_id.clone());
                activity.provider_request_started(
                    provider_request_ref,
                    Some(&provider_call_id),
                    namespace.as_str(),
                )?
            };
            self.events.send(RuntimeEvent::new(
                sequence,
                RuntimeEventKind::CallbackRequested(callback_id),
            ))?;
            self.emit(RuntimeEventKind::Activity(request_activity), None)?;
            return Ok(ProviderRequestDisposition::Exchange);
        }
        let payload = serde_json::to_vec(params).map_err(|_| malformed_notification())?;
        let request = CallbackRequest::extension(
            callback_id.clone(),
            self.runtime_id.clone(),
            self.sequence.fetch_add(1, Ordering::SeqCst),
            self.deadline,
            ProviderExtension::new(namespace.clone(), payload),
            MAX_CALLBACK_ARGUMENT_BYTES,
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.app_server.callback_arguments_too_large",
                "Codex provider request exceeded the adapter limit",
            )
        })?
        .with_provider_request_ref(provider_request_ref.clone());
        let sequence = request.event_sequence();
        self.callbacks.observe_and_close(request)?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id.clone()),
        ))?;
        let request_activity = self
            .activity
            .lock()
            .expect("activity lock poisoned")
            .provider_request_started(
                provider_request_ref.clone(),
                params.get("itemId").and_then(Value::as_str),
                namespace.as_str(),
            )?;
        self.emit(RuntimeEventKind::Activity(request_activity), None)?;
        Ok(ProviderRequestDisposition::Observed(
            ProviderRequestObservation::new(callback_id, namespace, provider_request_ref),
        ))
    }

}
