use super::{AcpBoundedText, AcpContentBlock};

#[derive(Clone, Debug, PartialEq)]
pub struct AcpToolCall {
    pub tool_call_id: AcpBoundedText,
    pub title: AcpBoundedText,
    pub kind: AcpToolKind,
    pub status: AcpToolCallStatus,
    pub content: Vec<AcpToolCallContent>,
    pub locations: Vec<AcpToolCallLocation>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AcpToolCallUpdate {
    pub tool_call_id: AcpBoundedText,
    pub title: Option<AcpBoundedText>,
    pub kind: Option<AcpToolKind>,
    pub status: Option<AcpToolCallStatus>,
    pub content_replacement: Option<Vec<AcpToolCallContent>>,
    pub locations_replacement: Option<Vec<AcpToolCallLocation>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpToolKind {
    Read,
    Edit,
    Delete,
    Move,
    Search,
    Execute,
    Think,
    Fetch,
    SwitchMode,
    Other(AcpBoundedText),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

impl AcpToolCallStatus {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AcpToolCallContent {
    Content(AcpContentBlock),
    Diff {
        path: AcpBoundedText,
        old_text: Option<AcpBoundedText>,
        new_text: AcpBoundedText,
    },
    Terminal {
        terminal_id: AcpBoundedText,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpToolCallLocation {
    pub path: AcpBoundedText,
    pub line: Option<u32>,
}
