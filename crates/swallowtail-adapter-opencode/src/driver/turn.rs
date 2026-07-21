struct OpenCodeTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<TurnCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: ActiveSlot,
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

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst) {
                let _ = self.cancellation.request().await;
            }
            join_active(&self.active).await
        })
    }
}

async fn pump_turn(
    mut subscription: Subscription,
    deadline: Option<Deadline>,
    services: HostServices,
    cancellation: Arc<TurnCancellation>,
    events: swallowtail_runtime::RuntimeEventSender,
    terminal: swallowtail_runtime::TerminalOutcomeSender,
    terminal_flag: Arc<AtomicBool>,
) {
    let mut deadline_wait =
        deadline.and_then(|deadline| services.time().map(|time| time.wait_until(deadline)));
    let mut sequence = 2;
    let mut output = None;
    let status = loop {
        match next_signal(&mut subscription, &mut deadline_wait).await {
            TurnSignal::Deadline => {
                let abort = cancellation.request().await;
                break (
                    TerminalStatus::TimedOut,
                    cleanup_from_result(abort.map(|_| ())),
                );
            }
            TurnSignal::Closed => {
                if cancellation.requested.load(Ordering::SeqCst) {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
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
                if cancellation.requested.load(Ordering::SeqCst) {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
                }
                break (
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            TurnSignal::Data(data) => match parse_event(&data, &cancellation.session_id) {
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
                Ok(Event::OutputDelta(text)) => {
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
                Ok(Event::OutputSnapshot(text)) => {
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
                Ok(Event::Idle) => break (TerminalStatus::Completed, CleanupOutcome::Clean),
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
                Ok(Event::StopAndAbort) => {
                    let abort = cancellation.request().await;
                    break (
                        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                            "swallowtail.opencode.provider_request_rejected",
                            "OpenCode requested unsupported provider interaction",
                        )),
                        cleanup_from_result(abort.map(|_| ())),
                    );
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
    let cleanup = merge_cleanup(status.1, stream_cleanup);
    events.mark_terminal();
    let mut outcome = TerminalOutcome::new(status.0, cleanup);
    if let Some(output) = output {
        outcome = outcome.with_output(output);
    }
    let _ = terminal.complete(outcome);
    terminal_flag.store(true, Ordering::SeqCst);
}


