use super::{
    ClaudeAgentSdkBashCommandView, ClaudeAgentSdkCallback, ClaudeAgentSdkCommand,
    ClaudeAgentSdkDiagnostic, ClaudeAgentSdkDiagnosticEvidence, ClaudeAgentSdkDiagnosticLevel,
    ClaudeAgentSdkEvent, ClaudeAgentSdkFailure, ClaudeAgentSdkFailureCode,
    ClaudeAgentSdkIdentityEvidence, ClaudeAgentSdkModelQualificationEvidence,
    ClaudeAgentSdkResponse, MAXIMUM_COMMAND_ID_BYTES, MAXIMUM_FAILURE_CODE_BYTES,
    MAXIMUM_FAILURE_MESSAGE_BYTES, MAXIMUM_MODEL_QUALIFICATION_CATALOGUE_SIZE,
    MAXIMUM_MODEL_QUALIFICATION_ID_BYTES, MAXIMUM_TEXT_BYTES, MODEL_QUALIFICATION_DIGEST,
    MODEL_QUALIFICATION_DIGEST_HEX_BYTES, bounded_text, failure, required_bool,
};
use crate::sdk::protocol::{ClaudeAgentSdkProtocolFailure, ClaudeAgentSdkProtocolFailureKind};
use serde_json::Value;
use std::collections::BTreeMap;

const MAXIMUM_RESULT_FIELDS: usize = 64;

pub(super) fn decode_response(
    value: &Value,
) -> Result<ClaudeAgentSdkResponse, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidResponse;
    let id = bounded_text(value, "id", MAXIMUM_COMMAND_ID_BYTES, invalid)?.to_owned();
    let command = bounded_text(value, "command", MAXIMUM_COMMAND_ID_BYTES, invalid)?;
    if ClaudeAgentSdkCommand::from_qualified(command).is_none() {
        return Err(failure(invalid));
    }
    let success = required_bool(value, "success", invalid)?;
    let data = value.get("data").cloned();
    match (success, &data, value.get("failure")) {
        (true, _, None) => {
            if data.as_ref().is_some_and(|data| !data.is_object()) {
                return Err(failure(invalid));
            }
            Ok(ClaudeAgentSdkResponse {
                id,
                command: command.to_owned(),
                success,
                data,
                failure_code: None,
                original_failure_code: None,
            })
        }
        (false, None, Some(record)) => {
            let failure = decode_failure(record, invalid)?;
            Ok(ClaudeAgentSdkResponse {
                id,
                command: command.to_owned(),
                success,
                data: None,
                failure_code: Some(failure.code),
                original_failure_code: failure.original_code,
            })
        }
        _ => Err(failure(invalid)),
    }
}

pub(super) fn decode_event(
    value: &Value,
) -> Result<ClaudeAgentSdkEvent, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidEvent;
    match value.get("event").and_then(Value::as_str) {
        Some("turn_started") => Ok(ClaudeAgentSdkEvent::TurnStarted),
        Some("progress") => Ok(ClaudeAgentSdkEvent::Progress),
        Some("turn_failed") => Ok(ClaudeAgentSdkEvent::TurnFailed),
        Some("output_delta") => {
            let delta = value
                .get("delta")
                .and_then(Value::as_str)
                .ok_or_else(|| failure(invalid))?;
            if delta.len() > super::MAXIMUM_RECORD_BYTES {
                return Err(failure(invalid));
            }
            Ok(ClaudeAgentSdkEvent::OutputDelta(delta.to_owned()))
        }
        Some("tool_started") => Ok(ClaudeAgentSdkEvent::ToolStarted {
            call_id: bounded_text(value, "toolCallId", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
            name: bounded_text(value, "toolName", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
        }),
        Some("tool_ended") => Ok(ClaudeAgentSdkEvent::ToolEnded {
            call_id: bounded_text(value, "toolCallId", MAXIMUM_TEXT_BYTES, invalid)?.to_owned(),
            failed: required_bool(value, "isError", invalid)?,
        }),
        Some("turn_ended") => {
            let subtype = nullable_label(value, "subtype", invalid)?;
            let stop_reason = bounded_label_allow_empty(value, "stopReason", invalid)?.to_owned();
            let failed = required_bool(value, "isError", invalid)?;
            let num_turns = nullable_nonnegative_integer(value, "numTurns", invalid)?;
            let duration_ms = nullable_nonnegative_integer(value, "durationMs", invalid)?;
            let error_text_present = required_bool(value, "errorTextPresent", invalid)?;
            let error_text_type =
                bounded_label(value, "errorTextType", MAXIMUM_TEXT_BYTES, invalid)?.to_owned();
            let result_field_presence = result_field_presence(value, invalid)?;
            Ok(ClaudeAgentSdkEvent::TurnEnded {
                stop_reason,
                failed,
                subtype,
                num_turns,
                duration_ms,
                error_text_present,
                error_text_type,
                result_field_presence,
            })
        }
        Some(_) => Err(failure(ClaudeAgentSdkProtocolFailureKind::UnknownRecord)),
        None => Err(failure(ClaudeAgentSdkProtocolFailureKind::MissingType)),
    }
}

pub(super) fn decode_callback(
    value: &Value,
) -> Result<ClaudeAgentSdkCallback, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidCallback;
    if value.get("callback").and_then(Value::as_str) != Some("can_use_tool") {
        return Err(failure(invalid));
    }
    let id = bounded_text(value, "id", MAXIMUM_COMMAND_ID_BYTES, invalid)?.to_owned();
    let tool_name = bounded_text(value, "toolName", MAXIMUM_TEXT_BYTES, invalid)?.to_owned();
    let bash_command = if tool_name == "Bash" {
        Some(decode_bash_command_view(value, invalid)?)
    } else {
        if ["command", "commandByteLength", "description", "truncated"]
            .into_iter()
            .any(|field| value.get(field).is_some())
        {
            return Err(failure(invalid));
        }
        None
    };
    Ok(ClaudeAgentSdkCallback {
        id,
        tool_name,
        bash_command,
    })
}

fn decode_bash_command_view(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<ClaudeAgentSdkBashCommandView, ClaudeAgentSdkProtocolFailure> {
    let text_bound = swallowtail_runtime::MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES;
    let command = bounded_text(value, "command", text_bound, kind)?.to_owned();
    let description = bounded_text(value, "description", text_bound, kind)?.to_owned();
    let command_byte_length = value
        .get("commandByteLength")
        .and_then(Value::as_u64)
        .and_then(|length| usize::try_from(length).ok())
        .ok_or_else(|| failure(kind))?;
    let truncated = value
        .get("truncated")
        .and_then(Value::as_bool)
        .ok_or_else(|| failure(kind))?;
    if command_byte_length < command.len()
        || (command_byte_length > text_bound && !truncated)
        || (!truncated && command_byte_length != command.len())
    {
        return Err(failure(kind));
    }
    Ok(ClaudeAgentSdkBashCommandView {
        command,
        command_byte_length,
        description,
        truncated,
    })
}

pub(super) fn decode_terminal(
    value: &Value,
) -> Result<ClaudeAgentSdkFailure, ClaudeAgentSdkProtocolFailure> {
    let record = value
        .get("failure")
        .ok_or_else(|| failure(ClaudeAgentSdkProtocolFailureKind::InvalidTerminal))?;
    let decoded = decode_failure(record, ClaudeAgentSdkProtocolFailureKind::InvalidTerminal)?;
    if decoded.original_code.is_some() {
        return Err(failure(ClaudeAgentSdkProtocolFailureKind::InvalidTerminal));
    }
    Ok(decoded)
}

pub(super) fn decode_diagnostic(
    value: &Value,
) -> Result<ClaudeAgentSdkDiagnostic, ClaudeAgentSdkProtocolFailure> {
    let invalid = ClaudeAgentSdkProtocolFailureKind::InvalidDiagnostic;
    let level = match value.get("level").and_then(Value::as_str) {
        Some("info") => ClaudeAgentSdkDiagnosticLevel::Info,
        Some("warning") => ClaudeAgentSdkDiagnosticLevel::Warning,
        Some("error") => ClaudeAgentSdkDiagnosticLevel::Error,
        _ => return Err(failure(invalid)),
    };
    bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, invalid)?;
    let code = bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, invalid)?.to_owned();
    let evidence = match value.get("evidence") {
        None => None,
        Some(value) if code == "supported_model_rejected" => {
            Some(ClaudeAgentSdkDiagnosticEvidence::ModelQualification(
                decode_model_qualification_evidence(value, invalid)?,
            ))
        }
        Some(value) if code == "sdk_version_mismatch" => {
            Some(ClaudeAgentSdkDiagnosticEvidence::SdkIdentity(
                decode_sdk_identity_evidence(value, invalid)?,
            ))
        }
        Some(_) => return Err(failure(invalid)),
    };
    Ok(ClaudeAgentSdkDiagnostic {
        level,
        code,
        evidence,
    })
}

fn decode_sdk_identity_evidence(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<ClaudeAgentSdkIdentityEvidence, ClaudeAgentSdkProtocolFailure> {
    let object = value
        .as_object()
        .filter(|object| object.len() == 4)
        .ok_or_else(|| failure(kind))?;
    const FIELDS: [&str; 4] = [
        "declaredSdkPackage",
        "declaredSdkVersion",
        "loadedSdkPackage",
        "loadedSdkVersion",
    ];
    if object.keys().any(|field| !FIELDS.contains(&field.as_str())) {
        return Err(failure(kind));
    }
    Ok(ClaudeAgentSdkIdentityEvidence {
        declared_sdk_package: bounded_object_text(object, "declaredSdkPackage", kind)?,
        declared_sdk_version: bounded_object_text(object, "declaredSdkVersion", kind)?,
        loaded_sdk_package: bounded_object_text(object, "loadedSdkPackage", kind)?,
        loaded_sdk_version: bounded_object_text(object, "loadedSdkVersion", kind)?,
    })
}

fn decode_model_qualification_evidence(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<ClaudeAgentSdkModelQualificationEvidence, ClaudeAgentSdkProtocolFailure> {
    let object = value
        .as_object()
        .filter(|object| object.len() == 11)
        .ok_or_else(|| failure(kind))?;
    const FIELDS: [&str; 11] = [
        "requestedModel",
        "effectiveModel",
        "catalogueSize",
        "catalogueDigest",
        "requestedMembership",
        "effectiveMembership",
        "querySource",
        "phase",
        "declaredSdkVersion",
        "loadedSdkVersion",
        "nativeVersion",
    ];
    if object.keys().any(|field| !FIELDS.contains(&field.as_str())) {
        return Err(failure(kind));
    }
    let requested_model = optional_bounded_text(
        object,
        "requestedModel",
        MAXIMUM_MODEL_QUALIFICATION_ID_BYTES,
        kind,
    )?;
    let effective_model = optional_bounded_text(
        object,
        "effectiveModel",
        MAXIMUM_MODEL_QUALIFICATION_ID_BYTES,
        kind,
    )?;
    let catalogue_size = object
        .get("catalogueSize")
        .and_then(Value::as_u64)
        .and_then(|size| usize::try_from(size).ok())
        .filter(|size| *size <= MAXIMUM_MODEL_QUALIFICATION_CATALOGUE_SIZE)
        .ok_or_else(|| failure(kind))?;
    let catalogue_digest = bounded_digest(object, kind)?;
    let requested_membership = object
        .get("requestedMembership")
        .and_then(Value::as_bool)
        .ok_or_else(|| failure(kind))?;
    let effective_membership = object
        .get("effectiveMembership")
        .and_then(Value::as_bool)
        .ok_or_else(|| failure(kind))?;
    let query_source = exact_text(object, "querySource", "sdk.query", kind)?;
    let phase = exact_text(object, "phase", "first-turn-model-qualification", kind)?;
    let declared_sdk_version = bounded_object_text(object, "declaredSdkVersion", kind)?;
    let loaded_sdk_version = bounded_object_text(object, "loadedSdkVersion", kind)?;
    let native_version = bounded_object_text(object, "nativeVersion", kind)?;
    Ok(ClaudeAgentSdkModelQualificationEvidence {
        requested_model,
        effective_model,
        catalogue_size,
        catalogue_digest,
        requested_membership,
        effective_membership,
        query_source,
        phase,
        declared_sdk_version,
        loaded_sdk_version,
        native_version,
    })
}

fn optional_bounded_text(
    object: &serde_json::Map<String, Value>,
    field: &str,
    maximum: usize,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<Option<String>, ClaudeAgentSdkProtocolFailure> {
    let value = object.get(field).ok_or_else(|| failure(kind))?;
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some(
        value
            .as_str()
            .filter(|text| !text.is_empty())
            .filter(|text| text.len() <= maximum && !text.chars().any(char::is_control))
            .ok_or_else(|| failure(kind))?
            .to_owned(),
    ))
}

fn bounded_object_text(
    object: &serde_json::Map<String, Value>,
    field: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<String, ClaudeAgentSdkProtocolFailure> {
    let value = object.get(field).ok_or_else(|| failure(kind))?;
    let text = value
        .as_str()
        .filter(|text| !text.is_empty())
        .filter(|text| {
            text.len() <= MAXIMUM_MODEL_QUALIFICATION_ID_BYTES
                && !text.chars().any(char::is_control)
        })
        .ok_or_else(|| failure(kind))?;
    Ok(text.to_owned())
}

fn exact_text(
    object: &serde_json::Map<String, Value>,
    field: &str,
    expected: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<String, ClaudeAgentSdkProtocolFailure> {
    let text = bounded_object_text(object, field, kind)?;
    if text != expected {
        return Err(failure(kind));
    }
    Ok(text)
}

fn bounded_digest(
    object: &serde_json::Map<String, Value>,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<String, ClaudeAgentSdkProtocolFailure> {
    let digest = bounded_object_text(object, "catalogueDigest", kind)?;
    let expected_length = MODEL_QUALIFICATION_DIGEST.len() + MODEL_QUALIFICATION_DIGEST_HEX_BYTES;
    if digest.len() != expected_length
        || !digest.starts_with(MODEL_QUALIFICATION_DIGEST)
        || !digest[MODEL_QUALIFICATION_DIGEST.len()..]
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(failure(kind));
    }
    Ok(digest)
}

fn decode_failure(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<ClaudeAgentSdkFailure, ClaudeAgentSdkProtocolFailure> {
    bounded_text(value, "message", MAXIMUM_FAILURE_MESSAGE_BYTES, kind)?;
    let code = bounded_text(value, "code", MAXIMUM_FAILURE_CODE_BYTES, kind)?;
    let code = ClaudeAgentSdkFailureCode::parse(code).ok_or_else(|| failure(kind))?;
    let original_code = match value.get("originalCode") {
        None => None,
        Some(original_code) => {
            let original_code = original_code
                .as_str()
                .filter(|value| value.len() <= MAXIMUM_FAILURE_CODE_BYTES)
                .and_then(ClaudeAgentSdkFailureCode::parse)
                .ok_or_else(|| failure(kind))?;
            Some(original_code)
        }
    };
    if (code == ClaudeAgentSdkFailureCode::SessionRejectedTerminal) != original_code.is_some() {
        return Err(failure(kind));
    }
    Ok(ClaudeAgentSdkFailure {
        code,
        original_code,
    })
}

fn nullable_label(
    value: &Value,
    field: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<Option<String>, ClaudeAgentSdkProtocolFailure> {
    let value = value.get(field).ok_or_else(|| failure(kind))?;
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some(
        bounded_label_value(value, MAXIMUM_TEXT_BYTES, kind)?.to_owned(),
    ))
}

fn nullable_nonnegative_integer(
    value: &Value,
    field: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<Option<u64>, ClaudeAgentSdkProtocolFailure> {
    let value = value.get(field).ok_or_else(|| failure(kind))?;
    if value.is_null() {
        return Ok(None);
    }
    value.as_u64().ok_or_else(|| failure(kind)).map(Some)
}

fn result_field_presence(
    value: &Value,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<BTreeMap<String, bool>, ClaudeAgentSdkProtocolFailure> {
    let fields = value
        .get("resultFieldPresence")
        .and_then(Value::as_object)
        .filter(|fields| fields.len() <= MAXIMUM_RESULT_FIELDS)
        .ok_or_else(|| failure(kind))?;
    fields
        .iter()
        .map(|(field, value)| {
            if !is_safe_label(field, MAXIMUM_TEXT_BYTES) {
                return Err(failure(kind));
            }
            value
                .as_bool()
                .map(|present| (field.clone(), present))
                .ok_or_else(|| failure(kind))
        })
        .collect()
}

fn bounded_label<'a>(
    value: &'a Value,
    field: &str,
    maximum: usize,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<&'a str, ClaudeAgentSdkProtocolFailure> {
    let value = value.get(field).ok_or_else(|| failure(kind))?;
    bounded_label_value(value, maximum, kind)
}

fn bounded_label_allow_empty<'a>(
    value: &'a Value,
    field: &str,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<&'a str, ClaudeAgentSdkProtocolFailure> {
    let text = value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| failure(kind))?;
    if is_safe_label(text, MAXIMUM_TEXT_BYTES) {
        Ok(text)
    } else {
        Err(failure(kind))
    }
}

fn bounded_label_value(
    value: &Value,
    maximum: usize,
    kind: ClaudeAgentSdkProtocolFailureKind,
) -> Result<&str, ClaudeAgentSdkProtocolFailure> {
    let text = value.as_str().ok_or_else(|| failure(kind))?;
    if is_safe_label(text, maximum) && !text.is_empty() {
        Ok(text)
    } else {
        Err(failure(kind))
    }
}

fn is_safe_label(value: &str, maximum: usize) -> bool {
    value.len() <= maximum
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
}
