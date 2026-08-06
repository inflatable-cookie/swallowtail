#![deny(missing_docs)]

use crate::{HarnessCommandId, InputLimitExceeded, OperationContent};
use swallowtail_core::HarnessMessageClass;

/// Harness transport acknowledgement for one correlated command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessCommandAcknowledgement {
    /// The harness accepted the command for its native scheduling class.
    Accepted,
    /// The harness rejected the command without accepting scheduled work.
    Rejected,
}

/// Correlated transport acknowledgement. It carries no model lifecycle state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessCommandResponse {
    command_id: HarnessCommandId,
    acknowledgement: HarnessCommandAcknowledgement,
}

impl HarnessCommandResponse {
    #[must_use]
    /// Creates a transport acknowledgement for one exact command.
    pub const fn new(
        command_id: HarnessCommandId,
        acknowledgement: HarnessCommandAcknowledgement,
    ) -> Self {
        Self {
            command_id,
            acknowledgement,
        }
    }

    #[must_use]
    /// Returns the correlated harness command identity.
    pub const fn command_id(&self) -> &HarnessCommandId {
        &self.command_id
    }

    #[must_use]
    /// Returns whether the harness accepted or rejected the command.
    pub const fn acknowledgement(&self) -> HarnessCommandAcknowledgement {
        self.acknowledgement
    }
}

/// Opaque prompt, steering, or follow-up content scheduled through harness RPC.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessScheduledMessage {
    command_id: HarnessCommandId,
    class: HarnessMessageClass,
    content: OperationContent,
}

impl HarnessScheduledMessage {
    #[must_use]
    /// Creates a message with an exact identity and scheduling class.
    pub const fn new(
        command_id: HarnessCommandId,
        class: HarnessMessageClass,
        content: OperationContent,
    ) -> Self {
        Self {
            command_id,
            class,
            content,
        }
    }

    #[must_use]
    /// Returns the correlated harness command identity.
    pub const fn command_id(&self) -> &HarnessCommandId {
        &self.command_id
    }

    #[must_use]
    /// Returns the prompt, steering, or follow-up scheduling class.
    pub const fn class(&self) -> HarnessMessageClass {
        self.class
    }

    #[must_use]
    /// Returns the redacted operation content.
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }
}

/// Kind of display-only harness extension UI observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessUiDisplayKind {
    /// Suggested title update.
    Title,
    /// Suggested status update.
    Status,
    /// Provider-defined bounded widget display.
    Widget,
    /// Display-only notification.
    Notification,
    /// Suggested editor content.
    EditorSuggestion,
}

/// Bounded display-only harness UI observation requiring no response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUiDisplay {
    kind: HarnessUiDisplayKind,
    content: OperationContent,
}

impl HarnessUiDisplay {
    /// Creates a display observation after enforcing its content-byte bound.
    pub fn new(
        kind: HarnessUiDisplayKind,
        content: OperationContent,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        if content.byte_len() > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "harness UI display",
                maximum_bytes,
                content.byte_len(),
            ));
        }
        Ok(Self { kind, content })
    }

    #[must_use]
    /// Returns the display-only UI kind.
    pub const fn kind(&self) -> HarnessUiDisplayKind {
        self.kind
    }

    #[must_use]
    /// Returns the bounded redacted display content.
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HarnessCommandAcknowledgement, HarnessCommandResponse, HarnessUiDisplay,
        HarnessUiDisplayKind,
    };
    use crate::{HarnessCommandId, OperationContent};

    #[test]
    fn acknowledgement_has_no_model_completion_state() {
        let response = HarnessCommandResponse::new(
            HarnessCommandId::new("private-command").unwrap(),
            HarnessCommandAcknowledgement::Accepted,
        );

        assert_eq!(
            response.acknowledgement(),
            HarnessCommandAcknowledgement::Accepted
        );
        assert!(!format!("{response:?}").contains("private-command"));
    }

    #[test]
    fn ui_records_are_bounded_and_redacted() {
        let display = HarnessUiDisplay::new(
            HarnessUiDisplayKind::Status,
            OperationContent::new("private status").unwrap(),
            32,
        )
        .unwrap();

        assert!(!format!("{display:?}").contains("private"));
    }
}
