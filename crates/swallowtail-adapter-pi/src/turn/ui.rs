use super::{ActiveTurn, MAXIMUM_DIALOG_BYTES, MAXIMUM_DIALOG_OPTIONS, malformed_ui_sequence};
use crate::failure::failure;
use crate::protocol::{PiUiDialog, PiUiDialogMethod, PiUiDisplay, PiUiDisplayKind};
use swallowtail_runtime::{
    CallbackId, CallbackRequest, Deadline, HarnessQuestionId, HarnessQuestionOptionId,
    HarnessUiDisplay, HarnessUiDisplayKind, HarnessUserInputChoiceMode, HarnessUserInputOption,
    HarnessUserInputQuestion, HarnessUserInputQuestionKind, HarnessUserInputRequest,
    OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
};

pub(crate) struct CallbackTimer {
    pub(crate) callback_id: CallbackId,
    pub(crate) deadline: Deadline,
}

impl ActiveTurn {
    pub(crate) fn handle_dialog(
        &self,
        dialog: PiUiDialog,
        deadline: Option<Deadline>,
    ) -> Result<Option<CallbackTimer>, RuntimeFailure> {
        self.claim_ui_id(&dialog.id)?;
        let callback_id = CallbackId::new(&dialog.id).map_err(|_| malformed_ui())?;
        let question_id = HarnessQuestionId::new(format!("{}:question", dialog.id))
            .map_err(|_| malformed_ui())?;
        let header = OperationContent::new(dialog.title).map_err(|_| malformed_ui())?;
        let prompt = dialog
            .prompt
            .map(OperationContent::new)
            .transpose()
            .map_err(|_| malformed_ui())?
            .unwrap_or_else(|| header.clone());
        let (kind, options) = match dialog.method {
            PiUiDialogMethod::Confirm => (
                HarnessUserInputQuestionKind::Choice {
                    mode: HarnessUserInputChoiceMode::Single,
                    allow_other: false,
                },
                [("true", "Yes"), ("false", "No")]
                    .into_iter()
                    .map(|(id, label)| {
                        Ok(HarnessUserInputOption::new(
                            HarnessQuestionOptionId::new(id).map_err(|_| malformed_ui())?,
                            OperationContent::new(label).map_err(|_| malformed_ui())?,
                            None,
                        ))
                    })
                    .collect::<Result<Vec<_>, RuntimeFailure>>()?,
            ),
            PiUiDialogMethod::Select => (
                HarnessUserInputQuestionKind::Choice {
                    mode: HarnessUserInputChoiceMode::Single,
                    allow_other: false,
                },
                dialog
                    .options
                    .into_iter()
                    .map(|option| {
                        Ok(HarnessUserInputOption::new(
                            HarnessQuestionOptionId::new(&option).map_err(|_| malformed_ui())?,
                            OperationContent::new(option).map_err(|_| malformed_ui())?,
                            None,
                        ))
                    })
                    .collect::<Result<Vec<_>, RuntimeFailure>>()?,
            ),
            PiUiDialogMethod::Input | PiUiDialogMethod::Editor => (
                HarnessUserInputQuestionKind::Text { secret: false },
                Vec::new(),
            ),
        };
        let question = HarnessUserInputQuestion::new(question_id, header, prompt, kind, options)
            .map_err(|_| malformed_ui())?;
        let ui = HarnessUserInputRequest::new(
            [question],
            None,
            1,
            MAXIMUM_DIALOG_OPTIONS,
            MAXIMUM_DIALOG_BYTES,
        )
        .map_err(|_| malformed_ui())?;
        let sequence = self.next_sequence();
        let request = CallbackRequest::harness_user_input(
            callback_id.clone(),
            self.runtime_id.clone(),
            sequence,
            deadline,
            ui.clone(),
        );
        self.callbacks
            .enqueue(request, dialog.id, dialog.method, ui)?;
        self.events.send(RuntimeEvent::new(
            sequence,
            RuntimeEventKind::CallbackRequested(callback_id.clone()),
        ))?;
        Ok(deadline.map(|deadline| CallbackTimer {
            callback_id,
            deadline,
        }))
    }

    pub(crate) fn handle_display(&self, display: PiUiDisplay) -> Result<(), RuntimeFailure> {
        self.claim_ui_id(&display.id)?;
        let kind = match display.kind {
            PiUiDisplayKind::Notification => HarnessUiDisplayKind::Notification,
            PiUiDisplayKind::Status => HarnessUiDisplayKind::Status,
            PiUiDisplayKind::Widget => HarnessUiDisplayKind::Widget,
            PiUiDisplayKind::Title => HarnessUiDisplayKind::Title,
            PiUiDisplayKind::EditorSuggestion => HarnessUiDisplayKind::EditorSuggestion,
        };
        let display = HarnessUiDisplay::new(
            kind,
            OperationContent::new(display.content).map_err(|_| malformed_ui())?,
            MAXIMUM_DIALOG_BYTES,
        )
        .map_err(|_| malformed_ui())?;
        self.events.send(RuntimeEvent::new(
            self.next_sequence(),
            RuntimeEventKind::HarnessUiDisplay(display),
        ))
    }

    fn claim_ui_id(&self, id: &str) -> Result<(), RuntimeFailure> {
        if self
            .ui_ids
            .lock()
            .expect("Pi UI id lock poisoned")
            .insert(id.to_owned())
        {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.pi.rpc.ui_id_reused",
                "Pi RPC reused an extension UI request id",
            ))
        }
    }

    pub(crate) fn callback_finished(
        &self,
        callback_id: CallbackId,
    ) -> crate::callback::CallbackFinishedFuture {
        self.callbacks.finished_future(callback_id)
    }

    pub(crate) fn expire_callback(&self, callback_id: &CallbackId) -> Option<serde_json::Value> {
        self.callbacks.expire(callback_id)
    }
}

fn malformed_ui() -> RuntimeFailure {
    malformed_ui_sequence()
}
