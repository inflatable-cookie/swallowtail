use super::{LocalWatcherEntry, LocalWatcherState};
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherId, WatcherTerminalCause};
use swallowtail_runtime::RuntimeTurnId;

pub(super) async fn monitor_watcher(
    state: Arc<Mutex<LocalWatcherState>>,
    turn: RuntimeTurnId,
    watcher_id: WatcherId,
    entry: Arc<LocalWatcherEntry>,
) {
    let mut output_failed = false;
    loop {
        match entry.process.read_output().await {
            Ok(Some(_chunk)) => {}
            Ok(None) => break,
            Err(_) => {
                output_failed = true;
                let _ = entry.process.force_stop().await;
                break;
            }
        }
    }
    let process_result = entry.process.wait().await;
    // Contract 059: ProcessHandle::wait already joins the root, cooperative
    // process-group cleanup, output readers, and process supervisor. The
    // watcher monitor then records terminal cause before the host join path
    // marks joined truth.
    let cause = match &process_result {
        Ok(exit) if exit.success() && !output_failed => WatcherTerminalCause::Completed,
        _ => WatcherTerminalCause::Failed,
    };
    let summary = if output_failed {
        super::support::summary("output_limit_exceeded")
    } else {
        super::support::summary(cause.as_str())
    };
    if let Err(error) = process_result {
        entry.record_join_error(error);
    }
    let snapshot = {
        let mut state = state.lock().expect("local watcher state lock poisoned");
        let Some(turn_state) = state.active.get_mut(&turn) else {
            return;
        };
        let _ = turn_state
            .registry
            .complete(&watcher_id, cause, Some(summary));
        turn_state
            .registry
            .inspect(turn_state.registry.owning_turn(), &watcher_id)
            .ok()
    };
    if let Some(snapshot) = snapshot {
        let _ = state
            .lock()
            .expect("local watcher state lock poisoned")
            .publish(&turn, snapshot);
    }
}
