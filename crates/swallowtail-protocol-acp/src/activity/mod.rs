use serde_json::Value;
use std::error::Error;
use std::fmt;

mod content_record;
mod decode;
mod metadata_record;
mod record;
mod tool_record;

pub use content_record::AcpContentBlock;
pub use metadata_record::{
    AcpCommand, AcpConfigCategory, AcpConfigChoice, AcpConfigChoices, AcpConfigGroup,
    AcpConfigKind, AcpConfigOption, AcpCost, AcpOptionalUpdate, AcpPlanEntry, AcpPlanEntryPriority,
    AcpPlanEntryStatus, AcpUsage,
};
pub use record::{
    AcpBoundedText, AcpMessageChunk, AcpMessageRole, AcpSessionUpdate, AcpSessionUpdateSemantics,
    DecodedSessionUpdate,
};
pub use tool_record::{
    AcpToolCall, AcpToolCallContent, AcpToolCallLocation, AcpToolCallStatus, AcpToolCallUpdate,
    AcpToolKind,
};

// Semantic decoding must accept every update the widest qualified ACP receive
// envelope can deliver. Transport framing remains independently configurable.
const DEFAULT_MAX_ACTIVITY_UPDATE_BYTES: usize = 4 * 1024 * 1024;
const DEFAULT_MAX_ACTIVITY_COLLECTION_ITEMS: usize = 256;
const DEFAULT_MAX_ACTIVITY_IDENTIFIER_BYTES: usize = 512;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityDecodeLimits {
    maximum_update_bytes: usize,
    maximum_collection_items: usize,
    maximum_identifier_bytes: usize,
}

impl ActivityDecodeLimits {
    #[must_use]
    pub const fn new(
        maximum_update_bytes: usize,
        maximum_collection_items: usize,
        maximum_identifier_bytes: usize,
    ) -> Self {
        Self {
            maximum_update_bytes,
            maximum_collection_items,
            maximum_identifier_bytes,
        }
    }

    #[must_use]
    pub const fn maximum_update_bytes(self) -> usize {
        self.maximum_update_bytes
    }

    #[must_use]
    pub const fn maximum_collection_items(self) -> usize {
        self.maximum_collection_items
    }

    #[must_use]
    pub const fn maximum_identifier_bytes(self) -> usize {
        self.maximum_identifier_bytes
    }
}

impl Default for ActivityDecodeLimits {
    fn default() -> Self {
        Self::new(
            DEFAULT_MAX_ACTIVITY_UPDATE_BYTES,
            DEFAULT_MAX_ACTIVITY_COLLECTION_ITEMS,
            DEFAULT_MAX_ACTIVITY_IDENTIFIER_BYTES,
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityDecodeErrorKind {
    ContentInvalid,
    IdentifierInvalid,
    LimitExceeded,
    MetadataInvalid,
    PlanEntriesInvalid,
    SessionIdMissing,
    ToolIdentityMissing,
    ToolStatusInvalid,
    UpdateKindInvalid,
    UpdateKindMissing,
    UsageInvalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityDecodeError {
    kind: ActivityDecodeErrorKind,
}

impl ActivityDecodeError {
    pub(crate) const fn new(kind: ActivityDecodeErrorKind) -> Self {
        Self { kind }
    }

    #[must_use]
    pub const fn kind(self) -> ActivityDecodeErrorKind {
        self.kind
    }
}

impl fmt::Display for ActivityDecodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self.kind {
            ActivityDecodeErrorKind::ContentInvalid => "ACP activity content is invalid",
            ActivityDecodeErrorKind::IdentifierInvalid => "ACP activity identifier is invalid",
            ActivityDecodeErrorKind::LimitExceeded => "ACP activity decode limit exceeded",
            ActivityDecodeErrorKind::MetadataInvalid => "ACP session metadata is invalid",
            ActivityDecodeErrorKind::PlanEntriesInvalid => "ACP plan entries are invalid",
            ActivityDecodeErrorKind::SessionIdMissing => {
                "ACP session update omitted its session id"
            }
            ActivityDecodeErrorKind::ToolIdentityMissing => {
                "ACP tool update omitted its tool-call id"
            }
            ActivityDecodeErrorKind::ToolStatusInvalid => "ACP tool status is invalid",
            ActivityDecodeErrorKind::UpdateKindInvalid => "ACP session-update kind is invalid",
            ActivityDecodeErrorKind::UpdateKindMissing => "ACP session-update kind is missing",
            ActivityDecodeErrorKind::UsageInvalid => "ACP usage update is invalid",
        })
    }
}

impl Error for ActivityDecodeError {}

pub fn decode_session_update(params: &Value) -> Result<DecodedSessionUpdate, ActivityDecodeError> {
    decode_session_update_with_limits(params, ActivityDecodeLimits::default())
}

pub fn decode_session_update_with_limits(
    params: &Value,
    limits: ActivityDecodeLimits,
) -> Result<DecodedSessionUpdate, ActivityDecodeError> {
    decode::decode(params, limits)
}
