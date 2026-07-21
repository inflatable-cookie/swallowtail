fn emit_headers(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    if let Some(request) = headers.get("request-id") {
        let reference = ProviderRequestRef::new(request).map_err(|_| {
            failure(
                "swallowtail.anthropic.request_id_invalid",
                "Anthropic request id was invalid",
            )
        })?;
        let kind = RuntimeEventKind::ProviderObservation(
            ProviderObservation::RequestCorrelation(reference),
        );
        emit(events, sequence, kind)?;
    }
    for (label, kind) in [
        ("requests", RateLimitKind::Requests),
        ("input-tokens", RateLimitKind::InputTokens),
        ("output-tokens", RateLimitKind::OutputTokens),
    ] {
        let limit = header_number(headers, &format!("anthropic-ratelimit-{label}-limit"));
        let remaining = header_number(
            headers,
            &format!("anthropic-ratelimit-{label}-remaining"),
        );
        if limit.is_some() || remaining.is_some() {
            let observation = RateLimitObservation::new(kind, limit, remaining, None);
            let event = RuntimeEventKind::ProviderObservation(
                ProviderObservation::RateLimit(observation),
            );
            emit(events, sequence, event)?;
        }
    }
    Ok(())
}

fn header_number(headers: &BTreeMap<String, String>, name: &str) -> Option<u64> {
    headers.get(name).and_then(|value| value.parse().ok())
}

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

fn provider_status(error: RuntimeFailure) -> TerminalStatus {
    TerminalStatus::ProviderFailed(error.diagnostic().clone())
}

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}
