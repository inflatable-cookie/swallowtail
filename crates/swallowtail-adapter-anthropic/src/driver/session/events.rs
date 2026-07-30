async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let task = {
        let mut active = active.lock().expect("active turn lock poisoned");
        if active
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            active.as_mut().and_then(|turn| turn.task.take())
        } else {
            None
        }
    };
    if let Some(task) = task {
        task.join().await?;
        *active.lock().expect("active turn lock poisoned") = None;
    }
    Ok(())
}

async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    if let Some(cancellation) = cancellation {
        let _ = cancellation.request().await;
    }
    let task = active
        .lock()
        .expect("active turn lock poisoned")
        .as_mut()
        .and_then(|turn| turn.task.take());
    let cleanup = match task {
        Some(task) => cleanup_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

async fn join_turn(active: &ActiveSlot, turn_id: &RuntimeTurnId) -> CleanupOutcome {
    let task = {
        let mut active = active.lock().expect("active turn lock poisoned");
        match active.as_mut() {
            Some(turn) if &turn.turn_id == turn_id => turn.task.take(),
            _ => return CleanupOutcome::NotApplicable,
        }
    };
    let cleanup = match task {
        Some(task) => cleanup_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

fn emit_attempt_usage(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    attempt: &swallowtail_runtime::DirectInferenceAttemptId,
    usage: TokenUsage,
) -> Result<(), RuntimeFailure> {
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptUsage(
            DirectAttemptUsageObservation::new(attempt.clone(), usage),
        )),
    )
}

fn emit_request(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    let Some(value) = headers.get("request-id") else {
        return Ok(());
    };
    let request = ProviderRequestRef::new(value).map_err(|_| {
        failure(
            "swallowtail.anthropic.request_id_invalid",
            "Anthropic request correlation was invalid",
        )
    })?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(request)),
    )
}

fn emit_content(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    value: String,
) -> Result<(), RuntimeFailure> {
    let content = OperationContent::new(value).map_err(|_| {
        failure(
            "swallowtail.anthropic.output_invalid",
            "Anthropic emitted empty output content",
        )
    })?;
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}

fn emit(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

fn invalidate(context: &TurnContext) {
    context.usable.store(false, Ordering::SeqCst);
    context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .invalidate();
}

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    result.map_or_else(
        |error| CleanupOutcome::Failed(error.diagnostic().clone()),
        |_| CleanupOutcome::Clean,
    )
}

fn history_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.history_state_invalid",
        "Anthropic private continuation history was not in the required state",
    )
}

fn exchange_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.tool_result_rejected",
        "Anthropic tool results did not match the active consumer wait",
    )
}

enum TurnFailure {
    Stopped(StopSignal, CleanupOutcome),
    Provider(RuntimeFailure, CleanupOutcome),
    Runtime(RuntimeFailure, CleanupOutcome),
}
