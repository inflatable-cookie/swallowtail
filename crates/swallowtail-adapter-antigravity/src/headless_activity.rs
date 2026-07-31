use std::collections::BTreeMap;
use swallowtail_core::{ActivityContentStream, ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityContent, ActivityContentChangeKind, ActivityContentUpdate,
    ActivityId, ActivityKind, ActivityLabel, ActivityLifecyclePhase, ActivityNamespace,
    ActivityObservation, ActivityOperationId, ActivityStatus, OperationContent, RuntimeFailure,
    SubagentId, SubagentParent, SubagentSnapshot, SubagentStatus, TerminalStatus,
};

const MAXIMUM_ACTIVITY_CONTENT_BYTES: usize = 64 * 1024;

pub(crate) struct AntigravityActivityProjection {
    operation_id: ActivityOperationId,
    steps: BTreeMap<u64, OpenActivity>,
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

impl AntigravityActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            steps: BTreeMap::new(),
            next_id: 0,
        }
    }

    pub(crate) fn assistant(
        &mut self,
        step: u64,
        active: bool,
        text: Option<&str>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = self.ensure_open(
            step,
            ActivityKind::AssistantMessage,
            Some(ActivityAssistantPhase::Final),
            ActivityDisclosure::ProviderDisplayContent,
            None,
        )?;
        let activity = self.steps.get(&step).cloned().ok_or_else(activity_drift)?;
        if let Some(text) = text.filter(|value| !value.is_empty()) {
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
        }
        if !active {
            self.steps.remove(&step);
            observations.push(self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
                None,
            )?);
        }
        Ok(observations)
    }

    pub(crate) fn tool(
        &mut self,
        step: u64,
        active: bool,
        name: &str,
        failed: bool,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let mut observations = self.ensure_open(
            step,
            ActivityKind::ProviderOwnedTool,
            None,
            ActivityDisclosure::ProviderDisplayContent,
            ActivityLabel::new(name.trim()).ok(),
        )?;
        let activity = self.steps.get(&step).cloned().ok_or_else(activity_drift)?;
        if !active {
            self.steps.remove(&step);
            observations.push(self.observation(
                &activity,
                ActivityLifecyclePhase::Completed,
                if failed {
                    ActivityStatus::Failed
                } else {
                    ActivityStatus::Completed
                },
                None,
            )?);
        }
        Ok(observations)
    }

    pub(crate) fn subagents(
        &mut self,
        step: u64,
        children: Vec<(String, Option<String>)>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let activity_id =
            ActivityId::new(format!("antigravity-headless:subagents:{}", self.next_id))
                .map_err(|_| activity_drift())?;
        let snapshots = children
            .into_iter()
            .map(|(id, label)| {
                let mut snapshot = SubagentSnapshot::new(
                    SubagentId::new(id).map_err(|_| activity_drift())?,
                    SubagentParent::Operation,
                    SubagentStatus::Completed,
                );
                if let Some(label) = label.and_then(|value| ActivityLabel::new(value).ok()) {
                    snapshot = snapshot.with_label(label);
                }
                Ok(snapshot)
            })
            .collect::<Result<Vec<_>, RuntimeFailure>>()?;
        let observation = ActivityObservation::new(
            activity_id,
            self.operation_id.clone(),
            ActivityKind::SubagentOrCollaboration,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(provider_ref(step)?)
        .with_subagents(snapshots)
        .map_err(|_| activity_drift())?;
        Ok(vec![observation])
    }

    pub(crate) fn unknown(
        &mut self,
        step: u64,
        step_type: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let namespace = ActivityNamespace::new(format!("antigravity.headless.{step_type}"))
            .map_err(|_| activity_drift())?;
        let observation = ActivityObservation::new(
            ActivityId::new(format!("antigravity-headless:unknown:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::Unknown(namespace),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())?
        .with_provider_activity_ref(provider_ref(step)?);
        Ok(vec![observation])
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let status = terminal_activity_status(terminal);
        let activities = std::mem::take(&mut self.steps);
        activities
            .into_values()
            .map(|activity| {
                self.observation(&activity, ActivityLifecyclePhase::Completed, status, None)
            })
            .collect()
    }

    fn ensure_open(
        &mut self,
        step: u64,
        kind: ActivityKind,
        assistant_phase: Option<ActivityAssistantPhase>,
        disclosure: ActivityDisclosure,
        label: Option<ActivityLabel>,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        if let Some(existing) = self.steps.get(&step) {
            if existing.kind != kind {
                return Err(activity_drift());
            }
            return Ok(Vec::new());
        }
        self.next_id = self.next_id.checked_add(1).ok_or_else(activity_drift)?;
        let activity = OpenActivity {
            id: ActivityId::new(format!("antigravity-headless:step:{}", self.next_id))
                .map_err(|_| activity_drift())?,
            provider_ref: provider_ref(step)?,
            kind,
            assistant_phase,
            disclosure,
            label,
        };
        let observation = self.observation(
            &activity,
            ActivityLifecyclePhase::Started,
            ActivityStatus::InProgress,
            None,
        )?;
        self.steps.insert(step, activity);
        Ok(vec![observation])
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

fn provider_ref(step: u64) -> Result<ProviderActivityRef, RuntimeFailure> {
    ProviderActivityRef::new(format!("step:{step}")).map_err(|_| activity_drift())
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

fn terminal_activity_status(status: &TerminalStatus) -> ActivityStatus {
    match status {
        TerminalStatus::Completed => ActivityStatus::Completed,
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => ActivityStatus::Cancelled,
        TerminalStatus::ProviderRequestObserved(_)
        | TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => ActivityStatus::Failed,
    }
}

fn activity_drift() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.headless.activity_drift",
        "Antigravity headless activity could not be projected safely",
    )
}
