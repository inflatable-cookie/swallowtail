use crate::exec_events::malformed_stream;
use serde_json::{Map, Value};
use swallowtail_core::{ModelId, ReasoningMode, SubagentControlActionKind};
use swallowtail_runtime::{
    ActivityActor, ActivityContent, OperationContent, RuntimeFailure, SubagentId, SubagentParent,
    SubagentSnapshot, SubagentStatus,
};

const MAX_DESCRIPTION_BYTES: usize = 16 * 1024;

pub(super) struct SubagentProjection {
    pub(super) actor: ActivityActor,
    pub(super) snapshots: Vec<SubagentSnapshot>,
    pub(super) control: Option<SubagentControlActionKind>,
}

impl SubagentProjection {
    pub(super) const fn primary() -> Self {
        Self {
            actor: ActivityActor::Primary,
            snapshots: Vec::new(),
            control: None,
        }
    }
}

pub(super) fn collaboration(
    item: &Value,
    owner_thread_id: Option<&str>,
) -> Result<SubagentProjection, RuntimeFailure> {
    let sender = required_text(item, "sender_thread_id")?;
    let parent = match owner_thread_id {
        Some(owner) if owner == sender => SubagentParent::Operation,
        Some(_) => SubagentParent::Subagent(subagent_id(sender)?),
        None => SubagentParent::Unknown,
    };
    let action = control_action(required_text(item, "tool")?)?;
    let states = item
        .get("agents_states")
        .and_then(Value::as_object)
        .ok_or_else(malformed_stream)?;
    let description = item
        .get("prompt")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(bounded_description)
        .transpose()?;
    let model = item
        .get("model")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| ModelId::new(value).map_err(|_| malformed_stream()))
        .transpose()?;
    let reasoning = item
        .get("reasoning_effort")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| ReasoningMode::new(value).map_err(|_| malformed_stream()))
        .transpose()?;
    let receivers = item
        .get("receiver_thread_ids")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?;
    if receivers.len() > 64 {
        return Err(malformed_stream());
    }
    let snapshots = receivers
        .iter()
        .map(|receiver| {
            let receiver = receiver.as_str().ok_or_else(malformed_stream)?;
            let mut snapshot = SubagentSnapshot::new(
                subagent_id(receiver)?,
                parent.clone(),
                state(states, receiver, action),
            );
            if let Some(description) = description.clone() {
                snapshot = snapshot.with_description(description);
            }
            if let Some(model) = model.clone() {
                snapshot = snapshot.with_model(model);
            }
            if let Some(reasoning) = reasoning.clone() {
                snapshot = snapshot.with_reasoning(reasoning);
            }
            Ok(snapshot)
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    Ok(SubagentProjection {
        actor: match owner_thread_id {
            Some(owner) if owner == sender => ActivityActor::Primary,
            _ => ActivityActor::Subagent(subagent_id(sender)?),
        },
        snapshots,
        control: Some(action),
    })
}

fn state(
    states: &Map<String, Value>,
    receiver: &str,
    action: SubagentControlActionKind,
) -> SubagentStatus {
    let Some(status) = states
        .get(receiver)
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
    else {
        return if action == SubagentControlActionKind::Spawn {
            SubagentStatus::Pending
        } else {
            SubagentStatus::Unknown
        };
    };
    match status {
        "pending_init" => SubagentStatus::Pending,
        "running" => SubagentStatus::Running,
        "interrupted" => SubagentStatus::Interrupted,
        "completed" => SubagentStatus::Completed,
        "errored" | "not_found" => SubagentStatus::Failed,
        "shutdown" => SubagentStatus::Shutdown,
        _ => SubagentStatus::Unknown,
    }
}

fn control_action(value: &str) -> Result<SubagentControlActionKind, RuntimeFailure> {
    match value {
        "spawn_agent" => Ok(SubagentControlActionKind::Spawn),
        "send_input" => Ok(SubagentControlActionKind::SendInput),
        "resume_agent" => Ok(SubagentControlActionKind::Resume),
        "wait" => Ok(SubagentControlActionKind::Wait),
        "close_agent" => Ok(SubagentControlActionKind::Close),
        _ => Err(malformed_stream()),
    }
}

fn bounded_description(value: &str) -> Result<ActivityContent, RuntimeFailure> {
    let end = value
        .char_indices()
        .map(|(index, _)| index)
        .take_while(|index| *index <= MAX_DESCRIPTION_BYTES)
        .last()
        .unwrap_or(0);
    let value = if value.len() <= MAX_DESCRIPTION_BYTES {
        value
    } else {
        &value[..end]
    };
    ActivityContent::new(
        OperationContent::new(value).map_err(|_| malformed_stream())?,
        MAX_DESCRIPTION_BYTES,
    )
    .map_err(|_| malformed_stream())
}

fn subagent_id(value: &str) -> Result<SubagentId, RuntimeFailure> {
    SubagentId::new(value).map_err(|_| malformed_stream())
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_stream)
}
