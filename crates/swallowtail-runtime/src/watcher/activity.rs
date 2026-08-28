use super::WatcherSnapshot;
use crate::{
    ActivityDisclosure, ActivityId, ActivityKind, ActivityLifecyclePhase, ActivityObservation,
    ActivityOperationId, ActivityStatus, InvalidActivityRecord, RuntimeTurnId,
};
use std::error::Error;
use std::fmt;
use swallowtail_core::{WatcherLifecyclePhase, WatcherRevision, WatcherTerminalCause};

/// Result of projecting one watcher snapshot onto turn activity vocabulary.
///
/// Terminal maps to a completed activity observation. Joined is cleanup truth
/// and must not emit a second completed observation for the same activity id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatcherActivityProjection {
    /// Deliver on the ordered turn activity stream.
    Activity(Box<ActivityObservation>),
    /// Cleanup join retained after the terminal activity observation.
    Joined {
        /// Presentation activity identity correlated to the watcher.
        activity_id: ActivityId,
        /// Exact terminal cause retained by the watcher.
        terminal_cause: WatcherTerminalCause,
        /// Monotonic revision after join.
        revision: WatcherRevision,
    },
}

/// Stable reason watcher activity projection failed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatcherActivityProjectionFailure {
    /// The supplied runtime turn does not own the watcher snapshot.
    ForeignIdentity,
    /// Activity identity construction rejected the retained correlation id.
    InvalidActivityRecord(InvalidActivityRecord),
}

impl fmt::Display for WatcherActivityProjectionFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForeignIdentity => {
                formatter.write_str("Watcher activity projection rejected a foreign owning turn")
            }
            Self::InvalidActivityRecord(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for WatcherActivityProjectionFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ForeignIdentity => None,
            Self::InvalidActivityRecord(error) => Some(error),
        }
    }
}

/// Projects host-owned watcher state onto the existing ordered turn activity stream.
///
/// The activity id is presentation identity only. Watcher control still requires
/// the watcher id and owning turn. Joined cleanup is returned separately so it
/// cannot duplicate a completed activity observation.
pub fn project_watcher_activity(
    runtime_turn: &RuntimeTurnId,
    snapshot: &WatcherSnapshot,
) -> Result<WatcherActivityProjection, WatcherActivityProjectionFailure> {
    if snapshot.owning_turn().as_str() != runtime_turn.as_str() {
        return Err(WatcherActivityProjectionFailure::ForeignIdentity);
    }

    let activity_id = ActivityId::new(snapshot.activity_id().as_str())
        .map_err(WatcherActivityProjectionFailure::InvalidActivityRecord)?;

    match snapshot.phase() {
        WatcherLifecyclePhase::Accepted => Ok(WatcherActivityProjection::Activity(Box::new(
            ActivityObservation::new(
                activity_id,
                ActivityOperationId::Turn(runtime_turn.clone()),
                ActivityKind::HostWatcher,
                ActivityLifecyclePhase::Started,
                ActivityStatus::Pending,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )
            .map_err(WatcherActivityProjectionFailure::InvalidActivityRecord)?,
        ))),
        WatcherLifecyclePhase::Running => Ok(WatcherActivityProjection::Activity(Box::new(
            ActivityObservation::new(
                activity_id,
                ActivityOperationId::Turn(runtime_turn.clone()),
                ActivityKind::HostWatcher,
                ActivityLifecyclePhase::Updated,
                ActivityStatus::InProgress,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )
            .map_err(WatcherActivityProjectionFailure::InvalidActivityRecord)?,
        ))),
        WatcherLifecyclePhase::Terminal => {
            let cause = snapshot
                .terminal_cause()
                .expect("terminal watcher snapshots retain an exact terminal cause");
            Ok(WatcherActivityProjection::Activity(Box::new(
                ActivityObservation::new(
                    activity_id,
                    ActivityOperationId::Turn(runtime_turn.clone()),
                    ActivityKind::HostWatcher,
                    ActivityLifecyclePhase::Completed,
                    activity_status_for_cause(cause),
                    None,
                    ActivityDisclosure::IdentityAndLifecycleOnly,
                )
                .map_err(WatcherActivityProjectionFailure::InvalidActivityRecord)?,
            )))
        }
        WatcherLifecyclePhase::Joined => {
            let cause = snapshot
                .terminal_cause()
                .expect("joined watcher snapshots retain an exact terminal cause");
            Ok(WatcherActivityProjection::Joined {
                activity_id,
                terminal_cause: cause,
                revision: snapshot.revision(),
            })
        }
    }
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
