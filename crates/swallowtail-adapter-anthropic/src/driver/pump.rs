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
    let mut message_id = None;
    let mut search_id = None;
    let activity = crate::activity::AnthropicActivityProjection::new(
        inputs.activity_operation_id.clone(),
    );
    let status = loop {
        match next_run_signal(&mut subscription, &mut deadline).await {
            RunSignal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            RunSignal::Closed if cancellation.is_requested() => break TerminalStatus::Cancelled,
            RunSignal::Closed => {
                let error = failure(
                    "swallowtail.anthropic.stream_disconnected",
                    "Anthropic stream closed before message completion",
                );
                emit_wire_debug(&services, &error, "http.pump.transport");
                break provider_status(error);
            }
            RunSignal::Item(Err(_)) if cancellation.is_requested() => {
                break TerminalStatus::Cancelled;
            }
            RunSignal::Item(Err(error)) => {
                emit_wire_debug(&services, &error, "http.pump.transport");
                break provider_status(error);
            }
            RunSignal::Item(Ok(StreamItem::Headers(headers))) => {
                if let Err(error) = emit_headers(&events, &mut sequence, &headers) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            RunSignal::Item(Ok(StreamItem::Frame(frame))) => match parse_event(&frame) {
                Err(error) => {
                    emit_protocol_debug(&services, &error, "http.pump.decode");
                    break provider_status(error);
                }
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
                    &mut message_id,
                    &mut search_id,
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
                    Ok(Applied::AssistantStarted(message_id)) => {
                        let observation = activity.assistant_started(
                            crate::activity::structured_assistant_id(),
                            &message_id,
                            swallowtail_runtime::ActivityAssistantPhase::Final,
                        );
                        if let Err(error) = observation.and_then(|observation| {
                            emit(
                                &events,
                                &mut sequence,
                                RuntimeEventKind::Activity(observation),
                            )
                        }) {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
                    }
                    Ok(Applied::Delta { message_id, delta }) => {
                        let content = OperationContent::new(delta).expect("delta is non-empty");
                        let observation = activity.assistant_delta(
                            crate::activity::structured_assistant_id(),
                            &message_id,
                            swallowtail_runtime::ActivityAssistantPhase::Final,
                            content.as_str(),
                        );
                        if let Err(error) = observation.and_then(|observation| {
                            emit(
                                &events,
                                &mut sequence,
                                RuntimeEventKind::Activity(observation),
                            )
                        }) {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
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
                    Ok(Applied::SearchStarted(id)) => {
                        let observation = activity.provider_tool(
                            &id,
                            swallowtail_runtime::ActivityLifecyclePhase::Started,
                            swallowtail_runtime::ActivityStatus::Pending,
                        );
                        if let Err(error) = observation.and_then(|observation| {
                            emit(
                                &events,
                                &mut sequence,
                                RuntimeEventKind::Activity(observation),
                            )
                        }) {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
                        if let Err(error) =
                            emit(&events, &mut sequence, RuntimeEventKind::ExternalSearchProgress)
                        {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
                    }
                    Ok(Applied::SearchUpdated(id)) => {
                        let observation = activity.provider_tool(
                            &id,
                            swallowtail_runtime::ActivityLifecyclePhase::Updated,
                            swallowtail_runtime::ActivityStatus::InProgress,
                        );
                        if let Err(error) = observation.and_then(|observation| {
                            emit(
                                &events,
                                &mut sequence,
                                RuntimeEventKind::Activity(observation),
                            )
                        }) {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
                    }
                    Ok(Applied::SearchCompleted(id)) => {
                        let observation = activity.provider_tool(
                            &id,
                            swallowtail_runtime::ActivityLifecyclePhase::Completed,
                            swallowtail_runtime::ActivityStatus::Completed,
                        );
                        if let Err(error) = observation.and_then(|observation| {
                            emit(
                                &events,
                                &mut sequence,
                                RuntimeEventKind::Activity(observation),
                            )
                        }) {
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
                    Ok(Applied::Complete(message_id)) => {
                        if !output.is_empty() {
                            let content = OperationContent::new(output.clone())
                                .expect("output is non-empty");
                            let observation = activity.assistant_completed(
                                crate::activity::structured_assistant_id(),
                                &message_id,
                                swallowtail_runtime::ActivityAssistantPhase::Final,
                                Some(content.as_str()),
                            );
                            if let Err(error) = observation.and_then(|observation| {
                                emit(
                                    &events,
                                    &mut sequence,
                                    RuntimeEventKind::Activity(observation),
                                )
                            }) {
                                break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                            }
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
                    Err(error) => {
                        emit_protocol_debug(&services, &error, "http.pump.map");
                        break provider_status(error);
                    }
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
    activity_operation_id: swallowtail_runtime::ActivityOperationId,
}

enum StreamState {
    Start,
    Message,
    TextContent,
    SearchUse,
    SearchResult,
    AfterContent,
    Delta,
    Complete,
}

enum Applied {
    None,
    Usage(TokenUsage),
    AssistantStarted(String),
    Delta { message_id: String, delta: String },
    SearchStarted(String),
    SearchUpdated(String),
    SearchCompleted(String),
    SearchProgress,
    Complete(String),
}

fn apply_event(
    event: Event,
    state: &mut StreamState,
    output: &mut String,
    search_allowed: bool,
    search_uses: &mut u32,
    message_id: &mut Option<String>,
    search_id: &mut Option<String>,
) -> Result<Applied, RuntimeFailure> {
    match (event, &*state) {
        (Event::MessageStart { id, usage }, StreamState::Start) => {
            *message_id = Some(id);
            *state = StreamState::Message;
            Ok(Applied::Usage(usage))
        }
        (Event::ContentStart(crate::protocol::ContentBlock::Text), StreamState::Message | StreamState::AfterContent) => {
            *state = StreamState::TextContent;
            // Guard: StreamState::Message is only reachable after
            // Event::MessageStart set message_id.
            Ok(Applied::AssistantStarted(
                message_id.clone().expect("message identity exists"),
            ))
        }
        (Event::ContentStart(crate::protocol::ContentBlock::SearchUse { id }),
            StreamState::Message | StreamState::AfterContent,
        ) if search_allowed && *search_uses < 2 => {
            *search_uses += 1;
            *search_id = Some(id.clone());
            *state = StreamState::SearchUse;
            Ok(Applied::SearchStarted(id))
        }
        (Event::ContentStart(crate::protocol::ContentBlock::SearchResult { tool_use_id }),
            StreamState::Message | StreamState::AfterContent,
        ) if search_allowed && search_id.as_deref() == Some(tool_use_id.as_str()) => {
            *state = StreamState::SearchResult;
            Ok(Applied::SearchProgress)
        }
        (Event::OutputDelta(delta), StreamState::TextContent) if !delta.is_empty() => {
            output.push_str(&delta);
            Ok(Applied::Delta {
                message_id: message_id.clone().expect("message identity exists"),
                delta,
            })
        }
        (Event::Citation, StreamState::TextContent) if search_allowed => {
            Ok(Applied::SearchProgress)
        }
        (Event::InputJsonDelta(_), StreamState::SearchUse) if search_allowed => {
            Ok(Applied::SearchUpdated(
                search_id.clone().expect("search identity exists"),
            ))
        }
        (Event::ContentStop, StreamState::TextContent | StreamState::SearchUse) => {
            *state = StreamState::AfterContent;
            Ok(Applied::None)
        }
        (Event::ContentStop, StreamState::SearchResult) => {
            *state = StreamState::AfterContent;
            Ok(Applied::SearchCompleted(
                search_id.take().expect("search identity exists"),
            ))
        }
        (Event::Usage(usage, _), StreamState::AfterContent | StreamState::Delta) => {
            *state = StreamState::Delta;
            Ok(Applied::Usage(usage))
        }
        (Event::MessageStop, StreamState::Delta) => {
            *state = StreamState::Complete;
            Ok(Applied::Complete(
                message_id.clone().expect("message identity exists"),
            ))
        }
        (Event::ProviderFailed(kind), _) => Err(provider_failure(kind, "message stream")),
        _ => Err(failure(
            "swallowtail.anthropic.stream_order_invalid",
            "Anthropic stream event order was invalid",
        )),
    }
}
