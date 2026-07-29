use crate::failure::malformed;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_protocol_acp::{AcpMessageChunk, AcpMessageRole, AcpPlanEntry, AcpSessionUpdate};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentChangeKind, ActivityContentUpdate, ActivityId,
    ActivityKind, ActivityLifecyclePhase, ActivityNamespace, ActivityObservation,
    ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeTurnId, TerminalStatus,
};

mod content;
mod tool;

use content::{content_update, terminal_status, text_content};

pub(crate) struct AcpActivityProjection {
    operation_id: ActivityOperationId,
    open: BTreeMap<String, OpenActivity>,
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
}

impl AcpActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            open: BTreeMap::new(),
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
            AcpSessionUpdate::Plan(entries) => self.plan(entries),
            AcpSessionUpdate::Unknown { namespace } => self.unknown(namespace.as_str()),
            AcpSessionUpdate::AvailableCommands(_)
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
        let status = terminal_status(terminal);
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
        let (kind, assistant_phase, stream, label) = match message.role {
            AcpMessageRole::Agent => (
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Final),
                ActivityContentStream::FinalAnswerText,
                "assistant",
            ),
            AcpMessageRole::Thought => (
                ActivityKind::ReasoningSummary,
                None,
                ActivityContentStream::ReasoningSummaryText,
                "thought",
            ),
            AcpMessageRole::User => unreachable!("user messages returned above"),
        };
        let provider_id = message.message_id.as_ref().map(|value| value.as_str());
        let key = format!("{label}:{}", provider_id.unwrap_or("turn"));
        let activity = self.open_or_insert(
            &key,
            provider_id,
            kind,
            assistant_phase,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityStatus::InProgress,
        )?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            content_update(
                text_content(&message.content)?,
                ActivityContentChangeKind::Delta,
                stream,
            )?,
        )?])
    }

    fn plan(
        &mut self,
        entries: &[AcpPlanEntry],
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open_or_insert(
            "plan",
            None,
            ActivityKind::Plan,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityStatus::InProgress,
        )?;
        let display = entries
            .iter()
            .map(|entry| entry.content.as_str())
            .collect::<Vec<_>>()
            .join("\n");
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            content_update(
                &display,
                ActivityContentChangeKind::ReplacementSnapshot,
                ActivityContentStream::PlanText,
            )?,
        )?])
    }

    fn unknown(&mut self, namespace: &str) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.next_id = self.next_id.saturating_add(1);
        let activity = OpenActivity {
            id: ActivityId::new(format!("kimi-code-acp:unknown:{}", self.next_id))
                .map_err(|_| malformed())?,
            provider_ref: None,
            kind: ActivityKind::Unknown(
                ActivityNamespace::new(format!("kimi-code.acp.{namespace}"))
                    .map_err(|_| malformed())?,
            ),
            assistant_phase: None,
            disclosure: ActivityDisclosure::IdentityAndLifecycleOnly,
            status: ActivityStatus::Completed,
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
        let id = ActivityId::new(format!("kimi-code-acp:{key}"))
            .or_else(|_| {
                self.next_id = self.next_id.saturating_add(1);
                ActivityId::new(format!("kimi-code-acp:minted:{}", self.next_id))
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
        if let Some(content) = content {
            observation = observation.with_content(content).map_err(|_| malformed())?;
        }
        Ok(observation)
    }
}

#[cfg(test)]
mod tests;
