use crate::failure::failure;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use swallowtail_core::{
    IntegrationFamilyId, InterfaceVersionBinding, ModelCatalogEntry, ModelCatalogObservations,
    ModelId, ModelMetadata, ModelTokenLimits, ProviderId, ReasoningMetadata, ReasoningMode,
};
use swallowtail_runtime::{CallbackResult, StructuredOutputDescriptor};
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

mod health;
pub(crate) use health::observe_health;
pub(crate) use health::require_health_matches;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Delete,
    Get,
    Post,
}

impl Request {
    pub(crate) fn delete(path: impl Into<String>) -> Self {
        Self {
            method: Method::Delete,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn get(path: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn post(path: impl Into<String>, body: Option<Value>) -> Self {
        Self {
            method: Method::Post,
            path: path.into(),
            query: Vec::new(),
            body: body.map(|value| serde_json::to_vec(&value).expect("JSON value serializes")),
        }
    }

    pub(crate) fn with_directory(mut self, directory: &str) -> Self {
        self.query
            .push(("directory".to_owned(), directory.to_owned()));
        self
    }

    pub(crate) fn with_query(mut self, key: &str, value: impl Into<String>) -> Self {
        self.query.push((key.to_owned(), value.into()));
        self
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub body: Vec<u8>,
    pub next_cursor: Option<String>,
}

#[derive(Deserialize)]
struct ProviderList {
    all: Vec<Provider>,
    #[serde(rename = "default")]
    defaults: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct Provider {
    id: String,
    models: BTreeMap<String, ProviderModel>,
}

#[derive(Deserialize)]
struct ProviderModel {
    id: String,
    name: Option<String>,
    limit: Option<ModelLimit>,
    capabilities: ModelCapabilities,
    #[serde(default)]
    variants: BTreeMap<String, Value>,
}

#[derive(Deserialize)]
struct ModelCapabilities {
    reasoning: bool,
    toolcall: bool,
}

#[derive(Clone, Copy, Deserialize)]
struct ModelLimit {
    input: Option<u64>,
    output: Option<u64>,
}

pub(crate) fn parse_catalog(response: &Response) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    require_success(response, "provider catalogue request")?;
    let providers: ProviderList = parse_json(&response.body, "provider catalogue response")?;
    let mut entries = Vec::new();
    for provider in providers.all {
        let provider_id = ProviderId::new(provider.id.clone()).map_err(|_| {
            failure(
                "swallowtail.opencode.catalog_invalid",
                "OpenCode returned an invalid provider identity",
            )
        })?;
        for (key, model) in provider.models {
            if key != model.id {
                return Err(failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned inconsistent model identities",
                ));
            }
            let model_id = ModelId::new(model.id.clone()).map_err(|_| {
                failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned an invalid model identity",
                )
            })?;
            let mut metadata = match model.name {
                Some(name) => ModelMetadata::with_display_name(name).map_err(|_| {
                    failure(
                        "swallowtail.opencode.catalog_invalid",
                        "OpenCode returned invalid model metadata",
                    )
                })?,
                None => ModelMetadata::default(),
            };
            metadata =
                metadata.with_default(providers.defaults.get(&provider.id) == Some(&model.id));
            if let Some(limit) = model.limit {
                metadata =
                    metadata.with_token_limits(ModelTokenLimits::new(limit.input, limit.output));
            }
            let modes = model
                .variants
                .keys()
                .map(|mode| {
                    ReasoningMode::new(mode.clone()).map_err(|_| {
                        failure(
                            "swallowtail.opencode.catalog_invalid",
                            "OpenCode returned an invalid reasoning variant",
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if !model.capabilities.reasoning && !modes.is_empty() {
                return Err(failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned inconsistent reasoning capability evidence",
                ));
            }
            if model.capabilities.reasoning {
                metadata = metadata.with_reasoning(ReasoningMetadata::new(modes, None));
            }
            metadata = metadata.with_catalog_observations(
                ModelCatalogObservations::new(
                    IntegrationFamilyId::new("opencode")
                        .expect("static OpenCode integration family is valid"),
                )
                .with_reasoning_supported(model.capabilities.reasoning)
                .with_tool_calling_supported(model.capabilities.toolcall),
            );
            entries.push(
                ModelCatalogEntry::new(model_id, metadata).with_provider_id(provider_id.clone()),
            );
        }
    }
    Ok(entries)
}

pub(crate) fn session_create(
    provider_id: &str,
    model_id: &str,
    directory: &str,
    consumer_callbacks: bool,
) -> Request {
    let fallback_action = if consumer_callbacks { "ask" } else { "deny" };
    Request::post(
        "/session",
        Some(json!({
            "title": "Swallowtail session",
            "model": {"id": model_id, "providerID": provider_id},
            "permission": [
                {"permission": "*", "pattern": "*", "action": fallback_action},
                {"permission": "read", "pattern": "*", "action": "allow"},
                {"permission": "glob", "pattern": "*", "action": "allow"},
                {"permission": "grep", "pattern": "*", "action": "allow"}
            ]
        })),
    )
    .with_directory(directory)
}

pub(crate) fn parse_session_for_version(
    response: &Response,
    expected: &InterfaceVersionBinding,
) -> Result<String, RuntimeFailure> {
    require_success(response, "session create request")?;
    #[derive(Deserialize)]
    struct Session {
        id: String,
        version: String,
    }
    let session: Session = parse_json(&response.body, "session create response")?;
    if session.version != expected.version().as_str() || session.id.trim().is_empty() {
        return Err(failure(
            "swallowtail.opencode.session_invalid",
            "OpenCode returned an invalid session binding",
        ));
    }
    Ok(session.id)
}

pub(crate) fn session_get(session_id: &str, directory: &str) -> Request {
    Request::get(format!("/session/{session_id}")).with_directory(directory)
}

pub(crate) fn require_existing_session(
    response: &Response,
    expected_version: &InterfaceVersionBinding,
    expected_session: &str,
) -> Result<(), RuntimeFailure> {
    let observed = parse_session_for_version(response, expected_version)?;
    if observed == expected_session {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.opencode.session_binding_mismatch",
            "OpenCode attached a different provider session",
        ))
    }
}

pub(crate) fn session_messages(
    session_id: &str,
    directory: &str,
    limit: usize,
    before: Option<&str>,
) -> Request {
    let request = Request::get(format!("/session/{session_id}/message"))
        .with_directory(directory)
        .with_query("limit", limit.to_string());
    match before {
        Some(before) => request.with_query("before", before),
        None => request,
    }
}

pub(crate) fn project_session_messages(
    response: &Response,
    session: &swallowtail_core::SessionRef,
    sequence: &mut u64,
) -> Result<Vec<swallowtail_runtime::SessionReplayItem>, RuntimeFailure> {
    require_success(response, "session messages request")?;
    let messages: Vec<Value> = parse_json(&response.body, "session messages response")?;
    let mut replay = Vec::new();
    for message in messages {
        let info = message.get("info").ok_or_else(|| {
            failure(
                "swallowtail.opencode.replay_malformed",
                "OpenCode returned malformed session history",
            )
        })?;
        if info.get("sessionID").and_then(Value::as_str) != Some(session.as_provider_value()) {
            return Err(failure(
                "swallowtail.opencode.replay_session_mismatch",
                "OpenCode returned history for a different provider session",
            ));
        }
        let role = info
            .get("role")
            .and_then(Value::as_str)
            .filter(|role| matches!(*role, "user" | "assistant"))
            .ok_or_else(|| {
                failure(
                    "swallowtail.opencode.replay_malformed",
                    "OpenCode returned malformed session history",
                )
            })?;
        let parts = message
            .get("parts")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                failure(
                    "swallowtail.opencode.replay_malformed",
                    "OpenCode returned malformed session history",
                )
            })?;
        if parts.is_empty() {
            replay.push(swallowtail_runtime::SessionReplayItem::new(
                session.clone(),
                next_sequence(sequence)?,
                swallowtail_runtime::SessionReplayKind::Configuration,
            ));
        }
        for part in parts {
            if part.get("sessionID").and_then(Value::as_str) != Some(session.as_provider_value()) {
                return Err(failure(
                    "swallowtail.opencode.replay_session_mismatch",
                    "OpenCode returned history for a different provider session",
                ));
            }
            let part_type = part.get("type").and_then(Value::as_str).ok_or_else(|| {
                failure(
                    "swallowtail.opencode.replay_malformed",
                    "OpenCode returned malformed session history",
                )
            })?;
            let replay_kind = match (part_type, role) {
                ("text", "user") => swallowtail_runtime::SessionReplayKind::UserMessage,
                ("text", "assistant") => swallowtail_runtime::SessionReplayKind::AgentMessage,
                ("reasoning", _) => swallowtail_runtime::SessionReplayKind::AgentReasoning,
                ("tool", _)
                    if part
                        .get("state")
                        .and_then(|state| state.get("status"))
                        .and_then(Value::as_str)
                        .is_some_and(|status| matches!(status, "completed" | "error")) =>
                {
                    swallowtail_runtime::SessionReplayKind::ToolCallUpdate
                }
                ("tool", _) => swallowtail_runtime::SessionReplayKind::ToolCall,
                ("patch" | "snapshot", _) => swallowtail_runtime::SessionReplayKind::ToolCallUpdate,
                ("subtask", _) => swallowtail_runtime::SessionReplayKind::ToolCall,
                ("step-start" | "step-finish" | "agent" | "retry" | "compaction" | "file", _) => {
                    swallowtail_runtime::SessionReplayKind::Configuration
                }
                _ => {
                    return Err(failure(
                        "swallowtail.opencode.replay_unsupported",
                        "OpenCode returned unsupported session history",
                    ));
                }
            };
            let item = match part.get("text") {
                Some(Value::String(text)) if !text.is_empty() => {
                    swallowtail_runtime::SessionReplayItem::with_content(
                        session.clone(),
                        next_sequence(sequence)?,
                        replay_kind,
                        swallowtail_runtime::OperationContent::new(text).map_err(|_| {
                            failure(
                                "swallowtail.opencode.replay_malformed",
                                "OpenCode returned malformed session history",
                            )
                        })?,
                    )
                }
                _ => swallowtail_runtime::SessionReplayItem::new(
                    session.clone(),
                    next_sequence(sequence)?,
                    replay_kind,
                ),
            };
            replay.push(item);
        }
    }
    Ok(replay)
}

fn next_sequence(sequence: &mut u64) -> Result<u64, RuntimeFailure> {
    let current = *sequence;
    *sequence = sequence.checked_add(1).ok_or_else(|| {
        failure(
            "swallowtail.opencode.replay_limit_exceeded",
            "OpenCode session history exceeded the adapter limit",
        )
    })?;
    Ok(current)
}

pub(crate) struct PromptPayload<'a> {
    pub(crate) content: &'a str,
    pub(crate) reasoning: Option<&'a ReasoningMode>,
    pub(crate) structured_output: Option<&'a StructuredOutputDescriptor>,
    pub(crate) file: Option<&'a crate::driver::input::FilePart>,
}

pub(crate) fn prompt(
    session_id: &str,
    provider_id: &str,
    model_id: &str,
    directory: &str,
    payload: PromptPayload<'_>,
) -> Result<Request, RuntimeFailure> {
    let mut body = json!({
        "model": {"providerID": provider_id, "modelID": model_id},
        "parts": [{"type": "text", "text": payload.content}]
    });
    if let Some(reasoning) = payload.reasoning {
        body["variant"] = json!(reasoning.as_str());
    }
    if let Some(file) = payload.file {
        body["parts"]
            .as_array_mut()
            .expect("prompt parts are an array")
            .push(json!({
                "type": "file",
                "mime": file.media_type,
                "filename": file.filename,
                "url": file.data_url,
            }));
    }
    if let Some(output) = payload.structured_output {
        let schema = match output.document() {
            swallowtail_runtime::SchemaDocument::Inline(bytes) => {
                serde_json::from_slice::<Value>(bytes).map_err(|_| {
                    failure(
                        "swallowtail.opencode.schema_invalid",
                        "OpenCode structured-output schema could not be encoded",
                    )
                })?
            }
            swallowtail_runtime::SchemaDocument::Reference(_) => {
                return Err(failure(
                    "swallowtail.opencode.schema_invalid",
                    "OpenCode structured-output schema could not be encoded",
                ));
            }
        };
        body["format"] = json!({
            "type": "json_schema",
            "schema": schema,
            "retryCount": 0
        });
    }
    Ok(
        Request::post(format!("/session/{session_id}/prompt_async"), Some(body))
            .with_directory(directory),
    )
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProviderRequestKind {
    Permission,
    Question { count: usize },
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct PendingProviderRequest {
    pub(crate) id: String,
    pub(crate) kind: ProviderRequestKind,
    pub(crate) payload: Vec<u8>,
}

pub(crate) fn callback_response(
    provider_id: &str,
    kind: ProviderRequestKind,
    result: &CallbackResult,
) -> Result<Request, RuntimeFailure> {
    let id = safe_path_id(provider_id)?;
    match kind {
        ProviderRequestKind::Permission => {
            let approved = match result {
                CallbackResult::Failure { .. } => false,
                CallbackResult::Success(payload) => {
                    let value: Value = serde_json::from_slice(payload.as_bytes())
                        .map_err(|_| callback_malformed())?;
                    let object = value
                        .as_object()
                        .filter(|object| object.len() == 1)
                        .ok_or_else(callback_malformed)?;
                    match object.get("reply").and_then(Value::as_str) {
                        Some("once") => true,
                        Some("reject") => false,
                        _ => return Err(callback_malformed()),
                    }
                }
            };
            let body = if approved {
                json!({"reply": "once"})
            } else {
                json!({
                    "reply": "reject",
                    "message": "Consumer rejected the one-shot request."
                })
            };
            Ok(Request::post(format!("/permission/{id}/reply"), Some(body)))
        }
        ProviderRequestKind::Question { count } => match result {
            CallbackResult::Failure { .. } => {
                Ok(Request::post(format!("/question/{id}/reject"), None))
            }
            CallbackResult::Success(payload) => {
                let value: Value =
                    serde_json::from_slice(payload.as_bytes()).map_err(|_| callback_malformed())?;
                validate_answers(&value, count)?;
                Ok(Request::post(format!("/question/{id}/reply"), Some(value)))
            }
        },
    }
}

fn validate_answers(value: &Value, question_count: usize) -> Result<(), RuntimeFailure> {
    let object = value
        .as_object()
        .filter(|object| object.len() == 1)
        .ok_or_else(callback_malformed)?;
    let answers = object
        .get("answers")
        .and_then(Value::as_array)
        .filter(|answers| answers.len() == question_count)
        .ok_or_else(callback_malformed)?;
    for answer in answers {
        let selections = answer
            .as_array()
            .filter(|selections| !selections.is_empty() && selections.len() <= 32)
            .ok_or_else(callback_malformed)?;
        if selections.iter().any(|selection| {
            selection
                .as_str()
                .is_none_or(|selection| selection.is_empty() || selection.len() > 4096)
        }) {
            return Err(callback_malformed());
        }
    }
    Ok(())
}

fn safe_path_id(value: &str) -> Result<&str, RuntimeFailure> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'~'))
    {
        Err(callback_malformed())
    } else {
        Ok(value)
    }
}

fn callback_malformed() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.callback_malformed",
        "OpenCode callback response was malformed",
    )
}

pub(crate) fn abort(session_id: &str, directory: &str) -> Request {
    Request::post(format!("/session/{session_id}/abort"), None).with_directory(directory)
}

pub(crate) fn session_delete(session_id: &str, directory: &str) -> Result<Request, RuntimeFailure> {
    if session_id.is_empty()
        || !session_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b'~'))
    {
        return Err(failure(
            "swallowtail.opencode.session_invalid",
            "OpenCode session identity is not a safe HTTP path segment",
        ));
    }
    Ok(Request::delete(format!("/session/{session_id}")).with_directory(directory))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SessionDeleteResponse {
    Applied,
    Rejected,
    Unconfirmed,
}

pub(crate) fn classify_session_delete(response: &Response) -> SessionDeleteResponse {
    if response.status == 200
        && serde_json::from_slice::<bool>(&response.body).is_ok_and(|value| value)
    {
        SessionDeleteResponse::Applied
    } else if (400..500).contains(&response.status) {
        SessionDeleteResponse::Rejected
    } else {
        SessionDeleteResponse::Unconfirmed
    }
}

pub(crate) fn require_no_content(response: &Response) -> Result<(), RuntimeFailure> {
    if response.status == 204 {
        Ok(())
    } else {
        Err(http_failure("prompt request"))
    }
}

pub(crate) fn require_abort_success(response: &Response) -> Result<(), RuntimeFailure> {
    require_success(response, "abort request")?;
    match serde_json::from_slice::<bool>(&response.body) {
        Ok(true) => Ok(()),
        _ => Err(failure(
            "swallowtail.opencode.abort_failed",
            "OpenCode did not acknowledge session abort",
        )),
    }
}

fn require_success(response: &Response, operation: &'static str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        Ok(())
    } else {
        Err(http_failure(operation))
    }
}

fn http_failure(operation: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.opencode.http_failed",
        format!("OpenCode {operation} failed"),
    ))
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    operation: &'static str,
) -> Result<T, RuntimeFailure> {
    serde_json::from_slice(bytes).map_err(|_| {
        RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.opencode.protocol_invalid",
            format!("OpenCode {operation} was invalid"),
        ))
    })
}

include!("protocol/events.rs");
include!("protocol/tests.rs");
