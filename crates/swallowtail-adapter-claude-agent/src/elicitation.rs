use crate::failure::malformed;
use serde_json::{Map, Value, json};
use swallowtail_runtime::{
    HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputChoiceMode, HarnessUserInputOption,
    HarnessUserInputQuestion, HarnessUserInputQuestionKind, HarnessUserInputRequest,
    HarnessUserInputResponse, OperationContent, RuntimeFailure,
};

const MAXIMUM_QUESTIONS: usize = 4;
const MAXIMUM_OPTIONS: usize = 4;
const MAXIMUM_BYTES: usize = 64 * 1024;
const MULTIPLE_QUESTION_MESSAGE: &str = "Please answer the following questions.";
const CUSTOM_DESCRIPTION: &str =
    "Type your own answer instead of choosing an option above (optional).";
const OPTION_META_KEY: &str = "_claude/askUserQuestionOption";
const CUSTOM_META_KEY: &str = "_askUserQuestionCustomAnswer";

pub(crate) fn request(params: &Value) -> Result<Option<HarnessUserInputRequest>, RuntimeFailure> {
    if params.get("mode").and_then(Value::as_str) != Some("form")
        || params
            .get("toolCallId")
            .and_then(Value::as_str)
            .is_none_or(str::is_empty)
    {
        return Ok(None);
    }
    let message = params
        .get("message")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(malformed)?;
    let schema = params
        .get("requestedSchema")
        .and_then(Value::as_object)
        .ok_or_else(malformed)?;
    if schema.get("type").and_then(Value::as_str) != Some("object")
        || schema
            .get("required")
            .is_some_and(|value| value.as_array().is_none_or(|items| !items.is_empty()))
    {
        return Ok(None);
    }
    let properties = schema
        .get("properties")
        .and_then(Value::as_object)
        .ok_or_else(malformed)?;
    let count = paired_question_count(properties);
    if count == 0
        || count > MAXIMUM_QUESTIONS
        || properties.len() != count.saturating_mul(2)
        || (count > 1 && message != MULTIPLE_QUESTION_MESSAGE)
    {
        return Ok(None);
    }

    let mut questions = Vec::with_capacity(count);
    for index in 0..count {
        let field_id = question_field(index);
        let field = properties
            .get(&field_id)
            .and_then(Value::as_object)
            .ok_or_else(malformed)?;
        let custom = properties
            .get(&custom_field(index))
            .and_then(Value::as_object)
            .ok_or_else(malformed)?;
        if !custom_field_is_supported(custom, &field_id) {
            return Ok(None);
        }
        let header = field
            .get("title")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(malformed)?;
        let prompt = if count == 1 {
            message
        } else {
            field
                .get("description")
                .and_then(Value::as_str)
                .filter(|value| !value.trim().is_empty())
                .ok_or_else(malformed)?
        };
        let (mode, options) = choice_field(field)?;
        let Some(options) = options else {
            return Ok(None);
        };
        questions.push(
            HarnessUserInputQuestion::new(
                HarnessQuestionId::new(field_id).map_err(|_| malformed())?,
                OperationContent::new(header).map_err(|_| malformed())?,
                OperationContent::new(prompt).map_err(|_| malformed())?,
                HarnessUserInputQuestionKind::Choice {
                    mode,
                    allow_other: true,
                },
                options,
            )
            .map_err(|_| malformed())?,
        );
    }

    HarnessUserInputRequest::new(
        questions,
        None,
        MAXIMUM_QUESTIONS,
        MAXIMUM_OPTIONS,
        MAXIMUM_BYTES,
    )
    .map(Some)
    .map_err(|_| malformed())
}

pub(crate) fn response_content(
    request: &HarnessUserInputRequest,
    response: &HarnessUserInputResponse,
) -> Result<Value, RuntimeFailure> {
    if !request.accepts(response) {
        return Err(malformed());
    }
    let mut content = Map::new();
    for question in request.questions() {
        let answer = response
            .answers()
            .find(|answer| answer.question_id() == question.id())
            .ok_or_else(malformed)?;
        if answer.is_skipped() {
            continue;
        }
        if let Some(text) = answer.text() {
            content.insert(
                format!("{}_custom", question.id().as_str()),
                Value::String(text.as_str().to_owned()),
            );
            continue;
        }
        let selected = answer
            .selected_options()
            .map(|option| Value::String(option.as_str().to_owned()))
            .collect::<Vec<_>>();
        let value = match question.kind() {
            HarnessUserInputQuestionKind::Choice {
                mode: HarnessUserInputChoiceMode::Single,
                ..
            } => selected.into_iter().next().ok_or_else(malformed)?,
            HarnessUserInputQuestionKind::Choice {
                mode: HarnessUserInputChoiceMode::Multiple,
                ..
            } => Value::Array(selected),
            HarnessUserInputQuestionKind::Text { .. } => return Err(malformed()),
        };
        content.insert(question.id().as_str().to_owned(), value);
    }
    Ok(Value::Object(content))
}

fn paired_question_count(properties: &Map<String, Value>) -> usize {
    (0..MAXIMUM_QUESTIONS)
        .take_while(|index| {
            properties.contains_key(&question_field(*index))
                && properties.contains_key(&custom_field(*index))
        })
        .count()
}

fn question_field(index: usize) -> String {
    format!("question_{index}")
}

fn custom_field(index: usize) -> String {
    format!("question_{index}_custom")
}

fn custom_field_is_supported(field: &Map<String, Value>, question_id: &str) -> bool {
    if field.get("type").and_then(Value::as_str) != Some("string")
        || field.get("title").and_then(Value::as_str) != Some("Other")
        || field.get("description").and_then(Value::as_str) != Some(CUSTOM_DESCRIPTION)
    {
        return false;
    }
    match field.get("_meta") {
        None | Some(Value::Null) => true,
        Some(meta) => meta.get(CUSTOM_META_KEY).is_some_and(|marker| {
            marker.get("questionId").and_then(Value::as_str) == Some(question_id)
                && marker.get("isCustomAnswer").and_then(Value::as_bool) == Some(true)
        }),
    }
}

fn choice_field(
    field: &Map<String, Value>,
) -> Result<
    (
        HarnessUserInputChoiceMode,
        Option<Vec<HarnessUserInputOption>>,
    ),
    RuntimeFailure,
> {
    let (mode, options) = match field.get("type").and_then(Value::as_str) {
        Some("string") => (
            HarnessUserInputChoiceMode::Single,
            field.get("oneOf").and_then(Value::as_array),
        ),
        Some("array") => (
            HarnessUserInputChoiceMode::Multiple,
            field
                .get("items")
                .and_then(Value::as_object)
                .and_then(|items| items.get("anyOf"))
                .and_then(Value::as_array),
        ),
        _ => return Ok((HarnessUserInputChoiceMode::Single, None)),
    };
    let Some(options) = options.filter(|items| (2..=MAXIMUM_OPTIONS).contains(&items.len())) else {
        return Ok((mode, None));
    };
    let mut parsed = Vec::with_capacity(options.len());
    for option in options {
        let option = option.as_object().ok_or_else(malformed)?;
        let label = option
            .get("const")
            .and_then(Value::as_str)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(malformed)?;
        let Some(description) = option_description(option, label)? else {
            return Ok((mode, None));
        };
        parsed.push(HarnessUserInputOption::new(
            HarnessQuestionOptionId::new(label).map_err(|_| malformed())?,
            OperationContent::new(label).map_err(|_| malformed())?,
            description
                .map(OperationContent::new)
                .transpose()
                .map_err(|_| malformed())?,
        ));
    }
    Ok((mode, Some(parsed)))
}

fn option_description<'a>(
    option: &'a Map<String, Value>,
    label: &str,
) -> Result<Option<Option<&'a str>>, RuntimeFailure> {
    let title = option
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(malformed)?;
    let native = option.get("description").and_then(Value::as_str);
    let detail = option
        .get("_meta")
        .and_then(|meta| meta.get(OPTION_META_KEY));
    if detail
        .and_then(|value| value.get("preview"))
        .is_some_and(|value| !value.is_null())
    {
        return Ok(None);
    }
    let legacy = detail
        .and_then(|value| value.get("description"))
        .and_then(Value::as_str);
    let description = native.or(legacy);
    let title_matches = match description {
        Some(_) if native.is_some() => title == label,
        Some(description) => title == format!("{label} — {description}"),
        None => title == label,
    };
    if !title_matches {
        return Ok(None);
    }
    Ok(Some(description))
}

pub(crate) fn accepted_response(content: Value) -> Value {
    json!({"action": "accept", "content": content})
}

pub(crate) fn declined_response() -> Value {
    json!({"action": "decline"})
}

#[cfg(test)]
mod tests {
    use super::*;
    use swallowtail_runtime::HarnessUserInputAnswer;

    fn form(selection: Value, custom_meta: Option<Value>) -> Value {
        let mut custom = json!({
            "type": "string",
            "title": "Other",
            "description": CUSTOM_DESCRIPTION
        });
        if let Some(meta) = custom_meta {
            custom["_meta"] = meta;
        }
        json!({
            "mode": "form",
            "sessionId": "session",
            "toolCallId": "tool",
            "message": "Which component should be used?",
            "requestedSchema": {
                "type": "object",
                "properties": {
                    "question_0": selection,
                    "question_0_custom": custom
                }
            }
        })
    }

    #[test]
    fn current_form_maps_to_typed_question_and_back_to_updated_input_content() {
        let params = form(
            json!({
                "type": "string",
                "title": "Component",
                "oneOf": [
                    {"const": "Card", "title": "Card", "description": "Use the card."},
                    {"const": "Panel", "title": "Panel", "description": "Use the panel."}
                ]
            }),
            Some(json!({
                CUSTOM_META_KEY: {"questionId": "question_0", "isCustomAnswer": true}
            })),
        );
        let request = request(&params).unwrap().expect("form is representable");
        let response = HarnessUserInputResponse::new(
            [HarnessUserInputAnswer::selected(
                HarnessQuestionId::new("question_0").unwrap(),
                [HarnessQuestionOptionId::new("Panel").unwrap()],
                None,
            )],
            4,
            1024,
        )
        .unwrap();

        assert_eq!(
            accepted_response(response_content(&request, &response).unwrap()),
            json!({"action": "accept", "content": {"question_0": "Panel"}})
        );
    }

    #[test]
    fn legacy_option_metadata_and_custom_text_remain_lossless() {
        let params = form(
            json!({
                "type": "array",
                "title": "Components",
                "items": {"anyOf": [
                    {
                        "const": "Card",
                        "title": "Card — Use the card.",
                        "_meta": {OPTION_META_KEY: {"description": "Use the card."}}
                    },
                    {
                        "const": "Panel",
                        "title": "Panel — Use the panel.",
                        "_meta": {OPTION_META_KEY: {"description": "Use the panel."}}
                    }
                ]}
            }),
            None,
        );
        let request = request(&params)
            .unwrap()
            .expect("legacy form is representable");
        let response = HarnessUserInputResponse::new(
            [HarnessUserInputAnswer::selected(
                HarnessQuestionId::new("question_0").unwrap(),
                [],
                Some(OperationContent::new("Custom component").unwrap()),
            )],
            4,
            1024,
        )
        .unwrap();

        assert_eq!(
            response_content(&request, &response).unwrap(),
            json!({"question_0_custom": "Custom component"})
        );
    }

    #[test]
    fn richer_forms_and_option_previews_are_not_flattened() {
        let numeric = json!({
            "mode": "form",
            "sessionId": "session",
            "toolCallId": "tool",
            "message": "Choose",
            "requestedSchema": {
                "type": "object",
                "properties": {"count": {"type": "integer"}}
            }
        });
        assert!(request(&numeric).unwrap().is_none());

        let preview = form(
            json!({
                "type": "string",
                "title": "Component",
                "oneOf": [
                    {
                        "const": "Card",
                        "title": "Card",
                        "_meta": {OPTION_META_KEY: {"preview": "<card />"}}
                    },
                    {"const": "Panel", "title": "Panel"}
                ]
            }),
            None,
        );
        assert!(request(&preview).unwrap().is_none());
    }
}
