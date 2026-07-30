use crate::failure::failure;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct GeminiHeadlessActivityProjection {
    operation_id: ActivityOperationId,
    assistant: Option<OpenActivity>,
    pending_tools: BTreeMap<String, Option<ActivityLabel>>,
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

impl GeminiHeadlessActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            assistant: None,
            pending_tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn assistant_delta(
        &mut self,
        delta: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = match self.assistant.clone() {
            Some(activity) => activity,
            None => {
                let activity = self.open(
                    "assistant",
                    None,
                    ActivityKind::AssistantMessage,
                    Some(ActivityAssistantPhase::Final),
                    ActivityDisclosure::ProviderDisplayContent,
                )?;
                self.assistant = Some(activity.clone());
                activity
            }
        };
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(
                delta,
                ActivityContentStream::FinalAnswerText,
                ActivityContentChangeKind::Delta,
            )?),
        )?])
    }

    pub(crate) fn tool_use(
        &mut self,
        tool_id: &str,
        name: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let label = activity_label(name);
        if self
            .pending_tools
            .insert(tool_id.to_owned(), label.clone())
            .is_some()
        {
            return Err(activity_drift());
        }
        self.completed_tool("tool-use", tool_id, ActivityStatus::Completed, label)
    }

    pub(crate) fn tool_result(
        &mut self,
        tool_id: &str,
        failed: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let label = self
            .pending_tools
            .remove(tool_id)
            .ok_or_else(activity_drift)?;
        self.completed_tool(
            "tool-result",
            tool_id,
            if failed {
                ActivityStatus::Failed
            } else {
                ActivityStatus::Completed
            },
            label,
        )
    }

    pub(crate) fn warning(&mut self) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.milestone(ActivityKind::WarningOrError, "warning", None)
    }

    pub(crate) fn unknown(
        &mut self,
        event_type: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.milestone(
            ActivityKind::Unknown(namespace(&format!("gemini.headless.{event_type}"))?),
            "unknown",
            None,
        )
    }

    pub(crate) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if !self.pending_tools.is_empty() {
            return Err(activity_drift());
        }
        let Some(activity) = self.assistant.take() else {
            return Ok(Vec::new());
        };
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            status,
            None,
        )?])
    }

    fn completed_tool(
        &mut self,
        label: &str,
        tool_id: &str,
        status: ActivityStatus,
        activity_label: Option<ActivityLabel>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut activity = self.open(
            label,
            Some(tool_id),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
        )?;
        activity.label = activity_label;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            status,
            None,
        )?])
    }

    fn milestone(
        &mut self,
        kind: ActivityKind,
        label: &str,
        provider_ref: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open(
            label,
            provider_ref,
            kind,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
        )?])
    }

    fn open(
        &mut self,
        label: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("gemini-headless:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: provider_ref
                .map(ProviderActivityRef::new)
                .transpose()
                .map_err(|_| activity_drift())?,
            kind,
            assistant_phase,
            disclosure,
            label: None,
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
        if let Some(provider_ref) = activity.provider_ref.clone() {
            observation = observation.with_provider_activity_ref(provider_ref);
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

fn namespace(value: &str) -> Result<ActivityNamespace, RuntimeFailure> {
    ActivityNamespace::new(value).map_err(|_| activity_drift())
}

fn activity_label(value: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(value.trim()).ok()
}

fn display(
    value: &str,
    stream: ActivityContentStream,
    change: ActivityContentChangeKind,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let value = OperationContent::new(value.to_owned()).map_err(|_| activity_drift())?;
    let value = ActivityContent::new(value, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, value))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.headless.activity_invalid",
        "Gemini headless activity did not match the qualified stream",
    )
}

#[cfg(test)]
mod tests;
