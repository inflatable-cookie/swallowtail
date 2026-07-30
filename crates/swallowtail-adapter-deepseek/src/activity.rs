use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityCorrelation, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, DirectToolCallId, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct DeepSeekActivityProjection {
    operation_id: ActivityOperationId,
}

impl DeepSeekActivityProjection {
    pub(crate) const fn new(operation_id: ActivityOperationId) -> Self {
        Self { operation_id }
    }

    pub(crate) fn assistant_delta(
        &self,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.assistant(
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::FinalAnswerText,
            )?,
        )
    }

    pub(crate) fn assistant_completed(
        &self,
        output: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.assistant(
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?,
        )
    }

    pub(crate) fn tool_completed(
        &self,
        call_id: &DirectToolCallId,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        ActivityObservation::new(
            ActivityId::new("deepseek:consumer-tool").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::ConsumerOwnedTool,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())
        .map(|observation| {
            observation.with_correlation(ActivityCorrelation::DirectToolCall(call_id.clone()))
        })
    }

    fn assistant(
        &self,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: ActivityContentUpdate,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        ActivityObservation::new(
            ActivityId::new("deepseek:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?
        .with_content(content)
        .map_err(|_| activity_drift())
    }
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
        "swallowtail.deepseek.activity_invalid",
        "DeepSeek activity did not match the qualified direct-inference stream",
    )
}
