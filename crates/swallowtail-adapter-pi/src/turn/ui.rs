use super::{ActiveTurn, MAXIMUM_DIALOG_BYTES, MAXIMUM_DIALOG_OPTIONS, malformed_ui_sequence};
use crate::failure::failure;
use crate::protocol::{PiUiDialog, PiUiDialogMethod, PiUiDisplay, PiUiDisplayKind};
use swallowtail_runtime::{
    CallbackId, CallbackRequest, Deadline, HarnessUiDialog, HarnessUiDialogKind, HarnessUiDisplay,
    HarnessUiDisplayKind, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
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
        let kind = match dialog.method {
            PiUiDialogMethod::Select => HarnessUiDialogKind::Select,
            PiUiDialogMethod::Confirm => HarnessUiDialogKind::Confirm,
            PiUiDialogMethod::Input | PiUiDialogMethod::Editor => HarnessUiDialogKind::Input,
        };
        let ui = HarnessUiDialog::new(
            kind,
            OperationContent::new(dialog.title).map_err(|_| malformed_ui())?,
            dialog
                .prompt
                .map(OperationContent::new)
                .transpose()
                .map_err(|_| malformed_ui())?,
            dialog
                .options
                .into_iter()
                .map(OperationContent::new)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| malformed_ui())?,
            MAXIMUM_DIALOG_OPTIONS,
            MAXIMUM_DIALOG_BYTES,
        )
        .map_err(|_| malformed_ui())?;
        let sequence = self.next_sequence();
        let request = CallbackRequest::harness_ui_dialog(
            callback_id.clone(),
            self.runtime_id.clone(),
            sequence,
            deadline,
            ui,
        );
        self.callbacks.enqueue(request, dialog.id, dialog.method)?;
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
