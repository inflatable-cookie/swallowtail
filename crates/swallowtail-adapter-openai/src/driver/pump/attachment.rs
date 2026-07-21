#[allow(clippy::too_many_arguments)]
async fn pump_attachment(
    subscription: &mut Subscription,
    stream: &mut BackgroundStream,
    output: &mut String,
    output_done: &mut Option<String>,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    cancellation: &RunCancellation,
    deadline: &mut Option<
        swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    >,
) -> AttachmentExit {
    loop {
        if cancellation.is_requested() {
            return AttachmentExit::Cancelled;
        }
        match next_run_signal(subscription, deadline).await {
            RunSignal::Deadline => return AttachmentExit::Deadline,
            RunSignal::Closed => return AttachmentExit::Disconnected,
            RunSignal::Item(Err(_)) if cancellation.is_requested() => {
                return AttachmentExit::Cancelled;
            }
            RunSignal::Item(Err(error)) => {
                return if matches!(
                    error.diagnostic().code(),
                    "swallowtail.openai.transport_failed"
                        | "swallowtail.openai.sse_disconnected"
                        | "swallowtail.openai.request_cancelled"
                ) {
                    AttachmentExit::Disconnected
                } else {
                    AttachmentExit::Terminal(FinalState::new(provider_status(error)))
                };
            }
            RunSignal::Item(Ok(StreamItem::Headers(headers))) => {
                if let Err(error) = emit_headers(events, sequence, &headers) {
                    return AttachmentExit::Terminal(FinalState::new(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    ));
                }
            }
            RunSignal::Item(Ok(StreamItem::Frame(frame))) => {
                let event = match stream.apply(&frame) {
                    Ok(event) => event,
                    Err(error) => {
                        return AttachmentExit::Terminal(FinalState::new(provider_status(error)));
                    }
                };
                match event {
                    ProviderEvent::Created(_) => {
                        return AttachmentExit::Terminal(FinalState::new(provider_status(
                            failure(
                                "swallowtail.openai.created_repeated",
                                "OpenAI repeated the response identity event",
                            ),
                        )));
                    }
                    ProviderEvent::Status(_) => {
                        if let Err(error) = emit(events, sequence, RuntimeEventKind::Progress) {
                            return AttachmentExit::Terminal(FinalState::new(
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            ));
                        }
                    }
                    ProviderEvent::OutputDelta(delta) => {
                        output.push_str(&delta);
                        if let Err(error) = emit_content(
                            events,
                            sequence,
                            RuntimeEventKind::OutputDelta,
                            delta,
                        ) {
                            return AttachmentExit::Terminal(FinalState::new(
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            ));
                        }
                    }
                    ProviderEvent::OutputDone(done) => *output_done = Some(done),
                    ProviderEvent::Terminal(snapshot) => {
                        return AttachmentExit::Terminal(stream_terminal(
                            snapshot,
                            output,
                            output_done.as_deref(),
                        ));
                    }
                    ProviderEvent::Error => {
                        return AttachmentExit::Terminal(FinalState::new(provider_status(
                            failure(
                                "swallowtail.openai.stream_provider_failed",
                                "OpenAI reported a provider stream failure",
                            ),
                        )));
                    }
                }
            }
        }
    }
}

fn stream_terminal(
    snapshot: ResponseSnapshot,
    output: &str,
    output_done: Option<&str>,
) -> FinalState {
    match snapshot.status {
        BackgroundStatus::Completed
            if snapshot.output_text.as_deref() == Some(output)
                && output_done == Some(output) =>
        {
            let mut state = FinalState::new(TerminalStatus::Completed);
            state.output = snapshot.output_text;
            state.usage = snapshot.usage;
            state
        }
        BackgroundStatus::Completed => FinalState::new(provider_status(failure(
            "swallowtail.openai.output_mismatch",
            "OpenAI completed output did not match ordered stream output",
        ))),
        BackgroundStatus::Cancelled => {
            let mut state = FinalState::new(TerminalStatus::Cancelled);
            state.cancellation = Some(ProviderCancellationOutcome::Confirmed);
            state.usage = snapshot.usage;
            state
        }
        BackgroundStatus::Incomplete => FinalState::new(provider_status(failure(
            "swallowtail.openai.response_incomplete",
            "OpenAI background response was incomplete",
        ))),
        BackgroundStatus::Failed => FinalState::new(provider_status(failure(
            "swallowtail.openai.response_failed",
            "OpenAI background response failed",
        ))),
        BackgroundStatus::Queued | BackgroundStatus::InProgress => FinalState::new(
            provider_status(failure(
                "swallowtail.openai.terminal_status_invalid",
                "OpenAI terminal event carried a non-terminal status",
            )),
        ),
    }
}
