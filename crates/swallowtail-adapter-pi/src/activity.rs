use crate::failure::failure;
use crate::protocol::PiAgentEvent;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    RuntimeTurnId,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct PiActivityProjection {
    operation_id: ActivityOperationId,
    message: Option<OpenActivity>,
    reasoning: Option<OpenActivity>,
    tools: BTreeMap<String, OpenActivity>,
    compaction: Option<OpenActivity>,
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

impl PiActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            message: None,
            reasoning: None,
            tools: BTreeMap::new(),
            compaction: None,
            next_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        event: &PiAgentEvent,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match event {
            PiAgentEvent::MessageStarted => {
                let activity = self.open(
                    "message",
                    None,
                    ActivityKind::AssistantMessage,
                    Some(ActivityAssistantPhase::ProviderUnspecified),
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )?;
                self.message = Some(activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            PiAgentEvent::OutputDelta(_) => {
                let activity = match self.message.clone() {
                    Some(activity) => activity,
                    None => {
                        let activity = self.open(
                            "message",
                            None,
                            ActivityKind::AssistantMessage,
                            Some(ActivityAssistantPhase::ProviderUnspecified),
                            ActivityDisclosure::IdentityAndLifecycleOnly,
                        )?;
                        self.message = Some(activity.clone());
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
            PiAgentEvent::MessageEnded(_) => {
                let mut observations = Vec::new();
                if let Some(reasoning) = self.reasoning.take() {
                    observations.push(self.observation(
                        &reasoning,
                        ActivityLifecyclePhase::Completed,
                        ActivityStatus::Completed,
                        None,
                    )?);
                }
                if let Some(message) = self.message.take() {
                    observations.push(self.observation(
                        &message,
                        ActivityLifecyclePhase::Completed,
                        ActivityStatus::Completed,
                        None,
                    )?);
                }
                Ok(observations)
            }
            PiAgentEvent::ReasoningStarted => {
                let activity = self.open(
                    "reasoning",
                    None,
                    ActivityKind::ReasoningSummary,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                )?;
                self.reasoning = Some(activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            PiAgentEvent::ReasoningDelta(delta) => {
                let activity = match self.reasoning.clone() {
                    Some(activity) => activity,
                    None => {
                        let activity = self.open(
                            "reasoning",
                            None,
                            ActivityKind::ReasoningSummary,
                            None,
                            ActivityDisclosure::ProviderDisplayContent,
                        )?;
                        self.reasoning = Some(activity.clone());
                        activity
                    }
                };
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Updated,
                    ActivityStatus::InProgress,
                    Some(content(
                        delta,
                        ActivityContentChangeKind::Delta,
                        ActivityContentStream::ReasoningSummaryText,
                    )?),
                )?])
            }
            PiAgentEvent::ReasoningEnded => {
                let activity = self.reasoning.take().ok_or_else(activity_drift)?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                )?])
            }
            PiAgentEvent::ToolStarted { call_id, name } => {
                if self.tools.contains_key(call_id) {
                    return Err(activity_drift());
                }
                let mut activity = self.open(
                    "tool",
                    Some(call_id),
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                )?;
                activity.label = activity_label(name);
                self.tools.insert(call_id.clone(), activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            PiAgentEvent::ToolUpdated { call_id, name } => {
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
            PiAgentEvent::ToolEnded {
                call_id,
                name,
                failed,
            } => {
                let mut activity = self.tools.remove(call_id).ok_or_else(activity_drift)?;
                if let Some(label) = activity_label(name) {
                    activity.label = Some(label);
                }
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    if *failed {
                        ActivityStatus::Failed
                    } else {
                        ActivityStatus::Completed
                    },
                    None,
                )?])
            }
            PiAgentEvent::CompactionStarted => {
                if self.compaction.is_some() {
                    return Err(activity_drift());
                }
                let activity = self.open(
                    "compaction",
                    None,
                    ActivityKind::ContextCompaction,
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )?;
                self.compaction = Some(activity.clone());
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Started,
                    ActivityStatus::InProgress,
                    None,
                )?])
            }
            PiAgentEvent::CompactionEnded => {
                let activity = self.compaction.take().ok_or_else(activity_drift)?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                )?])
            }
            PiAgentEvent::Unknown(namespace) => {
                let activity = self.open(
                    "unknown",
                    None,
                    ActivityKind::Unknown(
                        ActivityNamespace::new(format!("pi.rpc.{namespace}"))
                            .map_err(|_| activity_drift())?,
                    ),
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
            PiAgentEvent::RetryObserved => {
                let activity = self.open(
                    "unknown",
                    None,
                    ActivityKind::Unknown(
                        ActivityNamespace::new("pi.rpc.retry").map_err(|_| activity_drift())?,
                    ),
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
            PiAgentEvent::Started
            | PiAgentEvent::Settled
            | PiAgentEvent::Progress
            | PiAgentEvent::ProviderFailed => Ok(Vec::new()),
        }
    }

    pub(crate) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut open = Vec::new();
        if let Some(activity) = self.reasoning.take() {
            open.push(activity);
        }
        if let Some(activity) = self.message.take() {
            open.push(activity);
        }
        open.extend(std::mem::take(&mut self.tools).into_values());
        if let Some(activity) = self.compaction.take() {
            open.push(activity);
        }
        open.into_iter()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
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
            id: ActivityId::new(format!("pi-rpc:{label}:{}", self.next_id))
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
        "swallowtail.pi.rpc.activity_invalid",
        "Pi RPC observable activity did not match the qualified protocol",
    )
}

#[cfg(test)]
mod tests;
