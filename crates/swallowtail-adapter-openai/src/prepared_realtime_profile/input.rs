use std::num::NonZeroU64;
use swallowtail_core::{PlannedConnectionRolloverPolicy, RealtimeMediaConfig, ReasoningMode};
use swallowtail_runtime::{Deadline, RequestId};

type OpenAiRealtimeSessionProfileParts = (
    RequestId,
    RealtimeMediaConfig,
    Option<Deadline>,
    PlannedConnectionRolloverPolicy,
    Option<ReasoningMode>,
    Option<NonZeroU64>,
);

#[derive(Clone, Debug, Eq, PartialEq)]
/// Explicit media, deadline, rollover, reasoning, and output bounds for a Realtime session.
pub struct OpenAiRealtimeSessionProfileInput {
    request_id: RequestId,
    config: RealtimeMediaConfig,
    deadline: Option<Deadline>,
    rollover: PlannedConnectionRolloverPolicy,
    reasoning_mode: Option<ReasoningMode>,
    maximum_output_tokens: Option<NonZeroU64>,
}

impl OpenAiRealtimeSessionProfileInput {
    #[must_use]
    /// Creates session input without changing the supplied media or rollover policy.
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
            maximum_output_tokens: None,
        }
    }

    #[must_use]
    /// Selects one exact session-scoped Realtime reasoning effort.
    pub fn with_reasoning_mode(mut self, mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(mode);
        self
    }

    #[must_use]
    /// Adds an optional provider output-token maximum.
    pub const fn with_maximum_output_tokens(mut self, maximum: NonZeroU64) -> Self {
        self.maximum_output_tokens = Some(maximum);
        self
    }

    #[must_use]
    /// Creates the supported two-turn manual PCM profile.
    pub fn manual_pcm_two_turns(request_id: RequestId, deadline: Option<Deadline>) -> Self {
        Self::new(
            request_id,
            crate::openai_realtime_media_config(),
            deadline,
            PlannedConnectionRolloverPolicy::Disabled,
        )
    }

    pub(super) fn into_parts(self) -> OpenAiRealtimeSessionProfileParts {
        (
            self.request_id,
            self.config,
            self.deadline,
            self.rollover,
            self.reasoning_mode,
            self.maximum_output_tokens,
        )
    }
}
