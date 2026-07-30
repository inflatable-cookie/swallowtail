use crate::{HarnessCommandId, InputLimitExceeded, OperationContent};
use swallowtail_core::HarnessMessageClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessCommandAcknowledgement {
    Accepted,
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
    pub const fn command_id(&self) -> &HarnessCommandId {
        &self.command_id
    }

    #[must_use]
    pub const fn acknowledgement(&self) -> HarnessCommandAcknowledgement {
        self.acknowledgement
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessScheduledMessage {
    command_id: HarnessCommandId,
    class: HarnessMessageClass,
    content: OperationContent,
}

impl HarnessScheduledMessage {
    #[must_use]
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
    pub const fn command_id(&self) -> &HarnessCommandId {
        &self.command_id
    }

    #[must_use]
    pub const fn class(&self) -> HarnessMessageClass {
        self.class
    }

    #[must_use]
    pub const fn content(&self) -> &OperationContent {
        &self.content
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarnessUiDisplayKind {
    Title,
    Status,
    Widget,
    Notification,
    EditorSuggestion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUiDisplay {
    kind: HarnessUiDisplayKind,
    content: OperationContent,
}

impl HarnessUiDisplay {
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
    pub const fn kind(&self) -> HarnessUiDisplayKind {
        self.kind
    }

    #[must_use]
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
