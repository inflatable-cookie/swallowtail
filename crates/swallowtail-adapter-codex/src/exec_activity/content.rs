use super::ItemIdentity;
use crate::exec_events::malformed_stream;
use semver::Version;
use serde_json::Value;
use swallowtail_core::ActivityDisclosure;
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentStream, ActivityContentUpdate, ActivityKind,
    ActivityLabel, ActivityLifecyclePhase, ActivityNamespace, ActivityStatus, RuntimeFailure,
    TaskListSnapshot,
};

mod detail;
use detail::{
    collaboration, command, file_change, mcp_tool, normalized_text, reasoning, search, text,
    todo_list,
};

pub(super) struct ItemProjection {
    pub(super) identity: ItemIdentity,
    pub(super) status: ActivityStatus,
    pub(super) label: Option<ActivityLabel>,
    pub(super) content: Option<ActivityContentUpdate>,
    pub(super) subagent: super::subagent::SubagentProjection,
}

pub(super) fn item_projection(
    item: &Value,
    phase: ActivityLifecyclePhase,
    version: &Version,
    owner_thread_id: Option<&str>,
) -> Result<ItemProjection, RuntimeFailure> {
    let item_type = required_text(item, "type")?;
    let (identity, content, allowed) = match item_type {
        "agent_message" => (
            provider_display(
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Final),
            ),
            text(item, "text", ActivityContentStream::FinalAnswerText)?,
            completion_only(phase),
        ),
        "reasoning" => (
            provider_display(ActivityKind::ReasoningSummary, None),
            reasoning(item)?,
            completion_only(phase),
        ),
        "command_execution" => (
            provider_display(ActivityKind::CommandExecution, None),
            command(item)?,
            no_updates(phase),
        ),
        "file_change" => (
            provider_display(ActivityKind::FileChange, None),
            file_change(item)?,
            completion_only(phase),
        ),
        "mcp_tool_call" => (
            provider_display(ActivityKind::ProviderOwnedTool, None),
            mcp_tool(item, phase)?,
            no_updates(phase),
        ),
        "web_search" => (
            provider_display(ActivityKind::ExternalSearch, None),
            search(item, phase)?,
            no_updates(phase),
        ),
        "collab_tool_call" if *version >= Version::new(0, 92, 0) => (
            normalized(ActivityKind::SubagentOrCollaboration),
            collaboration(item, phase)?,
            no_updates(phase),
        ),
        "todo_list" => (normalized(ActivityKind::Task), todo_list(item)?, true),
        "error" => (
            normalized(ActivityKind::WarningOrError),
            normalized_text(required_text(item, "message")?)?,
            completion_only(phase),
        ),
        unknown => (
            identity_only(ActivityKind::Unknown(
                ActivityNamespace::new(format!("codex.exec.item.{unknown}"))
                    .map_err(|_| malformed_stream())?,
            )),
            None,
            true,
        ),
    };
    if !allowed {
        return Err(malformed_stream());
    }
    let label = match item_type {
        "mcp_tool_call" => item
            .get("server")
            .and_then(Value::as_str)
            .zip(item.get("tool").and_then(Value::as_str))
            .and_then(|(server, tool)| ActivityLabel::new(format!("{server}.{tool}")).ok()),
        _ => None,
    };
    let subagent = if item_type == "collab_tool_call"
        && matches!(&identity.kind, ActivityKind::SubagentOrCollaboration)
    {
        super::subagent::collaboration(item, owner_thread_id)?
    } else {
        super::subagent::SubagentProjection::primary()
    };
    Ok(ItemProjection {
        identity,
        status: item_status(item, phase)?,
        label,
        content,
        subagent,
    })
}

pub(super) fn task_list_snapshot(item: &Value) -> Result<TaskListSnapshot, RuntimeFailure> {
    detail::task_list_snapshot(item)
}

fn item_status(
    item: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<ActivityStatus, RuntimeFailure> {
    let status = match item.get("status").and_then(Value::as_str) {
        Some("pending") => ActivityStatus::Pending,
        Some("in_progress") => ActivityStatus::InProgress,
        Some("completed") => ActivityStatus::Completed,
        Some("failed" | "error") => ActivityStatus::Failed,
        Some("cancelled") => ActivityStatus::Cancelled,
        Some(_) => return Err(malformed_stream()),
        None if phase == ActivityLifecyclePhase::Completed => ActivityStatus::Completed,
        None => ActivityStatus::InProgress,
    };
    if (phase == ActivityLifecyclePhase::Completed) == status.is_terminal() {
        Ok(status)
    } else {
        Err(malformed_stream())
    }
}

fn provider_display(
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
) -> ItemIdentity {
    ItemIdentity {
        kind,
        assistant_phase,
        disclosure: ActivityDisclosure::ProviderDisplayContent,
    }
}

fn normalized(kind: ActivityKind) -> ItemIdentity {
    ItemIdentity {
        kind,
        assistant_phase: None,
        disclosure: ActivityDisclosure::AdapterNormalizedSummary,
    }
}

fn identity_only(kind: ActivityKind) -> ItemIdentity {
    ItemIdentity {
        kind,
        assistant_phase: None,
        disclosure: ActivityDisclosure::IdentityAndLifecycleOnly,
    }
}

fn completion_only(phase: ActivityLifecyclePhase) -> bool {
    phase == ActivityLifecyclePhase::Completed
}

fn no_updates(phase: ActivityLifecyclePhase) -> bool {
    phase != ActivityLifecyclePhase::Updated
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_stream)
}
