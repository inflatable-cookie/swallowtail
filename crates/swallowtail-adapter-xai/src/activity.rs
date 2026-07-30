use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct XaiActivityProjection {
    operation_id: ActivityOperationId,
    response_id: Option<String>,
}

impl XaiActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            response_id: None,
        }
    }

    pub(crate) fn started(
        &mut self,
        response_id: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        if self.response_id.replace(response_id.to_owned()).is_some() {
            return Err(activity_drift());
        }
        self.observation(
            response_id,
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
        )
    }

    pub(crate) fn delta(
        &self,
        response_id: &str,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.require_response(response_id)?;
        self.observation(
            response_id,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(
                delta,
                ActivityContentChangeKind::Delta,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    pub(crate) fn completed(
        &self,
        response_id: &str,
        output: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.require_response(response_id)?;
        self.observation(
            response_id,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    fn require_response(&self, response_id: &str) -> Result<(), RuntimeFailure> {
        if self.response_id.as_deref() == Some(response_id) {
            Ok(())
        } else {
            Err(activity_drift())
        }
    }

    fn observation(
        &self,
        response_id: &str,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            ActivityId::new("xai:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(
            ProviderActivityRef::new(response_id).map_err(|_| activity_drift())?,
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
        "swallowtail.xai.activity_invalid",
        "xAI activity did not match the qualified Responses WebSocket stream",
    )
}
