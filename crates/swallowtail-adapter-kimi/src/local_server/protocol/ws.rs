use serde_json::Value;
use swallowtail_runtime::RuntimeFailure;

use super::common::{
    decode_json_object, malformed, required_array, required_i64, required_object, required_string,
    required_u64,
};

const MAX_WS_FRAME_BYTES: usize = 64 * 1024;
const WS_PROTOCOL_VERSION: u64 = 2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WsFrame {
    ServerHello {
        protocol_version: u64,
    },
    Subscribe {
        session_count: usize,
        cursor_count: usize,
    },
    Abort,
    Ack {
        code: i64,
        accepted_count: usize,
        resync_count: usize,
    },
    Event(WsEventEnvelope),
    ResyncRequired {
        reason: ResyncReason,
        current_seq: u64,
    },
    Error {
        fatal: bool,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WsEventEnvelope {
    pub(crate) durable_seq: u64,
    pub(crate) epoch: Option<String>,
    pub(crate) volatile: bool,
    pub(crate) offset: Option<u64>,
    pub(crate) session_id: String,
    pub(crate) event: WsEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WsEvent {
    TurnStarted {
        turn_id: u64,
    },
    AssistantDelta {
        turn_id: u64,
        delta: String,
    },
    ThinkingDelta {
        turn_id: u64,
        delta: String,
    },
    StepStarted {
        turn_id: u64,
        step: u64,
        step_id: Option<String>,
    },
    StepEnded {
        turn_id: u64,
        step: u64,
        step_id: Option<String>,
        failed: bool,
    },
    ToolStarted {
        turn_id: u64,
        call_id: String,
        name: String,
    },
    ToolUpdated {
        turn_id: u64,
        call_id: String,
    },
    ToolEnded {
        turn_id: u64,
        call_id: String,
        failed: bool,
    },
    ShellStarted {
        command_id: String,
    },
    ShellUpdated {
        command_id: String,
    },
    ShellEnded {
        command_id: String,
        failed: bool,
    },
    SubagentSpawned {
        subagent_id: String,
        name: String,
    },
    SubagentUpdated {
        subagent_id: String,
    },
    SubagentEnded {
        subagent_id: String,
        failed: bool,
    },
    CompactionStarted,
    CompactionEnded {
        failed: bool,
    },
    TaskStarted {
        task_id: String,
    },
    TaskEnded {
        task_id: String,
        failed: bool,
    },
    TurnEnded {
        turn_id: u64,
        reason: TurnEndReason,
    },
    Retrying {
        turn_id: u64,
        step: u64,
        failed_attempt: u64,
        next_attempt: u64,
        max_attempts: u64,
    },
    AwaitingApproval,
    AwaitingQuestion,
    SessionAborted,
    Progress,
    Warning,
    Unknown(String),
    ProviderError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TurnEndReason {
    Completed,
    Cancelled,
    Failed,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ResyncReason {
    BufferOverflow,
    SessionRecreated,
    EpochChanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WsCloseKind {
    Normal,
    GoingAway,
    Unexpected,
}

pub(crate) fn decode_ws_frame(bytes: &[u8]) -> Result<WsFrame, RuntimeFailure> {
    let object = decode_json_object(bytes, MAX_WS_FRAME_BYTES)?;
    match required_string(&object, "type")? {
        "server_hello" => {
            let payload = required_object(&object, "payload")?;
            let protocol_version = required_u64(payload, "protocol_version")?;
            if protocol_version != WS_PROTOCOL_VERSION {
                return Err(malformed());
            }
            required_string(payload, "ws_connection_id")?;
            required_u64(payload, "max_event_buffer_size")?;
            Ok(WsFrame::ServerHello { protocol_version })
        }
        "subscribe" => {
            required_string(&object, "id")?;
            let payload = required_object(&object, "payload")?;
            let sessions = required_array(payload, "session_ids")?;
            let cursor_count = validate_cursors(payload.get("cursors"))?;
            Ok(WsFrame::Subscribe {
                session_count: sessions.len(),
                cursor_count,
            })
        }
        "abort" => {
            required_string(&object, "id")?;
            let payload = required_object(&object, "payload")?;
            required_string(payload, "session_id")?;
            required_string(payload, "prompt_id")?;
            Ok(WsFrame::Abort)
        }
        "ack" => {
            required_string(&object, "id")?;
            let code = required_i64(&object, "code")?;
            required_string(&object, "msg")?;
            let payload = required_object(&object, "payload")?;
            let accepted_count = payload
                .get("accepted")
                .or_else(|| payload.get("accepted_subscriptions"))
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            let resync_count = payload
                .get("resync_required")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            validate_cursors(payload.get("cursors"))?;
            Ok(WsFrame::Ack {
                code,
                accepted_count,
                resync_count,
            })
        }
        "resync_required" => {
            let payload = required_object(&object, "payload")?;
            required_string(payload, "session_id")?;
            let reason = match required_string(payload, "reason")? {
                "buffer_overflow" => ResyncReason::BufferOverflow,
                "session_recreated" => ResyncReason::SessionRecreated,
                "epoch_changed" => {
                    required_string(payload, "epoch")?;
                    ResyncReason::EpochChanged
                }
                _ => return Err(malformed()),
            };
            Ok(WsFrame::ResyncRequired {
                reason,
                current_seq: required_u64(payload, "current_seq")?,
            })
        }
        "error" => {
            let payload = required_object(&object, "payload")?;
            required_i64(payload, "code")?;
            required_string(payload, "msg")?;
            Ok(WsFrame::Error {
                fatal: payload
                    .get("fatal")
                    .and_then(Value::as_bool)
                    .ok_or_else(malformed)?,
            })
        }
        _ => {
            let durable_seq = required_u64(&object, "seq")?;
            required_string(&object, "timestamp")?;
            let session_id = required_string(&object, "session_id")?.to_owned();
            let volatile = object
                .get("volatile")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let epoch = object
                .get("epoch")
                .and_then(Value::as_str)
                .map(str::to_owned);
            if !volatile && epoch.as_deref().is_none_or(str::is_empty) {
                return Err(malformed());
            }
            let offset = object.get("offset").and_then(Value::as_u64);
            let payload = required_object(&object, "payload")?;
            let event = decode_event(required_string(&object, "type")?, payload)?;
            Ok(WsFrame::Event(WsEventEnvelope {
                durable_seq,
                epoch,
                volatile,
                offset,
                session_id,
                event,
            }))
        }
    }
}

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
        }),
        "subagent.started" | "subagent.suspended" => Ok(WsEvent::SubagentUpdated {
            subagent_id: required_string(payload, "subagentId")?.to_owned(),
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
    let Some(cursors) = value else {
        return Ok(0);
    };
    let cursors = cursors.as_object().ok_or_else(malformed)?;
    for cursor in cursors.values() {
        let cursor = cursor.as_object().ok_or_else(malformed)?;
        required_u64(cursor, "seq")?;
        if let Some(epoch) = cursor.get("epoch")
            && epoch.as_str().is_none_or(str::is_empty)
        {
            return Err(malformed());
        }
    }
    Ok(cursors.len())
}
