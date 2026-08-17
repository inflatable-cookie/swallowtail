use crate::failure::failure;
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use swallowtail_runtime::{RuntimeFailure, SessionReplayKind, TokenUsage};

pub(crate) const MAX_HTTP_BODY_BYTES: usize = 262_144;
pub(crate) const MAX_WEBSOCKET_FRAME_BYTES: usize = 262_144;
pub(crate) const MAX_RPC_ID_BYTES: usize = 128;
pub(crate) const MAX_HISTORY_ENTRIES: usize = 64;
pub(crate) const MAX_SEARCH_ITEMS: usize = 64;
pub(crate) const MAX_SESSIONS: usize = 512;
pub(crate) const MAX_LIVE_EVENTS: usize = 8_192;
const MAX_TEXT_BYTES: usize = 16_384;

const ALLOWLIST: &[WebMethod] = &[
    WebMethod::SessionList,
    WebMethod::SessionSearch,
    WebMethod::SessionCreate,
    WebMethod::SessionHistory,
    WebMethod::SessionModels,
    WebMethod::SessionPrompt,
    WebMethod::SessionCancel,
    WebMethod::SessionFork,
    WebMethod::WorkspaceList,
    WebMethod::WorkspaceArchiveSession,
    WebMethod::HostDescribe,
];

pub(crate) const fn method_allowlist() -> &'static [&'static str] {
    &[
        "session.list",
        "session.search",
        "session.create",
        "session.history",
        "session.models",
        "session.prompt",
        "session.cancel",
        "session.fork",
        "workspace.list",
        "workspace.archiveSession",
        "host.describe",
    ]
}

const MUX_FRAMES: &[&str] = &["session/subscribed", "session/event", "stream/error"];
#[allow(dead_code)]
const HOST_FRAMES: &[&str] = &[
    "host/session-added",
    "host/session-removed",
    "host/session-status",
    "host/agent-error",
    "host/workspace-changed",
    "host/workspace-removed",
    "host/workspace-order-changed",
    "host/archived-sessions-changed",
    "host/remote-event",
    "stream/error",
];
const SESSION_EVENT_TYPES: &[&str] = &[
    "turn/start",
    "turn/end",
    "step/start",
    "step/end",
    "user/message",
    "request/header",
    "request/context",
    "assistant/chunk",
    "assistant/message",
    "tool/call",
    "tool/result",
    "session/title",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WebMethod {
    SessionList,
    SessionSearch,
    SessionCreate,
    SessionHistory,
    SessionModels,
    SessionPrompt,
    SessionCancel,
    SessionFork,
    WorkspaceList,
    WorkspaceArchiveSession,
    HostDescribe,
}

impl WebMethod {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::SessionList => "session.list",
            Self::SessionSearch => "session.search",
            Self::SessionCreate => "session.create",
            Self::SessionHistory => "session.history",
            Self::SessionModels => "session.models",
            Self::SessionPrompt => "session.prompt",
            Self::SessionCancel => "session.cancel",
            Self::SessionFork => "session.fork",
            Self::WorkspaceList => "workspace.list",
            Self::WorkspaceArchiveSession => "workspace.archiveSession",
            Self::HostDescribe => "host.describe",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionSummary {
    pub(crate) session_id: String,
    pub(crate) updated_at: u64,
    pub(crate) running: bool,
    pub(crate) blank: bool,
    pub(crate) cwd: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionListPage {
    pub(crate) items: Vec<SessionSummary>,
    pub(crate) next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionCreateResult {
    pub(crate) session_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HostDescription {
    pub(crate) provider: Option<String>,
    pub(crate) model: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionSearchItem {
    pub(crate) session_id: String,
    pub(crate) snippet: String,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ModelSummary {
    pub(crate) provider: String,
    pub(crate) model: String,
    pub(crate) name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HistoryRecord {
    pub(crate) session_id: String,
    pub(crate) sequence: u64,
    pub(crate) kind: SessionReplayKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MuxEvent {
    pub(crate) session_id: String,
    pub(crate) sequence: u64,
    pub(crate) event_type: String,
    pub(crate) output_delta: Option<String>,
    pub(crate) usage: Option<TokenUsage>,
    pub(crate) terminal: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum MuxFrame {
    Subscribed { session_id: String, last_seq: i64 },
    Event(MuxEvent),
}

/// Builds one allowlisted Web RPC request envelope.
pub(crate) fn request_body(
    method: WebMethod,
    rpc_id: &str,
    payload: Value,
) -> Result<Vec<u8>, RuntimeFailure> {
    bounded_rpc_id(Some(rpc_id), "request rpc id")?;
    require_allowed_method(method)?;
    require_object(&payload, "request payload")?;
    let body = serde_json::to_vec(&json!({
        "type": "client-request",
        "rpcId": rpc_id,
        "method": method.as_str(),
        "payload": payload,
    }))
    .map_err(|_| malformed("request could not be encoded"))?;
    if body.len() > MAX_HTTP_BODY_BYTES {
        return Err(failure(
            "swallowtail.deepseek_harness.web.request_limit",
            "DeepSeek Harness Web request exceeds its bounded body limit",
        ));
    }
    Ok(body)
}

/// Decodes one HTTP response while keeping carrier and business failures distinct.
pub(crate) fn decode_unary_response(
    status: u16,
    bytes: &[u8],
    expected_rpc_id: &str,
) -> Result<Value, RuntimeFailure> {
    if status != 200 {
        return Err(carrier_failure(status));
    }
    if bytes.is_empty() || bytes.len() > MAX_HTTP_BODY_BYTES {
        return Err(malformed("response body is outside its bound"));
    }
    let envelope: Value =
        serde_json::from_slice(bytes).map_err(|_| malformed("response JSON is malformed"))?;
    let object = require_object(&envelope, "response envelope")?;
    require(
        object.get("type").and_then(Value::as_str) == Some("server-response"),
        "response envelope type is invalid",
    )?;
    require_rpc_id(
        object.get("rpcId").and_then(Value::as_str),
        "response rpc id",
    )?;
    require(
        object.get("rpcId").and_then(Value::as_str) == Some(expected_rpc_id),
        "response rpc id does not correlate",
    )?;
    let result = object
        .get("result")
        .ok_or_else(|| malformed("response result is missing"))?;
    let result = require_object(result, "response result")?;
    let ok = result
        .get("ok")
        .and_then(Value::as_bool)
        .ok_or_else(|| malformed("response result status is invalid"))?;
    if ok {
        let value = result
            .get("value")
            .ok_or_else(|| malformed("successful response value is missing"))?;
        return Ok(require_object(value, "successful response value")?
            .clone()
            .into());
    }
    let error = result
        .get("error")
        .ok_or_else(|| malformed("business error is missing"))?;
    let error = require_object(error, "business error")?;
    let code = error
        .get("code")
        .and_then(Value::as_str)
        .unwrap_or("provider-error");
    require_safe_text(code, "business error code")?;
    Err(failure(
        "swallowtail.deepseek_harness.web.provider_error",
        "DeepSeek Harness Web API returned a business error",
    ))
}

pub(crate) fn parse_session_list(value: &Value) -> Result<SessionListPage, RuntimeFailure> {
    let items = value
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("session.list items are missing"))?;
    if items.len() > MAX_SESSIONS {
        return Err(limit_failure("session.list exceeds its session bound"));
    }
    let items = items
        .iter()
        .map(parse_session_summary)
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    let next_cursor = match value.get("nextCursor") {
        None | Some(Value::Null) => None,
        Some(Value::String(cursor)) if !cursor.is_empty() && cursor.trim() == cursor => {
            require_safe_text(cursor, "session.list next cursor")?;
            Some(cursor.clone())
        }
        Some(_) => return Err(malformed("session.list nextCursor is invalid")),
    };
    Ok(SessionListPage { items, next_cursor })
}

pub(crate) fn parse_session_create(value: &Value) -> Result<SessionCreateResult, RuntimeFailure> {
    let session_id = bounded_id(value.get("sessionId"), "session.create session id")?;
    Ok(SessionCreateResult { session_id })
}

pub(crate) fn parse_host_description(value: &Value) -> Result<HostDescription, RuntimeFailure> {
    let value = require_object(value, "host description")?;
    let provider = optional_text(value.get("provider"), "host provider")?;
    let model = optional_text(value.get("model"), "host model")?;
    Ok(HostDescription { provider, model })
}

pub(crate) fn parse_search(
    value: &Value,
) -> Result<(Vec<SessionSearchItem>, bool), RuntimeFailure> {
    let items = value
        .get("items")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("session.search items are missing"))?;
    if items.len() > MAX_SEARCH_ITEMS {
        return Err(limit_failure("session.search exceeds its result bound"));
    }
    let has_more = value
        .get("hasMore")
        .and_then(Value::as_bool)
        .ok_or_else(|| malformed("session.search hasMore is missing"))?;
    let items = items
        .iter()
        .map(|item| {
            let item = require_object(item, "search result")?;
            Ok(SessionSearchItem {
                session_id: bounded_id(item.get("sessionId"), "search session id")?,
                snippet: bounded_text(item.get("snippet"), "search snippet")?.to_owned(),
            })
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    Ok((items, has_more))
}

#[allow(dead_code)]
pub(crate) fn parse_models(value: &Value) -> Result<Vec<ModelSummary>, RuntimeFailure> {
    let groups = value
        .get("groups")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("session.models groups are missing"))?;
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    for group in groups {
        let group = require_object(group, "model group")?;
        let provider = bounded_text(group.get("id"), "model provider")?;
        let entries = group
            .get("models")
            .and_then(Value::as_array)
            .ok_or_else(|| malformed("model group models are missing"))?;
        for entry in entries {
            let entry = require_object(entry, "model entry")?;
            let model = bounded_text(entry.get("id"), "model id")?;
            if !identities.insert(format!("{provider}:{model}")) {
                return Err(malformed(
                    "session.models contains duplicate model identity",
                ));
            }
            let name = entry
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or(model)
                .to_owned();
            require_safe_text(&name, "model name")?;
            models.push(ModelSummary {
                provider: provider.to_owned(),
                model: model.to_owned(),
                name,
            });
        }
    }
    if models.len() > MAX_SESSIONS {
        return Err(limit_failure("session.models exceeds its model bound"));
    }
    Ok(models)
}

pub(crate) fn parse_prompt(value: &Value) -> Result<(), RuntimeFailure> {
    require(
        value.get("accepted").and_then(Value::as_bool) == Some(true),
        "session.prompt was not accepted",
    )
}

pub(crate) fn parse_cancel(value: &Value) -> Result<(), RuntimeFailure> {
    require(
        value.get("accepted").and_then(Value::as_bool) == Some(true),
        "session.cancel was not accepted",
    )
}

pub(crate) fn parse_fork(value: &Value) -> Result<String, RuntimeFailure> {
    bounded_id(value.get("sessionId"), "session.fork child session id")
}

pub(crate) fn parse_archive(value: &Value, expected_session: &str) -> Result<(), RuntimeFailure> {
    let ids = value
        .get("archivedSessionIds")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("archive result is missing its archive set"))?;
    if ids.len() > MAX_SESSIONS {
        return Err(limit_failure("archive result exceeds its session bound"));
    }
    let contains = ids
        .iter()
        .filter_map(Value::as_str)
        .any(|id| id == expected_session);
    require(
        contains,
        "archive result does not confirm the target session",
    )
}

pub(crate) fn parse_history(
    value: &Value,
    expected_session: &str,
) -> Result<(Vec<HistoryRecord>, bool), RuntimeFailure> {
    let events = value
        .get("events")
        .and_then(Value::as_array)
        .ok_or_else(|| malformed("session.history events are missing"))?;
    if events.len() > MAX_HISTORY_ENTRIES {
        return Err(limit_failure("session.history exceeds its page bound"));
    }
    let has_more = value
        .get("hasMore")
        .and_then(Value::as_bool)
        .ok_or_else(|| malformed("session.history hasMore is missing"))?;
    let mut records = Vec::with_capacity(events.len());
    let mut previous = None;
    for entry in events {
        let entry = require_object(entry, "history entry")?;
        let event = require_object(
            entry
                .get("event")
                .ok_or_else(|| malformed("history event is missing"))?,
            "history event",
        )?;
        let sequence = event
            .get("seq")
            .and_then(Value::as_u64)
            .ok_or_else(|| malformed("history sequence is invalid"))?;
        require(
            previous.is_none_or(|value| value < sequence),
            "history sequence is not ascending",
        )?;
        previous = Some(sequence);
        let event_type = bounded_text(event.get("type"), "history event type")?;
        let kind = history_kind(event_type)?;
        records.push(HistoryRecord {
            session_id: expected_session.to_owned(),
            sequence,
            kind,
        });
    }
    Ok((records, has_more))
}

pub(crate) fn decode_mux_frame(bytes: &[u8]) -> Result<MuxFrame, RuntimeFailure> {
    if bytes.is_empty() || bytes.len() > MAX_WEBSOCKET_FRAME_BYTES {
        return Err(limit_failure("mux frame exceeds its bound"));
    }
    let envelope: Value =
        serde_json::from_slice(bytes).map_err(|_| malformed("mux frame JSON is malformed"))?;
    let object = require_object(&envelope, "mux envelope")?;
    require(
        object.get("type").and_then(Value::as_str) == Some("server-request"),
        "mux is downlink-only",
    )?;
    let rpc_id = object
        .get("rpcId")
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("mux rpc id is missing"))?;
    require_rpc_id(Some(rpc_id), "mux rpc id")?;
    let method = object
        .get("method")
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("mux method is missing"))?;
    require_allowed_frame(method, MUX_FRAMES)?;
    let payload = require_object(
        object
            .get("payload")
            .ok_or_else(|| malformed("mux payload is missing"))?,
        "mux payload",
    )?;
    require(
        payload.get("type").and_then(Value::as_str) == Some(method),
        "mux method and payload type differ",
    )?;
    match method {
        "session/subscribed" => Ok(MuxFrame::Subscribed {
            session_id: bounded_id(payload.get("sessionId"), "mux subscription session id")?,
            last_seq: payload
                .get("lastSeq")
                .and_then(Value::as_i64)
                .ok_or_else(|| malformed("mux subscription sequence is invalid"))?,
        }),
        "session/event" => {
            let session_id = bounded_id(payload.get("sessionId"), "mux event session id")?;
            let event = require_object(
                payload
                    .get("event")
                    .ok_or_else(|| malformed("mux event is missing"))?,
                "mux event",
            )?;
            let sequence = event
                .get("seq")
                .and_then(Value::as_u64)
                .ok_or_else(|| malformed("mux event sequence is invalid"))?;
            let event_type = bounded_text(event.get("type"), "mux event type")?;
            require(
                SESSION_EVENT_TYPES.contains(&event_type),
                "mux event type is not allowlisted",
            )?;
            let output_delta = extract_text_delta(event)?;
            let usage = extract_usage(event)?;
            Ok(MuxFrame::Event(MuxEvent {
                session_id,
                sequence,
                event_type: event_type.to_owned(),
                output_delta,
                usage,
                terminal: event_type == "turn/end",
            }))
        }
        "stream/error" => Err(failure(
            "swallowtail.deepseek_harness.web.stream_error",
            "DeepSeek Harness Web event stream reported a safe failure",
        )),
        _ => Err(malformed("mux frame method is not handled")),
    }
}

#[allow(dead_code)]
pub(crate) fn decode_host_frame(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    if bytes.is_empty() || bytes.len() > MAX_WEBSOCKET_FRAME_BYTES {
        return Err(limit_failure("host frame exceeds its bound"));
    }
    let envelope: Value =
        serde_json::from_slice(bytes).map_err(|_| malformed("host frame JSON is malformed"))?;
    let object = require_object(&envelope, "host envelope")?;
    require(
        object.get("type").and_then(Value::as_str) == Some("server-request"),
        "host stream is downlink-only",
    )?;
    require_rpc_id(object.get("rpcId").and_then(Value::as_str), "host rpc id")?;
    let method = object
        .get("method")
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("host method is missing"))?;
    require_allowed_frame(method, HOST_FRAMES)?;
    let payload = require_object(
        object
            .get("payload")
            .ok_or_else(|| malformed("host payload is missing"))?,
        "host payload",
    )?;
    require(
        payload.get("type").and_then(Value::as_str) == Some(method),
        "host method and payload type differ",
    )
}

fn parse_session_summary(value: &Value) -> Result<SessionSummary, RuntimeFailure> {
    let value = require_object(value, "session summary")?;
    Ok(SessionSummary {
        session_id: bounded_id(value.get("sessionId"), "session id")?,
        updated_at: value
            .get("updatedAt")
            .and_then(Value::as_u64)
            .ok_or_else(|| malformed("session updatedAt is invalid"))?,
        running: value
            .get("running")
            .and_then(Value::as_bool)
            .ok_or_else(|| malformed("session running is invalid"))?,
        blank: value
            .get("blank")
            .and_then(Value::as_bool)
            .ok_or_else(|| malformed("session blank is invalid"))?,
        cwd: match value.get("cwd") {
            None | Some(Value::Null) => None,
            Some(Value::String(cwd)) => {
                require_safe_text(cwd, "session cwd")?;
                Some(cwd.clone())
            }
            Some(_) => return Err(malformed("session cwd is invalid")),
        },
    })
}

fn history_kind(event_type: &str) -> Result<SessionReplayKind, RuntimeFailure> {
    match event_type {
        "user/message" => Ok(SessionReplayKind::UserMessage),
        "assistant/chunk" | "assistant/message" => Ok(SessionReplayKind::AgentMessage),
        "tool/call" => Ok(SessionReplayKind::ToolCall),
        "tool/result" => Ok(SessionReplayKind::ToolCallUpdate),
        "request/header" | "request/context" => Ok(SessionReplayKind::Configuration),
        "turn/start" | "turn/end" | "step/start" | "step/end" | "session/title" => {
            Ok(SessionReplayKind::Plan)
        }
        _ => Err(malformed("history event type is not projectable")),
    }
}

fn extract_text_delta(event: &Map<String, Value>) -> Result<Option<String>, RuntimeFailure> {
    if event.get("type").and_then(Value::as_str) != Some("assistant/chunk") {
        return Ok(None);
    }
    let data = event
        .get("data")
        .and_then(Value::as_object)
        .ok_or_else(|| malformed("assistant chunk data is missing"))?;
    let chunk = data
        .get("chunk")
        .and_then(Value::as_object)
        .ok_or_else(|| malformed("assistant chunk payload is missing"))?;
    if chunk.get("type").and_then(Value::as_str) != Some("text-delta") {
        return Ok(None);
    }
    let text = chunk
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("assistant text delta is invalid"))?;
    require(
        text.len() <= MAX_TEXT_BYTES,
        "assistant text delta exceeds its bound",
    )?;
    Ok(Some(text.to_owned()))
}

fn extract_usage(event: &Map<String, Value>) -> Result<Option<TokenUsage>, RuntimeFailure> {
    let event_type = event.get("type").and_then(Value::as_str);
    let data = match event.get("data").and_then(Value::as_object) {
        Some(data) => data,
        None if matches!(event_type, Some("assistant/chunk" | "assistant/message")) => {
            return Err(malformed("assistant usage data is missing"));
        }
        None => return Ok(None),
    };
    let usage = match event_type {
        Some("assistant/chunk") => data
            .get("chunk")
            .and_then(Value::as_object)
            .and_then(|chunk| chunk.get("usage")),
        Some("assistant/message") => data.get("usage"),
        _ => None,
    };
    let Some(usage) = usage else {
        return Ok(None);
    };
    let usage = require_object(usage, "assistant usage")?;
    let input = required_counter(usage, "inputTokens", "assistant input token count")?;
    let output = required_counter(usage, "outputTokens", "assistant output token count")?;
    let cache_read = optional_counter(usage, "cacheReadTokens", "assistant cache-read count")?;
    let cache_write = optional_counter(usage, "cacheWriteTokens", "assistant cache-write count")?;
    let reasoning = optional_counter(usage, "reasoningTokens", "assistant reasoning count")?;
    Ok(Some(
        TokenUsage::new(Some(input), Some(output))
            .with_cache_tokens(cache_read, cache_write)
            .with_reasoning_tokens(reasoning),
    ))
}

fn required_counter(
    object: &Map<String, Value>,
    key: &str,
    label: &'static str,
) -> Result<u64, RuntimeFailure> {
    object
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| malformed(label))
}

fn optional_counter(
    object: &Map<String, Value>,
    key: &str,
    label: &'static str,
) -> Result<Option<u64>, RuntimeFailure> {
    match object.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(value) => value.as_u64().map(Some).ok_or_else(|| malformed(label)),
    }
}

fn require_allowed_frame(method: &str, allowed: &[&str]) -> Result<(), RuntimeFailure> {
    require(
        allowed.contains(&method),
        "Web event frame is not allowlisted",
    )
}

fn require_allowed_method(method: WebMethod) -> Result<(), RuntimeFailure> {
    require(ALLOWLIST.contains(&method), "Web method is not allowlisted")
}

fn bounded_id(value: Option<&Value>, label: &str) -> Result<String, RuntimeFailure> {
    let value = bounded_text(value, label)?;
    require(
        value.len() <= MAX_RPC_ID_BYTES,
        "Web identity exceeds its bound",
    )?;
    Ok(value.to_owned())
}

fn bounded_rpc_id(value: Option<&str>, label: &str) -> Result<(), RuntimeFailure> {
    let value = value.ok_or_else(|| malformed("Web rpc id is missing"))?;
    require_safe_text(value, label)?;
    require(
        value.len() <= MAX_RPC_ID_BYTES,
        "Web rpc id exceeds its bound",
    )
}

fn require_rpc_id(value: Option<&str>, label: &str) -> Result<(), RuntimeFailure> {
    bounded_rpc_id(value, label)
}

fn bounded_text<'a>(value: Option<&'a Value>, label: &str) -> Result<&'a str, RuntimeFailure> {
    let value = value
        .and_then(Value::as_str)
        .ok_or_else(|| malformed("Web text field is invalid"))?;
    require_safe_text(value, label)?;
    Ok(value)
}

fn optional_text(value: Option<&Value>, label: &str) -> Result<Option<String>, RuntimeFailure> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => {
            require_safe_text(value, label)?;
            Ok(Some(value.clone()))
        }
        Some(_) => Err(malformed("Web optional text field is invalid")),
    }
}

fn require_safe_text(value: &str, _label: &str) -> Result<(), RuntimeFailure> {
    if value.is_empty()
        || value.len() > MAX_TEXT_BYTES
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        Err(malformed("Web text field is outside its bound"))
    } else {
        Ok(())
    }
}

fn require_object<'a>(
    value: &'a Value,
    _label: &str,
) -> Result<&'a Map<String, Value>, RuntimeFailure> {
    value
        .as_object()
        .ok_or_else(|| malformed("Web JSON field is not an object"))
}

fn require(condition: bool, message: &'static str) -> Result<(), RuntimeFailure> {
    if condition {
        Ok(())
    } else {
        Err(malformed(message))
    }
}

fn malformed(message: &'static str) -> RuntimeFailure {
    failure("swallowtail.deepseek_harness.web.protocol_invalid", message)
}

fn limit_failure(message: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.protocol_limit_exceeded",
        message,
    )
}

fn carrier_failure(status: u16) -> RuntimeFailure {
    let code = match status {
        400 => "swallowtail.deepseek_harness.web.carrier_malformed",
        404 => "swallowtail.deepseek_harness.web.carrier_not_found",
        413 => "swallowtail.deepseek_harness.web.carrier_too_large",
        415 => "swallowtail.deepseek_harness.web.carrier_media_type",
        500..=599 => "swallowtail.deepseek_harness.web.carrier_handler_failure",
        _ => "swallowtail.deepseek_harness.web.carrier_unexpected",
    };
    failure(
        code,
        "DeepSeek Harness Web API carrier rejected the request",
    )
}

#[cfg(test)]
mod tests {
    use super::{
        MuxFrame, WebMethod, decode_host_frame, decode_mux_frame, decode_unary_response,
        parse_archive, parse_history, parse_host_description, parse_models, request_body,
    };
    use serde_json::Value;

    fn fixture(path: &str) -> Vec<u8> {
        let root = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/deepseek-harness-web-0.1.0rc6/"
        );
        std::fs::read(format!("{root}{path}")).expect("fixture is readable")
    }

    fn unary_pair(method: &str) -> (u16, Vec<u8>, String) {
        let line = fixture("unary.jsonl");
        let record = line
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice::<Value>(line).expect("record JSON"))
            .collect::<Vec<_>>();
        let index = record
            .iter()
            .position(|value| value["body"]["method"] == method)
            .expect("method fixture")
            + 1;
        let response = &record[index];
        (
            response["status"].as_u64().expect("status") as u16,
            serde_json::to_vec(&response["body"]).expect("response JSON"),
            record[index - 1]["body"]["rpcId"]
                .as_str()
                .expect("rpc id")
                .to_owned(),
        )
    }

    #[test]
    fn request_body_cannot_construct_a_denied_method() {
        let body = request_body(
            WebMethod::SessionList,
            "fixture-rpc",
            Value::Object(Default::default()),
        )
        .expect("allowlisted request encodes");
        assert!(serde_json::from_slice::<Value>(&body).is_ok());
    }

    #[test]
    fn unary_response_preserves_business_success_shape() {
        let (status, body, rpc_id) = unary_pair("session.list");
        let value = decode_unary_response(status, &body, &rpc_id).expect("response decodes");
        assert_eq!(value["items"][0]["sessionId"], "fixture-session-1");
    }

    #[test]
    fn unary_business_error_is_not_treated_as_carrier_success() {
        let body = fixture("malformed.json");
        let value: Value = serde_json::from_slice(&body).expect("malformed corpus JSON");
        let response =
            serde_json::to_vec(&value["business_error_case"]["response"]).expect("response JSON");
        assert!(decode_unary_response(200, &response, "fixture-business-error").is_err());
    }

    #[test]
    fn mux_fixture_decodes_only_server_downlink_frames() {
        let frames = fixture("mux.jsonl");
        let decoded = frames
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| decode_mux_frame(line).expect("mux frame decodes"))
            .collect::<Vec<_>>();
        assert!(matches!(
            decoded[0],
            MuxFrame::Subscribed { last_seq: -1, .. }
        ));
        assert!(matches!(decoded.last(), Some(MuxFrame::Event(event)) if event.terminal));
    }

    #[test]
    fn host_fixture_and_history_keep_their_frozen_shapes() {
        let host = fixture("host.jsonl");
        for line in host
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
        {
            decode_host_frame(line).expect("host frame decodes");
        }
        let host_description = unary_pair("host.describe");
        let description = parse_host_description(
            &decode_unary_response(host_description.0, &host_description.1, &host_description.2)
                .expect("host description response"),
        )
        .expect("host description decodes");
        assert_eq!(description.provider.as_deref(), Some("fixture-provider"));
        assert_eq!(description.model.as_deref(), Some("fixture-model"));
        let history: Value =
            serde_json::from_slice(&fixture("history.json")).expect("history JSON");
        let first = &history["pages"][0]["response"]["result"]["value"];
        let (items, has_more) =
            parse_history(first, "fixture-session-1").expect("history page decodes");
        assert_eq!(items.len(), 2);
        assert!(has_more);
        let (status, body, rpc_id) = unary_pair("session.models");
        let models =
            parse_models(&decode_unary_response(status, &body, &rpc_id).expect("model response"))
                .expect("models decode");
        assert_eq!(models.len(), 1);
        let (status, body, rpc_id) = unary_pair("workspace.archiveSession");
        parse_archive(
            &decode_unary_response(status, &body, &rpc_id).expect("archive response"),
            "fixture-session-1",
        )
        .expect("archive confirms target");
    }
}
