use super::{
    ActivityAssistantPhase, ActivityContentStream, ActivityContentUpdate, ActivityDisclosure,
    ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityStatus,
    InvalidActivityRecord,
};

pub(super) fn validate_phase_status(
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
) -> Result<(), InvalidActivityRecord> {
    if matches!(phase, ActivityLifecyclePhase::Completed) != status.is_terminal() {
        return Err(InvalidActivityRecord::new(
            "Activity completion phase and terminal status must agree",
        ));
    }
    Ok(())
}

pub(super) fn validate_assistant_phase(
    kind: &ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
) -> Result<(), InvalidActivityRecord> {
    if matches!(kind, ActivityKind::AssistantMessage) != assistant_phase.is_some() {
        return Err(InvalidActivityRecord::new(
            "Assistant phase is required only for assistant-message activity",
        ));
    }
    Ok(())
}

pub(super) fn validate_content(
    observation: &ActivityObservation,
    content: &ActivityContentUpdate,
) -> Result<(), InvalidActivityRecord> {
    match observation.disclosure() {
        ActivityDisclosure::IdentityAndLifecycleOnly | ActivityDisclosure::Unavailable => {
            return Err(InvalidActivityRecord::new(
                "Activity disclosure does not permit content",
            ));
        }
        ActivityDisclosure::AdapterNormalizedSummary
            if content.stream() != ActivityContentStream::NormalizedSummary =>
        {
            return Err(InvalidActivityRecord::new(
                "Adapter summary disclosure requires normalized-summary content",
            ));
        }
        ActivityDisclosure::ProviderDisplayContent
            if content.stream() == ActivityContentStream::NormalizedSummary =>
        {
            return Err(InvalidActivityRecord::new(
                "Provider display disclosure cannot claim adapter-normalized content",
            ));
        }
        _ => {}
    }

    let compatible = match content.stream() {
        ActivityContentStream::IntermediateAssistantText => {
            matches!(observation.kind(), ActivityKind::AssistantMessage)
                && observation.assistant_phase() == Some(ActivityAssistantPhase::Intermediate)
        }
        ActivityContentStream::FinalAnswerText => {
            matches!(observation.kind(), ActivityKind::AssistantMessage)
                && observation.assistant_phase() == Some(ActivityAssistantPhase::Final)
        }
        ActivityContentStream::ReasoningSummaryText => {
            matches!(observation.kind(), ActivityKind::ReasoningSummary)
        }
        ActivityContentStream::PlanText => matches!(observation.kind(), ActivityKind::Plan),
        ActivityContentStream::CommandOutput => {
            matches!(observation.kind(), ActivityKind::CommandExecution)
        }
        ActivityContentStream::FileChangeOutput => {
            matches!(observation.kind(), ActivityKind::FileChange)
        }
        ActivityContentStream::ProviderToolDisplay => matches!(
            observation.kind(),
            ActivityKind::ProviderOwnedTool
                | ActivityKind::ConsumerOwnedTool
                | ActivityKind::ExternalSearch
                | ActivityKind::ImageView
        ),
        ActivityContentStream::NormalizedSummary => true,
    };
    if !compatible {
        return Err(InvalidActivityRecord::new(
            "Activity content stream does not match its activity kind",
        ));
    }
    Ok(())
}
