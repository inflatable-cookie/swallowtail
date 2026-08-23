use swallowtail_core::{PlannedConnectionRolloverPolicy, RealtimeMediaConfig, ReasoningMode};
use swallowtail_runtime::{Deadline, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
/// Consumer inputs for one bounded Gemini Live media session.
pub struct GeminiLiveSessionProfileInput {
    request_id: RequestId,
    config: RealtimeMediaConfig,
    deadline: Option<Deadline>,
    rollover: PlannedConnectionRolloverPolicy,
    reasoning_mode: Option<ReasoningMode>,
}

impl GeminiLiveSessionProfileInput {
    /// Creates a profile with explicit media and rollover policy.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        config: RealtimeMediaConfig,
        deadline: Option<Deadline>,
        rollover: PlannedConnectionRolloverPolicy,
    ) -> Self {
        Self {
            request_id,
            config,
            deadline,
            rollover,
            reasoning_mode: None,
        }
    }

    /// Selects one exact thinking level for every setup frame in the session.
    #[must_use]
    pub fn with_reasoning_mode(mut self, mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(mode);
        self
    }

    /// Creates the qualified manual PCM profile with one planned rollover.
    #[must_use]
    pub fn manual_pcm_with_one_rollover(request_id: RequestId, deadline: Option<Deadline>) -> Self {
        Self::new(
            request_id,
            crate::gemini_live_media_config(),
            deadline,
            crate::gemini_live_rollover_policy(),
        )
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        RealtimeMediaConfig,
        Option<Deadline>,
        PlannedConnectionRolloverPolicy,
        Option<ReasoningMode>,
    ) {
        (
            self.request_id,
            self.config,
            self.deadline,
            self.rollover,
            self.reasoning_mode,
        )
    }
}
