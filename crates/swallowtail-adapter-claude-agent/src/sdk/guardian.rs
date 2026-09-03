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

use crate::sdk::failure::failure;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
pub(crate) use watchdog::EscalationWatchdog;

mod watchdog;

use swallowtail_runtime::{
    CredentialLease, Deadline, HostServices, JoinedTask, ProcessHandle, ResourceLease,
    RuntimeFailure, ScopeId,
};

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

/// Everything the open path has acquired so far.
///
/// Recorded as each acquisition succeeds, so cleanup covers a partial open, not
/// only a complete one.
#[derive(Default)]
pub(crate) struct Acquisitions {
    pub(crate) credential: Option<CredentialLease>,
    pub(crate) resource: Option<ResourceLease>,
    pub(crate) process: Option<Arc<dyn ProcessHandle>>,
    pub(crate) pump: Option<Box<dyn JoinedTask>>,
}

impl Acquisitions {
    fn take(&mut self) -> Self {
        Self {
            credential: self.credential.take(),
            resource: self.resource.take(),
            process: self.process.take(),
            pump: self.pump.take(),
        }
    }
}

/// Guards one open attempt: on the caller's deadline, or on an explicit
/// failure signal, it terminates whatever the open path had acquired and
/// releases the leases in contract order.
pub(crate) struct OpenGuard {
    ledger: Arc<Mutex<Acquisitions>>,
    signal: Arc<Signal>,
    claimed: Arc<AtomicBool>,
    /// Set when the guard woke on the caller's deadline rather than on the
    /// failure signal, so the open path can report the deadline as the cause
    /// instead of whatever the collapsing connection said next.
    deadline_fired: Arc<AtomicBool>,
    // Behind a mutex so the guard stays `Sync`: the open future holds a
    // reference to it across awaits.
    task: Mutex<Option<Box<dyn JoinedTask>>>,
}

impl OpenGuard {
    /// Arms the guard before the first acquisition, so nothing can be acquired
    /// outside its reach.
    pub(crate) fn arm(
        services: &HostServices,
        request_id: &str,
        deadline: Deadline,
    ) -> Result<Self, RuntimeFailure> {
        let ledger: Arc<Mutex<Acquisitions>> = Arc::new(Mutex::new(Acquisitions::default()));
        let signal = Arc::new(Signal::default());
        let claimed = Arc::new(AtomicBool::new(false));
        let deadline_fired = Arc::new(AtomicBool::new(false));
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
        let task_claimed = Arc::clone(&claimed);
        let task_deadline_fired = Arc::clone(&deadline_fired);
        let task = services
            .task()
            .expect("validated sidecar task service")
            .spawn(
                scope,
                Box::pin(async move {
                    let mut expiry = time.wait_until(deadline);
                    let mut fired = Box::pin(task_signal.future());
                    std::future::poll_fn(|context| {
                        if fired.as_mut().poll(context).is_ready() {
                            Poll::Ready(())
                        } else if expiry.as_mut().poll(context).is_ready() {
                            task_deadline_fired.store(true, Ordering::SeqCst);
                            Poll::Ready(())
                        } else {
                            Poll::Pending
                        }
                    })
                    .await;
                    if task_claimed.load(Ordering::SeqCst) {
                        return;
                    }
                    let acquired = task_ledger
                        .lock()
                        .expect("SDK open-guard ledger lock poisoned")
                        .take();
                    release(acquired, &task_services).await;
                }),
            )?;
        Ok(Self {
            ledger,
            signal,
            claimed,
            deadline_fired,
            task: Mutex::new(Some(task)),
        })
    }

    pub(crate) fn record_credential(&self, lease: CredentialLease) {
        self.ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
            .credential = Some(lease);
    }

    pub(crate) fn record_resource(&self, lease: ResourceLease) {
        self.ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
            .resource = Some(lease);
    }

    pub(crate) fn record_process(&self, process: Arc<dyn ProcessHandle>) {
        self.ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
            .process = Some(process);
    }

    pub(crate) fn record_pump(&self, pump: Box<dyn JoinedTask>) {
        self.ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
            .pump = Some(pump);
    }

    /// Reports whether the caller's deadline, rather than a failure, released
    /// this guard.
    pub(crate) fn deadline_fired(&self) -> bool {
        self.deadline_fired.load(Ordering::SeqCst)
    }

    /// Takes ownership back on the success path. The guard task then exits
    /// without touching anything.
    pub(crate) fn claim(&self) -> Acquisitions {
        self.claimed.store(true, Ordering::SeqCst);
        let acquired = self
            .ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned")
            .take();
        self.signal.trigger();
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
            Some(task) => bounded
                .run(task.join())
                .await
                .is_some_and(|joined| joined.is_ok()),
            None => true,
        }
    }

    /// Validates the recorded credential lease without removing it, so a
    /// rejected lease is still released by the guard rather than by the caller.
    pub(crate) fn credential_matches(
        &self,
        scope: &ScopeId,
        reference: &swallowtail_core::CredentialRef,
        audience: &swallowtail_core::EndpointAudience,
    ) -> bool {
        let ledger = self
            .ledger
            .lock()
            .expect("SDK open-guard ledger lock poisoned");
        matches!(ledger.credential, Some(CredentialLease::Delegated(_)))
            && ledger.credential.as_ref().is_some_and(|lease| {
                lease.scope() == scope
                    && lease.reference() == reference
                    && lease.audience() == audience
            })
    }
}

async fn release(mut acquired: Acquisitions, services: &HostServices) {
    // Termination first: it is a request, and a stalled join must not delay it.
    if let Some(process) = acquired.process.take() {
        let _ = process.force_stop().await;
        let _ = process.wait().await;
    }
    if let Some(pump) = acquired.pump.take() {
        let _ = pump.join().await;
    }
    if let (Some(lease), Some(service)) = (acquired.resource.take(), services.working_resource()) {
        let _ = service.release(lease).await;
    }
    if let (Some(lease), Some(service)) = (acquired.credential.take(), services.credential()) {
        let _ = service.release(lease).await;
    }
}
