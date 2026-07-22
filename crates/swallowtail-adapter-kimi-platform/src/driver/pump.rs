async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut output = String::new();
    let mut state = StreamState::Start;
    let status = loop {
        match next_run_signal(&mut subscription, &mut deadline).await {
            RunSignal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            RunSignal::Closed if cancellation.is_requested() => break TerminalStatus::Cancelled,
            RunSignal::Closed => break provider_status(failure(
                "swallowtail.kimi_platform.stream_disconnected",
                "Kimi Platform stream closed before completion",
            )),
            RunSignal::Item(Err(_)) if cancellation.is_requested() => break TerminalStatus::Cancelled,
            RunSignal::Item(Err(error)) => break provider_status(error),
            RunSignal::Item(Ok(StreamItem::Frame(frame))) => match parse_events(&frame, MODEL_ID) {
                Err(error) => break provider_status(error),
                Ok(parsed) => {
                    let mut terminal = None;
                    for event in parsed {
                        match apply_event(event, &mut state, &mut output) {
                            Ok(Applied::None) => {}
                            Ok(Applied::Usage(usage)) => {
                                let kind = RuntimeEventKind::ProviderObservation(
                                    ProviderObservation::Usage(usage),
                                );
                                if let Err(error) = emit(&events, &mut sequence, kind) {
                                    terminal = Some(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
                                    break;
                                }
                            }
                            Ok(Applied::Reasoning(delta)) => {
                                if let Err(error) = emit_content(
                                    &events,
                                    &mut sequence,
                                    RuntimeEventKind::ReasoningProgress,
                                    delta,
                                ) {
                                    terminal = Some(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
                                    break;
                                }
                            }
                            Ok(Applied::Output(delta)) => {
                                if let Err(error) = emit_content(
                                    &events,
                                    &mut sequence,
                                    RuntimeEventKind::OutputDelta,
                                    delta,
                                ) {
                                    terminal = Some(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
                                    break;
                                }
                            }
                            Ok(Applied::Complete) => {
                                if let Err(error) = emit_output(&events, &mut sequence, &output) {
                                    terminal = Some(TerminalStatus::RuntimeFailed(error.diagnostic().clone()));
                                } else {
                                    terminal = Some(TerminalStatus::Completed);
                                }
                                break;
                            }
                            Err(error) => {
                                terminal = Some(provider_status(error));
                                break;
                            }
                        }
                    }
                    if let Some(terminal) = terminal { break terminal; }
                }
            },
        }
    };
    let connection = cleanup_result(subscription.close().await);
    let credential = access.release(&services).await;
    let cleanup = merge_cleanup(connection, credential);
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed) && !output.is_empty() {
        outcome = outcome.with_output(OperationContent::new(output).expect("output is non-empty"));
    }
    outcome
}

#[derive(Clone, Copy)]
enum StreamState { Start, Role, Reasoning, Output, Finished, Usage, Complete }

enum Applied { None, Reasoning(String), Output(String), Usage(TokenUsage), Complete }

fn apply_event(
    event: Event,
    state: &mut StreamState,
    output: &mut String,
) -> Result<Applied, RuntimeFailure> {
    match (event, *state) {
        (Event::RoleStart, StreamState::Start) => { *state = StreamState::Role; Ok(Applied::None) }
        (Event::ReasoningDelta(delta), StreamState::Start | StreamState::Role | StreamState::Reasoning) if !delta.is_empty() => {
            *state = StreamState::Reasoning;
            Ok(Applied::Reasoning(delta))
        }
        (Event::OutputDelta(delta), StreamState::Reasoning | StreamState::Output) if !delta.is_empty() => {
            *state = StreamState::Output;
            output.push_str(&delta);
            Ok(Applied::Output(delta))
        }
        (Event::Finished(_), StreamState::Output) => { *state = StreamState::Finished; Ok(Applied::None) }
        (Event::Usage(usage), StreamState::Finished) => { *state = StreamState::Usage; Ok(Applied::Usage(usage)) }
        (Event::Done, StreamState::Usage) => { *state = StreamState::Complete; Ok(Applied::Complete) }
        (Event::ProviderFailed(kind), _) => Err(provider_failure(kind, "chat stream")),
        _ => Err(failure(
            "swallowtail.kimi_platform.stream_order_invalid",
            "Kimi Platform stream event order was invalid",
        )),
    }
}

fn emit_content(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: String,
) -> Result<(), RuntimeFailure> {
    let content = OperationContent::new(content).expect("delta is non-empty");
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}

fn emit_output(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    output: &str,
) -> Result<(), RuntimeFailure> {
    if output.is_empty() {
        return Err(failure(
            "swallowtail.kimi_platform.output_missing",
            "Kimi Platform completed without bounded text output",
        ));
    }
    emit_content(events, sequence, RuntimeEventKind::OutputAvailable, output.to_owned())
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

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}
