use super::WatcherSnapshot;
use crate::{
    ActivityDisclosure, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, InvalidActivityRecord, RuntimeTurnId,
};
use swallowtail_core::{WatcherLifecyclePhase, WatcherTerminalCause};

/// Projects host-owned watcher state onto the existing ordered turn activity stream.
///
/// The activity id is presentation identity only. Watcher control still requires
/// the watcher id and owning turn.
pub fn project_watcher_activity(
    runtime_turn: &RuntimeTurnId,
    snapshot: &WatcherSnapshot,
) -> Result<ActivityObservation, InvalidActivityRecord> {
    let (phase, status) = match snapshot.phase() {
        WatcherLifecyclePhase::Accepted => {
            (ActivityLifecyclePhase::Started, ActivityStatus::Pending)
        }
        WatcherLifecyclePhase::Running => {
            (ActivityLifecyclePhase::Updated, ActivityStatus::InProgress)
        }
        WatcherLifecyclePhase::Terminal | WatcherLifecyclePhase::Joined => {
            let cause = snapshot
                .terminal_cause()
                .expect("terminal and joined watcher snapshots retain an exact terminal cause");
            (
                ActivityLifecyclePhase::Completed,
                activity_status_for_cause(cause),
            )
        }
    };

    // Activity ids are presentation correlation. Reuse the registry-retained id.
    let activity_id = ActivityId::new(snapshot.activity_id().as_str())?;
    ActivityObservation::new(
        activity_id,
        ActivityOperationId::Turn(runtime_turn.clone()),
        ActivityKind::HostWatcher,
        phase,
        status,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
}

const fn activity_status_for_cause(cause: WatcherTerminalCause) -> ActivityStatus {
    match cause {
        WatcherTerminalCause::Completed => ActivityStatus::Completed,
        WatcherTerminalCause::Failed => ActivityStatus::Failed,
        WatcherTerminalCause::Cancelled
        | WatcherTerminalCause::TimedOut
        | WatcherTerminalCause::Stopped => ActivityStatus::Cancelled,
    }
}
