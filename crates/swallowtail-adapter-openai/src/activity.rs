use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct OpenAiBackgroundActivityProjection {
    operation_id: ActivityOperationId,
    response_id: String,
}

impl OpenAiBackgroundActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId, response_id: String) -> Self {
        Self {
            operation_id,
            response_id,
        }
    }

    pub(crate) fn started(&self) -> Result<ActivityObservation, RuntimeFailure> {
        self.observation(
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
        )
    }

    pub(crate) fn delta(&self, delta: &str) -> Result<ActivityObservation, RuntimeFailure> {
        self.observation(
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    pub(crate) fn completed(&self, output: &str) -> Result<ActivityObservation, RuntimeFailure> {
        self.observation(
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    fn observation(
        &self,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            ActivityId::new("openai-background:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(
            ProviderActivityRef::new(&self.response_id).map_err(|_| activity_drift())?,
        );
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(observation)
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
        "swallowtail.openai.activity_invalid",
        "OpenAI background activity did not match the qualified Responses stream",
    )
}
