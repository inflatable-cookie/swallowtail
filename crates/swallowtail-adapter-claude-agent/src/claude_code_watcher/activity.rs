use std::collections::BTreeSet;
use swallowtail_core::{ActivityDisclosure, WatcherOwningTurn, WatcherTerminalCause};
use swallowtail_runtime::{
    ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation, ActivityOperationId,
    ActivityStatus, HostServices, RuntimeFailure, RuntimeTurnId, WatcherSnapshot,
};

use crate::failure::failure;

/// Polls host watcher snapshots and projects terminal HostWatcher activity once.
pub(crate) struct WatcherActivityFeed {
    turn: RuntimeTurnId,
    completed: BTreeSet<String>,
}

impl WatcherActivityFeed {
    pub(crate) fn new(turn: RuntimeTurnId) -> Self {
        Self {
            turn,
            completed: BTreeSet::new(),
        }
    }

    pub(crate) fn poll(
        &mut self,
        services: &HostServices,
    ) -> Result<Vec<ActivityObservation>, RuntimeFailure> {
        let Some(watcher) = services.watcher().cloned() else {
            return Ok(Vec::new());
        };
        let owning = WatcherOwningTurn::new(self.turn.as_str().to_owned()).map_err(|_| {
            failure(
                "swallowtail.claude_code.headless.watcher_turn_invalid",
                "Claude Code headless watcher activity could not bind the owning turn",
            )
        })?;
        let snapshots = match host_worker(move || futures_executor::block_on(watcher.list(owning)))?
        {
            Ok(snapshots) => snapshots,
            Err(error)
                if matches!(
                    error.diagnostic().code(),
                    "swallowtail.local_watcher.turn_not_found"
                        | "swallowtail.local_watcher.turn_retired"
                ) =>
            {
                return Ok(Vec::new());
            }
            Err(error) => return Err(error),
        };
        let mut observations = Vec::new();
        for snapshot in snapshots {
            if !snapshot.phase().is_terminal() {
                continue;
            }
            if !self
                .completed
                .insert(snapshot.activity_id().as_str().to_owned())
            {
                continue;
            }
            if let Some(observation) = completed_observation(&self.turn, &snapshot) {
                observations.push(observation);
            }
        }
        Ok(observations)
    }
}

fn completed_observation(
    turn: &RuntimeTurnId,
    snapshot: &WatcherSnapshot,
) -> Option<ActivityObservation> {
    let cause = snapshot.terminal_cause()?;
    ActivityObservation::new(
        ActivityId::new(snapshot.activity_id().as_str()).ok()?,
        ActivityOperationId::Turn(turn.clone()),
        ActivityKind::HostWatcher,
        ActivityLifecyclePhase::Completed,
        status_for_cause(cause),
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .ok()
}

const fn status_for_cause(cause: WatcherTerminalCause) -> ActivityStatus {
    match cause {
        WatcherTerminalCause::Completed => ActivityStatus::Completed,
        WatcherTerminalCause::Failed => ActivityStatus::Failed,
        WatcherTerminalCause::Cancelled
        | WatcherTerminalCause::TimedOut
        | WatcherTerminalCause::Stopped => ActivityStatus::Cancelled,
    }
}

fn host_worker<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
) -> Result<T, RuntimeFailure> {
    std::thread::spawn(work).join().map_err(|_| {
        failure(
            "swallowtail.claude_code.headless.watcher_host_worker_failed",
            "Claude Code watcher host worker failed",
        )
    })
}
