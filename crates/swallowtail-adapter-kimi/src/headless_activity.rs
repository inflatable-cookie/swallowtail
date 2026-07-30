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

pub(crate) struct KimiHeadlessActivityProjection {
    operation_id: ActivityOperationId,
    pending_tools: BTreeMap<String, Option<ActivityLabel>>,
    next_id: u64,
}

impl KimiHeadlessActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            pending_tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn assistant(
        &mut self,
        content: Option<&str>,
        tool_calls: &[(String, String)],
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = Vec::new();
        if let Some(content) = content.filter(|content| !content.is_empty()) {
            observations.push(self.completed(
                "assistant",
                None,
                ActivityKind::AssistantMessage,
                Some(ActivityAssistantPhase::Intermediate),
                ActivityDisclosure::ProviderDisplayContent,
                Some(display(
                    content,
                    ActivityContentStream::IntermediateAssistantText,
                )?),
            )?);
        }
        for (tool_id, name) in tool_calls {
            let label = activity_label(name);
            if self
                .pending_tools
                .insert(tool_id.clone(), label.clone())
                .is_some()
            {
                return Err(activity_drift());
            }
            let observation = self.completed(
                "tool-use",
                Some(tool_id),
                ActivityKind::ProviderOwnedTool,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                None,
            )?;
            observations.push(with_label(observation, label)?);
        }
        Ok(observations)
    }

    pub(crate) fn tool_result(
        &mut self,
        tool_id: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let label = self
            .pending_tools
            .remove(tool_id)
            .ok_or_else(activity_drift)?;
        let observation = self.completed(
            "tool-result",
            Some(tool_id),
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            None,
        )?;
        Ok(vec![with_label(observation, label)?])
    }

    pub(crate) fn retry(&mut self) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.unknown("retry")
    }

    pub(crate) fn unknown(
        &mut self,
        event_type: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        Ok(vec![self.completed(
            "unknown",
            None,
            ActivityKind::Unknown(namespace(&format!("kimi-code.headless.{event_type}"))?),
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
            None,
        )?])
    }

    pub(crate) fn ensure_idle(&self) -> Result<(), RuntimeFailure> {
        if self.pending_tools.is_empty() {
            Ok(())
        } else {
            Err(activity_drift())
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn completed(
        &mut self,
        label: &str,
        provider_ref: Option<&str>,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let mut observation = ActivityObservation::new(
            ActivityId::new(format!("kimi-code-headless:{label}:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            kind,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            assistant_phase,
            disclosure,
        )
        .map_err(|_| activity_drift())?;
        if let Some(provider_ref) = provider_ref {
            observation = observation.with_provider_activity_ref(
                ProviderActivityRef::new(provider_ref).map_err(|_| activity_drift())?,
            );
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

fn with_label(
    mut observation: ActivityObservation,
    label: Option<ActivityLabel>,
) -> Result<ActivityObservation, RuntimeFailure> {
    if let Some(label) = label {
        observation = observation
            .with_label(label)
            .map_err(|_| activity_drift())?;
    }
    Ok(observation)
}

fn display(
    value: &str,
    stream: ActivityContentStream,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let value = OperationContent::new(value.to_owned()).map_err(|_| activity_drift())?;
    let value = ActivityContent::new(value, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(
        ActivityContentChangeKind::ReplacementSnapshot,
        stream,
        value,
    ))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.headless.activity_invalid",
        "Kimi Code headless activity did not match the qualified stream",
    )
}

#[cfg(test)]
mod tests;
