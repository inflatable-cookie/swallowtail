//! Escalating shutdown for a Codex app-server child after stdin EOF.

use futures_channel::oneshot;
use futures_util::future::{Either, select};
use std::future::{Future, poll_fn};
use std::sync::{Arc, mpsc};
use std::task::Poll;
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    BlockingJob, BlockingWorkService, BoxFuture, Deadline, DeadlineObservation, HostServices,
    JoinedTask, MonotonicInstant, ProcessHandle, RuntimeFailure, ScopeId, ScopedTaskService,
    TimeService,
};

/// Grace given to EOF and then to graceful stop when there is no caller deadline.
const CLOSE_GRACE: Duration = Duration::from_millis(250);
/// Dedicated teardown budget (in monotonic ticks) when waiting for a pump to
/// settle after a refused `force_stop()`.
const REFUSED_STOP_BUDGET_TICKS: u64 = 500;
/// Bounded wall-clock fallback grace when a host supplies neither a
/// `TimeService` nor a `BlockingWorkService`.
const CLOCKLESS_TEARDOWN_GRACE: Duration = Duration::from_millis(100);

/// The host services a close uses to time its escalation stages.
pub(crate) struct CloseBudget {
    time: Option<Arc<dyn TimeService>>,
    blocking: Option<Arc<dyn BlockingWorkService>>,
    deadline: Option<Deadline>,
}

pub(crate) fn close_budget(services: &HostServices, deadline: Option<Deadline>) -> CloseBudget {
    CloseBudget {
        time: services.time().cloned(),
        blocking: services.blocking_work().cloned(),
        deadline,
    }
}

/// How one escalation stage waits for the pump to finish.
enum Wait<'a> {
    /// Wait until a caller-supplied deadline stage is observed.
    Until(&'a dyn TimeService, Deadline),
    /// Wait out the relative grace on the host's blocking pool.
    Grace(&'a dyn BlockingWorkService),
    /// No time source can bound the stage, so the close escalates without it.
    Unavailable,
}

fn wait_for<'a>(
    stage: Option<(&'a dyn TimeService, Deadline)>,
    blocking: Option<&'a dyn BlockingWorkService>,
) -> Wait<'a> {
    match (stage, blocking) {
        (Some((time, deadline)), _) => Wait::Until(time, deadline),
        (None, Some(blocking)) => Wait::Grace(blocking),
        (None, None) => Wait::Unavailable,
    }
}

/// Owns the pump join and signals when the pump can be joined. Ordinary host
/// joins may block on their first poll, so escalation races the signal instead.
pub(crate) struct AppServerTask {
    task: Box<dyn JoinedTask>,
    finished: oneshot::Receiver<()>,
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
            scope,
        })
    }

    pub(crate) async fn join(self) -> Result<(), RuntimeFailure> {
        let _ = self.finished.await;
        self.task.join().await
    }

    #[cfg(test)]
    pub(crate) fn from_parts(
        task: Box<dyn JoinedTask>,
        finished: oneshot::Receiver<()>,
        scope: ScopeId,
    ) -> Self {
        Self {
            task,
            finished,
            scope,
        }
    }
}

/// Outcome of racing one stop-and-join close sequence against a shared teardown budget.
pub(crate) struct BoundedCloseOutcome {
    /// Result of the stop future when it completed before the teardown budget ran out,
    /// or `None` when the stop step itself timed out.
    pub(crate) stop: Option<Result<(), RuntimeFailure>>,
    /// Whether the teardown budget ran out before the pump finished, triggering `on_abort`.
    pub(crate) aborted: bool,
    /// Result of joining the pump task.
    pub(crate) join: Result<(), RuntimeFailure>,
}

/// Shared lazy teardown timer carried across stop and pump-completion steps.
///
/// The timer is started only on the first `Poll::Pending`, so fast synchronous
/// exits do not consume extra `TimeService::wait_until` or blocking-pool slots.
struct CloseTimer {
    budget: CloseBudget,
    scope: ScopeId,
    deadline_wait: Option<BoxFuture<'static, DeadlineObservation>>,
    grace_cancel: Option<mpsc::Sender<()>>,
    grace_wait: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
    clockless_deadline: Option<Instant>,
    exhausted: bool,
}

impl CloseTimer {
    fn new(budget: CloseBudget, scope: ScopeId) -> Self {
        Self {
            budget,
            scope,
            deadline_wait: None,
            grace_cancel: None,
            grace_wait: None,
            clockless_deadline: None,
            exhausted: false,
        }
    }

    async fn race<T, F: Future<Output = T>>(&mut self, future: F) -> Option<T> {
        let mut future = std::pin::pin!(future);
        poll_fn(|context| {
            if let Poll::Ready(value) = future.as_mut().poll(context) {
                return Poll::Ready(Some(value));
            }
            if self.exhausted {
                return Poll::Ready(None);
            }
            if let Some(time) = self.budget.time.as_deref() {
                let wait = self.deadline_wait.get_or_insert_with(|| {
                    let budget_at = time.now().ticks().saturating_add(REFUSED_STOP_BUDGET_TICKS);
                    let target = match self.budget.deadline {
                        Some(deadline) => at(deadline.instant().ticks().min(budget_at)),
                        None => at(budget_at),
                    };
                    time.wait_until(target)
                });
                if wait.as_mut().poll(context).is_ready() {
                    self.deadline_wait = None;
                    self.exhausted = true;
                    return Poll::Ready(None);
                }
                return Poll::Pending;
            }
            if let Some(blocking) = self.budget.blocking.as_deref() {
                let wait = self.grace_wait.get_or_insert_with(|| {
                    let (cancel, cancelled) = mpsc::channel::<()>();
                    self.grace_cancel = Some(cancel);
                    let grace: BlockingJob = Box::new(move || {
                        let _ = cancelled.recv_timeout(CLOSE_GRACE);
                        Ok(())
                    });
                    blocking.run(self.scope.clone(), grace)
                });
                if wait.as_mut().poll(context).is_ready() {
                    self.grace_wait = None;
                    self.grace_cancel = None;
                    self.exhausted = true;
                    return Poll::Ready(None);
                }
                return Poll::Pending;
            }
            // Minimal `[Task, Process]` host with neither a time service nor a
            // blocking-work service: yield within a bounded wall-clock grace so
            // fast exits join cleanly while stuck stops or open stdout still abort.
            let deadline = *self
                .clockless_deadline
                .get_or_insert_with(|| Instant::now() + CLOCKLESS_TEARDOWN_GRACE);
            if Instant::now() >= deadline {
                self.exhausted = true;
                return Poll::Ready(None);
            }
            std::thread::yield_now();
            context.waker().wake_by_ref();
            Poll::Pending
        })
        .await
    }

    async fn finish(mut self) {
        drop(self.grace_cancel.take());
        if let Some(wait) = self.grace_wait.take() {
            let _ = wait.await;
        }
    }
}

impl AppServerTask {
    /// Bounds both an optional stop future and waiting for pump completion
    /// under one shared teardown budget, aborting and joining the pump if the
    /// budget expires at either step.
    pub(crate) async fn close_bounded<F>(
        mut self,
        budget: CloseBudget,
        stop: Option<F>,
        on_abort: impl FnOnce(),
    ) -> BoundedCloseOutcome
    where
        F: Future<Output = Result<(), RuntimeFailure>>,
    {
        let mut timer = CloseTimer::new(budget, self.scope.clone());
        let stop_result = match stop {
            Some(stop_future) => timer.race(stop_future).await,
            None => Some(Ok(())),
        };
        let finished_in_time = if self.finished.try_recv().is_err() {
            true
        } else if stop_result.is_none() {
            false
        } else {
            timer.race(&mut self.finished).await.is_some()
        };
        timer.finish().await;
        let aborted = !finished_in_time;
        if aborted {
            on_abort();
        }
        let join = self.join().await;
        BoundedCloseOutcome {
            stop: stop_result,
            aborted,
            join,
        }
    }

    #[cfg(test)]
    pub(crate) async fn join_or_escalate(
        self,
        process: &dyn ProcessHandle,
        budget: CloseBudget,
    ) -> Result<(), RuntimeFailure> {
        self.join_or_escalate_with_abort(process, budget, || {})
            .await
    }

    pub(crate) async fn join_or_escalate_with_abort(
        mut self,
        process: &dyn ProcessHandle,
        budget: CloseBudget,
        on_abort: impl FnOnce(),
    ) -> Result<(), RuntimeFailure> {
        let CloseBudget {
            time,
            blocking,
            deadline,
        } = budget;
        let stages = match (time.as_deref(), deadline) {
            (Some(time), Some(deadline)) => {
                let now = time.now().ticks();
                let remaining = deadline.instant().ticks().saturating_sub(now);
                let stop_at = at(now + remaining / 2);
                let kill_at = at(now + remaining - remaining.div_ceil(4));
                Some((time, stop_at, kill_at))
            }
            _ => None,
        };
        let blocking = blocking.as_deref();

        let stop = stages.map(|(time, stop_at, _)| (time, stop_at));
        if self.finishes_before(wait_for(stop, blocking)).await {
            return self.task.join().await;
        }
        let kill = stages.map(|(time, _, kill_at)| (time, kill_at));
        if process.request_stop().await.is_ok()
            && self.finishes_before(wait_for(kill, blocking)).await
        {
            return self.task.join().await;
        }
        let stop = process.force_stop().await;
        if stop.is_ok() {
            return self.join().await;
        }
        // When `force_stop` is refused and stdout remains open, wait up to the
        // remaining budget for the pump to finish on its own; if it is still
        // blocked on `read_output()` or `wait()`, abort the pump and join it
        // cleanly so even join-on-drop hosts never block indefinitely.
        let final_wait = stages.and_then(|(time, _, _)| deadline.map(|d| (time, d)));
        if !self.finishes_before(wait_for(final_wait, blocking)).await {
            on_abort();
        }
        let _ = self.join().await;
        stop
    }

    async fn finishes_before(&mut self, wait: Wait<'_>) -> bool {
        // The sender is dropped upon completion. A closed channel is the signal.
        if self.finished.try_recv().is_err() {
            return true;
        }
        match wait {
            Wait::Until(time, deadline) => matches!(
                select(&mut self.finished, time.wait_until(deadline)).await,
                Either::Left(_)
            ),
            Wait::Grace(blocking) => self.finishes_within_grace(blocking).await,
            Wait::Unavailable => false,
        }
    }

    /// Waits out the relative grace on the blocking pool. The scoped task
    /// service is never used here: a blocking receive on one of its workers
    /// would stall the executor. Dropping `cancel` ends the wait as soon as
    /// the pump finishes, so a fast exit does not pay the full grace.
    async fn finishes_within_grace(&mut self, blocking: &dyn BlockingWorkService) -> bool {
        let (cancel, cancelled) = mpsc::channel::<()>();
        let grace: BlockingJob = Box::new(move || {
            let _ = cancelled.recv_timeout(CLOSE_GRACE);
            Ok(())
        });
        let mut elapsed = blocking.run(self.scope.clone(), grace);
        match select(&mut self.finished, &mut elapsed).await {
            Either::Left(_) => {
                drop(cancel);
                let _ = elapsed.await;
                true
            }
            Either::Right(_) => false,
        }
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

    /// Blocking pool stand-in: each job runs on its own OS thread, as a host would run it.
    #[derive(Default)]
    struct ThreadBlocking {
        runs: AtomicUsize,
    }

    impl BlockingWorkService for ThreadBlocking {
        fn run(
            &self,
            _: ScopeId,
            job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            self.runs.fetch_add(1, Ordering::SeqCst);
            let (done, result) = oneshot::channel();
            std::thread::spawn(move || {
                let _ = done.send(job());
            });
            Box::pin(async move { result.await.unwrap_or(Ok(())) })
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

    fn task(child: &Arc<StubbornChild>) -> AppServerTask {
        let (finish, finished) = oneshot::channel();
        *child.finished.lock().expect("finish lock") = Some(finish);
        AppServerTask {
            task: Box::new(ChildTask(Arc::clone(child))),
            finished,
            scope: ScopeId::new("codex-close-test").unwrap(),
        }
    }

    fn budget() -> CloseBudget {
        CloseBudget {
            time: Some(Arc::new(PastClock)),
            blocking: None,
            deadline: Some(at(100)),
        }
    }

    fn unbudgeted(blocking: Option<Arc<dyn BlockingWorkService>>) -> CloseBudget {
        CloseBudget {
            time: None,
            blocking,
            deadline: None,
        }
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

    #[test]
    fn unbudgeted_eof_exit_joins_without_waiting_for_grace() {
        let child = Arc::new(StubbornChild::default());
        let task = task(&child);
        child.finished.lock().expect("finish lock").take();
        let blocking = Arc::new(ThreadBlocking::default());
        let result = futures_executor::block_on(task.join_or_escalate(
            child.as_ref(),
            unbudgeted(Some(blocking.clone() as Arc<dyn BlockingWorkService>)),
        ));
        assert!(result.is_ok());
        assert_eq!(blocking.runs.load(Ordering::SeqCst), 0);
        assert!(child.log.lock().expect("log lock").is_empty());
    }

    #[test]
    fn unbudgeted_child_exiting_during_grace_is_joined_without_escalation() {
        let child = Arc::new(StubbornChild::default());
        let task = task(&child);
        let finish = child.finished.lock().expect("finish lock").take();
        let exit = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(20));
            drop(finish);
        });
        let blocking = Arc::new(ThreadBlocking::default());
        let result = futures_executor::block_on(task.join_or_escalate(
            child.as_ref(),
            unbudgeted(Some(blocking.clone() as Arc<dyn BlockingWorkService>)),
        ));
        exit.join().expect("exit thread");
        assert!(result.is_ok());
        assert_eq!(blocking.runs.load(Ordering::SeqCst), 1);
        assert!(child.log.lock().expect("log lock").is_empty());
    }

    #[test]
    fn unbudgeted_child_ignoring_eof_is_stopped_then_killed_on_blocking_pool() {
        let child = Arc::new(StubbornChild::default());
        let blocking = Arc::new(ThreadBlocking::default());
        let result = futures_executor::block_on(task(&child).join_or_escalate(
            child.as_ref(),
            unbudgeted(Some(blocking.clone() as Arc<dyn BlockingWorkService>)),
        ));
        assert!(result.is_ok());
        assert_eq!(blocking.runs.load(Ordering::SeqCst), 2);
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }

    #[test]
    fn unbudgeted_close_without_any_timing_service_escalates_without_waiting() {
        let child = Arc::new(StubbornChild::default());
        let result = futures_executor::block_on(
            task(&child).join_or_escalate(child.as_ref(), unbudgeted(None)),
        );
        assert!(result.is_ok());
        assert_eq!(*child.log.lock().expect("log lock"), vec!["stop", "kill"]);
    }
}
