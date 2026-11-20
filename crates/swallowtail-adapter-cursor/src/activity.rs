use crate::failure::malformed;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpActivityProjection as SharedProjection, MessageClassification,
    PlanProjection,
};
use swallowtail_protocol_acp::{AcpContentBlock, AcpSessionUpdate};
use swallowtail_runtime::{ActivityObservation, RuntimeFailure, RuntimeTurnId, TerminalStatus};

pub(crate) const CURSOR_ACTIVITY_CONFIG: AcpActivityConfig = AcpActivityConfig {
    id_prefix: "cursor-agent-acp",
    namespace_prefix: "cursor-agent.acp",
    plan_projection: PlanProjection::TaskList,
    message_classification: MessageClassification::Standard,
    malformed,
};

pub(crate) struct AcpActivityProjection(SharedProjection);

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self(SharedProjection::new(turn_id, CURSOR_ACTIVITY_CONFIG))
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

pub(crate) fn text_content(content: &AcpContentBlock) -> Result<&str, RuntimeFailure> {
    swallowtail_acp_client::text_content(content, malformed)
}
