enum AttachmentExit {
    Terminal(ManagedFinal),
    Callbacks(Vec<String>),
    Cancelled,
    Deadline,
    Disconnected,
}

enum AttachmentSignal {
    Item(Result<ManagedStreamItem, RuntimeFailure>),
    Closed,
    Deadline,
}

#[allow(clippy::too_many_arguments)]
async fn pump_attachment(
    subscription: &mut ManagedSubscription,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    run_id: &RuntimeRunId,
    callbacks: &ManagedCallbackHub,
    live: &mut Vec<ManagedEvent>,
    processed: &mut BTreeSet<String>,
    pending_tools: &mut BTreeSet<String>,
    declared_tools: &BTreeSet<String>,
    output: &mut String,
    cancellation: &ManagedRunCancellation,
    operation_deadline: Deadline,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> AttachmentExit {
    loop {
        if cancellation.is_requested() {
            return AttachmentExit::Cancelled;
        }
        match next_attachment_signal(subscription, deadline).await {
            AttachmentSignal::Deadline => return AttachmentExit::Deadline,
            AttachmentSignal::Closed | AttachmentSignal::Item(Err(_)) => {
                return if cancellation.is_requested() {
                    AttachmentExit::Cancelled
                } else {
                    AttachmentExit::Disconnected
                };
            }
            AttachmentSignal::Item(Ok(ManagedStreamItem::Headers(headers))) => {
                if let Err(error) = emit_headers(events, sequence, &headers) {
                    return AttachmentExit::Terminal(ManagedFinal::status(provider_status(error)));
                }
            }
            AttachmentSignal::Item(Ok(ManagedStreamItem::Frame(frame))) => {
                let parsed = match crate::managed::parse_stream(&frame) {
                    Ok(parsed) => parsed,
                    Err(error) => {
                        return AttachmentExit::Terminal(ManagedFinal::status(provider_status(
                            error,
                        )));
                    }
                };
                for event in parsed {
                    if let Some(existing) = live.iter().find(|known| known.id() == event.id()) {
                        if existing != &event {
                            return AttachmentExit::Terminal(ManagedFinal::status(provider_status(
                                failure(
                                    "swallowtail.anthropic.managed.event_conflict",
                                    "Anthropic Managed Agents repeated contradictory event evidence",
                                ),
                            )));
                        }
                        continue;
                    }
                    live.push(event.clone());
                    match apply_event(
                        &event,
                        events,
                        sequence,
                        run_id,
                        callbacks,
                        processed,
                        pending_tools,
                        declared_tools,
                        output,
                        operation_deadline,
                    ) {
                        Ok(EventAction::Continue) => {}
                        Ok(EventAction::Callbacks(ids)) => {
                            return AttachmentExit::Callbacks(ids);
                        }
                        Ok(EventAction::Terminal(state)) => {
                            return AttachmentExit::Terminal(state);
                        }
                        Err(error) => {
                            return AttachmentExit::Terminal(ManagedFinal::status(provider_status(
                                error,
                            )));
                        }
                    }
                }
            }
        }
    }
}

async fn next_attachment_signal(
    subscription: &mut ManagedSubscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> AttachmentSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(item) => AttachmentSignal::Item(item),
                None => AttachmentSignal::Closed,
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(AttachmentSignal::Deadline);
        }
        Poll::Pending
    })
    .await
}
