use crate::failure::failure;
use crate::protocol::{Event, ToolStatus};
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    RuntimeTurnId,
};

pub(crate) mod profile;

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct OpenCodeActivityProjection {
    operation_id: ActivityOperationId,
    assistant: BTreeMap<String, OpenActivity>,
    reasoning: BTreeMap<String, OpenActivity>,
    tools: BTreeMap<String, OpenActivity>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenActivity {
    id: ActivityId,
    provider_ref: ProviderActivityRef,
    kind: ActivityKind,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    label: Option<ActivityLabel>,
}

impl OpenCodeActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            assistant: BTreeMap::new(),
            reasoning: BTreeMap::new(),
            tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        event: &Event,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match event {
            Event::OutputDelta { part_id, .. } | Event::OutputSnapshot { part_id, .. } => {
                let activity = match self.assistant.get(part_id).cloned() {
                    Some(activity) => activity,
                    None => {
                        let activity = self.open(
                            "assistant",
                            part_id,
                            ActivityKind::AssistantMessage,
                            Some(ActivityAssistantPhase::ProviderUnspecified),
                            ActivityDisclosure::IdentityAndLifecycleOnly,
                        )?;
                        self.assistant.insert(part_id.clone(), activity.clone());
                        activity
                    }
                };
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            Event::ReasoningSnapshot { part_id, text, .. } => {
                let activity = match self.reasoning.get(part_id).cloned() {
                    Some(activity) => activity,
                    None => {
                        let activity = self.open(
                            "reasoning",
                            part_id,
                            ActivityKind::ReasoningSummary,
                            None,
                            ActivityDisclosure::ProviderDisplayContent,
                        )?;
                        self.reasoning.insert(part_id.clone(), activity.clone());
                        activity
                    }
                };
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    Some(content(
                        text,
                        ActivityContentChangeKind::ReplacementSnapshot,
                        ActivityContentStream::ReasoningSummaryText,
                    )?),
                )?])
            }
            Event::ToolState {
                part_id,
                call_id,
                name,
                status,
            } => self.tool(part_id, call_id, name, *status),
            Event::Usage(part_id, _) => self.milestone(
                part_id,
                ActivityKind::Task,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
            ),
            Event::Unknown(namespace_value) => self.milestone(
                namespace_value,
                ActivityKind::Unknown(namespace(&format!("opencode.http.{namespace_value}"))?),
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
            ),
            Event::Idle => self.complete(ActivityStatus::Completed),
            Event::ProviderFailed => self.milestone(
                "provider-failed",
                ActivityKind::WarningOrError,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
            ),
            Event::Connected
            | Event::Busy
            | Event::Cancelled
            | Event::ProviderRequest(_)
            | Event::Foreign => Ok(Vec::new()),
        }
    }

    pub(crate) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if status == ActivityStatus::Completed && !self.tools.is_empty() {
            return Err(activity_drift());
        }
        let mut open = Vec::new();
        open.extend(std::mem::take(&mut self.reasoning).into_values());
        open.extend(std::mem::take(&mut self.assistant).into_values());
        open.extend(std::mem::take(&mut self.tools).into_values());
        open.into_iter()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
    }

    fn tool(
        &mut self,
        part_id: &str,
        call_id: &str,
        name: &str,
        status: ToolStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match status {
            ToolStatus::Pending => {
                if self.tools.contains_key(call_id) {
                    return Err(activity_drift());
                }
                let mut activity = self.open(
                    "tool",
                    call_id,
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                )?;
                activity.label = activity_label(name);
                self.tools.insert(call_id.to_owned(), activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::Pending,
                    None,
                )?])
            }
            ToolStatus::Running => {
                let activity = self.tools.get_mut(call_id).ok_or_else(activity_drift)?;
                if let Some(label) = activity_label(name) {
                    activity.label = Some(label);
                }
                let activity = activity.clone();
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            ToolStatus::Completed | ToolStatus::Failed => {
                let mut activity = self.tools.remove(call_id).ok_or_else(activity_drift)?;
                if let Some(label) = activity_label(name) {
                    activity.label = Some(label);
                }
                if part_id.is_empty() {
                    return Err(activity_drift());
                }
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    if status == ToolStatus::Failed {
                        ActivityStatus::Failed
                    } else {
                        ActivityStatus::Completed
                    },
                    None,
                )?])
            }
        }
    }

    fn milestone(
        &mut self,
        provider_ref: &str,
        kind: ActivityKind,
        disclosure: ActivityDisclosure,
        content: Option<ActivityContentUpdate>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let activity = self.open("milestone", provider_ref, kind, None, disclosure)?;
        Ok(vec![self.observation(
            &activity,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            content,
        )?])
    }

    fn open(
        &mut self,
        label: &str,
        provider_ref: &str,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
    ) -> Result<OpenActivity, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        Ok(OpenActivity {
            id: ActivityId::new(format!("opencode:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: ProviderActivityRef::new(provider_ref).map_err(|_| activity_drift())?,
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
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(activity.provider_ref.clone());
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

fn content(
    value: &str,
    change: ActivityContentChangeKind,
    stream: ActivityContentStream,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let content = OperationContent::new(value.to_owned()).map_err(|_| activity_drift())?;
    let content = ActivityContent::new(content, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(change, stream, content))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.activity_invalid",
        "OpenCode observable activity did not match the qualified event stream",
    )
}

#[cfg(test)]
mod tests;
