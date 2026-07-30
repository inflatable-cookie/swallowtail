use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct KimiPlatformActivityProjection {
    operation_id: ActivityOperationId,
    assistant_started: bool,
    reasoning_seen: bool,
    reasoning_completed: bool,
}

impl KimiPlatformActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            assistant_started: false,
            reasoning_seen: false,
            reasoning_completed: false,
        }
    }

    pub(crate) fn assistant_started(&mut self) -> Result<ActivityObservation, RuntimeFailure> {
        if self.assistant_started {
            return Err(activity_drift());
        }
        self.assistant_started = true;
        observation(
            "kimi-platform:assistant",
            &self.operation_id,
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
            None,
        )
    }

    pub(crate) fn reasoning_delta(
        &mut self,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        if self.reasoning_completed {
            return Err(activity_drift());
        }
        self.reasoning_seen = true;
        observation(
            "kimi-platform:reasoning",
            &self.operation_id,
            ActivityKind::ReasoningSummary,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            Some(display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::ReasoningSummaryText,
            )?),
        )
    }

    pub(crate) fn reasoning_completed(
        &mut self,
    ) -> Result<Option<ActivityObservation>, RuntimeFailure> {
        if !self.reasoning_seen || self.reasoning_completed {
            return Ok(None);
        }
        self.reasoning_completed = true;
        observation(
            "kimi-platform:reasoning",
            &self.operation_id,
            ActivityKind::ReasoningSummary,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            None,
        )
        .map(Some)
    }

    pub(crate) fn assistant_delta(
        &self,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        if !self.assistant_started {
            return Err(activity_drift());
        }
        observation(
            "kimi-platform:assistant",
            &self.operation_id,
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
            Some(display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    pub(crate) fn assistant_completed(
        &self,
        output: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        if !self.assistant_started {
            return Err(activity_drift());
        }
        observation(
            "kimi-platform:assistant",
            &self.operation_id,
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
            Some(display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn observation(
    id: &str,
    operation_id: &ActivityOperationId,
    kind: ActivityKind,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    content: Option<ActivityContentUpdate>,
) -> Result<ActivityObservation, RuntimeFailure> {
    let mut observation = ActivityObservation::new(
        ActivityId::new(id).map_err(|_| activity_drift())?,
        operation_id.clone(),
        kind,
        phase,
        status,
        assistant_phase,
        disclosure,
    )
    .map_err(|_| activity_drift())?;
    if let Some(content) = content {
        observation = observation
            .with_content(content)
            .map_err(|_| activity_drift())?;
    }
    Ok(observation)
}

fn display(
    value: &str,
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let value = bounded(value);
    let value = OperationContent::new(value).map_err(|_| activity_drift())?;
    let value = ActivityContent::new(value, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, value))
}

fn bounded(value: &str) -> String {
    if value.len() <= MAXIMUM_ACTIVITY_CONTENT_BYTES {
        return value.to_owned();
    }
    let mut end = MAXIMUM_ACTIVITY_CONTENT_BYTES;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_owned()
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.activity_invalid",
        "Kimi Platform activity did not match the qualified K3 stream",
    )
}
