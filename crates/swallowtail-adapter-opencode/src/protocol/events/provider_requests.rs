#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Event {
    Connected,
    Busy,
    OutputDelta {
        message_id: String,
        part_id: String,
        text: String,
    },
    OutputSnapshot {
        message_id: String,
        part_id: String,
        text: String,
    },
    ReasoningSnapshot {
        message_id: String,
        part_id: String,
        text: String,
    },
    ToolState {
        part_id: String,
        call_id: String,
        name: String,
        status: ToolStatus,
    },
    Usage(String, TokenUsage),
    Idle,
    Cancelled,
    ProviderFailed,
    ProviderRequest(PendingProviderRequest),
    Unknown(String),
    Foreign,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ToolStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

pub(crate) fn parse_event(data: &[u8], session_id: &str) -> Result<Event, RuntimeFailure> {
    let envelope: Value = parse_json(data, "event")?;
    let kind = envelope
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode event omitted its type",
            )
        })?;
    let properties = envelope
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            failure(
                "swallowtail.opencode.event_invalid",
                "OpenCode event omitted properties",
            )
        })?;
    if kind == "server.connected" {
        return Ok(Event::Connected);
    }
    let observed_session = properties
        .get("sessionID")
        .and_then(Value::as_str)
        .or_else(|| {
            properties
                .get("part")
                .and_then(Value::as_object)
                .and_then(|part| part.get("sessionID"))
                .and_then(Value::as_str)
        });
    if observed_session.is_some_and(|observed| observed != session_id) {
        return Ok(Event::Foreign);
    }
    if observed_session.is_none() {
        return Err(failure(
            "swallowtail.opencode.event_unknown",
            "OpenCode emitted an uncorrelated event type",
        ));
    }
    match kind {
        "session.status" => parse_status(properties),
        "session.idle" => Ok(Event::Idle),
        "message.part.delta" => parse_delta(properties),
        "message.part.updated" => parse_part(properties),
        "session.error" => parse_error(properties),
        "permission.asked" => parse_permission(properties),
        "question.asked" => parse_question(properties),
        _ => Ok(Event::Unknown(kind.to_owned())),
    }
}

fn parse_permission(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    let id = provider_request_id(properties)?;
    let permission = properties
        .get("permission")
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty() && value.len() <= 4096)
        .ok_or_else(provider_request_invalid)?;
    let patterns = bounded_string_array(properties.get("patterns"))?;
    let payload = serde_json::to_vec(&json!({
        "permission": permission,
        "patterns": patterns,
        "replies": ["once", "reject"]
    }))
    .map_err(|_| provider_request_invalid())?;
    Ok(Event::ProviderRequest(PendingProviderRequest {
        id,
        kind: ProviderRequestKind::Permission,
        payload,
    }))
}

fn parse_question(properties: &Map<String, Value>) -> Result<Event, RuntimeFailure> {
    let id = provider_request_id(properties)?;
    let questions = properties
        .get("questions")
        .and_then(Value::as_array)
        .filter(|questions| !questions.is_empty() && questions.len() <= 32)
        .ok_or_else(provider_request_invalid)?;
    for question in questions {
        let object = question.as_object().ok_or_else(provider_request_invalid)?;
        for field in ["question", "header"] {
            if object
                .get(field)
                .and_then(Value::as_str)
                .is_none_or(|value| value.is_empty() || value.len() > 4096)
            {
                return Err(provider_request_invalid());
            }
        }
        let options = object
            .get("options")
            .and_then(Value::as_array)
            .filter(|options| !options.is_empty() && options.len() <= 32)
            .ok_or_else(provider_request_invalid)?;
        for option in options {
            let option = option.as_object().ok_or_else(provider_request_invalid)?;
            for field in ["label", "description"] {
                if option
                    .get(field)
                    .and_then(Value::as_str)
                    .is_none_or(|value| value.is_empty() || value.len() > 4096)
                {
                    return Err(provider_request_invalid());
                }
            }
        }
        if !matches!(object.get("multiple"), Some(Value::Bool(_))) {
            return Err(provider_request_invalid());
        }
    }
    let payload = serde_json::to_vec(&json!({"questions": questions}))
        .map_err(|_| provider_request_invalid())?;
    Ok(Event::ProviderRequest(PendingProviderRequest {
        id,
        kind: ProviderRequestKind::Question {
            count: questions.len(),
        },
        payload,
    }))
}

fn provider_request_id(properties: &Map<String, Value>) -> Result<String, RuntimeFailure> {
    properties
        .get("id")
        .and_then(Value::as_str)
        .filter(|id| {
            !id.is_empty()
                && id.len() <= 1024
                && id.bytes().all(|byte| {
                    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'~')
                })
        })
        .map(str::to_owned)
        .ok_or_else(provider_request_invalid)
}

fn bounded_string_array(value: Option<&Value>) -> Result<Vec<&str>, RuntimeFailure> {
    let values = value
        .and_then(Value::as_array)
        .filter(|values| values.len() <= 32)
        .ok_or_else(provider_request_invalid)?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.is_empty() && value.len() <= 4096)
                .ok_or_else(provider_request_invalid)
        })
        .collect()
}

fn provider_request_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.provider_request_invalid",
        "OpenCode provider request was malformed",
    )
}

