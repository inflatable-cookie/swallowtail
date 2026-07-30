use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityCorrelation, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, DirectToolCallId, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct AnthropicActivityProjection {
    operation_id: ActivityOperationId,
}

impl AnthropicActivityProjection {
    pub(crate) const fn new(operation_id: ActivityOperationId) -> Self {
        Self { operation_id }
    }

    pub(crate) fn assistant_started(
        &self,
        activity_id: ActivityId,
        message_id: &str,
        phase: ActivityAssistantPhase,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.assistant(
            activity_id,
            message_id,
            phase,
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
        )
    }

    pub(crate) fn assistant_delta(
        &self,
        activity_id: ActivityId,
        message_id: &str,
        phase: ActivityAssistantPhase,
        delta: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let stream = if phase == ActivityAssistantPhase::Final {
            ActivityContentStream::FinalAnswerText
        } else {
            ActivityContentStream::IntermediateAssistantText
        };
        self.assistant(
            activity_id,
            message_id,
            phase,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(delta, ActivityContentChangeKind::Delta, stream)?),
        )
    }

    pub(crate) fn assistant_completed(
        &self,
        activity_id: ActivityId,
        message_id: &str,
        phase: ActivityAssistantPhase,
        output: Option<&str>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let content = output
            .map(|output| {
                let stream = if phase == ActivityAssistantPhase::Final {
                    ActivityContentStream::FinalAnswerText
                } else {
                    ActivityContentStream::IntermediateAssistantText
                };
                display(
                    output,
                    ActivityContentChangeKind::ReplacementSnapshot,
                    stream,
                )
            })
            .transpose()?;
        self.assistant(
            activity_id,
            message_id,
            phase,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            content,
        )
    }

    pub(crate) fn provider_tool(
        &self,
        provider_id: &str,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        ActivityObservation::new(
            ActivityId::new(format!("anthropic:web-search:{provider_id}"))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::ProviderOwnedTool,
            phase,
            status,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())
        .map(|observation| {
            observation.with_provider_activity_ref(
                ProviderActivityRef::new(provider_id)
                    .expect("validated Anthropic provider activity identity"),
            )
        })
    }

    pub(crate) fn consumer_tool(
        &self,
        call_id: &DirectToolCallId,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        ActivityObservation::new(
            ActivityId::new("anthropic:consumer-tool").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::ConsumerOwnedTool,
            phase,
            status,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())
        .map(|observation| {
            observation
                .with_provider_activity_ref(
                    ProviderActivityRef::new(call_id.as_str())
                        .expect("validated Anthropic tool identity"),
                )
                .with_correlation(ActivityCorrelation::DirectToolCall(call_id.clone()))
        })
    }

    fn assistant(
        &self,
        activity_id: ActivityId,
        message_id: &str,
        assistant_phase: ActivityAssistantPhase,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            activity_id,
            self.operation_id.clone(),
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(assistant_phase),
            ActivityDisclosure::ProviderDisplayContent,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(
            ProviderActivityRef::new(message_id).map_err(|_| activity_drift())?,
        );
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(observation)
    }
}

pub(crate) fn structured_assistant_id() -> ActivityId {
    ActivityId::new("anthropic:assistant").expect("static Anthropic activity id is valid")
}

pub(crate) fn attempt_assistant_id(
    attempt_id: &swallowtail_runtime::DirectInferenceAttemptId,
) -> Result<ActivityId, RuntimeFailure> {
    ActivityId::new(format!("anthropic:assistant:{}", attempt_id.as_str()))
        .map_err(|_| activity_drift())
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
        "swallowtail.anthropic.activity_invalid",
        "Anthropic activity did not match the qualified Messages stream",
    )
}
