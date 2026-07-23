use super::*;

pub(super) async fn run_tool_turn(
    work: &mut TurnWork,
    context: &mut TurnContext,
    events: &RuntimeEventSender,
    sequence: &mut u64,
    deadline: &mut swallowtail_runtime::BoxFuture<
        'static,
        swallowtail_runtime::DeadlineObservation,
    >,
    cancellation: &TurnCancellation,
) -> Result<(FinalAttempt, CleanupOutcome), TurnFailure> {
    let response = wait_work(
        context.transport.request(
            context.scope.clone(),
            work.endpoint.clone(),
            work.credential.copy(),
            work.initial_request.clone(),
            &context.services,
            Arc::clone(&context.cancelled),
        ),
        &mut context.cancel_receiver,
        deadline,
        cancellation,
    )
    .await
    .map_err(work_failure)?;
    require_success(&response)
        .map_err(|kind| TurnFailure::Provider(provider(kind), CleanupOutcome::Clean))?;
    emit_request(events, sequence, &response.headers).map_err(runtime_failure)?;
    let tool = parse_tool_attempt(
        &response.body,
        work.attempt.attempt_id().clone(),
        &context.config,
    )
    .map_err(|error| TurnFailure::Provider(protocol(error), CleanupOutcome::Clean))?;
    emit_usage(events, sequence, work.attempt.attempt_id(), tool.usage).map_err(runtime_failure)?;
    let call = tool.call.clone();
    context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .pause_for_tool_calls(&work.attempt, std::slice::from_ref(&call))
        .map_err(runtime_failure)?;
    record_private(context, work.attempt.attempt_id(), &tool.reasoning).map_err(runtime_failure)?;
    context
        .history
        .lock()
        .expect("history lock poisoned")
        .record_tool_attempt(work.request.content().as_str(), tool)
        .map_err(runtime_failure)?;
    let submitter = work
        .submitter
        .as_ref()
        .expect("first attempt has submitter");
    submitter
        .open(call.call_id().clone())
        .map_err(runtime_failure)?;
    work.call_sender
        .as_mut()
        .expect("first attempt has call sender")
        .try_send(Ok(call.clone()))
        .map_err(|_| {
            runtime_failure(failure(
                "swallowtail.deepseek.tool_channel_failed",
                "DeepSeek tool-call channel could not deliver the pending call",
            ))
        })?;
    work.call_sender.take();
    emit(
        events,
        sequence,
        RuntimeEventKind::DirectToolCallAvailable(call.call_id().clone()),
    )
    .map_err(runtime_failure)?;
    let results = wait_results(
        work.result_receiver
            .as_mut()
            .expect("first attempt has result receiver"),
        &mut context.cancel_receiver,
        deadline,
        cancellation,
    )
    .await
    .map_err(|stop| TurnFailure::Stopped(stop, CleanupOutcome::Clean))?;
    let next = context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .authorize_tool_results(&results)
        .map_err(runtime_failure)?;
    let result = results.first().expect("exact result exists");
    context
        .history
        .lock()
        .expect("history lock poisoned")
        .record_tool_result(result)
        .map_err(runtime_failure)?;
    let request = continuation_request(context).map_err(runtime_failure)?;
    run_final_attempt(
        &next,
        request,
        work,
        FinalAttemptFlow {
            context,
            events,
            sequence,
            deadline,
            cancellation,
        },
    )
    .await
}

fn continuation_request(context: &TurnContext) -> Result<HttpRequest, RuntimeFailure> {
    let history = context.history.lock().expect("history lock poisoned");
    let first = history.first()?;
    let body = encode_after_tool(
        first.user(),
        first.reasoning(),
        first.call_id(),
        first.tool_name(),
        first.arguments(),
        first.result()?,
        &context.tools,
    )
    .map_err(protocol)?;
    Ok(HttpRequest::completion(body, true))
}
