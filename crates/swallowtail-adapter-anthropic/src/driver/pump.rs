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
                "swallowtail.anthropic.stream_disconnected",
                "Anthropic stream closed before message completion",
            )),
            RunSignal::Item(Err(_)) if cancellation.is_requested() => {
                break TerminalStatus::Cancelled;
            }
            RunSignal::Item(Err(error)) => break provider_status(error),
            RunSignal::Item(Ok(StreamItem::Headers(headers))) => {
                if let Err(error) = emit_headers(&events, &mut sequence, &headers) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            RunSignal::Item(Ok(StreamItem::Frame(frame))) => match parse_event(&frame) {
                Err(error) => break provider_status(error),
                Ok(Event::Unknown) => {}
                Ok(Event::Ping) => {
                    if let Err(error) = emit(&events, &mut sequence, RuntimeEventKind::Keepalive) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                }
                Ok(event) => match apply_event(event, &mut state, &mut output) {
                    Ok(Applied::None) => {}
                    Ok(Applied::Usage(usage)) => {
                        let kind = RuntimeEventKind::ProviderObservation(
                            ProviderObservation::Usage(usage),
                        );
                        if let Err(error) = emit(&events, &mut sequence, kind) {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
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
                    Ok(Applied::Complete) => {
                        if !output.is_empty() {
                            let content = OperationContent::new(output.clone())
                                .expect("output is non-empty");
                            let event = RuntimeEvent::with_content(
                                sequence,
                                RuntimeEventKind::OutputAvailable,
                                content,
                            );
                            if let Err(error) = events.send(event) {
                                break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                            }
                        }
                        break TerminalStatus::Completed;
                    }
                    Err(error) => break provider_status(error),
                },
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

enum StreamState {
    Start,
    Message,
    Content,
    AfterContent,
    Delta,
    Complete,
}

enum Applied {
    None,
    Usage(TokenUsage),
    Delta(String),
    Complete,
}

fn apply_event(
    event: Event,
    state: &mut StreamState,
    output: &mut String,
) -> Result<Applied, RuntimeFailure> {
    match (event, &*state) {
        (Event::MessageStart(usage), StreamState::Start) => {
            *state = StreamState::Message;
            Ok(Applied::Usage(usage))
        }
        (Event::ContentStart, StreamState::Message) => {
            *state = StreamState::Content;
            Ok(Applied::None)
        }
        (Event::OutputDelta(delta), StreamState::Content) if !delta.is_empty() => {
            output.push_str(&delta);
            Ok(Applied::Delta(delta))
        }
        (Event::ContentStop, StreamState::Content) => {
            *state = StreamState::AfterContent;
            Ok(Applied::None)
        }
        (Event::Usage(usage), StreamState::AfterContent | StreamState::Delta) => {
            *state = StreamState::Delta;
            Ok(Applied::Usage(usage))
        }
        (Event::MessageStop, StreamState::Delta) => {
            *state = StreamState::Complete;
            Ok(Applied::Complete)
        }
        (Event::ProviderFailed(kind), _) => Err(provider_failure(kind, "message stream")),
        _ => Err(failure(
            "swallowtail.anthropic.stream_order_invalid",
            "Anthropic stream event order was invalid",
        )),
    }
}
