async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    inputs: PumpInputs,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut output = String::new();
    let mut state = StreamState::Start;
    let mut search_uses = 0_u32;
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
                Ok(event) => match apply_event(
                    event,
                    &mut state,
                    &mut output,
                    inputs.search_allowed,
                    &mut search_uses,
                ) {
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
                    Ok(Applied::SearchProgress) => {
                        if let Err(error) =
                            emit(&events, &mut sequence, RuntimeEventKind::ExternalSearchProgress)
                        {
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
    let attachment = inputs.attachment.release().await;
    let credential = access.release(&services).await;
    let cleanup = merge_cleanup(merge_cleanup(connection, attachment), credential);
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed) && !output.is_empty() {
        outcome = outcome.with_output(OperationContent::new(output).expect("output is non-empty"));
    }
    outcome
}

struct PumpInputs {
    attachment: input::SharedAttachment,
    search_allowed: bool,
}

enum StreamState {
    Start,
    Message,
    TextContent,
    SearchContent,
    AfterContent,
    Delta,
    Complete,
}

enum Applied {
    None,
    Usage(TokenUsage),
    Delta(String),
    SearchProgress,
    Complete,
}

fn apply_event(
    event: Event,
    state: &mut StreamState,
    output: &mut String,
    search_allowed: bool,
    search_uses: &mut u32,
) -> Result<Applied, RuntimeFailure> {
    match (event, &*state) {
        (Event::MessageStart(usage), StreamState::Start) => {
            *state = StreamState::Message;
            Ok(Applied::Usage(usage))
        }
        (Event::ContentStart(crate::protocol::ContentBlock::Text), StreamState::Message | StreamState::AfterContent) => {
            *state = StreamState::TextContent;
            Ok(Applied::None)
        }
        (Event::ContentStart(crate::protocol::ContentBlock::SearchUse),
            StreamState::Message | StreamState::AfterContent,
        ) if search_allowed && *search_uses < 2 => {
            *search_uses += 1;
            *state = StreamState::SearchContent;
            Ok(Applied::SearchProgress)
        }
        (Event::ContentStart(crate::protocol::ContentBlock::SearchResult),
            StreamState::Message | StreamState::AfterContent,
        ) if search_allowed && *search_uses > 0 => {
            *state = StreamState::SearchContent;
            Ok(Applied::SearchProgress)
        }
        (Event::OutputDelta(delta), StreamState::TextContent) if !delta.is_empty() => {
            output.push_str(&delta);
            Ok(Applied::Delta(delta))
        }
        (Event::Citation, StreamState::TextContent) if search_allowed => {
            Ok(Applied::SearchProgress)
        }
        (Event::InputJsonDelta(_), StreamState::SearchContent) if search_allowed => {
            Ok(Applied::None)
        }
        (Event::ContentStop, StreamState::TextContent | StreamState::SearchContent) => {
            *state = StreamState::AfterContent;
            Ok(Applied::None)
        }
        (Event::Usage(usage, _), StreamState::AfterContent | StreamState::Delta) => {
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
