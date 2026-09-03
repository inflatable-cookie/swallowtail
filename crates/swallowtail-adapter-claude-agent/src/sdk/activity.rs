//! Bounded activity projection for the Claude Agent SDK sidecar route.
//!
//! Only identity and lifecycle are disclosed. Tool inputs, tool results,
//! provider payloads, and paths never reach an observation.

use super::failure::failure;
use super::wire::ClaudeAgentSdkEvent;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase,
    ActivityObservation, ActivityOperationId, ActivityStatus, RuntimeFailure, RuntimeTurnId,
};

const MAXIMUM_OPEN_TOOLS: usize = 32;

pub(crate) struct SdkActivityProjection {
    operation_id: ActivityOperationId,
    message: Option<OpenActivity>,
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

impl SdkActivityProjection {
    pub(crate) fn new(turn_id: RuntimeTurnId) -> Self {
        Self {
            operation_id: ActivityOperationId::Turn(turn_id),
            message: None,
            tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        event: &ClaudeAgentSdkEvent,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match event {
            ClaudeAgentSdkEvent::OutputDelta(_) => {
                let (activity, phase) = match self.message.clone() {
                    Some(activity) => (activity, ActivityLifecyclePhase::Updated),
                    None => {
                        let activity = self.open(
                            "message",
                            None,
                            ActivityKind::AssistantMessage,
                            Some(ActivityAssistantPhase::ProviderUnspecified),
                            ActivityDisclosure::IdentityAndLifecycleOnly,
                        )?;
                        self.message = Some(activity.clone());
                        (activity, ActivityLifecyclePhase::Started)
                    }
                };
                Ok(vec![self.observation(
                    &activity,
                    phase,
                    ActivityStatus::InProgress,
                )?])
            }
            ClaudeAgentSdkEvent::ToolStarted { call_id, name } => {
                if self.tools.contains_key(call_id) || self.tools.len() >= MAXIMUM_OPEN_TOOLS {
                    return Err(activity_drift());
                }
                // The provider-intended tool name is display content; tool
                // inputs and results never leave the sidecar.
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
                )?])
            }
            ClaudeAgentSdkEvent::ToolEnded { call_id, failed } => {
                let activity = self.tools.remove(call_id).ok_or_else(activity_drift)?;
                Ok(vec![self.observation(
                    &activity,
                    ActivityLifecyclePhase::Completed,
                    if *failed {
                        ActivityStatus::Failed
                    } else {
                        ActivityStatus::Completed
                    },
                )?])
            }
            ClaudeAgentSdkEvent::TurnStarted
            | ClaudeAgentSdkEvent::Progress
            | ClaudeAgentSdkEvent::TurnEnded { .. }
            | ClaudeAgentSdkEvent::TurnFailed => Ok(Vec::new()),
        }
    }

    pub(crate) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut open = Vec::new();
        if let Some(activity) = self.message.take() {
            open.push(activity);
        }
        open.extend(std::mem::take(&mut self.tools).into_values());
        open.into_iter()
            .map(|activity| self.observation(&activity, ActivityLifecyclePhase::Completed, status))
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
            id: ActivityId::new(format!("claude-agent-sdk:{label}:{}", self.next_id))
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
        Ok(observation)
    }
}

fn activity_label(name: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(name).ok()
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.activity_drift",
        "Claude Agent SDK sidecar activity stream did not match the qualified sequence",
    )
}
