use super::fields::{error, object, optional_text, required_str, required_text};
use crate::activity::{
    AcpBoundedText, AcpContentBlock, ActivityDecodeError, ActivityDecodeErrorKind,
    ActivityDecodeLimits,
};
use serde_json::Value;

pub(super) fn block(
    value: &Value,
    _limits: ActivityDecodeLimits,
) -> Result<AcpContentBlock, ActivityDecodeError> {
    let value = object(value, ActivityDecodeErrorKind::ContentInvalid)?;
    match required_str(value, "type", ActivityDecodeErrorKind::ContentInvalid)? {
        "text" => Ok(AcpContentBlock::Text(required_text(
            value,
            "text",
            ActivityDecodeErrorKind::ContentInvalid,
        )?)),
        "image" => Ok(AcpContentBlock::Image {
            data: required_text(value, "data", ActivityDecodeErrorKind::ContentInvalid)?,
            mime_type: required_text(value, "mimeType", ActivityDecodeErrorKind::ContentInvalid)?,
            uri: optional_text(value, "uri", ActivityDecodeErrorKind::ContentInvalid)?,
        }),
        "audio" => Ok(AcpContentBlock::Audio {
            data: required_text(value, "data", ActivityDecodeErrorKind::ContentInvalid)?,
            mime_type: required_text(value, "mimeType", ActivityDecodeErrorKind::ContentInvalid)?,
        }),
        "resource_link" => resource_link(value),
        "resource" => embedded_resource(value),
        _ => Err(error(ActivityDecodeErrorKind::ContentInvalid)),
    }
}

fn resource_link(
    value: &serde_json::Map<String, Value>,
) -> Result<AcpContentBlock, ActivityDecodeError> {
    let size = match value.get("size") {
        None | Some(Value::Null) => None,
        Some(value) => Some(
            value
                .as_i64()
                .filter(|size| *size >= 0)
                .ok_or_else(|| error(ActivityDecodeErrorKind::ContentInvalid))?,
        ),
    };
    Ok(AcpContentBlock::ResourceLink {
        name: required_text(value, "name", ActivityDecodeErrorKind::ContentInvalid)?,
        uri: required_text(value, "uri", ActivityDecodeErrorKind::ContentInvalid)?,
        description: optional_text(
            value,
            "description",
            ActivityDecodeErrorKind::ContentInvalid,
        )?,
        mime_type: optional_text(value, "mimeType", ActivityDecodeErrorKind::ContentInvalid)?,
        size,
        title: optional_text(value, "title", ActivityDecodeErrorKind::ContentInvalid)?,
    })
}

fn embedded_resource(
    value: &serde_json::Map<String, Value>,
) -> Result<AcpContentBlock, ActivityDecodeError> {
    let resource = value
        .get("resource")
        .ok_or_else(|| error(ActivityDecodeErrorKind::ContentInvalid))
        .and_then(|resource| object(resource, ActivityDecodeErrorKind::ContentInvalid))?;
    let uri = required_text(resource, "uri", ActivityDecodeErrorKind::ContentInvalid)?;
    let mime_type = optional_text(
        resource,
        "mimeType",
        ActivityDecodeErrorKind::ContentInvalid,
    )?;
    match (resource.get("text"), resource.get("blob")) {
        (Some(Value::String(text)), None) => Ok(AcpContentBlock::EmbeddedTextResource {
            text: AcpBoundedText(text.clone()),
            uri,
            mime_type,
        }),
        (None, Some(Value::String(blob))) => Ok(AcpContentBlock::EmbeddedBlobResource {
            blob: AcpBoundedText(blob.clone()),
            uri,
            mime_type,
        }),
        _ => Err(error(ActivityDecodeErrorKind::ContentInvalid)),
    }
}
