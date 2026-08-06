use std::fmt;

use super::{
    AcpCommand, AcpConfigOption, AcpContentBlock, AcpOptionalUpdate, AcpPlanEntry, AcpToolCall,
    AcpToolCallUpdate, AcpUsage,
};

/// Bounded ACP text whose debug and display forms do not expose content.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AcpBoundedText(pub(crate) String);

impl AcpBoundedText {
    /// Returns the decoded text for semantic adapter processing.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the UTF-8 byte length of the decoded text.
    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Debug for AcpBoundedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("AcpBoundedText")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

impl fmt::Display for AcpBoundedText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted ACP content>")
    }
}

/// One decoded session update paired with its exact provider session identity.
#[derive(Clone, Debug, PartialEq)]
pub struct DecodedSessionUpdate {
    /// Provider session identity carried by the update envelope.
    pub session_id: AcpBoundedText,
    /// Typed update body.
    pub update: AcpSessionUpdate,
}

/// Replacement or delta semantics of one typed ACP session update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpSessionUpdateSemantics {
    /// Append content to an existing stream.
    ContentDelta,
    /// Create a new activity identity.
    Creation,
    /// Refine only fields supplied by the update.
    PartialUpdate,
    /// Replace the complete prior collection.
    ReplacementSnapshot,
    /// Replace one current scalar value.
    CurrentValue,
    /// Replace an evidence snapshot rather than accumulating it.
    EvidenceSnapshot,
    /// No portable update semantics are known.
    Unknown,
}

/// Typed semantic subset of ACP session updates.
#[derive(Clone, Debug, PartialEq)]
pub enum AcpSessionUpdate {
    /// User, agent, or thought content chunk.
    Message(AcpMessageChunk),
    /// Tool-call creation or already-terminal snapshot.
    ToolCall(AcpToolCall),
    /// Partial refinement of a tool call.
    ToolCallUpdate(AcpToolCallUpdate),
    /// Replacement plan snapshot.
    Plan(Vec<AcpPlanEntry>),
    /// Replacement advertised-command snapshot.
    AvailableCommands(Vec<AcpCommand>),
    /// Current harness mode value.
    CurrentMode(AcpBoundedText),
    /// Replacement configuration-option snapshot.
    ConfigOptions(Vec<AcpConfigOption>),
    /// Partial session-title or update-time metadata.
    SessionInfo {
        /// Optional title refinement.
        title: AcpOptionalUpdate<AcpBoundedText>,
        /// Optional provider timestamp refinement.
        updated_at: AcpOptionalUpdate<AcpBoundedText>,
    },
    /// Context usage and optional cost snapshot.
    Usage(AcpUsage),
    /// Recognized envelope with an unmodelled namespaced update kind.
    Unknown {
        /// Provider update namespace retained for safe observation.
        namespace: AcpBoundedText,
    },
}

impl AcpSessionUpdate {
    /// Returns how consumers should reconcile this update with prior state.
    #[must_use]
    pub const fn semantics(&self) -> AcpSessionUpdateSemantics {
        match self {
            Self::Message(_) => AcpSessionUpdateSemantics::ContentDelta,
            Self::ToolCall(_) => AcpSessionUpdateSemantics::Creation,
            Self::ToolCallUpdate(_) | Self::SessionInfo { .. } => {
                AcpSessionUpdateSemantics::PartialUpdate
            }
            Self::Plan(_) | Self::AvailableCommands(_) | Self::ConfigOptions(_) => {
                AcpSessionUpdateSemantics::ReplacementSnapshot
            }
            Self::CurrentMode(_) => AcpSessionUpdateSemantics::CurrentValue,
            Self::Usage(_) => AcpSessionUpdateSemantics::EvidenceSnapshot,
            Self::Unknown { .. } => AcpSessionUpdateSemantics::Unknown,
        }
    }
}

/// Author role carried by one ACP message chunk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpMessageRole {
    /// User-authored content.
    User,
    /// Agent-authored visible content.
    Agent,
    /// Agent thought or reasoning content.
    Thought,
}

/// One bounded ACP message-content chunk.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpMessageChunk {
    /// Provider-declared message role.
    pub role: AcpMessageRole,
    /// Optional provider message identity for correlation.
    pub message_id: Option<AcpBoundedText>,
    /// Typed content block.
    pub content: AcpContentBlock,
}
