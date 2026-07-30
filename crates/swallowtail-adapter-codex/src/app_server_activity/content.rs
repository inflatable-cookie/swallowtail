use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentStream,
    ActivityContentUpdate, ActivityLifecyclePhase, OperationContent, RuntimeFailure, TaskListItem,
    TaskListItemStatus, TaskListSnapshot,
};

mod detail;
pub(super) use detail::{
    collaboration, dynamic_tool, hook_prompt, hook_summary, image_view, mcp_tool, review, search,
    subagent,
};

const MAX_ACTIVITY_CONTENT_BYTES: usize = 1_048_576;

pub(super) fn text_delta(
    params: &Value,
    stream: ActivityContentStream,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    text_field(params, "delta", stream, ActivityContentChangeKind::Delta)
}

pub(super) fn text_field(
    value: &Value,
    field: &str,
    stream: ActivityContentStream,
    change: ActivityContentChangeKind,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let text = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(malformed_notification)?;
    bounded(text, stream, change)
}

pub(super) fn assistant_message(
    item: &Value,
    phase: ActivityAssistantPhase,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if lifecycle != ActivityLifecyclePhase::Completed
        || phase == ActivityAssistantPhase::ProviderUnspecified
    {
        return Ok(None);
    }
    let stream = match phase {
        ActivityAssistantPhase::Intermediate => ActivityContentStream::IntermediateAssistantText,
        ActivityAssistantPhase::Final => ActivityContentStream::FinalAnswerText,
        ActivityAssistantPhase::ProviderUnspecified => unreachable!(),
    };
    text_field(
        item,
        "text",
        stream,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(super) fn reasoning_summary(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if lifecycle != ActivityLifecyclePhase::Completed {
        return Ok(None);
    }
    let summary = item
        .get("summary")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?
        .iter()
        .map(|part| part.as_str().ok_or_else(malformed_notification))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    bounded(
        &summary,
        ActivityContentStream::ReasoningSummaryText,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(super) fn plan_item(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if lifecycle != ActivityLifecyclePhase::Completed {
        return Ok(None);
    }
    text_field(
        item,
        "text",
        ActivityContentStream::PlanText,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(super) fn plan_snapshot(
    params: &Value,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let plan = params
        .get("plan")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?;
    let mut lines = Vec::with_capacity(plan.len() + 1);
    if let Some(explanation) = params.get("explanation").and_then(Value::as_str)
        && !explanation.trim().is_empty()
    {
        lines.push(explanation.to_owned());
    }
    for entry in plan {
        let step = required_text(entry, "step")?;
        let status = required_text(entry, "status")?;
        lines.push(format!("[{status}] {step}"));
    }
    bounded(
        &lines.join("\n"),
        ActivityContentStream::PlanText,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(super) fn task_list_snapshot(params: &Value) -> Result<TaskListSnapshot, RuntimeFailure> {
    let items = params
        .get("plan")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?
        .iter()
        .map(|entry| {
            let status = match required_text(entry, "status")? {
                "pending" => TaskListItemStatus::Pending,
                "inProgress" => TaskListItemStatus::InProgress,
                "completed" => TaskListItemStatus::Completed,
                _ => return Err(malformed_notification()),
            };
            Ok(TaskListItem::new(
                OperationContent::new(required_text(entry, "step")?)
                    .map_err(|_| malformed_notification())?,
                status,
            ))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    TaskListSnapshot::new(items, 256, MAX_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| malformed_notification())
}

pub(super) fn command(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let command = required_text(item, "command")?;
    let mut lines = vec![format!("command: {command}")];
    if let Some(cwd) = item.get("cwd").and_then(Value::as_str) {
        lines.push(format!("cwd: {cwd}"));
    }
    if let Some(output) = item.get("aggregatedOutput").and_then(Value::as_str)
        && !output.is_empty()
    {
        lines.push(format!("output:\n{output}"));
    }
    if lifecycle == ActivityLifecyclePhase::Completed {
        if let Some(exit) = item.get("exitCode").and_then(Value::as_i64) {
            lines.push(format!("exit_status: {exit}"));
        }
        if let Some(duration) = item.get("durationMs").and_then(Value::as_u64) {
            lines.push(format!("duration_ms: {duration}"));
        }
    }
    bounded(
        &lines.join("\n"),
        ActivityContentStream::CommandOutput,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(super) fn file_changes(
    changes: Option<&Value>,
    change: ActivityContentChangeKind,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let changes = changes
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?;
    let mut parts = Vec::with_capacity(changes.len());
    for entry in changes {
        let path = required_text(entry, "path")?;
        let kind = required_text(entry, "kind")?;
        let diff = entry
            .get("diff")
            .and_then(Value::as_str)
            .unwrap_or_default();
        parts.push(format!("{kind}: {path}\n{diff}"));
    }
    bounded(
        &parts.join("\n"),
        ActivityContentStream::FileChangeOutput,
        change,
    )
}

fn normalized(text: &str) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    bounded(
        text,
        ActivityContentStream::NormalizedSummary,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

fn bounded(
    text: &str,
    stream: ActivityContentStream,
    change: ActivityContentChangeKind,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let operation = OperationContent::new(text).map_err(|_| malformed_notification())?;
    let content = ActivityContent::new(operation, MAX_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| malformed_notification())?;
    Ok(Some(ActivityContentUpdate::new(change, stream, content)))
}

fn required_text<'a>(value: &'a Value, field: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_notification)
}
