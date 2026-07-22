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
pub enum HarnessUiDialogKind {
    Confirm,
    Select,
    Input,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarnessUiDialog {
    kind: HarnessUiDialogKind,
    title: OperationContent,
    prompt: Option<OperationContent>,
    options: Vec<OperationContent>,
}

impl HarnessUiDialog {
    pub fn new(
        kind: HarnessUiDialogKind,
        title: OperationContent,
        prompt: Option<OperationContent>,
        options: impl IntoIterator<Item = OperationContent>,
        maximum_options: usize,
        maximum_bytes: usize,
    ) -> Result<Self, InputLimitExceeded> {
        let options: Vec<_> = options.into_iter().collect();
        if options.len() > maximum_options {
            return Err(InputLimitExceeded::new(
                "harness UI dialog options",
                maximum_options,
                options.len(),
            ));
        }
        let actual = title.byte_len()
            + prompt.as_ref().map_or(0, OperationContent::byte_len)
            + options
                .iter()
                .map(OperationContent::byte_len)
                .sum::<usize>();
        if actual > maximum_bytes {
            return Err(InputLimitExceeded::new(
                "harness UI dialog",
                maximum_bytes,
                actual,
            ));
        }
        Ok(Self {
            kind,
            title,
            prompt,
            options,
        })
    }

    #[must_use]
    pub const fn kind(&self) -> HarnessUiDialogKind {
        self.kind
    }

    #[must_use]
    pub const fn title(&self) -> &OperationContent {
        &self.title
    }

    #[must_use]
    pub const fn prompt(&self) -> Option<&OperationContent> {
        self.prompt.as_ref()
    }

    pub fn options(&self) -> impl ExactSizeIterator<Item = &OperationContent> {
        self.options.iter()
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
        HarnessCommandAcknowledgement, HarnessCommandResponse, HarnessUiDialog,
        HarnessUiDialogKind, HarnessUiDisplay, HarnessUiDisplayKind,
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
        let dialog = HarnessUiDialog::new(
            HarnessUiDialogKind::Select,
            OperationContent::new("private title").unwrap(),
            None,
            [OperationContent::new("private option").unwrap()],
            2,
            64,
        )
        .unwrap();
        let display = HarnessUiDisplay::new(
            HarnessUiDisplayKind::Status,
            OperationContent::new("private status").unwrap(),
            32,
        )
        .unwrap();

        assert_eq!(dialog.options().len(), 1);
        assert!(!format!("{dialog:?}").contains("private"));
        assert!(!format!("{display:?}").contains("private"));
    }
}
