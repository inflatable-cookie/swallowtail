use crate::failure::malformed;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpActivityProjection as SharedProjection, MessageClassification,
    PlanProjection,
};
use swallowtail_protocol_acp::AcpSessionUpdate;
use swallowtail_runtime::{ActivityObservation, RuntimeFailure, RuntimeTurnId, TerminalStatus};

pub(crate) const KIMI_ACTIVITY_CONFIG: AcpActivityConfig = AcpActivityConfig {
    id_prefix: "kimi-code-acp",
    namespace_prefix: "kimi-code.acp",
    plan_projection: PlanProjection::TaskList,
    message_classification: MessageClassification::Standard,
    malformed,
};

pub(crate) struct AcpActivityProjection(SharedProjection);

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self(SharedProjection::new(turn_id, KIMI_ACTIVITY_CONFIG))
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
