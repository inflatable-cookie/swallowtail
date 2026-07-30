use crate::turn_state::malformed_notification;
use serde_json::{Map, Value, json};
use swallowtail_runtime::{
    HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputAnswer, HarnessUserInputChoiceMode,
    HarnessUserInputOption, HarnessUserInputQuestion, HarnessUserInputQuestionKind,
    HarnessUserInputRequest, HarnessUserInputResponse, OperationContent, RuntimeFailure,
};

const MAXIMUM_QUESTIONS: usize = 16;
const MAXIMUM_OPTIONS: usize = 32;
const MAXIMUM_BYTES: usize = 64 * 1024;

pub(crate) fn request(params: &Value) -> Result<HarnessUserInputRequest, RuntimeFailure> {
    let questions = params
        .get("questions")
        .and_then(Value::as_array)
        .ok_or_else(malformed_notification)?
        .iter()
        .map(question)
        .collect::<Result<Vec<_>, _>>()?;
    let auto_resolution_ms = match params.get("autoResolutionMs") {
        None | Some(Value::Null) => None,
        Some(value) => Some(value.as_u64().ok_or_else(malformed_notification)?),
    };
    HarnessUserInputRequest::new(
        questions,
        auto_resolution_ms,
        MAXIMUM_QUESTIONS,
        MAXIMUM_OPTIONS,
        MAXIMUM_BYTES,
    )
    .map_err(|_| malformed_notification())
}

fn question(value: &Value) -> Result<HarnessUserInputQuestion, RuntimeFailure> {
    let object = value.as_object().ok_or_else(malformed_notification)?;
    let id = HarnessQuestionId::new(required_text(object, "id")?)
        .map_err(|_| malformed_notification())?;
    let header = OperationContent::new(required_text(object, "header")?)
        .map_err(|_| malformed_notification())?;
    let prompt = OperationContent::new(required_text(object, "question")?)
        .map_err(|_| malformed_notification())?;
    let allow_other = optional_bool(object, "isOther")?;
    let secret = optional_bool(object, "isSecret")?;
    let options = match object.get("options") {
        None | Some(Value::Null) => Vec::new(),
        Some(value) => value
            .as_array()
            .ok_or_else(malformed_notification)?
            .iter()
            .map(option)
            .collect::<Result<Vec<_>, _>>()?,
    };
    let kind = if options.is_empty() {
        HarnessUserInputQuestionKind::Text { secret }
    } else {
        if secret {
            return Err(malformed_notification());
        }
        HarnessUserInputQuestionKind::Choice {
            mode: HarnessUserInputChoiceMode::Single,
            allow_other,
        }
    };
    HarnessUserInputQuestion::new(id, header, prompt, kind, options)
        .map_err(|_| malformed_notification())
}

fn option(value: &Value) -> Result<HarnessUserInputOption, RuntimeFailure> {
    let object = value.as_object().ok_or_else(malformed_notification)?;
    let label = required_text(object, "label")?;
    Ok(HarnessUserInputOption::new(
        HarnessQuestionOptionId::new(label).map_err(|_| malformed_notification())?,
        OperationContent::new(label).map_err(|_| malformed_notification())?,
        Some(
            OperationContent::new(required_text(object, "description")?)
                .map_err(|_| malformed_notification())?,
        ),
    ))
}

pub(crate) fn response(
    request: &HarnessUserInputRequest,
    result: &swallowtail_runtime::CallbackResult,
) -> Result<Value, RuntimeFailure> {
    let answers = match result {
        swallowtail_runtime::CallbackResult::UserInput(response) => {
            if !request.accepts(response) {
                return Err(callback_response_invalid());
            }
            response
        }
        swallowtail_runtime::CallbackResult::Failure { .. } => {
            return Ok(skipped_response(request));
        }
        swallowtail_runtime::CallbackResult::Success(_) => {
            return Err(callback_response_invalid());
        }
    };
    Ok(json!({"answers": answer_map(answers)}))
}

fn answer_map(response: &HarnessUserInputResponse) -> Map<String, Value> {
    response
        .answers()
        .map(|answer| {
            let values = answer_values(answer);
            (
                answer.question_id().as_str().to_owned(),
                json!({"answers": values}),
            )
        })
        .collect()
}

fn answer_values(answer: &HarnessUserInputAnswer) -> Vec<String> {
    if answer.is_skipped() {
        return Vec::new();
    }
    let mut values = answer
        .selected_options()
        .map(|id| id.as_str().to_owned())
        .collect::<Vec<_>>();
    if let Some(text) = answer.text() {
        values.push(text.as_str().to_owned());
    }
    values
}

fn skipped_response(request: &HarnessUserInputRequest) -> Value {
    let answers: Map<String, Value> = request
        .questions()
        .map(|question| {
            (
                question.id().as_str().to_owned(),
                json!({"answers": Vec::<String>::new()}),
            )
        })
        .collect();
    json!({"answers": answers})
}

fn required_text<'a>(
    object: &'a Map<String, Value>,
    field: &str,
) -> Result<&'a str, RuntimeFailure> {
    object
        .get(field)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed_notification)
}

fn optional_bool(object: &Map<String, Value>, field: &str) -> Result<bool, RuntimeFailure> {
    match object.get(field) {
        None => Ok(false),
        Some(value) => value.as_bool().ok_or_else(malformed_notification),
    }
}

fn callback_response_invalid() -> RuntimeFailure {
    crate::rpc::failure(
        "swallowtail.codex.app_server.user_input_response_invalid",
        "Codex user-input response did not match the pending questions",
    )
}
