use std::collections::BTreeMap;
use swallowtail_core::{ActivityDisclosure, ProviderActivityRef};
use swallowtail_runtime::{
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityNamespace, ActivityObservation,
    ActivityOperationId, ActivityStatus, RuntimeFailure, TerminalStatus,
};

pub(crate) struct MuseActivityProjection {
    operation_id: ActivityOperationId,
    tasks: BTreeMap<String, ActivityId>,
    next_task: u64,
    next_unknown: u64,
}

impl MuseActivityProjection {
    pub(crate) fn new(operation_id: ActivityOperationId) -> Self {
        Self {
            operation_id,
            tasks: BTreeMap::new(),
            next_task: 0,
            next_unknown: 0,
        }
    }

    pub(crate) fn task(
        &mut self,
        task_id: &str,
        lifecycle: &str,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let terminal_status = match lifecycle {
            "completed" => Some(ActivityStatus::Completed),
            "failed" | "rejected" | "timed_out" => Some(ActivityStatus::Failed),
            "cancelled" => Some(ActivityStatus::Cancelled),
            _ => None,
        };
        let nonterminal_status = match lifecycle {
            "proposed" | "accepted" | "scheduled" => ActivityStatus::Pending,
            _ => ActivityStatus::InProgress,
        };
        let mut observations = Vec::new();
        let existing = self.tasks.get(task_id).cloned();
        let was_open = existing.is_some();
        let activity_id = match existing {
            Some(activity_id) => activity_id,
            None => {
                self.next_task = self.next_task.checked_add(1).ok_or_else(activity_drift)?;
                let activity_id = ActivityId::new(format!("muse-task:{}", self.next_task))
                    .map_err(|_| activity_drift())?;
                observations.push(observation(
                    activity_id.clone(),
                    self.operation_id.clone(),
                    task_id,
                    ActivityLifecyclePhase::Started,
                    if terminal_status.is_some() {
                        ActivityStatus::InProgress
                    } else {
                        nonterminal_status
                    },
                )?);
                self.tasks.insert(task_id.to_owned(), activity_id.clone());
                activity_id
            }
        };
        if let Some(status) = terminal_status {
            self.tasks.remove(task_id);
            observations.push(observation(
                activity_id,
                self.operation_id.clone(),
                task_id,
                ActivityLifecyclePhase::Completed,
                status,
            )?);
        } else if was_open {
            observations.push(observation(
                activity_id,
                self.operation_id.clone(),
                task_id,
                ActivityLifecyclePhase::Updated,
                nonterminal_status,
            )?);
        }
        Ok(observations)
    }

    pub(crate) fn unknown(
        &mut self,
        event_id: &str,
        payload_type: &str,
    ) -> Result<ActivityObservation, RuntimeFailure> {
        self.next_unknown = self
            .next_unknown
            .checked_add(1)
            .ok_or_else(activity_drift)?;
        let namespace = ActivityNamespace::new(format!("muse-code.headless.event.{payload_type}"))
            .or_else(|_| ActivityNamespace::new("muse-code.headless.event.unknown"))
            .map_err(|_| activity_drift())?;
        ActivityObservation::new(
            ActivityId::new(format!("muse-unknown:{}", self.next_unknown))
                .map_err(|_| activity_drift())?,
            self.operation_id.clone(),
            ActivityKind::Unknown(namespace),
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            None,
            ActivityDisclosure::IdentityAndLifecycleOnly,
        )
        .map_err(|_| activity_drift())
        .and_then(|observation| {
            ProviderActivityRef::new(event_id)
                .map(|reference| observation.with_provider_activity_ref(reference))
                .map_err(|_| activity_drift())
        })
    }

    pub(crate) fn complete(
        &mut self,
        terminal: &TerminalStatus,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(status) = terminal_activity_status(terminal) else {
            self.tasks.clear();
            return Ok(Vec::new());
        };
        let tasks = std::mem::take(&mut self.tasks);
        tasks
            .into_iter()
            .map(|(provider_ref, activity_id)| {
                observation(
                    activity_id,
                    self.operation_id.clone(),
                    &provider_ref,
                    ActivityLifecyclePhase::Completed,
                    status,
                )
            })
            .collect()
    }
}

fn observation(
    activity_id: ActivityId,
    operation_id: ActivityOperationId,
    provider_ref: &str,
    phase: ActivityLifecyclePhase,
    status: ActivityStatus,
) -> Result<ActivityObservation, RuntimeFailure> {
    let observation = ActivityObservation::new(
        activity_id,
        operation_id,
        ActivityKind::Task,
        phase,
        status,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .map_err(|_| activity_drift())?;
    ProviderActivityRef::new(provider_ref)
        .map(|reference| observation.with_provider_activity_ref(reference))
        .map_err(|_| activity_drift())
}

fn terminal_activity_status(terminal: &TerminalStatus) -> Option<ActivityStatus> {
    match terminal {
        TerminalStatus::Completed => Some(ActivityStatus::Completed),
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => Some(ActivityStatus::Cancelled),
        TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_) => Some(ActivityStatus::Failed),
        TerminalStatus::Detached | TerminalStatus::ProviderRequestObserved(_) => None,
    }
}

fn activity_drift() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.muse_code.headless.activity_projection_failed",
        "Muse Code activity could not be projected safely",
    )
}
