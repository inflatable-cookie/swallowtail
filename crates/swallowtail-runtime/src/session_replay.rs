use crate::OperationContent;
use swallowtail_core::SessionRef;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionReplayKind {
    UserMessage,
    AgentMessage,
    AgentReasoning,
    ToolCall,
    ToolCallUpdate,
    Plan,
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
    pub const fn provider_session_ref(&self) -> &SessionRef {
        &self.provider_session_ref
    }

    #[must_use]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    #[must_use]
    pub const fn kind(&self) -> SessionReplayKind {
        self.kind
    }

    #[must_use]
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
