use super::record::{AcpOpaqueExtensions, AcpSessionInfo, AcpSessionListPage};
use super::{
    AcpSessionListDecodeError, AcpSessionListDecodeErrorKind, AcpSessionListRequest,
    bounded_nonempty, error,
};
use crate::AcpBoundedText;
use serde_json::{Map, Value};
use std::collections::BTreeSet;
use std::path::Path;
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub(super) fn decode_result(
    request: &AcpSessionListRequest,
    result: &Value,
) -> Result<AcpSessionListPage, AcpSessionListDecodeError> {
    let encoded = serde_json::to_vec(result)
        .map_err(|_| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
    if encoded.len() > request.limits.maximum_response_bytes {
        return Err(error(AcpSessionListDecodeErrorKind::LimitExceeded));
    }
    let result = result
        .as_object()
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
    validate_meta(result.get("_meta"))?;
    let sessions = result
        .get("sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
    if sessions.len() > request.limits.maximum_sessions {
        return Err(error(AcpSessionListDecodeErrorKind::LimitExceeded));
    }
    let mut seen = BTreeSet::new();
    let sessions = sessions
        .iter()
        .map(|session| decode_session(request, session, &mut seen))
        .collect::<Result<Vec<_>, _>>()?;
    let next_cursor = optional_bounded_nonempty(
        result.get("nextCursor"),
        request.limits.maximum_cursor_bytes,
    )?;
    let extensions = AcpOpaqueExtensions::from_fields(
        result,
        &["sessions", "nextCursor"],
        request.limits.maximum_extension_bytes,
    )?;
    Ok(AcpSessionListPage {
        sessions,
        next_cursor,
        extensions,
    })
}

fn decode_session(
    request: &AcpSessionListRequest,
    value: &Value,
    seen: &mut BTreeSet<String>,
) -> Result<AcpSessionInfo, AcpSessionListDecodeError> {
    let session = value
        .as_object()
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
    validate_meta(session.get("_meta"))?;
    let session_id = required_bounded_nonempty(
        session,
        "sessionId",
        request.limits.maximum_identifier_bytes,
    )?;
    if !seen.insert(session_id.as_str().to_owned()) {
        return Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid));
    }
    let cwd = required_path(session, "cwd", request.limits.maximum_path_bytes)?;
    if request
        .cwd
        .as_ref()
        .is_some_and(|expected| expected != &cwd)
    {
        return Err(error(AcpSessionListDecodeErrorKind::ResourceMismatch));
    }
    let additional_directories = decode_additional_directories(request, session)?;
    let title =
        optional_bounded_nonempty(session.get("title"), request.limits.maximum_content_bytes)?;
    let updated_at = optional_bounded_nonempty(
        session.get("updatedAt"),
        request.limits.maximum_content_bytes,
    )?;
    let updated_at_unix_milliseconds = updated_at
        .as_ref()
        .map(|value| parse_timestamp(value.as_str()))
        .transpose()?;
    let extensions = AcpOpaqueExtensions::from_fields(
        session,
        &[
            "sessionId",
            "cwd",
            "additionalDirectories",
            "title",
            "updatedAt",
        ],
        request.limits.maximum_extension_bytes,
    )?;
    Ok(AcpSessionInfo {
        session_id,
        cwd,
        additional_directories,
        title,
        updated_at,
        updated_at_unix_milliseconds,
        extensions,
    })
}

fn decode_additional_directories(
    request: &AcpSessionListRequest,
    session: &Map<String, Value>,
) -> Result<Vec<AcpBoundedText>, AcpSessionListDecodeError> {
    let Some(value) = session.get("additionalDirectories") else {
        return Ok(Vec::new());
    };
    if !request.capabilities.additional_directories {
        return Err(error(AcpSessionListDecodeErrorKind::CapabilityInvalid));
    }
    let values = value
        .as_array()
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
    if values.len() > request.limits.maximum_additional_directories {
        return Err(error(AcpSessionListDecodeErrorKind::LimitExceeded));
    }
    let mut seen = BTreeSet::new();
    values
        .iter()
        .map(|value| {
            let path = value
                .as_str()
                .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))?;
            let path = bounded_response_path(path.to_owned(), request.limits.maximum_path_bytes)?;
            if !seen.insert(path.as_str().to_owned()) {
                return Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid));
            }
            Ok(path)
        })
        .collect()
}

fn parse_timestamp(value: &str) -> Result<u64, AcpSessionListDecodeError> {
    let timestamp = OffsetDateTime::parse(value, &Rfc3339)
        .map_err(|_| error(AcpSessionListDecodeErrorKind::TimestampInvalid))?;
    let milliseconds = timestamp.unix_timestamp_nanos() / 1_000_000;
    u64::try_from(milliseconds).map_err(|_| error(AcpSessionListDecodeErrorKind::TimestampInvalid))
}

fn validate_meta(value: Option<&Value>) -> Result<(), AcpSessionListDecodeError> {
    match value {
        None | Some(Value::Null | Value::Object(_)) => Ok(()),
        Some(_) => Err(error(AcpSessionListDecodeErrorKind::ExtensionInvalid)),
    }
}

fn required_path(
    object: &Map<String, Value>,
    key: &str,
    limit: usize,
) -> Result<AcpBoundedText, AcpSessionListDecodeError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))
        .and_then(|value| bounded_response_path(value.to_owned(), limit))
}

fn required_bounded_nonempty(
    object: &Map<String, Value>,
    key: &str,
    limit: usize,
) -> Result<AcpBoundedText, AcpSessionListDecodeError> {
    object
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::ResponseInvalid))
        .and_then(|value| bounded_nonempty(value.to_owned(), limit))
}

fn optional_bounded_nonempty(
    value: Option<&Value>,
    limit: usize,
) -> Result<Option<AcpBoundedText>, AcpSessionListDecodeError> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => bounded_nonempty(value.clone(), limit).map(Some),
        Some(_) => Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid)),
    }
}

fn bounded_response_path(
    value: String,
    limit: usize,
) -> Result<AcpBoundedText, AcpSessionListDecodeError> {
    if !Path::new(&value).is_absolute() {
        return Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid));
    }
    bounded_nonempty(value, limit)
}
