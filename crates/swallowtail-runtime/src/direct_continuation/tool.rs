use crate::{DirectInferenceAttemptId, DirectToolCallId, InputLimitExceeded, InputValueRequired};
use std::fmt;

#[derive(Clone, Eq, PartialEq)]
pub struct DirectToolArguments(Vec<u8>);

impl DirectToolArguments {
    pub fn new(
        bytes: impl Into<Vec<u8>>,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let bytes = bytes.into();
        if bytes.len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "direct tool arguments",
                maximum_bytes,
                bytes.len(),
            ))
        } else {
            Ok(Self(bytes))
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for DirectToolArguments {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl fmt::Debug for DirectToolArguments {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DirectToolArguments")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectToolCall {
    call_id: DirectToolCallId,
    attempt_id: DirectInferenceAttemptId,
    tool_name: String,
    arguments: DirectToolArguments,
}

impl DirectToolCall {
    pub fn new(
        call_id: DirectToolCallId,
        attempt_id: DirectInferenceAttemptId,
        tool_name: impl Into<String>,
        arguments: DirectToolArguments,
    ) -> Result<Self, InputValueRequired> {
        Ok(Self {
            call_id,
            attempt_id,
            tool_name: crate::input::required_text("direct tool name", tool_name)?,
            arguments,
        })
    }

    #[must_use]
    pub const fn call_id(&self) -> &DirectToolCallId {
        &self.call_id
    }

    #[must_use]
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    pub fn tool_name(&self) -> &str {
        &self.tool_name
    }

    #[must_use]
    pub const fn arguments(&self) -> &DirectToolArguments {
        &self.arguments
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct DirectToolResultContent(Vec<u8>);

impl DirectToolResultContent {
    pub fn new(
        bytes: impl Into<Vec<u8>>,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let bytes = bytes.into();
        if bytes.len() > maximum_bytes {
            Err(InputLimitExceeded::new(
                "direct tool result",
                maximum_bytes,
                bytes.len(),
            ))
        } else {
            Ok(Self(bytes))
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[must_use]
    pub fn byte_len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for DirectToolResultContent {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl fmt::Debug for DirectToolResultContent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("DirectToolResultContent")
            .field(&format_args!("<redacted:{} bytes>", self.byte_len()))
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectToolResult {
    call_id: DirectToolCallId,
    content: DirectToolResultContent,
}

impl DirectToolResult {
    #[must_use]
    pub const fn new(call_id: DirectToolCallId, content: DirectToolResultContent) -> Self {
        Self { call_id, content }
    }

    #[must_use]
    pub const fn call_id(&self) -> &DirectToolCallId {
        &self.call_id
    }

    #[must_use]
    pub const fn content(&self) -> &DirectToolResultContent {
        &self.content
    }
}
