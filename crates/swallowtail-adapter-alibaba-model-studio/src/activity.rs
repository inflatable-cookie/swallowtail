use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct AlibabaActivityProjection {
    operation_id: ActivityOperationId,
    item: Option<String>,
}

impl AlibabaActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            item: None,
        }
    }

    pub(crate) fn started(&mut self, item: &str) -> Result<ActivityObservation, RuntimeFailure> {
        if self.item.replace(item.to_owned()).is_some() {
            return Err(activity_drift());
        }
        self.observation(
            item,
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
        )
    }

    pub(crate) fn delta(
        &self,
        item: &str,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.require_item(item)?;
        self.observation(
            item,
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
        item: &str,
        output: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.require_item(item)?;
        self.observation(
            item,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(display(
                output,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::FinalAnswerText,
            )?),
        )
    }

    fn require_item(&self, item: &str) -> Result<(), RuntimeFailure> {
        if self.item.as_deref() == Some(item) {
            Ok(())
        } else {
            Err(activity_drift())
        }
    }

    fn observation(
        &self,
        item: &str,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            ActivityId::new("alibaba:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(ProviderActivityRef::new(item).map_err(|_| activity_drift())?);
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
        "swallowtail.alibaba_model_studio.activity_invalid",
        "Alibaba Model Studio activity did not match the qualified Responses stream",
    )
}
