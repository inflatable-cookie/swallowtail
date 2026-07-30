use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct BedrockActivityProjection {
    operation_id: ActivityOperationId,
    started: bool,
}

impl BedrockActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            started: false,
        }
    }

    pub(crate) fn started(&mut self) -> Result<ActivityObservation, RuntimeFailure> {
        if self.started {
            return Err(activity_drift());
        }
        self.started = true;
        self.observation(
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
        )
    }

    pub(crate) fn delta(&self, delta: &str) -> Result<ActivityObservation, RuntimeFailure> {
        if !self.started {
            return Err(activity_drift());
        }
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
        if !self.started {
            return Err(activity_drift());
        }
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
            ActivityId::new("bedrock:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?;
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
        "swallowtail.bedrock.activity_invalid",
        "Bedrock Runtime activity did not match the qualified SDK stream",
    )
}

#[cfg(test)]
mod tests {
    use super::BedrockActivityProjection;
    use swallowtail_runtime::{
        ActivityOperationId, OperationContent, RuntimeEvent, RuntimeEventKind, RuntimeRunId,
    };

    #[test]
    fn qualified_sdk_corpus_projects_one_complete_assistant_lifecycle() {
        let mut projection = BedrockActivityProjection::new(ActivityOperationId::Run(
            RuntimeRunId::new("bedrock-activity-fixture").expect("run id is valid"),
        ));
        let events = vec![
            RuntimeEvent::new(0, RuntimeEventKind::Started),
            RuntimeEvent::new(
                1,
                RuntimeEventKind::Activity(projection.started().expect("start projects")),
            ),
            RuntimeEvent::new(
                2,
                RuntimeEventKind::Activity(projection.delta("hello ").expect("delta projects")),
            ),
            RuntimeEvent::new(
                3,
                RuntimeEventKind::Activity(projection.delta("world").expect("delta projects")),
            ),
            RuntimeEvent::new(
                4,
                RuntimeEventKind::Activity(
                    projection
                        .completed("hello world")
                        .expect("completion projects"),
                ),
            ),
            RuntimeEvent::with_content(
                5,
                RuntimeEventKind::OutputAvailable,
                OperationContent::new("hello world").expect("output is valid"),
            ),
        ];
        swallowtail_testkit::assert_observable_activity_trace(
            &super::profile::activity_profile(),
            &events,
        );
    }
}
