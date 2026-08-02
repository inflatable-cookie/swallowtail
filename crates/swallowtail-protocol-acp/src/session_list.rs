use crate::{ACP_PROTOCOL_VERSION, Message, encode_message};
use serde_json::{Map, Value};
use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::AcpBoundedText;

mod decode;
mod record;

pub use record::{AcpOpaqueExtensions, AcpSessionInfo, AcpSessionListPage};

pub const ACP_SESSION_LIST_METHOD: &str = "session/list";

const DEFAULT_MAX_RESPONSE_BYTES: usize = 64 * 1024;
const DEFAULT_MAX_SESSIONS: usize = 1_000;
const DEFAULT_MAX_IDENTIFIER_BYTES: usize = 4_096;
const DEFAULT_MAX_PATH_BYTES: usize = 16_384;
const DEFAULT_MAX_CONTENT_BYTES: usize = 16_384;
const DEFAULT_MAX_CURSOR_BYTES: usize = 4_096;
const DEFAULT_MAX_EXTENSION_BYTES: usize = 16_384;
const DEFAULT_MAX_ADDITIONAL_DIRECTORIES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpSessionListLimits {
    maximum_response_bytes: usize,
    maximum_sessions: usize,
    maximum_identifier_bytes: usize,
    maximum_path_bytes: usize,
    maximum_content_bytes: usize,
    maximum_cursor_bytes: usize,
    maximum_extension_bytes: usize,
    maximum_additional_directories: usize,
}

impl AcpSessionListLimits {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        maximum_response_bytes: usize,
        maximum_sessions: usize,
        maximum_identifier_bytes: usize,
        maximum_path_bytes: usize,
        maximum_content_bytes: usize,
        maximum_cursor_bytes: usize,
        maximum_extension_bytes: usize,
        maximum_additional_directories: usize,
    ) -> Self {
        Self {
            maximum_response_bytes,
            maximum_sessions,
            maximum_identifier_bytes,
            maximum_path_bytes,
            maximum_content_bytes,
            maximum_cursor_bytes,
            maximum_extension_bytes,
            maximum_additional_directories,
        }
    }
}

impl Default for AcpSessionListLimits {
    fn default() -> Self {
        Self::new(
            DEFAULT_MAX_RESPONSE_BYTES,
            DEFAULT_MAX_SESSIONS,
            DEFAULT_MAX_IDENTIFIER_BYTES,
            DEFAULT_MAX_PATH_BYTES,
            DEFAULT_MAX_CONTENT_BYTES,
            DEFAULT_MAX_CURSOR_BYTES,
            DEFAULT_MAX_EXTENSION_BYTES,
            DEFAULT_MAX_ADDITIONAL_DIRECTORIES,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpSessionListCapabilities {
    list: bool,
    additional_directories: bool,
}

impl AcpSessionListCapabilities {
    #[must_use]
    pub const fn list(self) -> bool {
        self.list
    }

    #[must_use]
    pub const fn additional_directories(self) -> bool {
        self.additional_directories
    }
}

pub fn decode_session_list_capabilities(
    initialize_result: &Value,
) -> Result<AcpSessionListCapabilities, AcpSessionListDecodeError> {
    let result = initialize_result
        .as_object()
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::CapabilityInvalid))?;
    if result.get("protocolVersion").and_then(Value::as_u64) != Some(ACP_PROTOCOL_VERSION) {
        return Err(error(AcpSessionListDecodeErrorKind::CapabilityInvalid));
    }
    let agent = result
        .get("agentCapabilities")
        .and_then(Value::as_object)
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::CapabilityInvalid))?;
    let Some(session) = agent.get("sessionCapabilities") else {
        return Ok(AcpSessionListCapabilities {
            list: false,
            additional_directories: false,
        });
    };
    if session.is_null() {
        return Ok(AcpSessionListCapabilities {
            list: false,
            additional_directories: false,
        });
    }
    let session = session
        .as_object()
        .ok_or_else(|| error(AcpSessionListDecodeErrorKind::CapabilityInvalid))?;
    Ok(AcpSessionListCapabilities {
        list: capability(session.get("list"))?,
        additional_directories: capability(session.get("additionalDirectories"))?,
    })
}

fn capability(value: Option<&Value>) -> Result<bool, AcpSessionListDecodeError> {
    match value {
        None | Some(Value::Null) => Ok(false),
        Some(Value::Object(_)) => Ok(true),
        Some(_) => Err(error(AcpSessionListDecodeErrorKind::CapabilityInvalid)),
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct AcpSessionListRequest {
    id: Value,
    pub(super) cwd: Option<AcpBoundedText>,
    cursor: Option<AcpBoundedText>,
    pub(super) capabilities: AcpSessionListCapabilities,
    pub(super) limits: AcpSessionListLimits,
}

impl AcpSessionListRequest {
    pub fn new(
        id: Value,
        capabilities: AcpSessionListCapabilities,
        cwd: Option<String>,
        cursor: Option<String>,
        limits: AcpSessionListLimits,
    ) -> Result<Self, AcpSessionListDecodeError> {
        if !capabilities.list {
            return Err(error(AcpSessionListDecodeErrorKind::Unsupported));
        }
        if !valid_id(&id) {
            return Err(error(AcpSessionListDecodeErrorKind::RequestInvalid));
        }
        let cwd = cwd
            .map(|value| bounded_path(value, limits.maximum_path_bytes))
            .transpose()?;
        let cursor = cursor
            .map(|value| bounded_nonempty(value, limits.maximum_cursor_bytes))
            .transpose()?;
        Ok(Self {
            id,
            cwd,
            cursor,
            capabilities,
            limits,
        })
    }

    pub fn encode(&self) -> Result<Vec<u8>, crate::ProtocolError> {
        let mut params = Map::new();
        if let Some(cwd) = &self.cwd {
            params.insert("cwd".to_owned(), Value::String(cwd.as_str().to_owned()));
        }
        if let Some(cursor) = &self.cursor {
            params.insert(
                "cursor".to_owned(),
                Value::String(cursor.as_str().to_owned()),
            );
        }
        encode_message(&Message::Request {
            id: self.id.clone(),
            method: ACP_SESSION_LIST_METHOD.to_owned(),
            params: Value::Object(params),
        })
    }

    pub fn decode_response(
        &self,
        message: &Message,
    ) -> Result<AcpSessionListPage, AcpSessionListDecodeError> {
        let Message::Response { id, result } = message else {
            return Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid));
        };
        if id != &self.id {
            return Err(error(AcpSessionListDecodeErrorKind::CorrelationMismatch));
        }
        match result {
            Ok(result) => self.decode_result(result),
            Err(_) => Err(error(AcpSessionListDecodeErrorKind::ProviderRejected)),
        }
    }

    pub fn decode_result(
        &self,
        result: &Value,
    ) -> Result<AcpSessionListPage, AcpSessionListDecodeError> {
        decode::decode_result(self, result)
    }

    #[must_use]
    pub fn id(&self) -> &Value {
        &self.id
    }

    #[must_use]
    pub fn cwd(&self) -> Option<&str> {
        self.cwd.as_ref().map(AcpBoundedText::as_str)
    }

    #[must_use]
    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_ref().map(AcpBoundedText::as_str)
    }
}

impl fmt::Debug for AcpSessionListRequest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpSessionListRequest")
            .field("id", &self.id)
            .field(
                "cwd_bytes",
                &self.cwd.as_ref().map(AcpBoundedText::byte_len),
            )
            .field(
                "cursor_bytes",
                &self.cursor.as_ref().map(AcpBoundedText::byte_len),
            )
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpSessionListDecodeErrorKind {
    CapabilityInvalid,
    CorrelationMismatch,
    ExtensionInvalid,
    LimitExceeded,
    ProviderRejected,
    RequestInvalid,
    ResourceMismatch,
    ResponseInvalid,
    TimestampInvalid,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpSessionListDecodeError {
    kind: AcpSessionListDecodeErrorKind,
}

impl AcpSessionListDecodeError {
    #[must_use]
    pub const fn kind(self) -> AcpSessionListDecodeErrorKind {
        self.kind
    }
}

impl fmt::Display for AcpSessionListDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            AcpSessionListDecodeErrorKind::CapabilityInvalid => {
                "ACP session-list capability is invalid"
            }
            AcpSessionListDecodeErrorKind::CorrelationMismatch => {
                "ACP session-list response correlation failed"
            }
            AcpSessionListDecodeErrorKind::ExtensionInvalid => {
                "ACP session-list extensions are invalid"
            }
            AcpSessionListDecodeErrorKind::LimitExceeded => {
                "ACP session-list decode limit exceeded"
            }
            AcpSessionListDecodeErrorKind::ProviderRejected => {
                "ACP session-list request was rejected"
            }
            AcpSessionListDecodeErrorKind::RequestInvalid => "ACP session-list request is invalid",
            AcpSessionListDecodeErrorKind::ResourceMismatch => {
                "ACP session-list result does not match its requested resource"
            }
            AcpSessionListDecodeErrorKind::ResponseInvalid => {
                "ACP session-list response is invalid"
            }
            AcpSessionListDecodeErrorKind::TimestampInvalid => {
                "ACP session-list timestamp is invalid"
            }
            AcpSessionListDecodeErrorKind::Unsupported => {
                "ACP agent did not advertise session listing"
            }
        })
    }
}

impl Error for AcpSessionListDecodeError {}

fn bounded_path(value: String, limit: usize) -> Result<AcpBoundedText, AcpSessionListDecodeError> {
    if !Path::new(&value).is_absolute() {
        return Err(error(AcpSessionListDecodeErrorKind::RequestInvalid));
    }
    bounded_nonempty(value, limit)
}

fn bounded_nonempty(
    value: String,
    limit: usize,
) -> Result<AcpBoundedText, AcpSessionListDecodeError> {
    if value.trim().is_empty() {
        return Err(error(AcpSessionListDecodeErrorKind::ResponseInvalid));
    }
    if value.len() > limit {
        return Err(error(AcpSessionListDecodeErrorKind::LimitExceeded));
    }
    Ok(AcpBoundedText(value))
}

fn valid_id(id: &Value) -> bool {
    id.is_string() || id.is_u64() || id.is_i64()
}

pub(super) const fn error(kind: AcpSessionListDecodeErrorKind) -> AcpSessionListDecodeError {
    AcpSessionListDecodeError { kind }
}
