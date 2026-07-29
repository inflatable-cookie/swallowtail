use crate::{InputLimitExceeded, OperationContent};
use std::fmt;
pub use swallowtail_core::ActivityContentStream;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ActivityContentChangeKind {
    Delta,
    ReplacementSnapshot,
}

/// Bounded task content. It is not safe diagnostic text.
#[derive(Clone, Eq, PartialEq)]
pub struct ActivityContent(OperationContent);

impl ActivityContent {
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
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.byte_len()
    }

    #[must_use]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityContentUpdate {
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
    content: ActivityContent,
}

impl ActivityContentUpdate {
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
    pub const fn change(&self) -> ActivityContentChangeKind {
        self.change
    }

    #[must_use]
    pub const fn stream(&self) -> ActivityContentStream {
        self.stream
    }

    #[must_use]
    pub const fn content(&self) -> &ActivityContent {
        &self.content
    }
}
