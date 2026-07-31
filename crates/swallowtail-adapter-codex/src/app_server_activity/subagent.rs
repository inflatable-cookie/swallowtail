use crate::turn_state::malformed_notification;
use serde_json::{Map, Value};
use swallowtail_core::{ModelId, ReasoningMode, SubagentControlActionKind};
use swallowtail_runtime::{
    ActivityActor, ActivityContent, ActivityLabel, OperationContent, RuntimeFailure, SubagentId,
    SubagentParent, SubagentSnapshot, SubagentStatus,
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

    pub(super) fn lifecycle(child: SubagentId, status: SubagentStatus) -> Self {
        Self {
            actor: ActivityActor::Subagent(child.clone()),
            snapshots: vec![SubagentSnapshot::new(
                child,
                SubagentParent::Unknown,
                status,
            )],
            control: None,
        }
    }
}

pub(super) fn collaboration(
    item: &Value,
    owner_thread_id: &str,
) -> Result<SubagentProjection, RuntimeFailure> {
    let sender = required_text(item, "senderThreadId")?;
    let parent = if sender == owner_thread_id {
        SubagentParent::Operation
    } else {
        SubagentParent::Subagent(subagent_id(sender)?)
    };
    let action = control_action(required_text(item, "tool")?)?;
    let states = item
        .get("agentsStates")
        .and_then(Value::as_object)
        .ok_or_else(malformed_notification)?;
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
        .map(|value| ModelId::new(value).map_err(|_| malformed_notification()))
        .transpose()?;
    let reasoning = item
        .get("reasoningEffort")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| ReasoningMode::new(value).map_err(|_| malformed_notification()))
        .transpose()?;
    let receivers = item
        .get("receiverThreadIds")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?;
    if receivers.len() > 64 {
        return Err(malformed_notification());
    }
    let snapshots = receivers
        .iter()
        .map(|receiver| {
            let receiver = receiver.as_str().ok_or_else(malformed_notification)?;
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
        actor: if sender == owner_thread_id {
            ActivityActor::Primary
        } else {
            ActivityActor::Subagent(subagent_id(sender)?)
        },
        snapshots,
        control: Some(action),
    })
}

pub(super) fn activity(item: &Value) -> Result<SubagentProjection, RuntimeFailure> {
    let child = subagent_id(required_text(item, "agentThreadId")?)?;
    let kind = required_text(item, "kind")?;
    let status = match kind {
        "started" | "interacted" => SubagentStatus::Running,
        "interrupted" => SubagentStatus::Interrupted,
        _ => return Err(malformed_notification()),
    };
    let mut snapshot = SubagentSnapshot::new(child.clone(), SubagentParent::Unknown, status);
    if let Some(path) = item
        .get("agentPath")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
    {
        snapshot =
            snapshot.with_label(ActivityLabel::new(path).map_err(|_| malformed_notification())?);
    }
    Ok(SubagentProjection {
        actor: ActivityActor::Subagent(child),
        snapshots: vec![snapshot],
        control: None,
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
        "pendingInit" => SubagentStatus::Pending,
        "running" => SubagentStatus::Running,
        "interrupted" => SubagentStatus::Interrupted,
        "completed" => SubagentStatus::Completed,
        "errored" | "notFound" => SubagentStatus::Failed,
        "shutdown" => SubagentStatus::Shutdown,
        _ => SubagentStatus::Unknown,
    }
}

fn control_action(value: &str) -> Result<SubagentControlActionKind, RuntimeFailure> {
    match value {
        "spawnAgent" => Ok(SubagentControlActionKind::Spawn),
        "sendInput" => Ok(SubagentControlActionKind::SendInput),
        "resumeAgent" => Ok(SubagentControlActionKind::Resume),
        "wait" => Ok(SubagentControlActionKind::Wait),
        "closeAgent" => Ok(SubagentControlActionKind::Close),
        _ => Err(malformed_notification()),
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
        OperationContent::new(value).map_err(|_| malformed_notification())?,
        MAX_DESCRIPTION_BYTES,
    )
    .map_err(|_| malformed_notification())
}

fn subagent_id(value: &str) -> Result<SubagentId, RuntimeFailure> {
    SubagentId::new(value).map_err(|_| malformed_notification())
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_notification)
}
