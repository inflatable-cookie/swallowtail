fn decode_event(
    event_type: &str,
    payload: &serde_json::Map<String, Value>,
) -> Result<WsEvent, RuntimeFailure> {
    match event_type {
        "turn.started" => Ok(WsEvent::TurnStarted {
            turn_id: required_u64(payload, "turnId")?,
        }),
        "assistant.delta" => Ok(WsEvent::AssistantDelta {
            turn_id: required_u64(payload, "turnId")?,
            delta: payload
                .get("delta")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?
                .to_owned(),
        }),
        "thinking.delta" => Ok(WsEvent::ThinkingDelta {
            turn_id: required_u64(payload, "turnId")?,
            delta: payload
                .get("delta")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?
                .to_owned(),
        }),
        "turn.step.started" => Ok(WsEvent::StepStarted {
            turn_id: required_u64(payload, "turnId")?,
            step: required_u64(payload, "step")?,
            step_id: optional_string(payload, "stepId")?,
        }),
        "turn.step.completed" | "turn.step.interrupted" => Ok(WsEvent::StepEnded {
            turn_id: required_u64(payload, "turnId")?,
            step: required_u64(payload, "step")?,
            step_id: optional_string(payload, "stepId")?,
            failed: event_type == "turn.step.interrupted",
        }),
        "tool.call.delta" | "tool.progress" => Ok(WsEvent::ToolUpdated {
            turn_id: required_u64(payload, "turnId")?,
            call_id: required_string(payload, "toolCallId")?.to_owned(),
        }),
        "tool.call.started" => Ok(WsEvent::ToolStarted {
            turn_id: required_u64(payload, "turnId")?,
            call_id: required_string(payload, "toolCallId")?.to_owned(),
            name: required_string(payload, "name")?.to_owned(),
        }),
        "tool.result" => Ok(WsEvent::ToolEnded {
            turn_id: required_u64(payload, "turnId")?,
            call_id: required_string(payload, "toolCallId")?.to_owned(),
            failed: optional_bool(payload, "isError")?.unwrap_or(false),
        }),
        "shell.started" => Ok(WsEvent::ShellStarted {
            command_id: required_string(payload, "commandId")?.to_owned(),
        }),
        "shell.output" => Ok(WsEvent::ShellUpdated {
            command_id: required_string(payload, "commandId")?.to_owned(),
        }),
        "shell.completed" => Ok(WsEvent::ShellEnded {
            command_id: required_string(payload, "commandId")?.to_owned(),
            failed: payload
                .get("isError")
                .and_then(Value::as_bool)
                .ok_or_else(malformed)?,
        }),
        "subagent.spawned" => Ok(WsEvent::SubagentSpawned {
            subagent_id: required_string(payload, "subagentId")?.to_owned(),
            name: required_string(payload, "subagentName")?.to_owned(),
            parent_tool_call_id: required_string(payload, "parentToolCallId")?.to_owned(),
            background: payload
                .get("runInBackground")
                .and_then(Value::as_bool)
                .ok_or_else(malformed)?,
        }),
        "subagent.started" | "subagent.suspended" => Ok(WsEvent::SubagentUpdated {
            subagent_id: required_string(payload, "subagentId")?.to_owned(),
            suspended: event_type == "subagent.suspended",
        }),
        "subagent.completed" | "subagent.failed" => Ok(WsEvent::SubagentEnded {
            subagent_id: required_string(payload, "subagentId")?.to_owned(),
            failed: event_type == "subagent.failed",
        }),
        "compaction.started" => Ok(WsEvent::CompactionStarted),
        "compaction.completed" | "compaction.blocked" | "compaction.cancelled" => {
            Ok(WsEvent::CompactionEnded {
                failed: event_type != "compaction.completed",
            })
        }
        "task.started" | "background.task.started" => Ok(WsEvent::TaskStarted {
            task_id: task_id(payload)?,
        }),
        "task.terminated" | "background.task.terminated" => Ok(WsEvent::TaskEnded {
            task_id: task_id(payload)?,
            failed: task_failed(payload)?,
        }),
        "turn.ended" => Ok(WsEvent::TurnEnded {
            turn_id: required_u64(payload, "turnId")?,
            reason: match required_string(payload, "reason")? {
                "completed" => TurnEndReason::Completed,
                "cancelled" => TurnEndReason::Cancelled,
                "failed" => TurnEndReason::Failed,
                "blocked" => TurnEndReason::Blocked,
                _ => return Err(malformed()),
            },
        }),
        "event.session.status_changed" => {
            required_string(payload, "previous_status")?;
            match required_string(payload, "status")? {
                "awaiting_approval" => Ok(WsEvent::AwaitingApproval),
                "awaiting_question" => Ok(WsEvent::AwaitingQuestion),
                "aborted" => Ok(WsEvent::SessionAborted),
                "idle" | "running" => Ok(WsEvent::Progress),
                _ => Err(malformed()),
            }
        }
        "warning" => {
            required_string(payload, "message")?;
            Ok(WsEvent::Warning)
        }
        "error" => Ok(WsEvent::ProviderError),
        "turn.step.retrying" => Ok(WsEvent::Retrying {
            turn_id: required_u64(payload, "turnId")?,
            step: required_u64(payload, "step")?,
            failed_attempt: required_u64(payload, "failedAttempt")?,
            next_attempt: required_u64(payload, "nextAttempt")?,
            max_attempts: required_u64(payload, "maxAttempts")?,
        }),
        "agent.status.updated"
        | "agent.created"
        | "agent.disposed"
        | "event.session.work_changed"
        | "event.session.created"
        | "event.workspace.created"
        | "event.workspace.updated"
        | "event.workspace.deleted"
        | "event.config.changed"
        | "session.meta.updated" => Ok(WsEvent::Progress),
        _ => Ok(WsEvent::Unknown(event_type.to_owned())),
    }
}

fn optional_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<String>, RuntimeFailure> {
    match object.get(field) {
        Some(value) => value
            .as_str()
            .map(|value| Some(value.to_owned()))
            .ok_or_else(malformed),
        None => Ok(None),
    }
}

fn optional_bool(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<Option<bool>, RuntimeFailure> {
    match object.get(field) {
        Some(value) => value.as_bool().map(Some).ok_or_else(malformed),
        None => Ok(None),
    }
}

fn task_info(
    payload: &serde_json::Map<String, Value>,
) -> Result<&serde_json::Map<String, Value>, RuntimeFailure> {
    required_object(payload, "info")
}

fn task_id(payload: &serde_json::Map<String, Value>) -> Result<String, RuntimeFailure> {
    Ok(required_string(task_info(payload)?, "taskId")?.to_owned())
}

fn task_failed(payload: &serde_json::Map<String, Value>) -> Result<bool, RuntimeFailure> {
    Ok(matches!(
        required_string(task_info(payload)?, "status")?,
        "failed" | "timed_out" | "killed" | "lost"
    ))
}

pub(crate) const fn classify_ws_close(code: u16) -> WsCloseKind {
    match code {
        1000 => WsCloseKind::Normal,
        1001 => WsCloseKind::GoingAway,
        _ => WsCloseKind::Unexpected,
    }
}

fn validate_cursors(value: Option<&Value>) -> Result<usize, RuntimeFailure> {
    Ok(decode_cursors(value)?.len())
}

fn decode_cursors(value: Option<&Value>) -> Result<Vec<WsCursor>, RuntimeFailure> {
    let Some(cursors) = value else {
        return Ok(Vec::new());
    };
    let cursors = cursors.as_object().ok_or_else(malformed)?;
    let mut decoded = Vec::with_capacity(cursors.len());
    for (session_id, cursor) in cursors {
        let cursor = cursor.as_object().ok_or_else(malformed)?;
        let epoch = cursor
            .get("epoch")
            .map(|epoch| {
                epoch
                    .as_str()
                    .filter(|epoch| !epoch.is_empty())
                    .map(str::to_owned)
                    .ok_or_else(malformed)
            })
            .transpose()?;
        decoded.push(WsCursor {
            session_id: session_id.clone(),
            seq: required_u64(cursor, "seq")?,
            epoch,
        });
    }
    Ok(decoded)
}
