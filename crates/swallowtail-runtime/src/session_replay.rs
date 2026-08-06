use crate::OperationContent;
use swallowtail_core::SessionRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Provider-defined historical update category during session load or replay.
pub enum SessionReplayKind {
    /// Historical user-authored message.
    UserMessage,
    /// Historical agent-authored message.
    AgentMessage,
    /// Historical agent reasoning disclosed by the provider.
    AgentReasoning,
    /// Historical tool-call start.
    ToolCall,
    /// Historical tool-call refinement or result.
    ToolCallUpdate,
    /// Historical plan observation.
    Plan,
    /// Historical session configuration observation.
    Configuration,
}

/// One ordered historical update transported while loading provider state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionReplayItem {
    provider_session_ref: SessionRef,
    sequence: u64,
    kind: SessionReplayKind,
    content: Option<OperationContent>,
}

impl SessionReplayItem {
    #[must_use]
    /// Creates an identity-only historical update without content.
    pub const fn new(
        provider_session_ref: SessionRef,
        sequence: u64,
        kind: SessionReplayKind,
    ) -> Self {
        Self {
            provider_session_ref,
            sequence,
            kind,
            content: None,
        }
    }

    #[must_use]
    /// Creates a historical update carrying redacted operation content.
    pub fn with_content(
        provider_session_ref: SessionRef,
        sequence: u64,
        kind: SessionReplayKind,
        content: OperationContent,
    ) -> Self {
        Self {
            provider_session_ref,
            sequence,
            kind,
            content: Some(content),
        }
    }

    #[must_use]
    /// Returns the provider session whose history contains this item.
    pub const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    #[must_use]
    /// Returns the provider-defined monotonic replay sequence.
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    /// Returns the historical update category.
    pub const fn kind(&self) -> SessionReplayKind {
        self.kind
    }

    #[must_use]
    /// Returns the optional normalized historical content.
    pub const fn content(&self) -> Option<&OperationContent> {
        self.content.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionReplayItem, SessionReplayKind};
    use crate::OperationContent;
    use swallowtail_core::SessionRef;

    #[test]
    fn replay_content_and_provider_identity_are_redacted() {
        let item = SessionReplayItem::with_content(
            SessionRef::new("private/provider/session").expect("session ref is valid"),
            1,
            SessionReplayKind::AgentMessage,
            OperationContent::new("private historical output").expect("content is valid"),
        );

        let debug = format!("{item:?}");
        assert!(!debug.contains("private/provider/session"));
        assert!(!debug.contains("private historical output"));
    }
}
