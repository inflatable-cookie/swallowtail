enum EventAction {
    Continue,
    Callbacks(Vec<String>),
    Terminal(ManagedFinal),
}

#[allow(clippy::too_many_arguments)]
fn apply_event(
    event: &ManagedEvent,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    run_id: &RuntimeRunId,
    callbacks: &ManagedCallbackHub,
    processed: &mut BTreeSet<String>,
    pending_tools: &mut BTreeSet<String>,
    declared_tools: &BTreeSet<String>,
    output: &mut String,
    deadline: Deadline,
) -> Result<EventAction, RuntimeFailure> {
    if !processed.insert(event.id().to_owned()) {
        return Ok(EventAction::Continue);
    }
    match event.kind() {
        ManagedEventKind::Running
        | ManagedEventKind::Rescheduled
        | ManagedEventKind::Observed => Ok(EventAction::Continue),
        ManagedEventKind::Message(content) => {
            output.push_str(content.as_str());
            emit_content(
                events,
                sequence,
                RuntimeEventKind::OutputAvailable,
                content.clone(),
            )?;
            Ok(EventAction::Continue)
        }
        ManagedEventKind::CustomToolUse { name, input } => {
            if !declared_tools.contains(name) {
                return Err(failure(
                    "swallowtail.anthropic.managed.callback_tool_undeclared",
                    "Anthropic Managed Agents requested an undeclared custom tool",
                ));
            }
            let arguments = serde_json::to_vec(input).map_err(|_| {
                failure(
                    "swallowtail.anthropic.managed.callback_arguments_invalid",
                    "Anthropic Managed Agents custom tool arguments were invalid",
                )
            })?;
            let arguments = CallbackPayload::new(arguments, 64 * 1024).map_err(|_| {
                failure(
                    "swallowtail.anthropic.managed.callback_arguments_too_large",
                    "Anthropic Managed Agents custom tool arguments exceeded the adapter limit",
                )
            })?;
            let callback_id = CallbackId::new(format!("anthropic-managed-callback-{sequence}"))
                .map_err(|_| binding_failure("callback identity"))?;
            let provider_reference = ProviderRequestRef::new(event.id())
                .map_err(|_| binding_failure("provider callback identity"))?;
            let callback = CallbackRequest::run_tool_call(
                callback_id.clone(),
                run_id.clone(),
                *sequence,
                Some(deadline),
                name,
                arguments,
            )
            .map_err(|_| binding_failure("callback request"))?
            .with_provider_request_ref(provider_reference);
            callbacks.enqueue(event.id().to_owned(), callback)?;
            if !pending_tools.insert(event.id().to_owned()) {
                return Err(failure(
                    "swallowtail.anthropic.managed.callback_identity_reused",
                    "Anthropic Managed Agents reused a custom tool event identity",
                ));
            }
            emit(
                events,
                sequence,
                RuntimeEventKind::CallbackRequested(callback_id),
            )?;
            Ok(EventAction::Continue)
        }
        ManagedEventKind::Idle(IdleReason::RequiresAction(ids)) => {
            let required = ids.iter().cloned().collect::<BTreeSet<_>>();
            if &required != pending_tools {
                return Err(failure(
                    "swallowtail.anthropic.managed.callback_set_mismatch",
                    "Anthropic Managed Agents requires-action callbacks did not match pending tools",
                ));
            }
            Ok(EventAction::Callbacks(ids.clone()))
        }
        ManagedEventKind::Idle(IdleReason::EndTurn) => {
            if !pending_tools.is_empty() {
                return Err(failure(
                    "swallowtail.anthropic.managed.callback_abandoned",
                    "Anthropic Managed Agents ended while custom tools remained pending",
                ));
            }
            let mut state = ManagedFinal::status(TerminalStatus::Completed);
            state.output = OperationContent::new(output.clone()).ok();
            Ok(EventAction::Terminal(state))
        }
        ManagedEventKind::Idle(IdleReason::RetriesExhausted)
        | ManagedEventKind::ProviderError
        | ManagedEventKind::Terminated => Ok(EventAction::Terminal(ManagedFinal::status(
            provider_status(failure(
                "swallowtail.anthropic.managed.provider_failed",
                "Anthropic Managed Agents reported terminal provider failure",
            )),
        ))),
    }
}

enum CallbackSignal {
    Responses(Vec<(String, CallbackResult)>),
    Cancelled,
    Deadline,
    Failed(RuntimeFailure),
}

async fn await_callback_results(
    callbacks: &ManagedCallbackHub,
    ids: &[String],
    cancellation: &ManagedRunCancellation,
    deadline: &mut Option<
        swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    >,
) -> CallbackSignal {
    let mut responses = Box::pin(callbacks.wait_for(ids));
    std::future::poll_fn(|context| {
        if cancellation.is_requested() {
            return Poll::Ready(CallbackSignal::Cancelled);
        }
        if let Poll::Ready(result) = responses.as_mut().poll(context) {
            return Poll::Ready(match result {
                Ok(responses) => CallbackSignal::Responses(responses),
                Err(_) if cancellation.is_requested() => CallbackSignal::Cancelled,
                Err(error) => CallbackSignal::Failed(error),
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(CallbackSignal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn callback_content(result: CallbackResult) -> Result<OperationContent, RuntimeFailure> {
    match result {
        CallbackResult::Success(payload) => std::str::from_utf8(payload.as_bytes())
            .ok()
            .and_then(|text| OperationContent::new(text).ok())
            .ok_or_else(|| {
                failure(
                    "swallowtail.anthropic.managed.callback_result_invalid",
                    "Anthropic Managed Agents callback result was not valid bounded text",
                )
            }),
        CallbackResult::Failure { kind, .. } => Err(failure(
            "swallowtail.anthropic.managed.callback_consumer_failed",
            match kind {
                CallbackFailureKind::UnknownDeclaration => "Consumer rejected an unknown tool",
                CallbackFailureKind::Unsupported => "Consumer rejected an unsupported callback",
                CallbackFailureKind::ConsumerFailed => "Consumer tool execution failed",
                CallbackFailureKind::Cancelled => "Consumer tool execution was cancelled",
                CallbackFailureKind::TimedOut => "Consumer tool execution timed out",
            },
        )),
    }
}
