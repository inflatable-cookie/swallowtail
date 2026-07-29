use std::fmt;

use super::{
    AcpCommand, AcpConfigOption, AcpContentBlock, AcpOptionalUpdate, AcpPlanEntry, AcpToolCall,
    AcpToolCallUpdate, AcpUsage,
};

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AcpBoundedText(pub(crate) String);

impl AcpBoundedText {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

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

#[derive(Clone, Debug, PartialEq)]
pub struct DecodedSessionUpdate {
    pub session_id: AcpBoundedText,
    pub update: AcpSessionUpdate,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpSessionUpdateSemantics {
    ContentDelta,
    Creation,
    PartialUpdate,
    ReplacementSnapshot,
    CurrentValue,
    EvidenceSnapshot,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AcpSessionUpdate {
    Message(AcpMessageChunk),
    ToolCall(AcpToolCall),
    ToolCallUpdate(AcpToolCallUpdate),
    Plan(Vec<AcpPlanEntry>),
    AvailableCommands(Vec<AcpCommand>),
    CurrentMode(AcpBoundedText),
    ConfigOptions(Vec<AcpConfigOption>),
    SessionInfo {
        title: AcpOptionalUpdate<AcpBoundedText>,
        updated_at: AcpOptionalUpdate<AcpBoundedText>,
    },
    Usage(AcpUsage),
    Unknown {
        namespace: AcpBoundedText,
    },
}

impl AcpSessionUpdate {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpMessageRole {
    User,
    Agent,
    Thought,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpMessageChunk {
    pub role: AcpMessageRole,
    pub message_id: Option<AcpBoundedText>,
    pub content: AcpContentBlock,
}
