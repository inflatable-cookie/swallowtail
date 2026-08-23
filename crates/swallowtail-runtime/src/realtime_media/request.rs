use crate::{Deadline, RequestId};
use std::num::NonZeroU64;
use swallowtail_core::{
    PlannedConnectionRolloverPolicy, RealtimeMediaConfig, ReasoningMode, SessionProviderStatePolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Request to open one preflight-bound realtime media session.
pub struct OpenRealtimeMediaSessionRequest {
    request_id: RequestId,
    config: RealtimeMediaConfig,
    deadline: Option<Deadline>,
    maximum_output_tokens: Option<NonZeroU64>,
    reasoning_mode: Option<ReasoningMode>,
    provider_state_policy: SessionProviderStatePolicy,
    planned_connection_rollover: PlannedConnectionRolloverPolicy,
}

impl OpenRealtimeMediaSessionRequest {
    #[must_use]
    /// Creates a request with provider state prohibited and rollover disabled.
    pub const fn new(
        request_id: RequestId,
        config: RealtimeMediaConfig,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            request_id,
            config,
            deadline,
            maximum_output_tokens: None,
            reasoning_mode: None,
            provider_state_policy: SessionProviderStatePolicy::Prohibited,
            planned_connection_rollover: PlannedConnectionRolloverPolicy::Disabled,
        }
    }

    #[must_use]
    /// Returns the consumer request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the immutable media formats and session bounds.
    pub const fn config(&self) -> &RealtimeMediaConfig {
        &self.config
    }

    #[must_use]
    /// Returns the optional absolute session deadline.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    /// Sets an exact maximum output-token bound.
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    /// Returns the optional maximum output-token bound.
    pub const fn maximum_output_tokens(&self) -> Option<NonZeroU64> {
        self.maximum_output_tokens
    }

    #[must_use]
    /// Sets one exact portable reasoning selection for the session.
    pub fn with_reasoning_mode(mut self, mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(mode);
        self
    }

    #[must_use]
    /// Returns the optional portable reasoning selection.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }

    #[must_use]
    /// Returns the provider-state policy, which is prohibited for this request.
    pub const fn provider_state_policy(&self) -> SessionProviderStatePolicy {
        self.provider_state_policy
    }

    #[must_use]
    /// Sets the bounded planned connection-rollover policy.
    pub const fn with_planned_connection_rollover(
        mut self,
        policy: PlannedConnectionRolloverPolicy,
    ) -> Self {
        self.planned_connection_rollover = policy;
        self
    }

    #[must_use]
    /// Returns the planned connection-rollover policy.
    pub const fn planned_connection_rollover(&self) -> PlannedConnectionRolloverPolicy {
        self.planned_connection_rollover
    }
}
