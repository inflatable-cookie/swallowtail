use super::fields::{
    ensure_collection_bound, error, object, optional_text, required_array, required_identifier,
    required_str, required_text,
};
use crate::activity::{
    AcpBoundedText, AcpConfigCategory, AcpConfigChoice, AcpConfigChoices, AcpConfigGroup,
    AcpConfigKind, AcpConfigOption, ActivityDecodeError, ActivityDecodeErrorKind,
    ActivityDecodeLimits,
};
use serde_json::{Map, Value};

pub(super) fn options(
    update: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<Vec<AcpConfigOption>, ActivityDecodeError> {
    required_array(
        update,
        "configOptions",
        ActivityDecodeErrorKind::MetadataInvalid,
        limits,
    )?
    .iter()
    .map(|option| decode_option(option, limits))
    .collect()
}

fn decode_option(
    option: &Value,
    limits: ActivityDecodeLimits,
) -> Result<AcpConfigOption, ActivityDecodeError> {
    let option = object(option, ActivityDecodeErrorKind::MetadataInvalid)?;
    let category = optional_category(option, limits)?;
    let kind = match required_str(option, "type", ActivityDecodeErrorKind::MetadataInvalid)? {
        "select" => AcpConfigKind::Select {
            current_value: required_identifier(
                option,
                "currentValue",
                ActivityDecodeErrorKind::MetadataInvalid,
                limits,
            )?,
            options: select_choices(option, limits)?,
        },
        "boolean" => AcpConfigKind::Boolean {
            current_value: option
                .get("currentValue")
                .and_then(Value::as_bool)
                .ok_or_else(|| error(ActivityDecodeErrorKind::MetadataInvalid))?,
        },
        _ => return Err(error(ActivityDecodeErrorKind::MetadataInvalid)),
    };
    Ok(AcpConfigOption {
        id: required_identifier(
            option,
            "id",
            ActivityDecodeErrorKind::MetadataInvalid,
            limits,
        )?,
        name: required_text(option, "name", ActivityDecodeErrorKind::MetadataInvalid)?,
        description: optional_text(
            option,
            "description",
            ActivityDecodeErrorKind::MetadataInvalid,
        )?,
        category,
        kind,
    })
}

fn optional_category(
    option: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<Option<AcpConfigCategory>, ActivityDecodeError> {
    let category = match option.get("category") {
        None | Some(Value::Null) => return Ok(None),
        Some(category) => category,
    };
    let category = category
        .as_str()
        .ok_or_else(|| error(ActivityDecodeErrorKind::MetadataInvalid))?;
    super::fields::validate_identifier(category, limits)
        .map_err(|_| error(ActivityDecodeErrorKind::MetadataInvalid))?;
    Ok(Some(match category {
        "mode" => AcpConfigCategory::Mode,
        "model" => AcpConfigCategory::Model,
        "model_config" => AcpConfigCategory::ModelConfig,
        "thought_level" => AcpConfigCategory::ThoughtLevel,
        _ => AcpConfigCategory::Other(AcpBoundedText(category.to_owned())),
    }))
}

fn select_choices(
    option: &Map<String, Value>,
    limits: ActivityDecodeLimits,
) -> Result<AcpConfigChoices, ActivityDecodeError> {
    let values = required_array(
        option,
        "options",
        ActivityDecodeErrorKind::MetadataInvalid,
        limits,
    )?;
    let grouped = values
        .first()
        .and_then(Value::as_object)
        .is_some_and(|value| value.contains_key("group"));
    if grouped {
        values
            .iter()
            .map(|group| {
                let group = object(group, ActivityDecodeErrorKind::MetadataInvalid)?;
                let options = required_array(
                    group,
                    "options",
                    ActivityDecodeErrorKind::MetadataInvalid,
                    limits,
                )?
                .iter()
                .map(|choice| decode_choice(choice, limits))
                .collect::<Result<Vec<_>, _>>()?;
                ensure_collection_bound(options.len(), limits)?;
                Ok(AcpConfigGroup {
                    group: required_identifier(
                        group,
                        "group",
                        ActivityDecodeErrorKind::MetadataInvalid,
                        limits,
                    )?,
                    name: required_text(group, "name", ActivityDecodeErrorKind::MetadataInvalid)?,
                    options,
                })
            })
            .collect::<Result<Vec<_>, _>>()
            .map(AcpConfigChoices::Grouped)
    } else {
        values
            .iter()
            .map(|choice| decode_choice(choice, limits))
            .collect::<Result<Vec<_>, _>>()
            .map(AcpConfigChoices::Ungrouped)
    }
}

fn decode_choice(
    choice: &Value,
    limits: ActivityDecodeLimits,
) -> Result<AcpConfigChoice, ActivityDecodeError> {
    let choice = object(choice, ActivityDecodeErrorKind::MetadataInvalid)?;
    Ok(AcpConfigChoice {
        value: required_identifier(
            choice,
            "value",
            ActivityDecodeErrorKind::MetadataInvalid,
            limits,
        )?,
        name: required_text(choice, "name", ActivityDecodeErrorKind::MetadataInvalid)?,
        description: optional_text(
            choice,
            "description",
            ActivityDecodeErrorKind::MetadataInvalid,
        )?,
    })
}
