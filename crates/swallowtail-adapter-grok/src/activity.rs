use crate::failure::malformed;
use swallowtail_acp_client::{AcpActivityConfig, MessageClassification, PlanProjection};

pub(crate) const GROK_ACTIVITY_CONFIG: AcpActivityConfig = AcpActivityConfig {
    id_prefix: "grok-build-acp",
    namespace_prefix: "grok-build.acp",
    plan_projection: PlanProjection::TaskList,
    message_classification: MessageClassification::Standard,
    malformed,
};
