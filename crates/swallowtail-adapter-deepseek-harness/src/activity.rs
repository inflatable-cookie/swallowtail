use crate::failure::failure;
use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct DeepSeekHarnessActivityProjection {
    operation_id: ActivityOperationId,
    assistant: bool,
    tools: BTreeMap<String, OpenTool>,
    next_id: u64,
}

#[derive(Clone)]
struct OpenTool {
    activity_id: ActivityId,
    provider_ref: ProviderActivityRef,
    label: Option<ActivityLabel>,
}

impl DeepSeekHarnessActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            assistant: false,
            tools: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn assistant_delta(
        &mut self,
        text: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = Vec::new();
        if !self.assistant {
            self.assistant = true;
            observations.push(self.assistant_observation(
                ActivityLifecyclePhase::Started,
                ActivityStatus::InProgress,
                None,
            )?);
        }
        observations.push(self.assistant_observation(
            ActivityLifecyclePhase::Updated,
            ActivityStatus::InProgress,
            Some(display(text, ActivityContentChangeKind::Delta)?),
        )?);
        Ok(observations)
    }

    pub(crate) fn tool_started(
        &mut self,
        call_id: &str,
        name: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        if self.tools.contains_key(call_id) {
            return Err(activity_drift());
        }
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let activity_id = ActivityId::new(format!("deepseek-harness:tool:{}", self.next_id))
            .map_err(|_| activity_drift())?;
        let provider_ref =
            ProviderActivityRef::new(call_id.to_owned()).map_err(|_| activity_drift())?;
        let label = ActivityLabel::new(name.trim()).ok();
        self.tools.insert(
            call_id.to_owned(),
            OpenTool {
                activity_id: activity_id.clone(),
                provider_ref: provider_ref.clone(),
                label: label.clone(),
            },
        );
        observation(
            activity_id,
            self.operation_id.clone(),
            provider_ref,
            ActivityKind::ProviderOwnedTool,
            ActivityLifecyclePhase::Started,
            ActivityStatus::Pending,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            label,
            None,
        )
    }

    pub(crate) fn tool_finished(
        &mut self,
        call_id: &str,
        status: ActivityStatus,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        let tool = self.tools.remove(call_id).ok_or_else(activity_drift)?;
        observation(
            tool.activity_id,
            self.operation_id.clone(),
            tool.provider_ref,
            ActivityKind::ProviderOwnedTool,
            ActivityLifecyclePhase::Completed,
            status,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            tool.label,
            None,
        )
    }

    pub(crate) fn unknown(
        &mut self,
        event_type: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let namespace = ActivityNamespace::new(format!("deepseek-harness.event.{event_type}"))
            .or_else(|_| ActivityNamespace::new("deepseek-harness.event.unknown"))
            .map_err(|_| activity_drift())?;
        observation(
            ActivityId::new(format!("deepseek-harness:unknown:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ProviderActivityRef::new(event_type.to_owned()).map_err(|_| activity_drift())?,
            ActivityKind::Unknown(namespace),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
            None,
            None,
        )
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let status = match terminal {
            TerminalStatus::Completed => ActivityStatus::Completed,
            TerminalStatus::Cancelled => ActivityStatus::Cancelled,
            TerminalStatus::ProviderFailed(_)
            | TerminalStatus::HostFailed(_)
            | TerminalStatus::RuntimeFailed(_)
            | TerminalStatus::TimedOut
            | TerminalStatus::Detached
            | TerminalStatus::ProviderRequestObserved(_) => ActivityStatus::Failed,
        };
        let mut observations = Vec::new();
        if self.assistant {
            self.assistant = false;
            observations.push(self.assistant_observation(
                ActivityLifecyclePhase::Completed,
                status,
                None,
            )?);
        }
        for (_, tool) in std::mem::take(&mut self.tools) {
            observations.push(observation(
                tool.activity_id,
                self.operation_id.clone(),
                tool.provider_ref,
                ActivityKind::ProviderOwnedTool,
                ActivityLifecyclePhase::Completed,
                status,
                None,
                ActivityDisclosure::ProviderDisplayContent,
                tool.label,
                None,
            )?);
        }
        Ok(observations)
    }

    fn assistant_observation(
        &self,
        phase: ActivityLifecyclePhase,
        status: ActivityStatus,
        content: Option<ActivityContentUpdate>,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        observation(
            ActivityId::new("deepseek-harness:assistant").map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ProviderActivityRef::new("assistant").map_err(|_| activity_drift())?,
            ActivityKind::AssistantMessage,
            phase,
            status,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
            None,
            content,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn observation(
    activity_id: ActivityId,
    operation_id: ActivityOperationId,
    provider_ref: ProviderActivityRef,
    kind: ActivityKind,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
    assistant_phase: Option<ActivityAssistantPhase>,
    disclosure: ActivityDisclosure,
    label: Option<ActivityLabel>,
    content: Option<ActivityContentUpdate>,
) -> Result<ActivityObservation, RuntimeFailure> {
    let mut observation = ActivityObservation::new(
        activity_id,
        operation_id,
        kind,
        phase,
        status,
        assistant_phase,
        disclosure,
    )
    .map_err(|_| activity_drift())?
    .with_provider_activity_ref(provider_ref);
    if let Some(label) = label {
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

fn display(
    text: &str,
    change: ActivityContentChangeKind,
) -> Result<ActivityContentUpdate, RuntimeFailure> {
    let mut end = text.len().min(MAXIMUM_ACTIVITY_CONTENT_BYTES);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let content = OperationContent::new(&text[..end]).map_err(|_| activity_drift())?;
    let content = ActivityContent::new(content, MAXIMUM_ACTIVITY_CONTENT_BYTES)
        .map_err(|_| activity_drift())?;
    Ok(ActivityContentUpdate::new(
        change,
        ActivityContentStream::FinalAnswerText,
        content,
    ))
}

fn activity_drift() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.activity_invalid",
        "DeepSeek Harness activity did not match the qualified JSON-RPC stream",
    )
}
