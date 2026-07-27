use serde_json::Value;
use swallowtail_runtime::RuntimeFailure;

use super::common::{
    decode_json_object, malformed, required_array, required_i64, required_object, required_string,
};

pub(crate) const MAX_HTTP_BODY_BYTES: usize = 256 * 1024;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ServerMetadata {
    pub(crate) version: String,
    pub(crate) backend: String,
    pub(crate) websocket: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionRecord {
    pub(crate) id: String,
    pub(crate) archived: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct InteractiveSessionRecord {
    pub(crate) id: String,
    pub(crate) archived: bool,
    pub(crate) busy: bool,
    pub(crate) last_seq: u64,
    pub(crate) working_directory: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PromptStatus {
    Running,
    Queued,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PromptSubmission {
    pub(crate) id: String,
    pub(crate) status: PromptStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PendingProviderRequest {
    pub(crate) id: String,
    pub(crate) turn_id: Option<u64>,
    pub(crate) payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RestFailureKind {
    Validation,
    Unauthorized,
    Missing,
    Busy,
    Server,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum RestReply {
    Success(Value),
    Failure(RestFailureKind),
}

pub(crate) fn decode_rest(status: u16, bytes: &[u8]) -> Result<RestReply, RuntimeFailure> {
    let object = decode_json_object(bytes, MAX_HTTP_BODY_BYTES)?;
    let code = required_i64(&object, "code")?;
    required_string(&object, "msg")?;
    required_string(&object, "request_id")?;
    let data = object.get("data").ok_or_else(malformed)?.clone();

    if (200..300).contains(&status) && code == 0 {
        return Ok(RestReply::Success(data));
    }

    let kind = match (status, code) {
        (400, 40_001 | 40_002) => RestFailureKind::Validation,
        (401, 40_101) => RestFailureKind::Unauthorized,
        (404, 40_401..=40_415) => RestFailureKind::Missing,
        (409, 40_901) => RestFailureKind::Busy,
        (500..=599, _) | (_, 50_001) => RestFailureKind::Server,
        _ => return Err(malformed()),
    };
    Ok(RestReply::Failure(kind))
}

pub(crate) fn decode_health(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    if object.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(malformed());
    }
    Ok(())
}

pub(crate) fn decode_metadata(bytes: &[u8]) -> Result<ServerMetadata, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    let capabilities = required_object(object, "capabilities")?;
    let dangerous_bypass_auth = object
        .get("dangerous_bypass_auth")
        .and_then(Value::as_bool)
        .ok_or_else(malformed)?;
    if dangerous_bypass_auth {
        return Err(malformed());
    }
    Ok(ServerMetadata {
        version: required_string(object, "server_version")?.to_owned(),
        backend: required_string(object, "backend")?.to_owned(),
        websocket: capabilities
            .get("websocket")
            .and_then(Value::as_bool)
            .ok_or_else(malformed)?,
    })
}

pub(crate) fn decode_session(bytes: &[u8]) -> Result<SessionRecord, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    Ok(SessionRecord {
        id: required_string(object, "id")?.to_owned(),
        archived: object
            .get("archived")
            .and_then(Value::as_bool)
            .ok_or_else(malformed)?,
    })
}

pub(crate) fn decode_interactive_session(
    bytes: &[u8],
) -> Result<InteractiveSessionRecord, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    let metadata = required_object(object, "metadata")?;
    Ok(InteractiveSessionRecord {
        id: required_string(object, "id")?.to_owned(),
        archived: object
            .get("archived")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        busy: object
            .get("busy")
            .and_then(Value::as_bool)
            .ok_or_else(malformed)?,
        last_seq: object
            .get("last_seq")
            .and_then(Value::as_u64)
            .ok_or_else(malformed)?,
        working_directory: required_string(metadata, "cwd")?.to_owned(),
    })
}

pub(crate) fn decode_prompt_submission(bytes: &[u8]) -> Result<PromptSubmission, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    let status = match required_string(object, "status")? {
        "running" => PromptStatus::Running,
        "queued" => PromptStatus::Queued,
        "blocked" => PromptStatus::Blocked,
        _ => return Err(malformed()),
    };
    Ok(PromptSubmission {
        id: required_string(object, "prompt_id")?.to_owned(),
        status,
    })
}

pub(crate) fn decode_pending_approvals(
    bytes: &[u8],
    expected_session: &str,
) -> Result<Vec<PendingProviderRequest>, RuntimeFailure> {
    decode_pending(bytes, expected_session, "approval_id", validate_approval)
}

pub(crate) fn decode_pending_questions(
    bytes: &[u8],
    expected_session: &str,
) -> Result<Vec<PendingProviderRequest>, RuntimeFailure> {
    decode_pending(bytes, expected_session, "question_id", validate_question)
}

pub(crate) fn decode_callback_resolution(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    if object.get("resolved").and_then(Value::as_bool) != Some(true) {
        return Err(malformed());
    }
    required_string(object, "resolved_at")?;
    Ok(())
}

pub(crate) fn decode_question_dismissal(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    if object.get("dismissed").and_then(Value::as_bool) != Some(true) {
        return Err(malformed());
    }
    required_string(object, "dismissed_at")?;
    Ok(())
}

pub(crate) fn decode_archive(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let object = data.as_object().ok_or_else(malformed)?;
    if object.len() != 1 || object.get("archived").and_then(Value::as_bool) != Some(true) {
        return Err(malformed());
    }
    Ok(())
}

fn decode_pending(
    bytes: &[u8],
    expected_session: &str,
    id_key: &str,
    validate: fn(&serde_json::Map<String, Value>) -> Result<(), RuntimeFailure>,
) -> Result<Vec<PendingProviderRequest>, RuntimeFailure> {
    let RestReply::Success(data) = decode_rest(200, bytes)? else {
        return Err(malformed());
    };
    let data = data.as_object().ok_or_else(malformed)?;
    let items = required_array(data, "items")?;
    if items.len() > 32 {
        return Err(malformed());
    }
    items
        .iter()
        .map(|item| {
            let object = item.as_object().ok_or_else(malformed)?;
            if required_string(object, "session_id")? != expected_session {
                return Err(malformed());
            }
            validate(object)?;
            let payload = serde_json::to_vec(item).map_err(|_| malformed())?;
            Ok(PendingProviderRequest {
                id: required_string(object, id_key)?.to_owned(),
                turn_id: object.get("turn_id").and_then(Value::as_u64),
                payload,
            })
        })
        .collect()
}

fn validate_approval(object: &serde_json::Map<String, Value>) -> Result<(), RuntimeFailure> {
    required_string(object, "tool_call_id")?;
    required_string(object, "tool_name")?;
    required_string(object, "action")?;
    object.get("tool_input_display").ok_or_else(malformed)?;
    required_string(object, "created_at")?;
    required_string(object, "expires_at")?;
    Ok(())
}

fn validate_question(object: &serde_json::Map<String, Value>) -> Result<(), RuntimeFailure> {
    required_string(object, "created_at")?;
    let questions = required_array(object, "questions")?;
    if questions.is_empty() || questions.len() > 4 {
        return Err(malformed());
    }
    for question in questions {
        let question = question.as_object().ok_or_else(malformed)?;
        required_string(question, "id")?;
        required_string(question, "question")?;
        let options = required_array(question, "options")?;
        if !(2..=4).contains(&options.len()) {
            return Err(malformed());
        }
        for option in options {
            let option = option.as_object().ok_or_else(malformed)?;
            required_string(option, "id")?;
            required_string(option, "label")?;
        }
    }
    Ok(())
}

pub(crate) fn inspect_openapi(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let object = decode_json_object(bytes, MAX_HTTP_BODY_BYTES)?;
    required_string(&object, "openapi")?;
    let paths = required_object(&object, "paths")?;
    for required in [
        "/api/v1/healthz",
        "/api/v1/meta",
        "/api/v1/sessions",
        "/api/v1/sessions/{session_id}",
        "/api/v1/sessions/{session_id}:archive",
        "/api/v1/sessions/{session_id}:restore",
    ] {
        if !paths.contains_key(required) {
            return Err(malformed());
        }
    }
    if paths.iter().any(|(path, operations)| {
        path.starts_with("/api/v1/sessions/")
            && (path.ends_with(":delete")
                || operations
                    .as_object()
                    .is_some_and(|operations| operations.contains_key("delete")))
    }) {
        return Err(malformed());
    }
    Ok(())
}

pub(crate) fn inspect_asyncapi(bytes: &[u8]) -> Result<(), RuntimeFailure> {
    let object = decode_json_object(bytes, MAX_HTTP_BODY_BYTES)?;
    required_string(&object, "asyncapi")?;
    let channels = required_object(&object, "channels")?;
    if !channels.values().any(|channel| {
        channel
            .as_object()
            .and_then(|channel| channel.get("address"))
            .and_then(Value::as_str)
            == Some("/api/v1/ws")
    }) {
        return Err(malformed());
    }
    Ok(())
}
