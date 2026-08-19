use crate::failure::failure;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, OperationContent, RuntimeFailure, TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(super) struct QoderHeadlessActivityProjection {
    operation_id: ActivityOperationId,
    assistant: Option<OpenActivity>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
}

impl QoderHeadlessActivityProjection {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            assistant: None,
            next_id: 0,
        }
    }

    pub(super) fn text_delta(
        &mut self,
        text: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = Vec::new();
        let activity = match self.assistant.clone() {
            Some(activity) => activity,
            None => {
                let activity = self.open()?;
                observations.push(self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?);
                self.assistant = Some(activity.clone());
                activity
            }
        };
        observations.push(self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(text)?),
        )?);
        Ok(observations)
    }

    pub(super) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(status) = terminal_activity_status(terminal) else {
            self.assistant = None;
            return Ok(Vec::new());
        };
        let mut observations = Vec::new();
        if let Some(activity) = self.assistant.take() {
            observations.push(self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                status,
                None,
            )?);
        }
        Ok(observations)
    }

    fn open(&mut self) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("qoder-headless:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            kind: ActivityKind::AssistantMessage,
            assistant_phase: Some(ActivityAssistantPhase::Final),
            disclosure: ActivityDisclosure::ProviderDisplayContent,
        })
    }

    fn observation(
        &self,
        activity: &OpenActivity,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let mut observation = ActivityObservation::new(
            activity.id.clone(),
            self.operation_id.clone(),
            activity.kind.clone(),
            phase,
            status,
            activity.assistant_phase,
            activity.disclosure,
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

fn display(text: &str) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let mut end = text.len().min(MAXIMUM_ACTIVITY_CONTENT_BYTES);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let content = OperationContent::new(&text[..end]).map_err(|_| activity_drift())?;
    let content = ActivityContent::new(content, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(
        ActivityContentChangeKind::Delta,
        ActivityContentStream::FinalAnswerText,
        content,
    ))
}

fn terminal_activity_status(status: &TerminalStatus) -> Option<ActivityStatus> {
    match status {
        TerminalStatus::Detached => None,
        TerminalStatus::Completed => Some(ActivityStatus::Completed),
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => Some(ActivityStatus::Cancelled),
        TerminalStatus::ProviderRequestObserved(_)
        | TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => Some(ActivityStatus::Failed),
    }
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.qoder.headless.activity_invalid",
        "Qoder headless activity did not match the qualified stream",
    )
}
