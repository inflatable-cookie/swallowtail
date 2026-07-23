use crate::protocol::NativeEvent;

async fn pump_run(
    mut subscription: Subscription,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut output = String::new();
    let mut state = StreamState::Streaming;
    let status = loop {
        match next_run_signal(&mut subscription, &mut deadline).await {
            RunSignal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            RunSignal::Closed if cancellation.is_requested() => break TerminalStatus::Cancelled,
            RunSignal::Closed => {
                break provider_status(failure(
                    "swallowtail.ollama.stream_disconnected",
                    "Ollama stream closed before completion",
                ));
            }
            RunSignal::Item(Err(_)) if cancellation.is_requested() => {
                break TerminalStatus::Cancelled;
            }
            RunSignal::Item(Err(error)) => break provider_status(error),
            RunSignal::Item(Ok(event)) => match apply_event(event, &mut state, &mut output) {
                Ok(Applied::Usage(usage)) => {
                    let kind =
                        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage));
                    if let Err(error) = emit(&events, &mut sequence, kind) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    if output.is_empty() {
                        break provider_status(failure(
                            "swallowtail.ollama.output_missing",
                            "Ollama completed without text output",
                        ));
                    }
                    let content =
                        OperationContent::new(output.clone()).expect("output is non-empty");
                    let event = RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        content,
                    );
                    if let Err(error) = events.send(event) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    break TerminalStatus::Completed;
                }
                Ok(Applied::Delta(delta)) => {
                    let content = OperationContent::new(delta).expect("delta is non-empty");
                    let event = RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputDelta,
                        content,
                    );
                    sequence += 1;
                    if let Err(error) = events.send(event) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                }
                Ok(Applied::None) => {}
                Err(error) => break provider_status(error),
            },
        }
    };
    let cleanup = match subscription.close().await {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    };
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed) && !output.is_empty() {
        outcome = outcome.with_output(OperationContent::new(output).expect("output is non-empty"));
    }
    outcome
}

enum StreamState {
    Streaming,
    Finished,
}

enum Applied {
    None,
    Usage(TokenUsage),
    Delta(String),
}

fn apply_event(
    event: NativeEvent,
    state: &mut StreamState,
    output: &mut String,
) -> Result<Applied, RuntimeFailure> {
    match (event, &*state) {
        (NativeEvent::OutputDelta(delta), StreamState::Streaming) if !delta.is_empty() => {
            output.push_str(&delta);
            Ok(Applied::Delta(delta))
        }
        (NativeEvent::Finished(reason), StreamState::Streaming)
            if matches!(reason.as_str(), "stop" | "length") =>
        {
            *state = StreamState::Finished;
            Ok(Applied::None)
        }
        (NativeEvent::Usage(usage), StreamState::Finished) => Ok(Applied::Usage(usage)),
        (NativeEvent::ProviderFailed, _) => Err(failure(
            "swallowtail.ollama.stream_failed",
            "Ollama reported a stream failure",
        )),
        _ => Err(failure(
            "swallowtail.ollama.stream_order_invalid",
            "Ollama stream event order was invalid",
        )),
    }
}

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

fn provider_status(error: RuntimeFailure) -> TerminalStatus {
    TerminalStatus::ProviderFailed(error.diagnostic().clone())
}
