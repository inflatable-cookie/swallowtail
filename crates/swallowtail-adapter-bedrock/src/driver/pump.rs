enum PumpSignal {
    Update(Option<Result<StreamUpdate, RuntimeFailure>>),
    Deadline,
}

async fn next_signal(
    updates: &mut Pin<Box<dyn Stream<Item = Result<StreamUpdate, RuntimeFailure>> + Send>>,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> PumpSignal {
    poll_fn(|context| {
        if let Poll::Ready(update) = updates.as_mut().poll_next(context) {
            return Poll::Ready(PumpSignal::Update(update));
        }
        if let Some(deadline) = deadline && deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(PumpSignal::Deadline);
        }
        Poll::Pending
    }).await
}

async fn pump_run(
    updates: mpsc::Receiver<Result<StreamUpdate, RuntimeFailure>>,
    blocking: BoxFuture<'static, Result<(), RuntimeFailure>>,
    access: &mut AccessLease,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> TerminalOutcome {
    let mut updates: Pin<Box<dyn Stream<Item = Result<StreamUpdate, RuntimeFailure>> + Send>> = Box::pin(updates);
    let mut sequence = 1;
    let mut output = String::new();
    let mut usage_seen = false;
    let mut timed_out = false;
    let mut stream_failure = None;
    loop {
        match next_signal(&mut updates, &mut deadline).await {
            PumpSignal::Deadline => {
                timed_out = true;
                deadline = None;
                cancellation.request_signal();
            }
            PumpSignal::Update(Some(Err(error))) => {
                stream_failure = Some(error);
                break;
            }
            PumpSignal::Update(Some(Ok(StreamUpdate::TextDelta(delta)))) => {
                output.push_str(&delta);
                match OperationContent::new(delta) {
                    Ok(content) => {
                        if let Err(error) = events.send(RuntimeEvent::with_content(sequence, RuntimeEventKind::OutputDelta, content)) {
                            stream_failure = Some(error);
                            cancellation.request_signal();
                            break;
                        }
                        sequence += 1;
                    }
                    Err(_) => {
                        stream_failure = Some(failure("swallowtail.bedrock.empty_delta", "Bedrock Runtime returned an empty output delta"));
                        cancellation.request_signal();
                        break;
                    }
                }
            }
            PumpSignal::Update(Some(Ok(StreamUpdate::Usage(usage)))) => {
                usage_seen = true;
                let observation = ProviderObservation::Usage(RuntimeTokenUsage::new(Some(usage.input), Some(usage.output)));
                if let Err(error) = events.send(RuntimeEvent::new(sequence, RuntimeEventKind::ProviderObservation(observation))) {
                    stream_failure = Some(error);
                    cancellation.request_signal();
                    break;
                }
                sequence += 1;
            }
            PumpSignal::Update(Some(Ok(_))) => {}
            PumpSignal::Update(None) => break,
        }
    }
    let sdk_result = blocking.await;
    let cleanup = access.release(&services).await;
    let status = if timed_out {
        TerminalStatus::TimedOut
    } else if cancellation.is_requested() {
        TerminalStatus::Cancelled
    } else if let Some(error) = stream_failure.or_else(|| sdk_result.err()) {
        TerminalStatus::ProviderFailed(error.diagnostic().clone())
    } else if !usage_seen {
        TerminalStatus::ProviderFailed(SafeDiagnostic::new("swallowtail.bedrock.stream_incomplete", "Bedrock Runtime stream ended without final usage"))
    } else {
        TerminalStatus::Completed
    };
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed) && !output.is_empty() {
        outcome = outcome.with_output(OperationContent::new(output).expect("non-empty output is valid"));
    }
    outcome
}
