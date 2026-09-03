//! Host-owned guards that make termination and release unskippable.
//!
//! A caller deadline bounds the *public future*. Dropping that future at expiry
//! must not strand an acquired credential lease, a resolved resource, a running
//! sidecar, or a pump task, and it must not skip the host termination request.
//!
//! So the work that must still happen lives in a host task instead of in the
//! future the caller is waiting on. The task fires on whichever comes first, an
//! explicit signal or the caller's deadline, and does nothing at all once the
//! success path has claimed what it guards. Nothing here invents a duration:
//! the deadline is the caller's own, observed through the host `TimeService`.
//!
//! Claim and cleanup are one atomic choice made under a single lock; see
//! [`ledger`] for why three separate pieces of state cannot express it.

use crate::sdk::failure::failure;
pub(crate) use joins::{bounded_join, join_if_finished};
pub(crate) use ledger::Acquisitions;
pub(crate) use ledger::RecordingLease;
use ledger::{DeadlineFlag, GuardLedger};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
pub(crate) use watchdog::EscalationWatchdog;

mod joins;
mod ledger;
mod watchdog;

use swallowtail_runtime::{Deadline, HostServices, JoinedTask, RuntimeFailure, ScopeId};

#[derive(Default)]
struct SignalState {
    triggered: bool,
    waiter: Option<Waker>,
}

/// One-shot cross-task signal, so a guard can be released early without
/// waiting for the deadline it also watches.
#[derive(Default)]
pub(crate) struct Signal {
    state: Mutex<SignalState>,
}

impl Signal {
    pub(crate) fn trigger(&self) {
        let mut state = self.state.lock().expect("SDK guard signal lock poisoned");
        state.triggered = true;
        if let Some(waiter) = state.waiter.take() {
            waiter.wake();
        }
    }

    pub(super) fn future(self: &Arc<Self>) -> SignalFuture {
        SignalFuture(Arc::clone(self))
    }
}

pub(super) struct SignalFuture(Arc<Signal>);

impl Future for SignalFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.0.state.lock().expect("SDK guard signal lock poisoned");
        if state.triggered {
            Poll::Ready(())
        } else {
            state.waiter = Some(context.waker().clone());
            Poll::Pending
        }
    }
}

/// Guards one open attempt: on the caller's deadline, or on an explicit
/// failure signal, it terminates whatever the open path had acquired and
/// releases the leases in contract order.
pub(crate) struct OpenGuard {
    ledger: Arc<GuardLedger>,
    signal: Arc<Signal>,
    deadline: Arc<DeadlineFlag>,
    // Behind a mutex so the guard stays `Sync`: the open future holds a
    // reference to it across awaits.
    task: Mutex<Option<Box<dyn JoinedTask>>>,
}

impl OpenGuard {
    /// Arms the guard before the first acquisition, so nothing can be acquired
    /// outside its reach. The returned lease must be held by the open future
    /// itself: dropping it is what tells cleanup that no further acquisition
    /// can arrive.
    pub(crate) fn arm(
        services: &HostServices,
        request_id: &str,
        deadline: Deadline,
    ) -> Result<(Self, RecordingLease), RuntimeFailure> {
        let (ledger, lease) = GuardLedger::new();
        let signal = Arc::new(Signal::default());
        let fired = Arc::new(DeadlineFlag::default());
        let scope =
            ScopeId::new(format!("claude-agent-sdk:open-guard:{request_id}")).map_err(|_| {
                failure(
                    "swallowtail.claude-agent.sdk.scope_invalid",
                    "Claude Agent SDK sidecar open-guard scope was invalid",
                )
            })?;
        let time = services
            .time()
            .cloned()
            .expect("validated sidecar time service");
        let task_services = services.clone();
        let task_ledger = Arc::clone(&ledger);
        let task_signal = Arc::clone(&signal);
        let task_fired = Arc::clone(&fired);
        let task = services
            .task()
            .expect("validated sidecar task service")
            .spawn(
                scope,
                Box::pin(async move {
                    let mut expiry = time.wait_until(deadline);
                    let mut signalled = Box::pin(task_signal.future());
                    std::future::poll_fn(|context| {
                        if signalled.as_mut().poll(context).is_ready() {
                            Poll::Ready(())
                        } else if expiry.as_mut().poll(context).is_ready() {
                            task_fired.set();
                            Poll::Ready(())
                        } else {
                            Poll::Pending
                        }
                    })
                    .await;
                    // One atomic choice: either open already owns what it
                    // acquired, or this guard does.
                    if !task_ledger.begin_cleanup() {
                        return;
                    }
                    let acquired = task_ledger.take_for_cleanup().await;
                    release(acquired, &task_services).await;
                }),
            )?;
        Ok((
            Self {
                ledger,
                signal,
                deadline: fired,
                task: Mutex::new(Some(task)),
            },
            lease,
        ))
    }

    pub(crate) fn ledger(&self) -> &Arc<GuardLedger> {
        &self.ledger
    }

    /// Reports whether the caller's deadline, rather than a failure, released
    /// this guard.
    pub(crate) fn deadline_fired(&self) -> bool {
        self.deadline.fired()
    }

    /// Takes ownership back on the success path. `None` means cleanup won the
    /// transition, so open must not report success. The guard task then exits
    /// without touching anything when a claim succeeds.
    pub(crate) fn claim(&self) -> Option<Acquisitions> {
        let acquired = self.ledger.claim();
        if acquired.is_some() {
            self.signal.trigger();
        }
        acquired
    }

    /// Releases the guard on a failure path and joins its task while the
    /// caller's bound allows. Returns whether cleanup completed inside the
    /// bound; an unjoined guard keeps running under host ownership.
    pub(crate) async fn fire(&self, bounded: &super::bounded::HostBound) -> bool {
        self.signal.trigger();
        let task = self
            .task
            .lock()
            .expect("SDK open-guard task lock poisoned")
            .take();
        match task {
            Some(task) => bounded_join(bounded, task).await,
            None => true,
        }
    }
}

async fn release(mut acquired: Acquisitions, services: &HostServices) {
    // Termination first: it is a request, and a stalled join must not delay it.
    if let Some(process) = acquired.process.take() {
        let _ = process.force_stop().await;
        let _ = process.wait().await;
    }
    // The pump is joined only if it already finished. A pump still blocked on a
    // stopped sidecar's transport must not hold the leases below hostage, and
    // its handle stays owned rather than dropped.
    if let Some(pump) = acquired.pump.take() {
        let _ = join_if_finished(pump).await;
    }
    if let (Some(lease), Some(service)) = (acquired.resource.take(), services.working_resource()) {
        let _ = service.release(lease).await;
    }
    if let (Some(lease), Some(service)) = (acquired.credential.take(), services.credential()) {
        let _ = service.release(lease).await;
    }
}
