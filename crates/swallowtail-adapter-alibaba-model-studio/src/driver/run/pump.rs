async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    run_id: RuntimeRunId,
) -> TerminalOutcome {
    let mut provider = ResponseStream::default();
    let mut sequence = 1;
    let mut output = None;
    let mut activity = crate::activity::AlibabaActivityProjection::new(
        swallowtail_runtime::ActivityOperationId::Run(run_id),
    );
    let status = loop {
        match next_signal(&mut subscription, &mut deadline).await {
            Signal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            Signal::Closed if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Closed => {
                break if output.is_some() {
                    TerminalStatus::Completed
                } else {
                    let diagnostic = SafeDiagnostic::new(
                        "swallowtail.alibaba_model_studio.stream_disconnected",
                        "Alibaba Model Studio stream closed before completion",
                    );
                    services.emit_failure_debug(
                        DebugObservationKind::WireInbound,
                        ROUTE,
                        "http.pump.transport",
                        diagnostic.code(),
                        diagnostic.message(),
                    );
                    TerminalStatus::ProviderFailed(diagnostic)
                };
            }
            Signal::Item(Err(_)) if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Item(Err(error)) => {
                emit_wire_debug(&services, &error, "http.pump.transport");
                break TerminalStatus::ProviderFailed(error.diagnostic().clone());
            }
            Signal::Item(Ok(StreamItem::Correlation(reference))) => {
                if let Err(error) = emit(
                    &events,
                    &mut sequence,
                    RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                        reference,
                    )),
                ) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Frame(frame))) => match provider.apply(&frame) {
                Err(error) => {
                    let diagnostic = error.diagnostic();
                    services.emit_failure_debug(
                        DebugObservationKind::ProtocolParse,
                        ROUTE,
                        "http.pump.decode",
                        diagnostic.code(),
                        diagnostic.message(),
                    );
                    break TerminalStatus::ProviderFailed(diagnostic.clone());
                }
                Ok(
                    ProviderEvent::Created(_)
                    | ProviderEvent::Progress(_)
                    | ProviderEvent::Unknown(_),
                ) => {
                    if let Err(error) = emit(&events, &mut sequence, RuntimeEventKind::Progress) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::AssistantStarted(item)) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(match activity.started(&item) {
                            Ok(observation) => observation,
                            Err(error) => {
                                break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                            }
                        }),
                    ) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::TextDelta { item, content }) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(match activity.delta(&item, content.as_str()) {
                            Ok(observation) => observation,
                            Err(error) => {
                                break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                            }
                        }),
                    ) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputDelta,
                        content,
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    sequence += 1;
                }
                Ok(ProviderEvent::TextDone { .. }) => {}
                Ok(ProviderEvent::Completed {
                    item,
                    output: completed,
                    usage,
                    ..
                }) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(
                            match activity.completed(&item, completed.as_str()) {
                                Ok(observation) => observation,
                                Err(error) => {
                                    break TerminalStatus::RuntimeFailed(
                                        error.diagnostic().clone(),
                                    );
                                }
                            },
                        ),
                    ) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        completed.clone(),
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    sequence += 1;
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                    ) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    output = Some(completed);
                }
            },
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

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    swallowtail_runtime::emit(events, sequence, kind)
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
