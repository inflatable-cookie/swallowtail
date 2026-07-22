use crate::{Deadline, RequestId};
use swallowtail_core::{
    PlannedConnectionRolloverPolicy, RealtimeMediaConfig, SessionProviderStatePolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenRealtimeMediaSessionRequest {
    request_id: RequestId,
    config: RealtimeMediaConfig,
    deadline: Option<Deadline>,
    provider_state_policy: SessionProviderStatePolicy,
    planned_connection_rollover: PlannedConnectionRolloverPolicy,
}

impl OpenRealtimeMediaSessionRequest {
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        config: RealtimeMediaConfig,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            config,
            deadline,
            provider_state_policy: SessionProviderStatePolicy::Prohibited,
            planned_connection_rollover: PlannedConnectionRolloverPolicy::Disabled,
        }
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn config(&self) -> &RealtimeMediaConfig {
        &self.config
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn provider_state_policy(&self) -> SessionProviderStatePolicy {
        self.provider_state_policy
    }

    #[must_use]
    pub const fn with_planned_connection_rollover(
        mut self,
        policy: PlannedConnectionRolloverPolicy,
    ) -> Self {
        self.planned_connection_rollover = policy;
        self
    }

    #[must_use]
    pub const fn planned_connection_rollover(&self) -> PlannedConnectionRolloverPolicy {
        self.planned_connection_rollover
    }
}
