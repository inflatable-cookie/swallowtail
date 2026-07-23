struct FinalAttemptFlow<'a> {
    context: &'a mut TurnContext,
    events: &'a RuntimeEventSender,
    sequence: &'a mut u64,
    deadline:
        &'a mut swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &'a TurnCancellation,
}

async fn run_final_attempt(
    attempt: &DirectInferenceAttempt,
    request: HttpRequest,
    work: &TurnWork,
    flow: FinalAttemptFlow<'_>,
) -> Result<(FinalAttempt, CleanupOutcome), TurnFailure> {
    let mut subscription = flow
        .context
        .transport
        .subscribe(
            flow.context.scope.clone(),
            work.endpoint.clone(),
            work.credential.copy(),
            request,
            &flow.context.services,
            Arc::clone(&flow.context.cancelled),
        )
        .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::NotApplicable))?;
    let mut parser = FinalStreamParser::new(&flow.context.config);
    loop {
        match next_stream_signal(
            &mut subscription,
            &mut flow.context.cancel_receiver,
            flow.deadline,
            flow.cancellation,
        )
        .await
        {
            StreamSignal::Item(Ok(StreamItem::Metadata(headers))) => {
                emit_request(flow.events, flow.sequence, &headers).map_err(runtime_failure)?;
            }
            StreamSignal::Item(Ok(StreamItem::Data(bytes))) => {
                let updates = parser.push(&bytes).map_err(|error| {
                    TurnFailure::Provider(protocol(error), CleanupOutcome::Clean)
                })?;
                for update in updates {
                    emit_update(flow.events, flow.sequence, attempt.attempt_id(), update)
                        .map_err(runtime_failure)?;
                }
            }
            StreamSignal::Item(Err(error)) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(if flow.cancellation.is_requested() {
                    TurnFailure::Stopped(stop_from_cancellation(flow.cancellation), cleanup)
                } else {
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Closed => {
                let cleanup = cleanup_result(subscription.close().await);
                let final_attempt = parser.finish().map_err(|error| {
                    TurnFailure::Provider(protocol(error), cleanup.clone())
                })?;
                return Ok((final_attempt, cleanup));
            }
            StreamSignal::Stopped(stop) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(TurnFailure::Stopped(stop, cleanup));
            }
        }
    }
}

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    result.map_or_else(
        |error| CleanupOutcome::Failed(error.diagnostic().clone()),
        |_| CleanupOutcome::Clean,
    )
}

fn stop_from_cancellation(cancellation: &TurnCancellation) -> StopSignal {
    if cancellation.reason() == 2 {
        StopSignal::TimedOut
    } else {
        StopSignal::Cancelled
    }
}
