use super::LocalWatcherState;
use std::sync::{Arc, Mutex};
use swallowtail_core::{WatcherId, WatcherTerminalCause};
use swallowtail_runtime::{ProcessHandle, RuntimeTurnId};

pub(super) async fn monitor_watcher(
    state: Arc<Mutex<LocalWatcherState>>,
    turn: RuntimeTurnId,
    watcher_id: WatcherId,
    process: Arc<dyn ProcessHandle>,
) {
    let mut output_failed = false;
    loop {
        match process.read_output().await {
            Ok(Some(_chunk)) => {}
            Ok(None) => break,
            Err(_) => {
                output_failed = true;
                let _ = process.force_stop().await;
                break;
            }
        }
    }
    let cause = match process.wait().await {
        Ok(exit) if exit.success() && !output_failed => WatcherTerminalCause::Completed,
        Ok(_) | Err(_) => WatcherTerminalCause::Failed,
    };
    let summary = if output_failed {
        super::support::summary("output_limit_exceeded")
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
