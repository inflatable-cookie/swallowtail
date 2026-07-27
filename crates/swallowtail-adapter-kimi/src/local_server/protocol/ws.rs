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
    TurnStarted { turn_id: u64 },
    AssistantDelta { turn_id: u64, delta: String },
    ThinkingDelta { turn_id: u64, delta: String },
    TurnEnded { turn_id: u64, reason: TurnEndReason },
    AwaitingApproval,
    AwaitingQuestion,
    SessionAborted,
    Progress,
    Warning,
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
        "turn.step.started"
        | "turn.step.completed"
        | "turn.step.retrying"
        | "turn.step.interrupted"
        | "tool.call.delta"
        | "tool.call.started"
        | "tool.progress"
        | "shell.output"
        | "shell.started"
        | "shell.completed"
        | "agent.status.updated"
        | "agent.created"
        | "agent.disposed"
        | "event.session.work_changed"
        | "session.meta.updated" => Ok(WsEvent::Progress),
        _ => Err(super::common::unsupported_event()),
    }
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
