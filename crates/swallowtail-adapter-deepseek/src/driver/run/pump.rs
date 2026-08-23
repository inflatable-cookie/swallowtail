#[allow(clippy::too_many_arguments)]
async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    thinking_mode: Option<crate::DeepSeekThinkingMode>,
    activity_operation_id: swallowtail_runtime::ActivityOperationId,
) -> TerminalOutcome {
    let mut parser = FinalStreamParser::new_with_thinking_mode(&deepseek_v4_config(), thinking_mode);
    let mut sequence = 1;
    let mut output = None;
    let activity = crate::activity::DeepSeekActivityProjection::new(activity_operation_id);
    let status = 'pump: loop {
        match next_signal(&mut subscription, &mut deadline).await {
            Signal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            Signal::Closed if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Closed => match (if thinking_mode.is_some() {
                parser
                    .finish_without_private()
                    .map(|final_output| final_output.output)
            } else {
                parser.finish().map(|final_attempt| final_attempt.output)
            })
            .map_err(protocol)
            {
                Ok(output_text) => {
                    let content = match OperationContent::new(output_text) {
                        Ok(content) => content,
                        Err(_) => {
                            let diagnostic = SafeDiagnostic::new(
                                "swallowtail.deepseek.output_invalid",
                                "DeepSeek emitted invalid output content",
                            );
                            services.emit_failure_debug(
                                DebugObservationKind::ProtocolParse,
                                ROUTE,
                                "http.pump.map",
                                diagnostic.code(),
                                diagnostic.message(),
                            );
                            break TerminalStatus::RuntimeFailed(diagnostic);
                        }
                    };
                    let completed = match activity.assistant_completed(content.as_str()) {
                        Ok(completed) => completed,
                        Err(error) => {
                            break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        }
                    };
                    if let Err(error) = events.send(RuntimeEvent::new(
                        sequence,
                        RuntimeEventKind::Activity(completed),
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    sequence += 1;
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        content.clone(),
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    output = Some(content);
                    break TerminalStatus::Completed;
                }
                Err(error) => {
                    emit_wire_debug(&services, &error, "http.pump.transport");
                    break TerminalStatus::ProviderFailed(error.diagnostic().clone());
                }
            },
            Signal::Item(Err(_)) if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Item(Err(error)) => {
                emit_wire_debug(&services, &error, "http.pump.transport");
                break TerminalStatus::ProviderFailed(error.diagnostic().clone());
            }
            Signal::Item(Ok(StreamItem::Metadata(headers))) => {
                if let Err(error) = emit_request(&events, &mut sequence, &headers) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Data(bytes))) => {
                match parser.push(&bytes).map_err(protocol) {
                    Err(error) => {
                        emit_protocol_debug(&services, &error, "http.pump.decode");
                        break TerminalStatus::ProviderFailed(error.diagnostic().clone());
                    }
                    Ok(updates) => {
                        for update in updates {
                            if let Err(error) =
                                emit_update(&events, &mut sequence, &activity, update)
                            {
                                break 'pump TerminalStatus::RuntimeFailed(
                                    error.diagnostic().clone(),
                                );
                            }
                        }
                    }
                }
            }
        }
    };
    let stream_cleanup = cleanup_result(subscription.close().await);
    let credential_cleanup = access.release(&services).await;
    let cleanup = merge_cleanup(stream_cleanup, credential_cleanup);
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed)
        && let Some(output) = output
    {
        outcome = outcome.with_output(output);
    }
    outcome
}

fn emit_request(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    let Some(value) = headers.get("x-request-id") else {
        return Ok(());
    };
    let reference = ProviderRequestRef::new(value.clone()).map_err(|_| {
        failure(
            "swallowtail.deepseek.request_id_invalid",
            "DeepSeek request correlation was invalid",
        )
    })?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(reference)),
    )
}

fn emit_update(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    activity: &crate::activity::DeepSeekActivityProjection,
    update: FinalStreamUpdate,
) -> Result<(), RuntimeFailure> {
    match update {
        FinalStreamUpdate::Output(delta) => {
            let content = OperationContent::new(delta).map_err(|_| {
                failure(
                    "swallowtail.deepseek.output_invalid",
                    "DeepSeek emitted invalid output content",
                )
            })?;
            events.send(RuntimeEvent::new(
                *sequence,
                RuntimeEventKind::Activity(activity.assistant_delta(content.as_str())?),
            ))?;
            *sequence += 1;
            events.send(RuntimeEvent::with_content(
                *sequence,
                RuntimeEventKind::OutputDelta,
                content,
            ))?;
            *sequence += 1;
            Ok(())
        }
        FinalStreamUpdate::Usage(usage) => emit_usage(events, sequence, usage),
        FinalStreamUpdate::Finished(_) => emit(events, sequence, RuntimeEventKind::Progress),
    }
}

fn emit_usage(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    usage: Usage,
) -> Result<(), RuntimeFailure> {
    if usage.prompt_tokens.saturating_add(usage.completion_tokens) != usage.total_tokens {
        return Err(failure(
            "swallowtail.deepseek.usage_invalid",
            "DeepSeek usage totals were inconsistent",
        ));
    }
    let usage = TokenUsage::new(Some(usage.prompt_tokens), Some(usage.completion_tokens))
        .with_cache_tokens(Some(usage.cache_hit_tokens), None)
        .with_cache_miss_input_tokens(Some(usage.cache_miss_tokens));
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
    )
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

enum Signal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed,
    Deadline,
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Signal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(item.map_or(Signal::Closed, Signal::Item));
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Signal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn emit_protocol_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::ProtocolParse,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

fn emit_wire_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::WireInbound,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}
