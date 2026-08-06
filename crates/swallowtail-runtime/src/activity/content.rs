use crate::{InputLimitExceeded, OperationContent};
use std::fmt;
pub use swallowtail_core::ActivityContentStream;

/// How an activity content update changes the selected content stream.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityContentChangeKind {
    /// Appends content to the existing stream value.
    Delta,
    /// Replaces the complete previously observed stream value.
    ReplacementSnapshot,
}

/// Bounded task content. It is not safe diagnostic text.
#[derive(Clone, Eq, PartialEq)]
pub struct ActivityContent(OperationContent);

impl ActivityContent {
    /// Creates bounded activity content from operation content.
    pub fn new(
        content: OperationContent,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        if content.byte_len() > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "activity content",
                maximum_bytes,
                content.byte_len(),
            ));
        }
        Ok(Self(content))
    }

    #[must_use]
    /// Returns the unredacted content for an authorized consumer projection.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    /// Returns the encoded UTF-8 byte length.
    pub fn byte_len(&self) -> usize {
        self.0.byte_len()
    }

    #[must_use]
    /// Recovers the underlying operation content.
    pub fn into_operation_content(self) -> OperationContent {
        self.0
    }
}

impl fmt::Debug for ActivityContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ActivityContent")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

impl fmt::Display for ActivityContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted activity content>")
    }
}

/// One bounded change to a named activity content stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityContentUpdate {
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
    content: ActivityContent,
}

impl ActivityContentUpdate {
    /// Creates an update with explicit change and stream semantics.
    #[must_use]
    pub const fn new(
        change: ActivityContentChangeKind,
        stream: ActivityContentStream,
        content: ActivityContent,
    ) -> Self {
        Self {
            change,
            stream,
            content,
        }
    }

    #[must_use]
    /// Returns whether this update appends or replaces content.
    pub const fn change(&self) -> ActivityContentChangeKind {
        self.change
    }

    #[must_use]
    /// Returns the semantic stream receiving this update.
    pub const fn stream(&self) -> ActivityContentStream {
        self.stream
    }

    #[must_use]
    /// Returns the bounded task content.
    pub const fn content(&self) -> &ActivityContent {
        &self.content
    }
}
