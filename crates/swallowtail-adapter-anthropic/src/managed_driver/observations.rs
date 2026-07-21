fn require_success(response: &ManagedResponse, operation: &str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.anthropic.managed.provider_failed",
            format!("Anthropic Managed Agents {operation} failed"),
        ))
    }
}

fn emit_headers(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    if let Some(request) = headers.get("request-id") {
        let reference = ProviderRequestRef::new(request).map_err(|_| {
            failure(
                "swallowtail.anthropic.managed.request_id_invalid",
                "Anthropic Managed Agents request id was invalid",
            )
        })?;
        emit(
            events,
            sequence,
            RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                reference,
            )),
        )?;
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
            emit(
                events,
                sequence,
                RuntimeEventKind::ProviderObservation(ProviderObservation::RateLimit(
                    RateLimitObservation::new(kind, limit, remaining, None),
                )),
            )?;
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

fn emit_content(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: OperationContent,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}
