use std::collections::BTreeMap;
use swallowtail_core::ProviderRequestRef;
use swallowtail_runtime::{
    OperationContent, ProviderObservation, RateLimitKind, RateLimitObservation, RuntimeEvent,
    RuntimeEventKind, RuntimeEventSender,
};

fn emit_headers(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    if let Some(request) = headers.get("x-request-id") {
        let reference = ProviderRequestRef::new(request).map_err(|_| {
            failure(
                "swallowtail.openai.request_id_invalid",
                "OpenAI request identity was invalid",
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
        ("tokens", RateLimitKind::Tokens),
    ] {
        let limit = header_number(headers, &format!("x-ratelimit-limit-{label}"));
        let remaining = header_number(headers, &format!("x-ratelimit-remaining-{label}"));
        let reset = headers
            .get(&format!("x-ratelimit-reset-{label}"))
            .and_then(|value| parse_reset(value));
        if limit.is_some() || remaining.is_some() || reset.is_some() {
            emit(
                events,
                sequence,
                RuntimeEventKind::ProviderObservation(ProviderObservation::RateLimit(
                    RateLimitObservation::new(kind, limit, remaining, reset),
                )),
            )?;
        }
    }
    Ok(())
}

fn header_number(headers: &BTreeMap<String, String>, name: &str) -> Option<u64> {
    headers.get(name).and_then(|value| value.parse().ok())
}

fn parse_reset(value: &str) -> Option<u64> {
    if let Some(milliseconds) = value.strip_suffix("ms") {
        milliseconds.parse().ok()
    } else {
        value
            .strip_suffix('s')?
            .parse::<u64>()
            .ok()?
            .checked_mul(1_000)
    }
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

fn emit_content(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: String,
) -> Result<(), RuntimeFailure> {
    let content = OperationContent::new(content).map_err(|_| {
        failure(
            "swallowtail.openai.output_invalid",
            "OpenAI returned invalid text output",
        )
    })?;
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}
