struct FinalAttemptFlow<'a> {
    context: &'a mut TurnContext,
    events: &'a RuntimeEventSender,
    sequence: &'a mut u64,
    deadline:
        &'a mut swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &'a TurnCancellation,
    activity: &'a crate::activity::DeepSeekActivityProjection,
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
                let updates = match parser.push(&bytes) {
                    Ok(updates) => updates,
                    Err(error) => {
                        let failure = protocol(error);
                        emit_protocol_debug(&flow.context.services, &failure, "http.pump.decode");
                        return Err(TurnFailure::Provider(failure, CleanupOutcome::Clean));
                    }
                };
                for update in updates {
                    emit_update(
                        flow.events,
                        flow.sequence,
                        attempt.attempt_id(),
                        flow.activity,
                        update,
                    )
                    .map_err(runtime_failure)?;
                }
            }
            StreamSignal::Item(Err(error)) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(if flow.cancellation.is_requested() {
                    TurnFailure::Stopped(stop_from_cancellation(flow.cancellation), cleanup)
                } else {
                    emit_wire_debug(&flow.context.services, &error, "http.pump.transport");
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Closed => {
                let cleanup = cleanup_result(subscription.close().await);
                let final_attempt = match parser.finish() {
                    Ok(final_attempt) => final_attempt,
                    Err(error) => {
                        let failure = protocol(error);
                        emit_wire_debug(&flow.context.services, &failure, "http.pump.transport");
                        return Err(TurnFailure::Provider(failure, cleanup));
                    }
                };
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
