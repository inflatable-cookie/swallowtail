use crate::rpc::failure;
use serde_json::Value;
use swallowtail_core::SessionRef;
use swallowtail_runtime::{OperationContent, RuntimeFailure, SessionReplayItem, SessionReplayKind};

pub(crate) const MAXIMUM_REPLAY_TURNS: usize = 4096;
pub(crate) const MAXIMUM_REPLAY_ITEMS: usize = 16_384;
pub(crate) const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;

pub(crate) fn validate_thread_history_bounds(response: &Value) -> Result<(), RuntimeFailure> {
    let Some(turns) = response
        .get("thread")
        .and_then(|thread| thread.get("turns"))
    else {
        return Ok(());
    };
    let turns = turns.as_array().ok_or_else(malformed_replay)?;
    if turns.len() > MAXIMUM_REPLAY_TURNS {
        return Err(replay_limit());
    }
    let mut items = 0usize;
    let mut bytes = 0usize;
    for turn in turns {
        let turn_items = turn
            .get("items")
            .and_then(Value::as_array)
            .ok_or_else(malformed_replay)?;
        items = items.saturating_add(turn_items.len());
        if items > MAXIMUM_REPLAY_ITEMS {
            return Err(replay_limit());
        }
        for item in turn_items {
            bytes = bytes.saturating_add(
                serde_json::to_vec(item)
                    .map_err(|_| malformed_replay())?
                    .len(),
            );
            if bytes > MAXIMUM_REPLAY_BYTES {
                return Err(replay_limit());
            }
        }
    }
    Ok(())
}

pub(crate) fn project_thread_history(
    response: &Value,
    session: &SessionRef,
) -> Result<Vec<SessionReplayItem>, RuntimeFailure> {
    validate_thread_history_bounds(response)?;
    let turns = response
        .get("thread")
        .and_then(|thread| thread.get("turns"))
        .and_then(Value::as_array)
        .ok_or_else(malformed_replay)?;
    let mut replay = Vec::new();
    for turn in turns {
        let items = turn
            .get("items")
            .and_then(Value::as_array)
            .ok_or_else(malformed_replay)?;
        for item in items {
            replay.push(project_item(
                item,
                session,
                u64::try_from(replay.len()).map_err(|_| replay_limit())?,
            )?);
        }
    }
    Ok(replay)
}

fn project_item(
    item: &Value,
    session: &SessionRef,
    sequence: u64,
) -> Result<SessionReplayItem, RuntimeFailure> {
    let kind = item
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(malformed_replay)?;
    let (kind, text) = match kind {
        "userMessage" => (
            SessionReplayKind::UserMessage,
            text_content(item.get("content"))?,
        ),
        "agentMessage" => (
            SessionReplayKind::AgentMessage,
            optional_text(item, "text")?,
        ),
        "reasoning" => (
            SessionReplayKind::AgentReasoning,
            string_arrays(item, &["summary", "content"])?,
        ),
        "plan" => (SessionReplayKind::Plan, optional_text(item, "text")?),
        "commandExecution"
        | "mcpToolCall"
        | "dynamicToolCall"
        | "collabAgentToolCall"
        | "webSearch" => (SessionReplayKind::ToolCall, None),
        "fileChange" | "subAgentActivity" => (SessionReplayKind::ToolCallUpdate, None),
        "hookPrompt" | "imageView" | "imageGeneration" | "sleep" | "enteredReviewMode"
        | "exitedReviewMode" | "contextCompaction" => (SessionReplayKind::Configuration, None),
        _ => return Err(unsupported_replay()),
    };
    match text {
        Some(text) if !text.is_empty() => Ok(SessionReplayItem::with_content(
            session.clone(),
            sequence,
            kind,
            OperationContent::new(text).map_err(|_| malformed_replay())?,
        )),
        _ => Ok(SessionReplayItem::new(session.clone(), sequence, kind)),
    }
}

fn text_content(value: Option<&Value>) -> Result<Option<String>, RuntimeFailure> {
    let values = value
        .and_then(Value::as_array)
        .ok_or_else(malformed_replay)?;
    let text = values
        .iter()
        .filter_map(|value| {
            (value.get("type").and_then(Value::as_str) == Some("text"))
                .then(|| value.get("text").and_then(Value::as_str))
                .flatten()
        })
        .collect::<Vec<_>>();
    Ok((!text.is_empty()).then(|| text.join("\n")))
}

fn optional_text(value: &Value, field: &str) -> Result<Option<String>, RuntimeFailure> {
    value
        .get(field)
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or_else(malformed_replay)
        })
        .transpose()
}

fn string_arrays(value: &Value, fields: &[&str]) -> Result<Option<String>, RuntimeFailure> {
    let mut text = Vec::new();
    for field in fields {
        let Some(values) = value.get(*field) else {
            continue;
        };
        for value in values.as_array().ok_or_else(malformed_replay)? {
            text.push(value.as_str().ok_or_else(malformed_replay)?);
        }
    }
    Ok((!text.is_empty()).then(|| text.join("\n")))
}

fn malformed_replay() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.replay_malformed",
        "Codex app-server returned malformed session history",
    )
}

fn unsupported_replay() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.replay_unsupported",
        "Codex app-server returned unsupported session history",
    )
}

fn replay_limit() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.replay_limit_exceeded",
        "Codex app-server session history exceeded the adapter limit",
    )
}
