enum PumpSignal {
    Update(Option<Result<StreamUpdate, RuntimeFailure>>),
    Deadline,
}

struct RunPumpContext {
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    run_id: RuntimeRunId,
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
    context: RunPumpContext,
) -> TerminalOutcome {
    let RunPumpContext {
        services,
        events,
        cancellation,
        mut deadline,
        run_id,
    } = context;
    let mut updates: Pin<Box<dyn Stream<Item = Result<StreamUpdate, RuntimeFailure>> + Send>> = Box::pin(updates);
    let mut sequence = 1;
    let mut output = String::new();
    let mut usage_seen = false;
    let mut timed_out = false;
    let mut stream_failure = None;
    let mut activity = crate::activity::BedrockActivityProjection::new(
        swallowtail_runtime::ActivityOperationId::Run(run_id),
    );
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
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(match activity.delta(&delta) {
                        Ok(observation) => observation,
                        Err(error) => {
                            stream_failure = Some(error);
                            cancellation.request_signal();
                            break;
                        }
                    }),
                )) {
                    stream_failure = Some(error);
                    cancellation.request_signal();
                    break;
                }
                sequence += 1;
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
                if !output.is_empty() {
                    let content = OperationContent::new(output.clone())
                        .expect("non-empty Bedrock output is valid");
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        content,
                    )) {
                        stream_failure = Some(error);
                        cancellation.request_signal();
                        break;
                    }
                    sequence += 1;
                }
            }
            PumpSignal::Update(Some(Ok(StreamUpdate::MessageStarted))) => {
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(match activity.started() {
                        Ok(observation) => observation,
                        Err(error) => {
                            stream_failure = Some(error);
                            cancellation.request_signal();
                            break;
                        }
                    }),
                )) {
                    stream_failure = Some(error);
                    cancellation.request_signal();
                    break;
                }
                sequence += 1;
            }
            PumpSignal::Update(Some(Ok(StreamUpdate::MessageStopped(_)))) => {
                if output.is_empty() {
                    stream_failure = Some(failure(
                        "swallowtail.bedrock.empty_output",
                        "Bedrock Runtime completed without output",
                    ));
                    cancellation.request_signal();
                    break;
                }
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(match activity.completed(&output) {
                        Ok(observation) => observation,
                        Err(error) => {
                            stream_failure = Some(error);
                            cancellation.request_signal();
                            break;
                        }
                    }),
                )) {
                    stream_failure = Some(error);
                    cancellation.request_signal();
                    break;
                }
                sequence += 1;
            }
            PumpSignal::Update(Some(Ok(StreamUpdate::ContentBlockStopped))) => {}
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
