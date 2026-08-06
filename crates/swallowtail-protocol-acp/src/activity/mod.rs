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

/// Independent size and collection limits for semantic session-update decoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityDecodeLimits {
    maximum_update_bytes: usize,
    maximum_collection_items: usize,
    maximum_identifier_bytes: usize,
}

impl ActivityDecodeLimits {
    /// Creates explicit update, collection, and identifier limits.
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

    /// Returns the largest encoded session update accepted for decoding.
    #[must_use]
    pub const fn maximum_update_bytes(self) -> usize {
        self.maximum_update_bytes
    }

    /// Returns the maximum entries accepted in one activity collection.
    #[must_use]
    pub const fn maximum_collection_items(self) -> usize {
        self.maximum_collection_items
    }

    /// Returns the maximum bytes accepted in one activity identifier.
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

/// Stable classification of a semantic ACP session-update decode failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityDecodeErrorKind {
    /// A content block used an invalid shape or encoding.
    ContentInvalid,
    /// An activity identity was empty, oversized, or malformed.
    IdentifierInvalid,
    /// A configured semantic decode bound was exceeded.
    LimitExceeded,
    /// Session-scoped metadata had an invalid shape.
    MetadataInvalid,
    /// A plan snapshot contained invalid entries.
    PlanEntriesInvalid,
    /// Session-update parameters omitted their session identity.
    SessionIdMissing,
    /// A tool call or refinement omitted its correlation identity.
    ToolIdentityMissing,
    /// A tool-call status was unsupported or malformed.
    ToolStatusInvalid,
    /// A named session-update kind had an invalid payload.
    UpdateKindInvalid,
    /// Session-update parameters omitted the update discriminator.
    UpdateKindMissing,
    /// Usage or cost evidence had an invalid shape.
    UsageInvalid,
}

/// Bounded activity decoding failure without provider payload content.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActivityDecodeError {
    kind: ActivityDecodeErrorKind,
}

impl ActivityDecodeError {
    pub(crate) const fn new(kind: ActivityDecodeErrorKind) -> Self {
        Self { kind }
    }

    /// Returns the stable activity failure classification.
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

/// Decodes one ACP session update using the default semantic limits.
pub fn decode_session_update(params: &Value) -> Result<DecodedSessionUpdate, ActivityDecodeError> {
    decode_session_update_with_limits(params, ActivityDecodeLimits::default())
}

/// Decodes one ACP session update using caller-selected semantic limits.
pub fn decode_session_update_with_limits(
    params: &Value,
    limits: ActivityDecodeLimits,
) -> Result<DecodedSessionUpdate, ActivityDecodeError> {
    decode::decode(params, limits)
}
