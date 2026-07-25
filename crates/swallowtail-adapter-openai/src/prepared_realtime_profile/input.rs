use swallowtail_core::{PlannedConnectionRolloverPolicy, RealtimeMediaConfig};
use swallowtail_runtime::{Deadline, RequestId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiRealtimeSessionProfileInput {
    request_id: RequestId,
    config: RealtimeMediaConfig,
    deadline: Option<Deadline>,
    rollover: PlannedConnectionRolloverPolicy,
}

impl OpenAiRealtimeSessionProfileInput {
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
        }
    }

    #[must_use]
    pub fn manual_pcm_two_turns(request_id: RequestId, deadline: Option<Deadline>) -> Self {
        Self::new(
            request_id,
            crate::openai_realtime_media_config(),
            deadline,
            PlannedConnectionRolloverPolicy::Disabled,
        )
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        RequestId,
        RealtimeMediaConfig,
        Option<Deadline>,
        PlannedConnectionRolloverPolicy,
    ) {
        (self.request_id, self.config, self.deadline, self.rollover)
    }
}
