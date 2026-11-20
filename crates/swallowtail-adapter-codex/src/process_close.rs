//! Escalating shutdown for a Codex app-server child after stdin EOF.

use futures_channel::oneshot;
use futures_util::future::{Either, select};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use swallowtail_runtime::{
    BoxFuture, Deadline, HostServices, JoinedTask, MonotonicInstant, ProcessHandle, RuntimeFailure,
    ScopeId, ScopedTaskService, TimeService,
};

/// Grace given to EOF and then to graceful stop when there is no caller deadline.
const CLOSE_GRACE: Duration = Duration::from_millis(250);

pub(crate) struct CloseBudget {
    time: Arc<dyn TimeService>,
    deadline: Deadline,
}

pub(crate) fn close_budget(
    services: &HostServices,
    deadline: Option<Deadline>,
) -> Option<CloseBudget> {
    Some(CloseBudget {
        time: services.time()?.clone(),
        deadline: deadline?,
    })
}

/// Owns the pump join and signals when the pump can be joined. Ordinary host
/// joins may block on their first poll, so escalation races the signal instead.
pub(crate) struct AppServerTask {
    task: Box<dyn JoinedTask>,
    finished: oneshot::Receiver<()>,
    service: Arc<dyn ScopedTaskService>,
    scope: ScopeId,
}

impl AppServerTask {
    pub(crate) fn spawn(
        service: Arc<dyn ScopedTaskService>,
        scope: ScopeId,
        pump: BoxFuture<'static, ()>,
    ) -> Result<Self, RuntimeFailure> {
        let (finish, finished) = oneshot::channel();
        let task = service.spawn(
            scope.clone(),
            Box::pin(async move {
                pump.await;
                // Dropping the sender wakes close even if the pump unwinds.
                drop(finish);
            }),
        )?;
        Ok(Self {
            task,
            finished,
            service,
            scope,
        })
    }

    pub(crate) async fn join(self) -> Result<(), RuntimeFailure> {
        let _ = self.finished.await;
        self.task.join().await
    }

    pub(crate) async fn join_or_escalate(
        mut self,
        process: &dyn ProcessHandle,
        budget: Option<CloseBudget>,
    ) -> Result<(), RuntimeFailure> {
        let stages = budget.as_ref().map(|budget| {
            let now = budget.time.now().ticks();
            let remaining = budget.deadline.instant().ticks().saturating_sub(now);
            let stop_at = at(now + remaining / 2);
            let kill_at = at(now + remaining - remaining.div_ceil(4));
            (budget.time.as_ref(), stop_at, kill_at)
        });

        let stop_at = stages.map(|(time, stop_at, _)| (time, stop_at));
        if self.finishes_before(stop_at).await {
            return self.task.join().await;
        }
        let kill_at = stages.map(|(time, _, kill_at)| (time, kill_at));
        if process.request_stop().await.is_ok() && self.finishes_before(kill_at).await {
            return self.task.join().await;
        }
        let stop = process.force_stop().await;
        let join = self.join().await;
        stop.and(join)
    }

    async fn finishes_before(&mut self, stage: Option<(&dyn TimeService, Deadline)>) -> bool {
        // The sender is dropped upon completion. A closed channel is the signal.
        if self.finished.try_recv().is_err() {
            return true;
        }
        if let Some((time, deadline)) = stage {
            return matches!(
                select(&mut self.finished, time.wait_until(deadline)).await,
                Either::Left(_)
            );
        }

        // Without a caller deadline, wait for a short relative grace period in
        // the scoped task service. This interval never sets a terminal timeout.
        let (cancel, cancelled) = mpsc::channel::<()>();
        let (elapsed, expiry) = oneshot::channel::<()>();
        let timer = self.service.spawn(
            self.scope.clone(),
            Box::pin(async move {
                let _ = cancelled.recv_timeout(CLOSE_GRACE);
                drop(elapsed);
            }),
        );
        let Ok(timer) = timer else {
            return false;
        };
        let finished = matches!(select(&mut self.finished, expiry).await, Either::Left(_));
        drop(cancel);
        let _ = timer.join().await;
        finished
    }
}

fn at(ticks: u64) -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(ticks))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::future::BoxFuture;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};
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
        finished: Mutex<Option<oneshot::Sender<()>>>,
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
            self.finished.lock().expect("finish lock").take();
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    struct ChildTask(Arc<StubbornChild>);

    impl JoinedTask for ChildTask {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            assert!(self.0.finished.lock().expect("finish lock").is_none());
            Box::pin(async { Ok(()) })
        }
    }

    struct UnusedTaskService;

    impl ScopedTaskService for UnusedTaskService {
        fn spawn(
            &self,
            _: ScopeId,
            _: BoxFuture<'static, ()>,
        ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
            panic!("a deadline-bound close does not spawn a timer")
        }
    }

    fn task(child: &Arc<StubbornChild>) -> AppServerTask {
        let (finish, finished) = oneshot::channel();
        *child.finished.lock().expect("finish lock") = Some(finish);
        AppServerTask {
            task: Box::new(ChildTask(Arc::clone(child))),
            finished,
            service: Arc::new(UnusedTaskService),
            scope: ScopeId::new("codex-close-test").unwrap(),
        }
    }

    fn budget() -> Option<CloseBudget> {
        Some(CloseBudget {
            time: Arc::new(PastClock),
            deadline: at(100),
        })
    }

    #[test]
    fn child_ignoring_eof_is_stopped_then_killed_before_join() {
        let child = Arc::new(StubbornChild::default());
        let result =
            futures_executor::block_on(task(&child).join_or_escalate(child.as_ref(), budget()));
        assert!(result.is_ok());
        assert_eq!(child.stops.load(Ordering::SeqCst), 1);
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }

    #[test]
    fn failed_graceful_stop_escalates_straight_to_kill() {
        let child = Arc::new(StubbornChild {
            requests_fail: true,
            ..StubbornChild::default()
        });
        let result =
            futures_executor::block_on(task(&child).join_or_escalate(child.as_ref(), budget()));
        assert!(result.is_ok());
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }

    #[test]
    fn exit_on_eof_is_joined_without_escalation() {
        let child = Arc::new(StubbornChild::default());
        let task = task(&child);
        child.finished.lock().expect("finish lock").take();
        let result = futures_executor::block_on(task.join_or_escalate(child.as_ref(), budget()));
        assert!(result.is_ok());
        assert!(child.log.lock().expect("log lock").is_empty());
    }
}
