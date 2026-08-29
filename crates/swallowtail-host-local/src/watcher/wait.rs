use super::{LocalWatcherEntry, LocalWatcherState};
use crate::output::failure;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, TryLockError};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{WatcherId, WatcherLifecyclePhase, WatcherOwningTurn, WatcherTerminalCause};
use swallowtail_runtime::{RuntimeFailure, RuntimeTurnId, WatcherWaitRepresentation};

/// Deferred local wait. Construction only validates identity and captures the
/// host-owned entry; all joins happen after a poll observes task completion.
pub(super) struct LocalWatcherWait {
    state: Arc<Mutex<LocalWatcherState>>,
    turn: RuntimeTurnId,
    owning_turn: WatcherOwningTurn,
    watcher_id: WatcherId,
    entry: Arc<LocalWatcherEntry>,
}

impl LocalWatcherWait {
    pub(super) fn new(
        state: Arc<Mutex<LocalWatcherState>>,
        turn: RuntimeTurnId,
        owning_turn: WatcherOwningTurn,
        watcher_id: WatcherId,
        entry: Arc<LocalWatcherEntry>,
    ) -> Self {
        Self {
            state,
            turn,
            owning_turn,
            watcher_id,
            entry,
        }
    }
}

impl Future for LocalWatcherWait {
    type Output = Result<WatcherWaitRepresentation, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let join_guard = match this.entry.join_lock.try_lock() {
            Ok(guard) => guard,
            Err(TryLockError::WouldBlock) => {
                this.entry.join_signal.register(context.waker());
                return Poll::Pending;
            }
            Err(TryLockError::Poisoned(_)) => {
                return Poll::Ready(Err(failure(
                    "swallowtail.local_watcher.join_lock_poisoned",
                    "Local watcher join state is unavailable",
                )));
            }
        };

        if let Some(error) = this.entry.join_error() {
            return Poll::Ready(Err(error));
        }
        if this.entry.joined.load(std::sync::atomic::Ordering::Acquire) {
            return Poll::Ready(this.representation());
        }

        let task = {
            let mut task_slot = this
                .entry
                .task
                .lock()
                .expect("local watcher task lock poisoned");
            let Some(task) = task_slot.as_ref() else {
                drop(join_guard);
                let error = failure(
                    "swallowtail.local_watcher.join_not_available",
                    "Local watcher task join is unavailable",
                );
                this.entry.record_join_error(error.clone());
                return Poll::Ready(Err(error));
            };
            if !task.is_finished() {
                task.register_waker(context.waker());
                // Registering can race the worker's final notification. The
                // second observation closes that race without sleeping or
                // blocking the executor thread.
                if !task.is_finished() {
                    drop(task_slot);
                    drop(join_guard);
                    return Poll::Pending;
                }
            }
            task_slot.take().expect("local watcher task was retained")
        };

        // LocalJoinedTask reports finished only after its worker has returned.
        // The worker awaits ProcessHandle::wait before returning, so both
        // joins below must be ready. Poll them directly instead of entering a
        // nested executor or blocking the caller while the join lock is held.
        let noop_waker = Waker::noop();
        let mut join_context = Context::from_waker(noop_waker);
        let mut task_join = task.join();
        match task_join.as_mut().poll(&mut join_context) {
            Poll::Ready(Ok(())) => {}
            Poll::Ready(Err(error)) => {
                this.entry.record_join_error(error.clone());
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {
                let error = failure(
                    "swallowtail.local_watcher.join_not_ready",
                    "Local watcher task reported finished but its join was not ready",
                );
                this.entry.record_join_error(error.clone());
                return Poll::Ready(Err(error));
            }
        }

        let process = Arc::clone(&this.entry.process);
        let mut process_wait = Box::pin(async move { process.wait().await });
        match process_wait.as_mut().poll(&mut join_context) {
            Poll::Ready(Ok(_)) => {}
            Poll::Ready(Err(error)) => {
                this.entry.record_join_error(error.clone());
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {
                let error = failure(
                    "swallowtail.local_watcher.process_not_ready",
                    "Local watcher task finished before its process wait was ready",
                );
                this.entry.record_join_error(error.clone());
                return Poll::Ready(Err(error));
            }
        }

        let result = this.mark_joined();

        match result {
            Ok(WatcherWaitRepresentation::Satisfied(_)) => {
                this.entry
                    .joined
                    .store(true, std::sync::atomic::Ordering::Release);
                this.entry.join_signal.notify();
                Poll::Ready(result)
            }
            Ok(_) => {
                let error = failure(
                    "swallowtail.local_watcher.wait_not_joined",
                    "Local watcher cleanup did not reach joined truth",
                );
                this.entry.record_join_error(error.clone());
                Poll::Ready(Err(error))
            }
            Err(error) => {
                this.entry.record_join_error(error.clone());
                Poll::Ready(Err(error))
            }
        }
    }
}

impl LocalWatcherWait {
    fn mark_joined(&self) -> Result<WatcherWaitRepresentation, RuntimeFailure> {
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let Some(turn_state) = state.active.get_mut(&self.turn) else {
            return Err(if state.is_retired(&self.turn) {
                super::support::turn_retired_failure()
            } else {
                super::support::turn_missing_failure()
            });
        };
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        let snapshot = turn_state
            .registry
            .inspect(&self.owning_turn, &self.watcher_id)
            .map_err(super::support::registry_failure)?;
        if matches!(
            snapshot.phase(),
            WatcherLifecyclePhase::Accepted | WatcherLifecyclePhase::Running
        ) {
            turn_state
                .registry
                .complete(
                    &self.watcher_id,
                    WatcherTerminalCause::Failed,
                    Some(super::support::summary("failed")),
                )
                .map_err(super::support::registry_failure)?;
        }
        if !matches!(
            turn_state
                .registry
                .inspect(&self.owning_turn, &self.watcher_id)
                .map_err(super::support::registry_failure)?
                .phase(),
            WatcherLifecyclePhase::Joined
        ) {
            turn_state
                .registry
                .join(&self.watcher_id)
                .map_err(super::support::registry_failure)?;
        }
        turn_state
            .registry
            .wait_representation(&self.owning_turn, &self.watcher_id)
            .map_err(super::support::registry_failure)
    }

    fn representation(&self) -> Result<WatcherWaitRepresentation, RuntimeFailure> {
        let state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let Some(turn_state) = state.active.get(&self.turn) else {
            return Err(if state.is_retired(&self.turn) {
                super::support::turn_retired_failure()
            } else {
                super::support::turn_missing_failure()
            });
        };
        if turn_state.closed {
            return Err(failure(
                "swallowtail.local_watcher.turn_closed",
                "Watcher turn cleanup has already closed this turn",
            ));
        }
        turn_state
            .registry
            .wait_representation(&self.owning_turn, &self.watcher_id)
            .map_err(super::support::registry_failure)
    }
}
