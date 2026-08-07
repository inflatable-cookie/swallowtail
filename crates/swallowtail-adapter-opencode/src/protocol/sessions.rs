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

pub(crate) fn session_list(directory: &str, start: u32, limit: u32) -> Request {
    Request::get("/session")
        .with_directory(directory)
        .with_query("start", start.to_string())
        .with_query("limit", limit.to_string())
}

pub(crate) fn session_status(directory: &str) -> Request {
    Request::get("/session/status").with_directory(directory)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OpenCodeSessionObservation {
    pub id: String,
    pub directory: String,
    pub title: String,
    pub version: String,
    pub updated_at: u64,
    pub parent: bool,
    pub archived: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum OpenCodeSessionStatus {
    Idle,
    Active,
    Unavailable,
}

pub(crate) fn parse_session_list(response: &Response) -> Result<Vec<OpenCodeSessionObservation>, RuntimeFailure> {
    require_success(response, "session catalogue request")?;
    let values: Vec<Value> = parse_json(&response.body, "session catalogue response")?;
    values.into_iter().map(parse_session_observation).collect()
}

pub(crate) fn parse_session_lookup(response: &Response) -> Result<OpenCodeSessionObservation, RuntimeFailure> {
    require_success(response, "session lookup request")?;
    let value: Value = parse_json(&response.body, "session lookup response")?;
    parse_session_observation(value)
}

pub(crate) fn parse_session_statuses(response: &Response) -> Result<BTreeMap<String, OpenCodeSessionStatus>, RuntimeFailure> {
    require_success(response, "session status request")?;
    let values: BTreeMap<String, Value> = parse_json(&response.body, "session status response")?;
    values.into_iter().map(|(id, value)| {
        if id.trim().is_empty() {
            return Err(session_catalogue_invalid());
        }
        let status = match value.get("type").and_then(Value::as_str) {
            Some("idle") => OpenCodeSessionStatus::Idle,
            Some("busy" | "retry") => OpenCodeSessionStatus::Active,
            _ => OpenCodeSessionStatus::Unavailable,
        };
        Ok((id, status))
    }).collect()
}

fn parse_session_observation(value: Value) -> Result<OpenCodeSessionObservation, RuntimeFailure> {
    let string = |key: &str| value.get(key).and_then(Value::as_str).filter(|value| !value.trim().is_empty()).map(str::to_owned).ok_or_else(session_catalogue_invalid);
    let time = value.get("time").and_then(Value::as_object).ok_or_else(session_catalogue_invalid)?;
    let updated_at = time.get("updated").and_then(Value::as_u64).ok_or_else(session_catalogue_invalid)?;
    let parent = value.get("parentID").is_some_and(|parent| parent.as_str().is_some_and(|parent| !parent.trim().is_empty()));
    let archived = time.get("archived").is_some_and(|archived| !archived.is_null());
    Ok(OpenCodeSessionObservation {
        id: string("id")?,
        directory: string("directory")?,
        title: string("title")?,
        version: string("version")?,
        updated_at,
        parent,
        archived,
    })
}

fn session_catalogue_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.session_catalogue.invalid_response",
        "OpenCode returned malformed session catalogue evidence",
    )
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
