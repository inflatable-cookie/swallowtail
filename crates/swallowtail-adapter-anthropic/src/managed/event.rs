use crate::failure::failure;
use serde_json::Value;
use std::collections::BTreeMap;
use swallowtail_runtime::{OperationContent, RuntimeFailure};

const MAX_STREAM_BYTES: usize = 512 * 1024;
const MAX_RECONCILED_EVENTS: usize = 2_048;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum IdleReason {
    EndTurn,
    RequiresAction(Vec<String>),
    RetriesExhausted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ManagedEventKind {
    Running,
    Rescheduled,
    Message(OperationContent),
    CustomToolUse { name: String, input: Value },
    Idle(IdleReason),
    Terminated,
    ProviderError,
    Observed,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct ManagedEvent {
    id: String,
    event_type: String,
    raw: Value,
    kind: ManagedEventKind,
}

impl std::fmt::Debug for ManagedEvent {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ManagedEvent")
            .field("event_type", &self.event_type)
            .finish_non_exhaustive()
    }
}

impl ManagedEvent {
    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    #[cfg(test)]
    pub(crate) fn event_type(&self) -> &str {
        &self.event_type
    }

    pub(crate) const fn kind(&self) -> &ManagedEventKind {
        &self.kind
    }
}

pub(crate) fn parse_stream(input: &str) -> Result<Vec<ManagedEvent>, RuntimeFailure> {
    if input.len() > MAX_STREAM_BYTES || !input.ends_with("\n\n") {
        return Err(protocol_failure("event stream framing"));
    }
    input
        .trim_end_matches('\n')
        .split("\n\n")
        .filter(|frame| frame.lines().any(|line| !line.starts_with(':')))
        .map(parse_frame)
        .collect()
}

pub(crate) fn parse_history(input: &[u8]) -> Result<Vec<ManagedEvent>, RuntimeFailure> {
    if input.len() > MAX_STREAM_BYTES {
        return Err(protocol_failure("event history bound"));
    }
    let value: Value =
        serde_json::from_slice(input).map_err(|_| protocol_failure("event history JSON"))?;
    if value.get("next_page").is_some_and(|page| !page.is_null()) {
        return Err(protocol_failure("event history pagination"));
    }
    value
        .get("data")
        .and_then(Value::as_array)
        .ok_or_else(|| protocol_failure("event history data"))?
        .iter()
        .cloned()
        .map(parse_value)
        .collect()
}

pub(crate) fn reconcile(
    history: impl IntoIterator<Item = ManagedEvent>,
    live: impl IntoIterator<Item = ManagedEvent>,
) -> Result<Vec<ManagedEvent>, RuntimeFailure> {
    let mut by_id = BTreeMap::new();
    let mut ordered = Vec::new();
    for event in history.into_iter().chain(live) {
        if let Some(existing) = by_id.get(event.id()) {
            if existing != &event {
                return Err(protocol_failure("contradictory duplicate event"));
            }
            continue;
        }
        if ordered.len() == MAX_RECONCILED_EVENTS {
            return Err(protocol_failure("event reconciliation bound"));
        }
        by_id.insert(event.id.clone(), event.clone());
        ordered.push(event);
    }
    Ok(ordered)
}

fn parse_frame(frame: &str) -> Result<ManagedEvent, RuntimeFailure> {
    let mut event_name = None;
    let mut data = Vec::new();
    for line in frame.lines() {
        if line.starts_with(':') {
            continue;
        }
        if let Some(value) = line.strip_prefix("event:") {
            event_name = Some(value.trim());
        } else if let Some(value) = line.strip_prefix("data:") {
            data.push(value.trim_start());
        }
    }
    let event_name = event_name.ok_or_else(|| protocol_failure("event name"))?;
    let value: Value =
        serde_json::from_str(&data.join("\n")).map_err(|_| protocol_failure("event payload"))?;
    if value.get("type").and_then(Value::as_str) != Some(event_name) {
        return Err(protocol_failure("event type agreement"));
    }
    parse_value(value)
}

fn parse_value(value: Value) -> Result<ManagedEvent, RuntimeFailure> {
    let event_type = text(&value, "/type", "event type")?;
    if matches!(event_type.as_str(), "event_start" | "event_delta") {
        return Err(protocol_failure("non-authoritative preview event"));
    }
    let id = text(&value, "/id", "event identity")?;
    match value.get("processed_at") {
        Some(Value::String(_)) | Some(Value::Null) => {}
        _ => return Err(protocol_failure("event processed timestamp")),
    }
    let kind = match event_type.as_str() {
        "session.status_running" => ManagedEventKind::Running,
        "session.status_rescheduled" => ManagedEventKind::Rescheduled,
        "session.status_idle" => ManagedEventKind::Idle(parse_idle_reason(&value)?),
        "session.status_terminated" => ManagedEventKind::Terminated,
        "session.error" => ManagedEventKind::ProviderError,
        "agent.message" => ManagedEventKind::Message(parse_message(&value)?),
        "agent.custom_tool_use" => ManagedEventKind::CustomToolUse {
            name: text(&value, "/name", "custom tool name")?,
            input: value
                .pointer("/input")
                .cloned()
                .ok_or_else(|| protocol_failure("custom tool input"))?,
        },
        "agent.thinking"
        | "span.model_request_start"
        | "span.model_request_end"
        | "user.message"
        | "user.interrupt"
        | "user.custom_tool_result" => ManagedEventKind::Observed,
        _ => return Err(protocol_failure("unsupported event type")),
    };
    Ok(ManagedEvent {
        id,
        event_type,
        raw: value,
        kind,
    })
}

fn parse_message(value: &Value) -> Result<OperationContent, RuntimeFailure> {
    let blocks = value
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(|| protocol_failure("agent message content"))?;
    let mut output = String::new();
    for block in blocks {
        if block.get("type").and_then(Value::as_str) != Some("text") {
            return Err(protocol_failure("agent message content type"));
        }
        output.push_str(&text(block, "/text", "agent message text")?);
    }
    OperationContent::new(output).map_err(|_| protocol_failure("agent message output"))
}

fn parse_idle_reason(value: &Value) -> Result<IdleReason, RuntimeFailure> {
    match value.pointer("/stop_reason/type").and_then(Value::as_str) {
        Some("end_turn") => Ok(IdleReason::EndTurn),
        Some("retries_exhausted") => Ok(IdleReason::RetriesExhausted),
        Some("requires_action") => {
            let ids = value
                .pointer("/stop_reason/event_ids")
                .and_then(Value::as_array)
                .ok_or_else(|| protocol_failure("requires-action event identities"))?
                .iter()
                .map(|id| {
                    id.as_str()
                        .filter(|id| !id.trim().is_empty())
                        .map(str::to_owned)
                        .ok_or_else(|| protocol_failure("requires-action event identity"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            if ids.is_empty() {
                return Err(protocol_failure("empty requires-action event identities"));
            }
            Ok(IdleReason::RequiresAction(ids))
        }
        _ => Err(protocol_failure("idle stop reason")),
    }
}

fn text(value: &Value, pointer: &str, subject: &str) -> Result<String, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| protocol_failure(subject))
}

fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.protocol_invalid",
        format!("Anthropic Managed Agents {subject} was invalid"),
    )
}
