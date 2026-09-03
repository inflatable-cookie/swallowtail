//! The bound is proved against a real host task, not a cooperative fake.
//!
//! `LocalScopedTaskService` hands back a handle that owns its worker thread:
//! joining it blocks, and so does dropping it. A stalled task is therefore the
//! exact shape that can overrun a caller deadline, so the regression uses one,
//! composed through the real local host so reservation and relinquishment are
//! available.
//!
//! Every task the route later transfers is started under a reservation the
//! operation took before its first effect. That is what makes the transfer
//! non-fallible while the work is unfinished, and it is proved here rather than
//! assumed: an unreserved stalled task keeps ordinary join-and-drop ownership
//! and is refused, and a shutdown that begins after the grant still cannot
//! refuse the handoff.

use super::{TaskEvidence, TaskOwner, bounded_join, reserve_reap, spawn_reserved};
use crate::sdk::bounded::HostBound;
use futures_executor::block_on;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Poll, Waker};
use std::time::{Duration, Instant};
use swallowtail_core::ExecutionHostId;
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, JoinedTask, MonotonicInstant, ScopeId,
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

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("claude-agent-sdk.joins.local").expect("host id is valid")
}

fn scope() -> ScopeId {
    ScopeId::new("claude-agent-sdk:joins-test").expect("scope is valid")
}

fn local_host() -> LocalHostServices {
    LocalProcessHost::builder(LocalProcessLimits::default()).build_services(host_id())
}

fn stalled_body(gate: &Arc<Gate>) -> BoxFuture<'static, ()> {
    let task_gate = Arc::clone(gate);
    Box::pin(std::future::poll_fn(move |context| {
        if task_gate.open.load(Ordering::SeqCst) {
            return Poll::Ready(());
        }
        *task_gate.waker.lock().expect("gate lock poisoned") = Some(context.waker().clone());
        if task_gate.open.load(Ordering::SeqCst) {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }))
}

/// One stalled task started the way every guardian is: reservation first.
fn stalled_task(services: &HostServices, gate: &Arc<Gate>) -> Box<dyn JoinedTask> {
    let reservation = reserve_reap(services, &scope()).expect("the local host reserves reap");
    spawn_reserved(services, reservation, stalled_body(gate))
        .expect("the local host starts a reservation-backed task")
}

/// One stalled task started the ordinary way, with no reservation at all.
fn unreserved_stalled_task(services: &HostServices, gate: &Arc<Gate>) -> Box<dyn JoinedTask> {
    services
        .task()
        .expect("local composition registers a task service")
        .spawn(scope(), stalled_body(gate))
        .expect("local task service spawns")
}

fn bound() -> HostBound {
    HostBound::new(
        Arc::new(ElapsedTime),
        Deadline::at(MonotonicInstant::from_ticks(1)),
    )
}

#[test]
fn a_stalled_local_task_is_relinquished_to_its_exact_host_at_the_deadline() {
    let local = local_host();
    let gate = Arc::new(Gate::default());
    let task = stalled_task(local.services(), &gate);
    let expected = host_id();
    let scope = scope();
    let owner = TaskOwner::new(local.services(), &expected, &scope);

    let started = Instant::now();
    let evidence = block_on(bounded_join(&bound(), &owner, task));
    let elapsed = started.elapsed();

    assert_eq!(
        evidence,
        TaskEvidence::Relinquished,
        "an unfinished task is transferred to its owning host, not joined"
    );
    assert_ne!(
        evidence,
        TaskEvidence::Joined,
        "acceptance for reap is never join evidence"
    );
    assert!(
        elapsed < Duration::from_secs(5),
        "the bounded join returned only after {elapsed:?}, so it blocked on the task"
    );

    // The host reaps it once its own work ends. The outer reaper shutdown is
    // the host lifecycle's call, made here by the test acting as that owner,
    // never by the route.
    gate.open();
    local
        .shutdown_task_reapers()
        .expect("the host joins what it accepted");
}

#[test]
fn a_scope_other_than_the_spawn_scope_cannot_transfer_ownership() {
    let local = local_host();
    let gate = Arc::new(Gate::default());
    let task = stalled_task(local.services(), &gate);
    let expected = host_id();
    let other = ScopeId::new("claude-agent-sdk:joins-test:other").expect("scope is valid");
    let owner = TaskOwner::new(local.services(), &expected, &other);

    let refused = owner
        .transfer(task)
        .expect_err("a mismatched scope must not transfer ownership");
    // Ownership came back unchanged, so ordinary rules still apply to it.
    gate.open();
    drop(refused);
    let _ = local.shutdown_task_reapers();
}

#[test]
fn a_host_other_than_the_selected_one_cannot_transfer_ownership() {
    let local = local_host();
    let gate = Arc::new(Gate::default());
    let task = stalled_task(local.services(), &gate);
    let other = ExecutionHostId::new("claude-agent-sdk.joins.other").expect("host id is valid");
    let scope = scope();
    let owner = TaskOwner::new(local.services(), &other, &scope);

    let refused = owner
        .transfer(task)
        .expect_err("a mismatched execution host must not transfer ownership");
    gate.open();
    drop(refused);
    let _ = local.shutdown_task_reapers();
}

#[test]
fn a_finished_local_task_still_joins() {
    let local = local_host();
    let gate = Arc::new(Gate::default());
    gate.open();
    let task = stalled_task(local.services(), &gate);
    while !task.is_finished() {
        std::thread::yield_now();
    }
    let expected = host_id();
    let scope = scope();
    let owner = TaskOwner::new(local.services(), &expected, &scope);
    assert_eq!(
        block_on(bounded_join(&bound(), &owner, task)),
        TaskEvidence::Joined,
        "a task that finishes is joined, not transferred"
    );
    let _ = local.shutdown_task_reapers();
}

#[test]
fn an_unreserved_stalled_task_keeps_ordinary_join_and_drop_ownership() {
    // The pre-admitted reservation is load-bearing, not decoration. Without it
    // the host refuses the handoff and ownership stays exactly where it was,
    // which is why the route reserves before it acquires anything at all.
    let local = local_host();
    let gate = Arc::new(Gate::default());
    let task = unreserved_stalled_task(local.services(), &gate);
    let expected = host_id();
    let scope = scope();
    let owner = TaskOwner::new(local.services(), &expected, &scope);

    let refused = owner
        .transfer(task)
        .expect_err("an unreserved task cannot be upgraded through a late transfer");
    gate.open();
    drop(refused);
    let _ = local.shutdown_task_reapers();
}

#[test]
fn a_shutdown_that_starts_after_the_grant_cannot_refuse_the_handoff() {
    // The reservation is host-committed capacity, so an outer shutdown racing
    // it waits for the issued grant to settle rather than refusing a valid
    // late handoff.
    let local = local_host();
    let gate = Arc::new(Gate::default());
    let task = stalled_task(local.services(), &gate);
    let expected = host_id();
    let scope = scope();

    let shutdown_local = local.clone();
    let shutdown_gate = Arc::clone(&gate);
    let shutdown = std::thread::spawn(move || {
        // Starts while the grant is live and the task is still stalled.
        let joined = shutdown_local.shutdown_task_reapers();
        assert!(shutdown_gate.open.load(Ordering::SeqCst));
        joined
    });

    let owner = TaskOwner::new(local.services(), &expected, &scope);
    let started = Instant::now();
    let evidence = block_on(bounded_join(&bound(), &owner, task));
    let elapsed = started.elapsed();
    assert_eq!(
        evidence,
        TaskEvidence::Relinquished,
        "a shutdown racing an issued grant must not refuse the handoff"
    );
    assert!(
        elapsed < Duration::from_secs(5),
        "the transfer took {elapsed:?}, so it blocked on the stalled task"
    );

    gate.open();
    shutdown
        .join()
        .expect("the outer owner thread completes")
        .expect("the host joins what it accepted");
}
