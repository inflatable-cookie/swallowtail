async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    activity_operation_id: swallowtail_runtime::ActivityOperationId,
) -> TerminalOutcome {
    let mut parser = FinalStreamParser::new(&deepseek_v4_config());
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
            Signal::Closed => match parser.finish().map_err(protocol) {
                Ok(final_attempt) => {
                    let content = match OperationContent::new(final_attempt.output) {
                        Ok(content) => content,
                        Err(_) => {
                            break TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                "swallowtail.deepseek.output_invalid",
                                "DeepSeek emitted invalid output content",
                            ));
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
                Err(error) => break TerminalStatus::ProviderFailed(error.diagnostic().clone()),
            },
            Signal::Item(Err(_)) if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Item(Err(error)) => {
                break TerminalStatus::ProviderFailed(error.diagnostic().clone());
            }
            Signal::Item(Ok(StreamItem::Metadata(headers))) => {
                if let Err(error) = emit_request(&events, &mut sequence, &headers) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Data(bytes))) => {
                match parser.push(&bytes).map_err(protocol) {
                    Err(error) => break TerminalStatus::ProviderFailed(error.diagnostic().clone()),
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
