use super::{AcpBoundedText, AcpContentBlock};

/// Complete ACP tool-call creation or snapshot.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpToolCall {
    /// Provider tool-call correlation identity.
    pub tool_call_id: AcpBoundedText,
    /// Provider display title, distinct from result content.
    pub title: AcpBoundedText,
    /// Portable or provider-defined tool classification.
    pub kind: AcpToolKind,
    /// Current lifecycle status, which may already be terminal.
    pub status: AcpToolCallStatus,
    /// Current bounded display or result content.
    pub content: Vec<AcpToolCallContent>,
    /// Current source locations associated with the call.
    pub locations: Vec<AcpToolCallLocation>,
}

/// Partial ACP refinement of an existing tool-call identity.
#[derive(Clone, Debug, PartialEq)]
pub struct AcpToolCallUpdate {
    /// Provider tool-call correlation identity.
    pub tool_call_id: AcpBoundedText,
    /// Optional replacement display title.
    pub title: Option<AcpBoundedText>,
    /// Optional refined tool classification.
    pub kind: Option<AcpToolKind>,
    /// Optional refined lifecycle status.
    pub status: Option<AcpToolCallStatus>,
    /// Optional complete replacement of prior content.
    pub content_replacement: Option<Vec<AcpToolCallContent>>,
    /// Optional complete replacement of prior locations.
    pub locations_replacement: Option<Vec<AcpToolCallLocation>>,
}

/// Portable ACP tool classification with a lossless provider fallback.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AcpToolKind {
    /// Read content from a resource.
    Read,
    /// Modify content in place.
    Edit,
    /// Delete a resource.
    Delete,
    /// Move or rename a resource.
    Move,
    /// Search local or remote content.
    Search,
    /// Execute a command or program.
    Execute,
    /// Perform provider-visible reasoning work.
    Think,
    /// Fetch a remote resource.
    Fetch,
    /// Change the harness operating mode.
    SwitchMode,
    /// Provider-defined tool kind retained verbatim.
    Other(AcpBoundedText),
}

/// Lifecycle status reported for an ACP tool call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpToolCallStatus {
    /// Accepted but not yet executing.
    Pending,
    /// Currently executing.
    InProgress,
    /// Finished successfully.
    Completed,
    /// Finished unsuccessfully.
    Failed,
}

impl AcpToolCallStatus {
    /// Returns whether no later lifecycle progress is expected.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed)
    }
}

/// One typed content item attached to an ACP tool call.
#[derive(Clone, Debug, PartialEq)]
pub enum AcpToolCallContent {
    /// General ACP content block.
    Content(AcpContentBlock),
    /// File-oriented before/after diff.
    Diff {
        /// Affected path.
        path: AcpBoundedText,
        /// Previous content when reported.
        old_text: Option<AcpBoundedText>,
        /// Replacement content.
        new_text: AcpBoundedText,
    },
    /// Reference to a separately managed terminal stream.
    Terminal {
        /// Provider terminal identity.
        terminal_id: AcpBoundedText,
    },
}

/// Source location associated with an ACP tool call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpToolCallLocation {
    /// Provider-supplied path.
    pub path: AcpBoundedText,
    /// Optional one-based line number.
    pub line: Option<u32>,
}
