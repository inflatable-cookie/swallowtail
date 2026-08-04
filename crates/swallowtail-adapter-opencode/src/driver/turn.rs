struct OpenCodeTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<TurnCancellation>,
    detachment: Option<Arc<TurnDetachment>>,
    terminal_flag: Arc<AtomicBool>,
    active: ActiveSlot,
    callbacks: Option<swallowtail_runtime::CallbackExchange>,
    attachment: input::SharedAttachment,
}

impl TurnHandle for OpenCodeTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&swallowtail_core::TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<swallowtail_runtime::CallbackExchange> {
        self.callbacks.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn detachment(&self) -> Option<&dyn OperationDetachmentControl> {
        self.detachment
            .as_deref()
            .map(|control| control as &dyn OperationDetachmentControl)
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst)
                && !self
                    .detachment
                    .as_ref()
                    .is_some_and(|detachment| detachment.is_requested())
            {
                let _ = self.cancellation.request().await;
            }
            let active = join_active(&self.active).await;
            merge_cleanup(active, self.attachment.release().await)
        })
    }
}

struct TurnPump {
    turn_id: RuntimeTurnId,
    subscription: Subscription,
    deadline: Option<Deadline>,
    services: HostServices,
    cancellation: Arc<TurnCancellation>,
    detachment: Option<Arc<TurnDetachment>>,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: swallowtail_runtime::TerminalOutcomeSender,
    terminal_flag: Arc<AtomicBool>,
    callback_hub: Option<callback::CallbackHub>,
    callback_operation: swallowtail_runtime::CallbackOperationId,
}

async fn pump_turn(pump: TurnPump) {
    let TurnPump {
        turn_id,
        mut subscription,
        deadline,
        services,
        cancellation,
        detachment,
        events,
        terminal,
        terminal_flag,
        callback_hub,
        callback_operation,
    } = pump;
    let mut deadline_wait =
        deadline.and_then(|deadline| services.time().map(|time| time.wait_until(deadline)));
    let mut sequence = 2;
    let mut output = None;
    let mut usage: Option<swallowtail_runtime::TokenUsage> = None;
    let mut usage_part_ids = BTreeSet::new();
    let mut activity = crate::activity::OpenCodeActivityProjection::new(turn_id);
    let mut status = loop {
        match next_signal(&mut subscription, &mut deadline_wait).await {
            TurnSignal::Deadline => {
                let abort = cancellation.request().await;
                break (
                    TerminalStatus::TimedOut,
                    cleanup_from_result(abort.map(|_| ())),
                );
            }
            TurnSignal::Closed => {
                if cancellation.is_requested() {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
                }
                if detachment
                    .as_ref()
                    .is_some_and(|detachment| detachment.is_requested())
                {
                    break (TerminalStatus::Detached, CleanupOutcome::Clean);
                }
                break (
                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.opencode.sse_disconnected",
                        "OpenCode SSE stream disconnected before terminal state",
                    )),
                    CleanupOutcome::Clean,
                );
            }
            TurnSignal::Failure(error) => {
                if cancellation.is_requested() {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
                }
                if detachment
                    .as_ref()
                    .is_some_and(|detachment| detachment.is_requested())
                {
                    break (TerminalStatus::Detached, CleanupOutcome::Clean);
                }
                break (
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            TurnSignal::Data(data) => match parse_event(&data, &cancellation.session_id).and_then(
                |event| project_event(&mut activity, &events, &mut sequence, event),
            ) {
                Ok(Event::Connected | Event::Foreign) => {}
                Ok(Event::Busy) => {
                    if let Err(error) =
                        events.send(RuntimeEvent::new(sequence, RuntimeEventKind::Progress))
                    {
                        let abort = cancellation.request().await;
                        break (
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup_from_result(abort.map(|_| ())),
                        );
                    }
                    sequence += 1;
                }
                Ok(Event::OutputDelta { text, .. }) => {
                    if let Ok(content) = swallowtail_runtime::OperationContent::new(text) {
                        if let Err(error) = events.send(RuntimeEvent::with_content(
                            sequence,
                            RuntimeEventKind::OutputDelta,
                            content,
                        )) {
                            let abort = cancellation.request().await;
                            break (
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup_from_result(abort.map(|_| ())),
                            );
                        }
                        sequence += 1;
                    }
                }
                Ok(Event::OutputSnapshot { text, .. }) => {
                    if let Ok(content) = swallowtail_runtime::OperationContent::new(text) {
                        output = Some(content.clone());
                        if let Err(error) = events.send(RuntimeEvent::with_content(
                            sequence,
                            RuntimeEventKind::OutputAvailable,
                            content,
                        )) {
                            let abort = cancellation.request().await;
                            break (
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup_from_result(abort.map(|_| ())),
                            );
                        }
                        sequence += 1;
                    }
                }
                Ok(Event::ReasoningSnapshot { text, .. }) => {
                    if let Ok(content) = swallowtail_runtime::OperationContent::new(text) {
                        if let Err(error) = events.send(RuntimeEvent::with_content(
                            sequence,
                            RuntimeEventKind::ReasoningProgress,
                            content,
                        )) {
                            let abort = cancellation.request().await;
                            break (
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup_from_result(abort.map(|_| ())),
                            );
                        }
                        sequence += 1;
                    }
                }
                Ok(Event::ToolState { .. } | Event::Unknown(_)) => {}
                Ok(Event::Usage(part_id, observed)) => {
                    if !usage_part_ids.insert(part_id) {
                        let abort = cancellation.request().await;
                        break (
                            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                "swallowtail.opencode.usage_duplicate",
                                "OpenCode repeated one token-usage record",
                            )),
                            cleanup_from_result(abort.map(|_| ())),
                        );
                    }
                    usage = match usage {
                        Some(current) => match current.checked_add_disjoint(observed) {
                            Some(total) => Some(total),
                            None => {
                                let abort = cancellation.request().await;
                                break (
                                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                        "swallowtail.opencode.usage_overflow",
                                        "OpenCode token usage exceeded the supported range",
                                    )),
                                    cleanup_from_result(abort.map(|_| ())),
                                );
                            }
                        },
                        None => Some(observed),
                    };
                }
                Ok(Event::Idle) if usage.is_some() => {
                    break (TerminalStatus::Completed, CleanupOutcome::Clean);
                }
                Ok(Event::Idle) => {
                    break (
                        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                            "swallowtail.opencode.usage_missing",
                            "OpenCode completed without required token usage",
                        )),
                        CleanupOutcome::Clean,
                    );
                }
                Ok(Event::Cancelled) => break (TerminalStatus::Cancelled, CleanupOutcome::Clean),
                Ok(Event::ProviderFailed) => {
                    break (
                        TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                            "swallowtail.opencode.provider_failed",
                            "OpenCode reported a provider failure",
                        )),
                        CleanupOutcome::Clean,
                    );
                }
                Ok(Event::ProviderRequest(provider)) => {
                    let Some(callbacks) = &callback_hub else {
                        let abort = cancellation.request().await;
                        break (
                            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                "swallowtail.opencode.provider_request_rejected",
                                "OpenCode requested unsupported provider interaction",
                            )),
                            cleanup_from_result(abort.map(|_| ())),
                        );
                    };
                    match callbacks.enqueue(
                        callback_operation.clone(),
                        sequence,
                        deadline,
                        provider,
                    ) {
                        Ok(callback_id) => {
                            if let Err(error) = events.send(RuntimeEvent::new(
                                sequence,
                                RuntimeEventKind::CallbackRequested(callback_id),
                            )) {
                                let abort = cancellation.request().await;
                                break (
                                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                    cleanup_from_result(abort.map(|_| ())),
                                );
                            }
                            sequence += 1;
                        }
                        Err(error) => {
                            let abort = cancellation.request().await;
                            break (
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup_from_result(abort.map(|_| ())),
                            );
                        }
                    }
                }
                Err(error) => {
                    let abort = cancellation.request().await;
                    break (
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        cleanup_from_result(abort.map(|_| ())),
                    );
                }
            },
        }
    };
    let stream_cleanup = cleanup_from_result(subscription.close().await);
    if let Some(callbacks) = &callback_hub {
        let reason = match &status.0 {
            TerminalStatus::TimedOut => swallowtail_runtime::CallbackAbandonment::TimedOut,
            TerminalStatus::Cancelled => {
                swallowtail_runtime::CallbackAbandonment::TurnCancelled
            }
            _ => swallowtail_runtime::CallbackAbandonment::TurnTerminated,
        };
        callbacks.abandon(reason);
    }
    let cleanup = merge_cleanup(status.1, stream_cleanup);
    let activity_status = match &status.0 {
        TerminalStatus::Detached => None,
        TerminalStatus::Completed => Some(swallowtail_runtime::ActivityStatus::Completed),
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => {
            Some(swallowtail_runtime::ActivityStatus::Cancelled)
        }
        TerminalStatus::ProviderRequestObserved(_)
        | TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => Some(swallowtail_runtime::ActivityStatus::Failed),
    };
    if let Some(activity_status) = activity_status {
        match activity.complete(activity_status) {
            Ok(observations) => {
                for observation in observations {
                    if let Err(error) = events.send(RuntimeEvent::new(
                        sequence,
                        RuntimeEventKind::Activity(observation),
                    )) {
                        status.0 = TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                        break;
                    }
                    sequence += 1;
                }
            }
            Err(error) => {
                status.0 = TerminalStatus::RuntimeFailed(error.diagnostic().clone());
            }
        }
    };
    if let Some(usage) = usage
        && let Err(error) = events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::ProviderObservation(
                swallowtail_runtime::ProviderObservation::Usage(usage),
            ),
        ))
    {
        status.0 = TerminalStatus::RuntimeFailed(error.diagnostic().clone());
    }
    events.mark_terminal();
    let mut outcome = TerminalOutcome::new(status.0, cleanup);
    if let Some(output) = output {
        outcome = outcome.with_output(output);
    }
    terminal_flag.store(true, Ordering::SeqCst);
    let _ = terminal.complete(outcome);
}

fn project_event(
    activity: &mut crate::activity::OpenCodeActivityProjection,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    event: Event,
) -> Result<Event, RuntimeFailure> {
    for observation in activity.project(&event)? {
        events.send(RuntimeEvent::new(
            *sequence,
            RuntimeEventKind::Activity(observation),
        ))?;
        *sequence += 1;
    }
    Ok(event)
}
