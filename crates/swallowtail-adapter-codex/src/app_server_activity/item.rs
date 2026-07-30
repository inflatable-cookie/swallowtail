use super::content;
use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_core::ActivityDisclosure;
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityStatus, RuntimeFailure,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ItemIdentity {
    pub(super) kind: ActivityKind,
    pub(super) assistant_phase: Option<ActivityAssistantPhase>,
    pub(super) disclosure: ActivityDisclosure,
}

impl ItemIdentity {
    pub(super) const fn new(
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Self {
        Self {
            kind,
            assistant_phase,
            disclosure,
        }
    }
}

pub(super) struct ItemProjection {
    pub(super) identity: ItemIdentity,
    pub(super) status: ActivityStatus,
    pub(super) label: Option<ActivityLabel>,
    pub(super) content: Option<swallowtail_runtime::ActivityContentUpdate>,
    pub(super) subagent: super::subagent::SubagentProjection,
}

pub(super) fn item_projection(
    item: &Value,
    phase: ActivityLifecyclePhase,
    owner_thread_id: Option<&str>,
) -> Result<ItemProjection, RuntimeFailure> {
    let item_type = item
        .get("type")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_notification)?;
    let status = item_status(item, phase)?;
    let (identity, content) = match item_type {
        "agentMessage" => {
            let assistant_phase = match item.get("phase").and_then(Value::as_str) {
                Some("commentary") => ActivityAssistantPhase::Intermediate,
                Some("final_answer") => ActivityAssistantPhase::Final,
                Some(_) | None => ActivityAssistantPhase::ProviderUnspecified,
            };
            let disclosure = if assistant_phase == ActivityAssistantPhase::ProviderUnspecified {
                ActivityDisclosure::IdentityAndLifecycleOnly
            } else {
                ActivityDisclosure::ProviderDisplayContent
            };
            (
                ItemIdentity::new(
                    ActivityKind::AssistantMessage,
                    Some(assistant_phase),
                    disclosure,
                ),
                content::assistant_message(item, assistant_phase, phase)?,
            )
        }
        "reasoning" => (
            provider_display(ActivityKind::ReasoningSummary),
            content::reasoning_summary(item, phase)?,
        ),
        "plan" => (
            provider_display(ActivityKind::Plan),
            content::plan_item(item, phase)?,
        ),
        "commandExecution" => (
            provider_display(ActivityKind::CommandExecution),
            content::command(item, phase)?,
        ),
        "fileChange" => (
            provider_display(ActivityKind::FileChange),
            content::file_changes(
                item.get("changes"),
                swallowtail_runtime::ActivityContentChangeKind::ReplacementSnapshot,
            )?,
        ),
        "mcpToolCall" => (
            provider_display(ActivityKind::ProviderOwnedTool),
            content::mcp_tool(item, phase)?,
        ),
        "dynamicToolCall" => (
            provider_display(ActivityKind::ConsumerOwnedTool),
            content::dynamic_tool(item, phase)?,
        ),
        "webSearch" => (
            provider_display(ActivityKind::ExternalSearch),
            content::search(item)?,
        ),
        "imageView" => (
            provider_display(ActivityKind::ImageView),
            content::image_view(item)?,
        ),
        "collabAgentToolCall" => (
            normalized(ActivityKind::SubagentOrCollaboration),
            content::collaboration(item, phase)?,
        ),
        "subAgentActivity" => (
            normalized(ActivityKind::SubagentOrCollaboration),
            content::subagent(item)?,
        ),
        "enteredReviewMode" | "exitedReviewMode" => (
            normalized(ActivityKind::ReviewTransition),
            content::review(item, item_type)?,
        ),
        "contextCompaction" => (identity_only(ActivityKind::ContextCompaction), None),
        "hookPrompt" => (normalized(ActivityKind::Hook), content::hook_prompt(item)?),
        unknown => (
            identity_only(ActivityKind::Unknown(
                ActivityNamespace::new(format!("codex.app-server.item.{unknown}"))
                    .map_err(|_| malformed_notification())?,
            )),
            None,
        ),
    };
    let label = match item_type {
        "mcpToolCall" => item
            .get("server")
            .and_then(Value::as_str)
            .zip(item.get("tool").and_then(Value::as_str))
            .and_then(|(server, tool)| ActivityLabel::new(format!("{server}.{tool}")).ok()),
        "dynamicToolCall" => item
            .get("tool")
            .and_then(Value::as_str)
            .and_then(|tool| ActivityLabel::new(tool).ok()),
        _ => None,
    };
    let subagent = match item_type {
        "collabAgentToolCall" => super::subagent::collaboration(
            item,
            owner_thread_id.ok_or_else(malformed_notification)?,
        )?,
        "subAgentActivity" => super::subagent::activity(item)?,
        _ => super::subagent::SubagentProjection::primary(),
    };
    Ok(ItemProjection {
        identity,
        status,
        label,
        content,
        subagent,
    })
}

fn item_status(
    item: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<ActivityStatus, RuntimeFailure> {
    let supplied = item.get("status").and_then(Value::as_str);
    let status = match supplied {
        Some("pending") => ActivityStatus::Pending,
        Some("inProgress" | "running") => ActivityStatus::InProgress,
        Some("completed" | "success") => ActivityStatus::Completed,
        Some("failed" | "error") => ActivityStatus::Failed,
        Some("cancelled" | "declined") => ActivityStatus::Cancelled,
        Some(_) => return Err(malformed_notification()),
        None if phase == ActivityLifecyclePhase::Completed => ActivityStatus::Completed,
        None => ActivityStatus::InProgress,
    };
    if (phase == ActivityLifecyclePhase::Completed) == status.is_terminal() {
        Ok(status)
    } else {
        Err(malformed_notification())
    }
}

fn provider_display(kind: ActivityKind) -> ItemIdentity {
    ItemIdentity::new(kind, None, ActivityDisclosure::ProviderDisplayContent)
}

fn normalized(kind: ActivityKind) -> ItemIdentity {
    ItemIdentity::new(kind, None, ActivityDisclosure::AdapterNormalizedSummary)
}

fn identity_only(kind: ActivityKind) -> ItemIdentity {
    ItemIdentity::new(kind, None, ActivityDisclosure::IdentityAndLifecycleOnly)
}
