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
                let _ = entry.lease.force_stop().await;
                break;
            }
        }
    }
    let process_result = entry.process.wait().await;
    // Contract 059: terminal only after the contained workload is terminal and
    // the containment scope is empty. Reuse the durable entry proof so later
    // wait/join paths do not require a second independent supervisor join.
    let contained = entry.prove_empty_and_join().await;
    let cause = match (&process_result, &contained) {
        (Ok(exit), Ok(())) if exit.success() && !output_failed => WatcherTerminalCause::Completed,
        _ => WatcherTerminalCause::Failed,
    };
    let summary = if output_failed {
        super::support::summary("output_limit_exceeded")
    } else if contained.is_err() {
        super::support::summary("containment_join_failed")
    } else {
        super::support::summary(cause.as_str())
    };
    if let Err(error) = contained {
        entry.record_join_error(error);
    } else if let Err(error) = process_result {
        entry.record_join_error(error);
    }
    let mut state = state.lock().expect("local watcher state lock poisoned");
    if let Some(turn_state) = state.active.get_mut(&turn) {
        let _ = turn_state
            .registry
            .complete(&watcher_id, cause, Some(summary));
    }
}
