use super::{bounded, normalized, required_text, text_field};
use crate::turn_state::malformed_notification;
use serde_json::Value;
use swallowtail_runtime::{
    ActivityContentChangeKind, ActivityContentStream, ActivityContentUpdate,
    ActivityLifecyclePhase, RuntimeFailure,
};

pub(in crate::app_server_activity) fn mcp_tool(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut lines = Vec::new();
    if lifecycle == ActivityLifecyclePhase::Completed {
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
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(in crate::app_server_activity) fn dynamic_tool(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let mut lines = Vec::new();
    if lifecycle == ActivityLifecyclePhase::Completed
        && let Some(contents) = item.get("contentItems").and_then(Value::as_array)
    {
        for entry in contents {
            if let Some(text) = entry.get("text").and_then(Value::as_str) {
                lines.push(text.to_owned());
            }
        }
    }
    bounded(
        &lines.join("\n"),
        ActivityContentStream::ProviderToolDisplay,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(in crate::app_server_activity) fn search(
    item: &Value,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    text_field(
        item,
        "query",
        ActivityContentStream::ProviderToolDisplay,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(in crate::app_server_activity) fn image_view(
    item: &Value,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    text_field(
        item,
        "path",
        ActivityContentStream::ProviderToolDisplay,
        ActivityContentChangeKind::ReplacementSnapshot,
    )
}

pub(in crate::app_server_activity) fn collaboration(
    item: &Value,
    lifecycle: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let tool = required_text(item, "tool")?;
    let summary = if lifecycle == ActivityLifecyclePhase::Completed {
        format!("collaboration {tool} completed")
    } else {
        format!("collaboration {tool} started")
    };
    normalized(&summary)
}

pub(in crate::app_server_activity) fn subagent(
    item: &Value,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    normalized(&format!("subagent {}", required_text(item, "kind")?))
}

pub(in crate::app_server_activity) fn review(
    item: &Value,
    item_type: &str,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let transition = if item_type == "enteredReviewMode" {
        "entered review"
    } else {
        "exited review"
    };
    let review = item
        .get("review")
        .and_then(Value::as_str)
        .unwrap_or_default();
    normalized(&format!("{transition}: {review}"))
}

pub(in crate::app_server_activity) fn hook_prompt(
    item: &Value,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let fragments = item
        .get("fragments")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?;
    let text = fragments
        .iter()
        .map(|fragment| required_text(fragment, "text"))
        .collect::<Result<Vec<_>, _>>()?
        .join("\n");
    normalized(&text)
}

pub(in crate::app_server_activity) fn hook_summary(
    run: &Value,
    phase: ActivityLifecyclePhase,
) -> Result<Option<ActivityContentUpdate>, RuntimeFailure> {
    let event = required_text(run, "eventName")?;
    let state = if phase == ActivityLifecyclePhase::Completed {
        "completed"
    } else {
        "started"
    };
    let mut summary = format!("hook {event} {state}");
    if let Some(duration) = run.get("durationMs").and_then(Value::as_u64) {
        summary.push_str(&format!(" in {duration} ms"));
    }
    normalized(&summary)
}
