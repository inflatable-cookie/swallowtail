use crate::failure::failure;
use crate::managed::{ManagedEvent, ManagedEventKind};
use std::collections::BTreeMap;
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContentUpdate, ActivityId, ActivityKind, ActivityLabel,
    ActivityLifecyclePhase, ActivityNamespace, ActivityObservation, ActivityOperationId,
    ActivityStatus, RuntimeFailure, RuntimeRunId,
};

pub(crate) mod profile;

pub(crate) struct ManagedActivityProjection {
    operation_id: ActivityOperationId,
    pending_tools: BTreeMap<String, String>,
    next_id: u64,
}

impl ManagedActivityProjection {
    pub(crate) fn new(run_id: RuntimeRunId) -> Self {
        Self {
            operation_id: ActivityOperationId::Run(run_id),
            pending_tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn project(
        &mut self,
        event: &ManagedEvent,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        match event.kind() {
            ManagedEventKind::Message(_) => self.completed(
                event.id(),
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::ProviderUnspecified),
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                ActivityStatus::Completed,
            ),
            ManagedEventKind::Thinking => self.completed(
                event.id(),
                ActivityKind::ReasoningSummary,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                ActivityStatus::Completed,
            ),
            ManagedEventKind::ProviderToolUse { tool_use_id, name } => {
                if self
                    .pending_tools
                    .insert(tool_use_id.clone(), name.clone())
                    .is_some()
                {
                    return Err(activity_drift());
                }
                Ok(Vec::new())
            }
            ManagedEventKind::ProviderToolResult {
                tool_use_id,
                failed,
            } => {
                let name = self
                    .pending_tools
                    .remove(tool_use_id)
                    .ok_or_else(activity_drift)?;
                let observations = self.completed(
                    tool_use_id,
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                    None,
                    if *failed {
                        ActivityStatus::Failed
                    } else {
                        ActivityStatus::Completed
                    },
                )?;
                with_label(observations, activity_label(&name))
            }
            ManagedEventKind::Running
            | ManagedEventKind::Rescheduled
            | ManagedEventKind::Observed
            | ManagedEventKind::Idle(_) => self.completed(
                event.id(),
                ActivityKind::Task,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                ActivityStatus::Completed,
            ),
            ManagedEventKind::ProviderError | ManagedEventKind::Terminated => self.completed(
                event.id(),
                ActivityKind::WarningOrError,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                ActivityStatus::Failed,
            ),
            ManagedEventKind::Unknown(namespace_value) => self.completed(
                event.id(),
                ActivityKind::Unknown(namespace(&format!("anthropic.managed.{namespace_value}"))?),
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
                None,
                ActivityStatus::Completed,
            ),
            ManagedEventKind::CustomToolUse { .. } => Ok(Vec::new()),
        }
    }

    pub(crate) fn complete(
        &mut self,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let pending = std::mem::take(&mut self.pending_tools);
        if status == ActivityStatus::Completed && !pending.is_empty() {
            return Err(activity_drift());
        }
        pending
            .into_iter()
            .map(|(tool_use_id, name)| {
                let observations = self.completed(
                    &tool_use_id,
                    ActivityKind::ProviderOwnedTool,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                    None,
                    status,
                )?;
                with_label(observations, activity_label(&name))
                    .and_then(|mut observations| observations.pop().ok_or_else(activity_drift))
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments)]
    fn completed(
        &mut self,
        provider_ref: &str,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        content: Option<ActivityContentUpdate>,
        status: ActivityStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let id = ActivityId::new(format!("anthropic-managed:activity:{}", self.next_id))
            .map_err(|_| activity_drift())?;
        let provider_ref = ProviderActivityRef::new(provider_ref).map_err(|_| activity_drift())?;
        let mut observation = ActivityObservation::new(
            id,
            self.operation_id.clone(),
            kind,
            ActivityLifecyclePhase::Completed,
            status,
            assistant_phase,
            disclosure,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(provider_ref);
        if let Some(content) = content {
            observation = observation
                .with_content(content)
                .map_err(|_| activity_drift())?;
        }
        Ok(vec![observation])
    }
}

fn namespace(value: &str) -> Result<ActivityNamespace, RuntimeFailure> {
    ActivityNamespace::new(value).map_err(|_| activity_drift())
}

fn activity_label(value: &str) -> Option<ActivityLabel> {
    ActivityLabel::new(value.trim()).ok()
}

fn with_label(
    mut observations: Vec<ActivityObservation>,
    label: Option<ActivityLabel>,
) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
    if let Some(label) = label
        && let Some(observation) = observations.first_mut()
    {
        *observation = observation
            .clone()
            .with_label(label)
            .map_err(|_| activity_drift())?;
    }
    Ok(observations)
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.activity_invalid",
        "Anthropic Managed Agents activity did not match authoritative persisted events",
    )
}

#[cfg(test)]
mod tests;
