use super::required_text;
use crate::exec_events::malformed_stream;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityContent, ActivityContentChangeKind, ActivityContentStream, ActivityContentUpdate,
    ActivityLifecyclePhase, OperationContent, RuntimeFailure, TaskListItem, TaskListItemStatus,
    TaskListSnapshot,
};

const MAX_ACTIVITY_CONTENT_BYTES: usize = 1_048_576;

pub(super) fn command(item: &Value) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut lines = vec![format!("command: {}", required_text(item, "command")?)];
    if let Some(output) = item.get("aggregated_output").and_then(Value::as_str)
        && !output.is_empty()
    {
        lines.push(format!("output:\n{output}"));
    }
    if let Some(exit) = item.get("exit_code").and_then(Value::as_i64) {
        lines.push(format!("exit_status: {exit}"));
    }
    bounded(&lines.join("\n"), ActivityContentStream::CommandOutput)
}

pub(super) fn file_change(item: &Value) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let changes = item
        .get("changes")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?;
    let lines = changes
        .iter()
        .map(|change| {
            Ok(format!(
                "{}: {}",
                required_text(change, "kind")?,
                required_text(change, "path")?
            ))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    bounded(&lines.join("\n"), ActivityContentStream::FileChangeOutput)
}

pub(super) fn mcp_tool(
    item: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut lines = Vec::new();
    if phase == ActivityLifecyclePhase::Completed {
        if let Some(contents) = item.pointer("/result/content").and_then(Value::as_array) {
            for entry in contents {
                if entry.get("type").and_then(Value::as_str) == Some("text") {
                    lines.push(required_text(entry, "text")?.to_owned());
                }
            }
        }
        if let Some(error) = item.get("error").and_then(Value::as_str) {
            lines.push(error.to_owned());
        }
    }
    bounded(
        &lines.join("\n"),
        ActivityContentStream::ProviderToolDisplay,
    )
}

pub(super) fn search(
    item: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut query = optional_text(item.get("query"))?.filter(|query| !query.trim().is_empty());
    if query.is_none() {
        query =
            optional_text(item.pointer("/action/query"))?.filter(|query| !query.trim().is_empty());
    }
    match query {
        Some(query) => bounded(query, ActivityContentStream::ProviderToolDisplay),
        None if phase == ActivityLifecyclePhase::Started => Ok(None),
        None if phase == ActivityLifecyclePhase::Completed
            && item.pointer("/action/type").and_then(Value::as_str) == Some("other") =>
        {
            Ok(None)
        }
        None => Err(malformed_stream()),
    }
}

fn optional_text(value: Option<&Value>) -> Result<Option<&str>, RuntimeFailure> {
    value
        .map(|value| value.as_str().ok_or_else(malformed_stream))
        .transpose()
}

pub(super) fn reasoning(item: &Value) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let summary = item
        .get("text")
        .or_else(|| item.get("summary"))
        .and_then(Value::as_str)
        .filter(|summary| !summary.trim().is_empty())
        .ok_or_else(malformed_stream)?;
    bounded(summary, ActivityContentStream::ReasoningSummaryText)
}

pub(super) fn collaboration(
    item: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let state = if phase == ActivityLifecyclePhase::Completed {
        "completed"
    } else {
        "started"
    };
    normalized_text(&format!(
        "collaboration {} {state}",
        required_text(item, "tool")?
    ))
}

pub(super) fn todo_list(item: &Value) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let items = item
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?;
    let lines = items
        .iter()
        .map(|entry| {
            let marker = if entry
                .get("completed")
                .and_then(Value::as_bool)
                .ok_or_else(malformed_stream)?
            {
                "x"
            } else {
                " "
            };
            Ok(format!("[{marker}] {}", required_text(entry, "text")?))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    normalized_text(&lines.join("\n"))
}

pub(super) fn task_list_snapshot(item: &Value) -> Result<TaskListSnapshot, RuntimeFailure> {
    let items = item
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(malformed_stream)?
        .iter()
        .map(|entry| {
            let status = if entry
                .get("completed")
                .and_then(Value::as_bool)
                .ok_or_else(malformed_stream)?
            {
                TaskListItemStatus::Completed
            } else {
                TaskListItemStatus::Pending
            };
            Ok(TaskListItem::new(
                OperationContent::new(required_text(entry, "text")?)
                    .map_err(|_| malformed_stream())?,
                status,
            ))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    TaskListSnapshot::new(items, 256, MAX_ACTIVITY_CONTENT_BYTES).map_err(|_| malformed_stream())
}

pub(super) fn text(
    item: &Value,
    field: &str,
    stream: ActivityContentStream,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    bounded(required_text(item, field)?, stream)
}

pub(super) fn normalized_text(text: &str) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    bounded(text, ActivityContentStream::NormalizedSummary)
}

fn bounded(
    text: &str,
    stream: ActivityContentStream,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    if text.trim().is_empty() {
        return Ok(None);
    }
    let operation = OperationContent::new(text).map_err(|_| malformed_stream())?;
    let content = ActivityContent::new(operation, MAX_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| malformed_stream())?;
    Ok(Some(ActivityContentUpdate::new(
        ActivityContentChangeKind::ReplacementSnapshot,
        stream,
        content,
    )))
}
