//! Escalating shutdown for a Codex app-server child after stdin EOF.
//!
//! Closing stdin asks `codex app-server` to exit. A child that ignores EOF
//! must not be waited on forever, and must not be abandoned by the caller's
//! cleanup bound either: the remaining budget is split so a graceful stop is
//! requested at its midpoint and a forced stop at three quarters. The reader
//! task is joined after each escalation so the child is reaped before cleanup
//! reports.

use futures_util::future::{Either, select};
use std::sync::Arc;
use swallowtail_runtime::{
    Deadline, JoinedTask, MonotonicInstant, ProcessHandle, RuntimeFailure, TimeService,
};

/// Host time and the caller's hard deadline that bound one close.
pub(crate) struct CloseBudget {
    time: Arc<dyn TimeService>,
    deadline: Deadline,
}

impl CloseBudget {
    pub(crate) fn new(time: Arc<dyn TimeService>, deadline: Deadline) -> Self {
        Self { time, deadline }
    }
}

/// Joins `task` after stdin EOF, escalating to stop when the child lingers.
///
/// Without a budget there is no clock to escalate against, so the join waits
/// exactly as before. With one, the stop and kill points are computed from the
/// host's own ticks, which keeps the policy independent of tick units.
pub(crate) async fn join_or_escalate(
    process: &dyn ProcessHandle,
    task: Box<dyn JoinedTask>,
    budget: Option<CloseBudget>,
) -> Result<(), RuntimeFailure> {
    let Some(budget) = budget else {
        return task.join().await;
    };
    let now = budget.time.now().ticks();
    let remaining = budget.deadline.instant().ticks().saturating_sub(now);
    let stop_at = now.saturating_add(remaining / 2);
    let kill_at = now.saturating_add(remaining / 4 * 3);

    let mut joined = task.join();
    if let Either::Left((result, _)) =
        select(&mut joined, budget.time.wait_until(at(stop_at))).await
    {
        return result;
    }
    if process.request_stop().await.is_err() {
        let _ = process.force_stop().await;
        return joined.await;
    }
    if let Either::Left((result, _)) =
        select(&mut joined, budget.time.wait_until(at(kill_at))).await
    {
        return result;
    }
    let _ = process.force_stop().await;
    joined.await
}

/// Builds the close budget from a caller deadline and host time, when both exist.
pub(crate) fn close_budget(
    services: &swallowtail_runtime::HostServices,
    deadline: Option<Deadline>,
) -> Option<CloseBudget> {
    let deadline = deadline?;
    let time = services.time()?.clone();
    Some(CloseBudget::new(time, deadline))
}

fn at(ticks: u64) -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(ticks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::BoxFuture;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use swallowtail_runtime::{
        DeadlineObservation, ProcessExit, ProcessInputChunk, ProcessOutputChunk,
    };

    /// Clock that is already past every wait, so each escalation point fires.
    struct PastClock;

    impl TimeService for PastClock {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(0)
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            Box::pin(async move {
                DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(u64::MAX))
            })
        }
    }

    /// Child that exits only once it has been force-stopped.
    #[derive(Default)]
    struct StubbornChild {
        stops: AtomicUsize,
        killed: Arc<AtomicBool>,
        requests_fail: bool,
        log: Mutex<Vec<&'static str>>,
    }

    impl ProcessHandle for StubbornChild {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(async { Ok(None) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.stops.fetch_add(1, Ordering::SeqCst);
            self.log.lock().expect("log lock").push("stop");
            let fail = self.requests_fail;
            Box::pin(async move {
                if fail {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "test.stop_refused",
                        "stop refused",
                    )))
                } else {
                    Ok(())
                }
            })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.log.lock().expect("log lock").push("kill");
            self.killed.store(true, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct ChildTask(Arc<AtomicBool>);

    impl JoinedTask for ChildTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let killed = self.0;
            Box::pin(std::future::poll_fn(move |_| {
                if killed.load(Ordering::SeqCst) {
                    std::task::Poll::Ready(Ok(()))
                } else {
                    std::task::Poll::Pending
                }
            }))
        }
    }

    fn budget() -> Option<CloseBudget> {
        Some(CloseBudget::new(
            Arc::new(PastClock),
            Deadline::at(MonotonicInstant::from_ticks(100)),
        ))
    }

    #[test]
    fn child_ignoring_eof_is_stopped_then_killed_before_join() {
        let child = StubbornChild::default();
        let task = Box::new(ChildTask(Arc::clone(&child.killed)));
        let result = futures_executor_block_on(join_or_escalate(&child, task, budget()));
        assert!(result.is_ok());
        assert_eq!(child.stops.load(Ordering::SeqCst), 1);
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }

    #[test]
    fn failed_graceful_stop_escalates_straight_to_kill() {
        let child = StubbornChild {
            requests_fail: true,
            ..StubbornChild::default()
        };
        let task = Box::new(ChildTask(Arc::clone(&child.killed)));
        let result = futures_executor_block_on(join_or_escalate(&child, task, budget()));
        assert!(result.is_ok());
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }

    #[test]
    fn exit_on_eof_is_joined_without_escalation() {
        let child = StubbornChild::default();
        child.killed.store(true, Ordering::SeqCst);
        let task = Box::new(ChildTask(Arc::clone(&child.killed)));
        let result = futures_executor_block_on(join_or_escalate(&child, task, budget()));
        assert!(result.is_ok());
        assert!(child.log.lock().expect("log lock").is_empty());
    }

    fn futures_executor_block_on<F: std::future::Future>(future: F) -> F::Output {
        futures_util::future::FutureExt::now_or_never(std::pin::pin!(future))
            .expect("escalation completes without external wakeups")
    }
}
