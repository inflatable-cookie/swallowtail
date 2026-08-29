use super::LocalWatcherState;
use crate::containment::ProcessContainmentLease;
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherId, WatcherTerminalCause};
use swallowtail_runtime::{ProcessHandle, RuntimeTurnId};

pub(super) async fn monitor_watcher(
    state: Arc<Mutex<LocalWatcherState>>,
    turn: RuntimeTurnId,
    watcher_id: WatcherId,
    process: Arc<dyn ProcessHandle>,
    lease: Arc<dyn ProcessContainmentLease>,
) {
    let mut output_failed = false;
    loop {
        match process.read_output().await {
            Ok(Some(_chunk)) => {}
            Ok(None) => break,
            Err(_) => {
                output_failed = true;
                let _ = lease.force_stop().await;
                break;
            }
        }
    }
    // Contract 059: terminal only after the contained workload is terminal.
    let contained = lease.prove_empty_and_join().await;
    let cause = match contained {
        Ok(()) if !output_failed => WatcherTerminalCause::Completed,
        Ok(()) | Err(_) => WatcherTerminalCause::Failed,
    };
    let summary = if output_failed {
        super::support::summary("output_limit_exceeded")
    } else if contained.is_err() {
        super::support::summary("containment_join_failed")
    } else {
        super::support::summary(cause.as_str())
    };
    let mut state = state.lock().expect("local watcher state lock poisoned");
    if let Some(turn_state) = state.active.get_mut(&turn) {
        let _ = turn_state
            .registry
            .complete(&watcher_id, cause, Some(summary));
    }
}
