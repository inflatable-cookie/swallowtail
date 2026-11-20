use crate::connection::gemini_acp_diagnostics;
use swallowtail_acp_client::AcpActivityProjection as SharedProjection;
use swallowtail_protocol_acp::AcpSessionUpdate;
use swallowtail_runtime::{ActivityObservation, RuntimeFailure, RuntimeTurnId, TerminalStatus};

pub(crate) struct AcpActivityProjection(SharedProjection);

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self(SharedProjection::new(
            turn_id,
            gemini_acp_diagnostics().activity,
        ))
    }

    pub(crate) fn project(
        &mut self,
        update: &AcpSessionUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.0.project(update)
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.0.complete(terminal)
    }
}
