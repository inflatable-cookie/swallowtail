fn provider_terminal(error: RuntimeFailure, context: &TurnContext) -> TerminalOutcome {
    invalidate(context);
    TerminalOutcome::new(
        TerminalStatus::ProviderFailed(error.diagnostic().clone()),
        CleanupOutcome::Clean,
    )
}

#[allow(clippy::too_many_arguments)]
async fn run_attempt(
    attempt: &DirectInferenceAttempt,
    wire: Request,
    work: &TurnWork,
    context: &mut TurnContext,
    events: &RuntimeEventSender,
    sequence: &mut u64,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
    activity: &crate::activity::AnthropicActivityProjection,
) -> Result<AttemptOutcome, TurnFailure> {
    let mut subscription = context
        .transport
        .subscribe(
            context.scope.clone(),
            context.endpoint.clone(),
            work.credential.copy(),
            wire,
            &context.services,
            Arc::clone(&context.cancelled),
        )
        .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::NotApplicable))?;
    let mut parser = AttemptParser::new(
        attempt.attempt_id().clone(),
        context
            .plan
            .requirements()
            .direct_continuation()
            .expect("validated continuation")
            .config()
            .maximum_tool_argument_bytes()
            .get() as usize,
        context
            .plan
            .requirements()
            .direct_continuation()
            .expect("validated continuation")
            .config()
            .maximum_private_continuation_bytes()
            .get() as usize,
        context.thinking.is_some(),
    );
    loop {
        match next_signal(
            &mut subscription,
            &mut context.cancel_receiver,
            deadline,
            cancellation,
        )
        .await
        {
            StreamSignal::Item(Ok(StreamItem::Headers(headers))) => {
                emit_request(events, sequence, &headers)
                    .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))?;
            }
            StreamSignal::Item(Ok(StreamItem::Frame(frame))) => {
                let event = match parse_event(&frame) {
                    Ok(event) => event,
                    Err(error) => {
                        super::emit_protocol_debug(
                            &context.services,
                            &error,
                            "http.pump.decode",
                        );
                        return Err(TurnFailure::Provider(error, CleanupOutcome::Clean));
                    }
                };
                if let Err(error) = parser.apply(event, events, sequence, activity) {
                    super::emit_protocol_debug(&context.services, &error, "http.pump.map");
                    return Err(TurnFailure::Provider(error, CleanupOutcome::Clean));
                }
            }
            StreamSignal::Item(Err(error)) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(if cancellation.is_requested() {
                    TurnFailure::Stopped(cancellation.stop_reason(), cleanup)
                } else {
                    super::emit_wire_debug(&context.services, &error, "http.pump.transport");
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Closed => {
                let cleanup = cleanup_result(subscription.close().await);
                return parser.finish().map_err(|error| {
                    super::emit_wire_debug(&context.services, &error, "http.pump.transport");
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Stopped(stop) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(TurnFailure::Stopped(stop, cleanup));
            }
        }
    }
}
