use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct OllamaActivityProjection {
    operation_id: ActivityOperationId,
    seen: bool,
}

impl OllamaActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            seen: false,
        }
    }

    pub(crate) fn delta(&mut self, delta: &str) -> Result<ActivityObservation, RuntimeFailure> {
        self.seen = true;
        self.observation(
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::FinalAnswerText,
            )?,
        )
    }

    pub(crate) fn completed(&self, output: &str) -> Result<ActivityObservation, RuntimeFailure> {
        if !self.seen {
            return Err(activity_drift());
        }
        self.observation(
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?,
        )
    }

    fn observation(
        &self,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: ActivityContentUpdate,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        ActivityObservation::new(
            ActivityId::new("ollama:assistant").map_err(|_| activity_drift())?,
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
        "swallowtail.ollama.activity_invalid",
        "Ollama activity did not match the qualified native text stream",
    )
}
