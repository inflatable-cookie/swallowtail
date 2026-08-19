use crate::failure::failure;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure, TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(super) struct VibeHeadlessActivityProjection {
    operation_id: ActivityOperationId,
    assistant: Option<OpenActivity>,
    thought: Option<OpenActivity>,
    tools: BTreeMap<String, OpenActivity>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    label: Option<ActivityLabel>,
}

impl VibeHeadlessActivityProjection {
    pub(super) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            assistant: None,
            thought: None,
            tools: BTreeMap::new(),
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
                let activity = self.open(
                    None,
                    ActivityKind::AssistantMessage,
                    Some(ActivityAssistantPhase::Final),
                    ActivityDisclosure::ProviderDisplayContent,
                    None,
                )?;
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
            Some(display(
                text,
                ActivityContentStream::FinalAnswerText,
                ActivityContentChangeKind::Delta,
            )?),
        )?);
        Ok(observations)
    }

    pub(super) fn thought_delta(
        &mut self,
        text: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = Vec::new();
        let activity = match self.thought.clone() {
            Some(activity) => activity,
            None => {
                let activity = self.open(
                    None,
                    ActivityKind::ReasoningSummary,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                    None,
                )?;
                observations.push(self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?);
                self.thought = Some(activity.clone());
                activity
            }
        };
        observations.push(self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(
                text,
                ActivityContentStream::ReasoningSummaryText,
                ActivityContentChangeKind::Delta,
            )?),
        )?);
        Ok(observations)
    }

    pub(super) fn tool_completed(
        &mut self,
        tool_name: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open(
            Some(tool_name),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityLabel::new(tool_name.trim()).ok(),
        )?;
        Ok(vec![
            self.observation(
                &activity,
                ActivityLifecyclePhase::Started,
                ActivityStatus::Pending,
                None,
            )?,
            self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
                None,
            )?,
        ])
    }

    pub(super) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(status) = terminal_activity_status(terminal) else {
            self.assistant = None;
            self.thought = None;
            self.tools.clear();
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
        if let Some(activity) = self.thought.take() {
            observations.push(self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                status,
                None,
            )?);
        }
        for activity in std::mem::take(&mut self.tools).into_values() {
            observations.push(self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                status,
                None,
            )?);
        }
        Ok(observations)
    }

    fn open(
        &mut self,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        label: Option<ActivityLabel>,
    ) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("mistral-vibe-headless:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: provider_ref
                .map(ProviderActivityRef::new)
                .transpose()
                .map_err(|_| activity_drift())?,
            kind,
            assistant_phase,
            disclosure,
            label,
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
        if let Some(reference) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(reference);
        }
        if let Some(label) = activity.label.clone() {
            observation = observation
                .with_label(label)
                .map_err(|_| activity_drift())?;
        }
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(observation)
    }
}

fn display(
    text: &str,
    stream: ActivityContentStream,
    change: ActivityContentChangeKind,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let mut end = text.len().min(MAXIMUM_ACTIVITY_CONTENT_BYTES);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let content = OperationContent::new(&text[..end]).map_err(|_| activity_drift())?;
    let content = ActivityContent::new(content, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, content))
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
        "swallowtail.mistral-vibe.headless.activity_invalid",
        "Mistral Vibe headless activity did not match the qualified stream",
    )
}
