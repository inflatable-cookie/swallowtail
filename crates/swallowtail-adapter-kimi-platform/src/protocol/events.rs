#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Event {
    RoleStart,
    ReasoningDelta(String),
    OutputDelta(String),
    Finished(String),
    Usage(TokenUsage),
    Done,
    ProviderFailed(ProviderErrorKind),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderErrorKind {
    Authentication,
    Permission,
    ModelUnavailable,
    Quota,
    RateLimited,
    Unavailable,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SseFrame(SseRecord);

#[derive(Default)]
pub(crate) struct SseDecoder(swallowtail_protocol_openai_chat::SseDecoder);

impl SseDecoder {
    pub(crate) fn push(&mut self, chunk: &[u8]) -> Result<Vec<SseFrame>, RuntimeFailure> {
        self.0
            .push(chunk)
            .map(|records| records.into_iter().map(SseFrame).collect())
            .map_err(map_sse_failure)
    }

    pub(crate) fn finish(self) -> Result<(), RuntimeFailure> {
        self.0.finish().map_err(map_sse_failure)
    }
}

pub(crate) fn parse_events(
    frame: &SseFrame,
    expected_model: &str,
) -> Result<Vec<Event>, RuntimeFailure> {
    let SseRecord::Data(data) = &frame.0 else {
        return Ok(vec![Event::Done]);
    };
    let payload = decode_payload(data, CodecLimits::default())
        .map_err(|_| protocol_failure("stream event"))?;
    let Payload::Chunk(chunk) = payload else {
        let Payload::Error(error) = payload else {
            unreachable!()
        };
        return Ok(vec![Event::ProviderFailed(classify_error(
            &error.error.kind,
        ))]);
    };
    if chunk.object.as_deref() != Some("chat.completion.chunk")
        || chunk.model.as_deref() != Some(expected_model)
    {
        return Err(failure(
            "swallowtail.kimi_platform.returned_model_mismatch",
            "Kimi Platform returned a model outside the selected route",
        ));
    }
    if !chunk.unknown_fields.is_empty() || chunk.choices.len() != 1 {
        return Err(unknown_semantics());
    }
    let choice = &chunk.choices[0];
    if choice.index != 0 || !choice.unknown_fields.is_empty() {
        return Err(unknown_semantics());
    }
    let mut events = Vec::new();
    if choice.delta.role.as_deref() == Some("assistant") {
        events.push(Event::RoleStart);
    } else if choice.delta.role.is_some() {
        return Err(unknown_semantics());
    }
    if let Some(content) = &choice.delta.content {
        events.push(Event::OutputDelta(content.clone()));
    }
    for field in &choice.delta.unknown_fields {
        if field.name() == "reasoning_content" {
            let Some(reasoning) = field.value().as_str() else {
                return Err(unknown_semantics());
            };
            events.push(Event::ReasoningDelta(reasoning.to_owned()));
        } else {
            return Err(unknown_semantics());
        }
    }
    if let Some(reason) = &choice.finish_reason {
        if !matches!(reason.as_str(), "stop" | "length") {
            return Err(unknown_semantics());
        }
        events.push(Event::Finished(reason.clone()));
    }
    if let Some(usage) = &chunk.usage {
        let mut cached = None;
        for field in &usage.unknown_fields {
            if field.name() == "cached_tokens" {
                cached = field.value().as_u64();
                if cached.is_none() {
                    return Err(unknown_semantics());
                }
            } else {
                return Err(unknown_semantics());
            }
        }
        events.push(Event::Usage(
            TokenUsage::new(usage.prompt_tokens, usage.completion_tokens)
                .with_cache_tokens(cached, None),
        ));
    }
    if events.is_empty() {
        return Err(unknown_semantics());
    }
    Ok(events)
}

fn classify_error(kind: &str) -> ProviderErrorKind {
    match kind {
        "invalid_authentication_error" => ProviderErrorKind::Authentication,
        "permission_denied_error" => ProviderErrorKind::Permission,
        "resource_not_found_error" => ProviderErrorKind::ModelUnavailable,
        "exceeded_current_quota_error" => ProviderErrorKind::Quota,
        "rate_limit_reached_error" => ProviderErrorKind::RateLimited,
        "server_unavailable" | "engine_overloaded_error" => ProviderErrorKind::Unavailable,
        _ => ProviderErrorKind::Other,
    }
}

pub(crate) fn provider_failure(kind: ProviderErrorKind, operation: &str) -> RuntimeFailure {
    let (code, label) = match kind {
        ProviderErrorKind::Authentication => {
            ("authentication_rejected", "authentication was rejected")
        }
        ProviderErrorKind::Permission => ("permission_denied", "permission was denied"),
        ProviderErrorKind::ModelUnavailable => ("model_unavailable", "model was unavailable"),
        ProviderErrorKind::Quota => ("quota_unavailable", "quota was unavailable"),
        ProviderErrorKind::RateLimited => ("rate_limited", "was rate limited"),
        ProviderErrorKind::Unavailable => ("provider_unavailable", "was unavailable"),
        ProviderErrorKind::Other => ("provider_failed", "failed"),
    };
    failure(
        match code {
            "authentication_rejected" => "swallowtail.kimi_platform.authentication_rejected",
            "permission_denied" => "swallowtail.kimi_platform.permission_denied",
            "model_unavailable" => "swallowtail.kimi_platform.model_unavailable",
            "quota_unavailable" => "swallowtail.kimi_platform.quota_unavailable",
            "rate_limited" => "swallowtail.kimi_platform.rate_limited",
            "provider_unavailable" => "swallowtail.kimi_platform.provider_unavailable",
            _ => "swallowtail.kimi_platform.provider_failed",
        },
        format!("Kimi Platform {operation} {label}"),
    )
}

fn unknown_semantics() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.content_semantics_unknown",
        "Kimi Platform emitted semantics outside the frozen K3 fixture",
    )
}

fn map_sse_failure(error: ProtocolError) -> RuntimeFailure {
    match error.kind() {
        ProtocolErrorKind::BufferLimitExceeded | ProtocolErrorKind::WireLimitExceeded => failure(
            "swallowtail.kimi_platform.sse_limit",
            "Kimi Platform SSE input exceeded its limit",
        ),
        ProtocolErrorKind::IncompleteRecord => failure(
            "swallowtail.kimi_platform.sse_disconnected",
            "Kimi Platform SSE disconnected during an event",
        ),
        _ => protocol_failure("SSE framing"),
    }
}
