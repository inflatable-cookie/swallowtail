//! The bound is proved against a real host task, not a cooperative fake.
//!
//! `LocalScopedTaskService` hands back a handle that owns its worker thread:
//! joining it blocks, and so does dropping it. A stalled task is therefore the
//! exact shape that can overrun a caller deadline, so the regression uses one.

use super::{bounded_join, join_if_finished};
use crate::sdk::bounded::HostBound;
use futures_executor::block_on;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::time::{Duration, Instant};
use swallowtail_core::ExecutionHostId;
use swallowtail_host_local::LocalScopedTaskService;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, ScopeId, ScopedTaskService,
    TimeService,
};

/// A clock whose deadline is always already observed, so the bound is the only
/// thing that can end a wait here.
struct ElapsedTime;

impl TimeService for ElapsedTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(u64::MAX)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move {
            DeadlineObservation::new(deadline, MonotonicInstant::from_ticks(u64::MAX))
        })
    }
}

/// A task that only ends when the test releases it.
#[derive(Default)]
struct Gate {
    open: AtomicBool,
    waker: Mutex<Option<Waker>>,
}

impl Gate {
    fn open(&self) {
        self.open.store(true, Ordering::SeqCst);
        if let Some(waker) = self.waker.lock().expect("gate lock poisoned").take() {
            waker.wake();
        }
    }
}

fn stalled_task(gate: &Arc<Gate>) -> Box<dyn swallowtail_runtime::JoinedTask> {
    let service = LocalScopedTaskService::new(
        ExecutionHostId::new("claude-agent-sdk.joins.local").expect("host id is valid"),
    );
    let task_gate = Arc::clone(gate);
    service
        .spawn(
            ScopeId::new("claude-agent-sdk:joins-test").expect("scope is valid"),
            Box::pin(std::future::poll_fn(move |context| {
                if task_gate.open.load(Ordering::SeqCst) {
                    return Poll::Ready(());
                }
                *task_gate.waker.lock().expect("gate lock poisoned") =
                    Some(context.waker().clone());
                if task_gate.open.load(Ordering::SeqCst) {
                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            })),
        )
        .expect("local task service spawns")
}

fn bound() -> HostBound {
    HostBound::new(
        Arc::new(ElapsedTime),
        Deadline::at(MonotonicInstant::from_ticks(1)),
    )
}

#[test]
fn a_stalled_local_task_cannot_hold_a_bounded_join() {
    let gate = Arc::new(Gate::default());
    let task = stalled_task(&gate);

    let started = Instant::now();
    let joined = block_on(bounded_join(&bound(), task));
    let elapsed = started.elapsed();

    assert!(!joined, "an unfinished task is never reported as joined");
    assert!(
        elapsed < Duration::from_secs(5),
        "the bounded join returned only after {elapsed:?}, so it blocked on the task"
    );

    // Retention is not detachment: the handle is still owned, and the task
    // still ends when its own work does.
    gate.open();
}

#[test]
fn an_unfinished_local_task_is_retained_rather_than_dropped() {
    let gate = Arc::new(Gate::default());
    let task = stalled_task(&gate);

    let started = Instant::now();
    let joined = block_on(join_if_finished(task));
    let elapsed = started.elapsed();

    assert!(!joined, "an unfinished task is never reported as joined");
    assert!(
        elapsed < Duration::from_secs(5),
        "releasing an unfinished handle blocked for {elapsed:?}"
    );
    gate.open();
}

#[test]
fn a_finished_local_task_still_joins() {
    let gate = Arc::new(Gate::default());
    gate.open();
    let task = stalled_task(&gate);
    while !task.is_finished() {
        std::thread::yield_now();
    }
    assert!(
        block_on(bounded_join(&bound(), task)),
        "a task that finishes is joined, not retained"
    );
}
