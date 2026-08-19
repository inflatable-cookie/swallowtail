use crate::failure::malformed;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_protocol_acp::{AcpMessageChunk, AcpMessageRole, AcpSessionUpdate};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentChangeKind, ActivityContentUpdate, ActivityId,
    ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace, ActivityObservation,
    ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeTurnId, TerminalStatus,
};

mod content;
mod tool;

use content::{content_update, terminal_status, text_content};

pub(crate) struct AcpActivityProjection {
    operation_id: ActivityOperationId,
    open: BTreeMap<String, OpenActivity>,
    closed: BTreeSet<String>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: Option<ProviderActivityRef>,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    status: ActivityStatus,
    label: Option<ActivityLabel>,
}

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            open: BTreeMap::new(),
            closed: BTreeSet::new(),
            next_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        update: &AcpSessionUpdate,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match update {
            AcpSessionUpdate::Message(message) => self.message(message),
            AcpSessionUpdate::ToolCall(call) => self.tool_start(call),
            AcpSessionUpdate::ToolCallUpdate(update) => self.tool_update(update),
            AcpSessionUpdate::Unknown { namespace } => self.unknown(namespace.as_str()),
            AcpSessionUpdate::Plan(_)
            | AcpSessionUpdate::AvailableCommands(_)
            | AcpSessionUpdate::CurrentMode(_)
            | AcpSessionUpdate::ConfigOptions(_)
            | AcpSessionUpdate::SessionInfo { .. }
            | AcpSessionUpdate::Usage(_) => Ok(Vec::new()),
        }
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(status) = terminal_status(terminal) else {
            return Ok(Vec::new());
        };
        std::mem::take(&mut self.open)
            .into_values()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
    }

    fn message(
        &mut self,
        message: &AcpMessageChunk,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if message.role == AcpMessageRole::User {
            return Ok(Vec::new());
        }
        let text = text_content(&message.content)?;
        let provider_id = message.message_id.as_ref().map(|value| value.as_str());
        let (kind, phase, disclosure, stream, label) = match message.role {
            AcpMessageRole::Agent if text.starts_with("[MODE_UPDATE]") => (
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::ProviderUnspecified),
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                "mode-display",
            ),
            AcpMessageRole::Agent => (
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Final),
                ActivityDisclosure::ProviderDisplayContent,
                Some(ActivityContentStream::FinalAnswerText),
                "assistant",
            ),
            AcpMessageRole::Thought if text.starts_with("Warning:") => (
                ActivityKind::WarningOrError,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                "warning",
            ),
            AcpMessageRole::Thought => (
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                Some(ActivityContentStream::ReasoningSummaryText),
                "thought",
            ),
            AcpMessageRole::User => unreachable!("user messages returned above"),
        };
        let key = format!("{label}:{}", provider_id.unwrap_or("turn"));
        let activity = self.open_or_insert(
            &key,
            provider_id,
            kind,
            phase,
            disclosure,
            ActivityStatus::InProgress,
        )?;
        let content = stream
            .map(|stream| content_update(text, ActivityContentChangeKind::Delta, stream))
            .transpose()?
            .flatten();
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            content,
        )?])
    }

    fn unknown(&mut self, namespace: &str) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.next_id = self.next_id.saturating_add(1);
        let activity = OpenActivity {
            id: ActivityId::new(format!("cline-acp:unknown:{}", self.next_id))
                .map_err(|_| malformed())?,
            provider_ref: None,
            kind: ActivityKind::Unknown(
                ActivityNamespace::new(format!("cline.acp.{namespace}"))
                    .map_err(|_| malformed())?,
            ),
            assistant_phase: None,
            disclosure: ActivityDisclosure::IdentityAndLifecycleOnly,
            status: ActivityStatus::Completed,
            label: None,
        };
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    fn open_or_insert(
        &mut self,
        key: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        status: ActivityStatus,
    ) -> Result<OpenActivity, RuntimeFailure> {
        if let Some(activity) = self.open.get(key) {
            // Portable identity is fixed by the first emitted observation.
            return Ok(activity.clone());
        }
        let id = ActivityId::new(format!("cline-acp:{key}"))
            .or_else(|_| {
                self.next_id = self.next_id.saturating_add(1);
                ActivityId::new(format!("cline-acp:minted:{}", self.next_id))
            })
            .map_err(|_| malformed())?;
        let provider_ref = provider_ref
            .map(ProviderActivityRef::new)
            .transpose()
            .map_err(|_| malformed())?;
        let activity = OpenActivity {
            id,
            provider_ref,
            kind,
            assistant_phase,
            disclosure,
            status,
            label: None,
        };
        self.open.insert(key.to_owned(), activity.clone());
        Ok(activity)
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
        .map_err(|_| malformed())?;
        if let Some(reference) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(reference);
        }
        if let Some(label) = activity.label.clone() {
            observation = observation.with_label(label).map_err(|_| malformed())?;
        }
        if let Some(content) = content {
            observation = observation.with_content(content).map_err(|_| malformed())?;
        }
        Ok(observation)
    }
}

#[cfg(test)]
mod tests;
