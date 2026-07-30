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
    user_input: Option<&HarnessUserInputRequest>,
    result: &CallbackResult,
) -> Result<Request, RuntimeFailure> {
    let id = safe_path_id(provider_id)?;
    match kind {
        ProviderRequestKind::Permission => {
            let approved = match result {
                CallbackResult::Failure { .. } => false,
                CallbackResult::UserInput(_) => return Err(callback_malformed()),
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
            CallbackResult::UserInput(response) => {
                let request = user_input.ok_or_else(callback_malformed)?;
                if request.questions().len() != count {
                    return Err(callback_malformed());
                }
                if !request.accepts(response) {
                    return Err(callback_malformed());
                }
                let value = typed_answers(request, response)?;
                Ok(Request::post(format!("/question/{id}/reply"), Some(value)))
            }
            CallbackResult::Success(_) => Err(callback_malformed()),
        },
    }
}

pub(crate) fn question_request(payload: &[u8]) -> Result<HarnessUserInputRequest, RuntimeFailure> {
    let value: Value = serde_json::from_slice(payload).map_err(|_| callback_malformed())?;
    let questions = value
        .get("questions")
        .and_then(Value::as_array)
        .ok_or_else(callback_malformed)?
        .iter()
        .enumerate()
        .map(|(index, question)| {
            let question = question.as_object().ok_or_else(callback_malformed)?;
            let multiple = question
                .get("multiple")
                .and_then(Value::as_bool)
                .ok_or_else(callback_malformed)?;
            let options = question
                .get("options")
                .and_then(Value::as_array)
                .ok_or_else(callback_malformed)?
                .iter()
                .map(|option| {
                    let option = option.as_object().ok_or_else(callback_malformed)?;
                    let label = option
                        .get("label")
                        .and_then(Value::as_str)
                        .ok_or_else(callback_malformed)?;
                    let description = option
                        .get("description")
                        .and_then(Value::as_str)
                        .ok_or_else(callback_malformed)?;
                    Ok(HarnessUserInputOption::new(
                        HarnessQuestionOptionId::new(label).map_err(|_| callback_malformed())?,
                        OperationContent::new(label).map_err(|_| callback_malformed())?,
                        Some(OperationContent::new(description).map_err(|_| callback_malformed())?),
                    ))
                })
                .collect::<Result<Vec<_>, RuntimeFailure>>()?;
            HarnessUserInputQuestion::new(
                HarnessQuestionId::new(format!("question-{index}"))
                    .map_err(|_| callback_malformed())?,
                OperationContent::new(
                    question
                        .get("header")
                        .and_then(Value::as_str)
                        .ok_or_else(callback_malformed)?,
                )
                .map_err(|_| callback_malformed())?,
                OperationContent::new(
                    question
                        .get("question")
                        .and_then(Value::as_str)
                        .ok_or_else(callback_malformed)?,
                )
                .map_err(|_| callback_malformed())?,
                HarnessUserInputQuestionKind::Choice {
                    mode: if multiple {
                        HarnessUserInputChoiceMode::Multiple
                    } else {
                        HarnessUserInputChoiceMode::Single
                    },
                    allow_other: false,
                },
                options,
            )
            .map_err(|_| callback_malformed())
        })
        .collect::<Result<Vec<_>, _>>()?;
    HarnessUserInputRequest::new(questions, None, 32, 32, 256 * 1024)
        .map_err(|_| callback_malformed())
}

fn typed_answers(
    request: &HarnessUserInputRequest,
    response: &swallowtail_runtime::HarnessUserInputResponse,
) -> Result<Value, RuntimeFailure> {
    let answers = request
        .questions()
        .map(|question| {
            let answer = response
                .answers()
                .find(|answer| answer.question_id() == question.id())
                .ok_or_else(callback_malformed)?;
            if answer.is_skipped() || answer.text().is_some() {
                return Err(callback_malformed());
            }
            let values = answer
                .selected_options()
                .map(|id| Value::String(id.as_str().to_owned()))
                .collect::<Vec<_>>();
            if values.is_empty() {
                return Err(callback_malformed());
            }
            Ok(Value::Array(values))
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    Ok(json!({"answers": answers}))
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

