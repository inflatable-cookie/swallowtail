//! The enclosing guardian for one session's whole cleanup continuation.
//!
//! The guardian task is started **before the session takes any effect**, under
//! the reap reservation `open_session` pre-admitted. Nothing about activating it
//! later is fallible: close moves the owned set into a slot and triggers a
//! signal. That matters because the fallible part of starting host work is
//! creating the worker, and a worker creation that fails after the session holds
//! a live pump, process, and two leases would drop them outside any ordered
//! continuation. Reserving capacity does not make worker creation infallible, so
//! the creation itself is moved ahead of every effect.
//!
//! Activation hands the guardian the connection, the sidecar process, the pump,
//! any remaining turn-deadline task, and both leases at once, and the guardian
//! runs the single ordered continuation in [`super::cleanup`].
//!
//! The caller waits for that continuation inside its own cleanup deadline. Every
//! way that wait can end without the continuation finishing — expiry, caller
//! cancellation, or the runtime rejecting the public cleanup future before it is
//! ever polled — transfers *this guardian* through the held reservation. It
//! never transfers the pump on its own, never releases a lease around work that
//! is still live, and never reports a stronger cleanup outcome than the host
//! observed. `AcceptedForReap` says only that the host took ownership of the
//! remaining continuation.

use super::cleanup::{CleanupReport, Cooperative, Owned};
use super::{Signal, TaskOwner, bounded_join, cleanup, joins};
use crate::sdk::bounded::HostBound;
use std::sync::{Arc, Mutex};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    Deadline, HostServices, JoinedTask, RuntimeFailure, ScopeId, TaskReapReservation,
};

/// One activation: everything the guardian must clean up, and how.
struct CleanupWork {
    owned: Owned,
    cooperative: Cooperative,
    deadline: Deadline,
}

/// State shared with the guardian's own task.
#[derive(Default)]
struct GuardianState {
    /// `Some` once close activated the guardian. Taken by the task.
    work: Mutex<Option<CleanupWork>>,
    /// Triggered by activation or by abandonment; either ends the task's wait.
    started: Arc<Signal>,
    /// Triggered only after the ordered continuation ran to its last release.
    cleaned: Arc<Signal>,
    report: Mutex<Option<CleanupReport>>,
}

/// One host-owned task that owns everything a closing session still holds.
pub(crate) struct SessionGuardian {
    scope: ScopeId,
    execution_host_id: ExecutionHostId,
    services: HostServices,
    state: Arc<GuardianState>,
    task: Mutex<Option<Box<dyn JoinedTask>>>,
}

impl SessionGuardian {
    /// Starts the guardian task before the session acquires anything.
    ///
    /// Only this call can fail, and it fails while the operation still owns
    /// nothing: no credential, no working resource, no sidecar process, no
    /// pump, and no provider contact. After it returns, activation is
    /// infallible.
    pub(crate) fn arm(
        services: &HostServices,
        reservation: Box<dyn TaskReapReservation>,
        scope: ScopeId,
        request_id: &str,
    ) -> Result<Self, RuntimeFailure> {
        let time = services
            .time()
            .cloned()
            .expect("validated sidecar time service");
        let task_services = services.clone();
        let state = Arc::new(GuardianState::default());
        let task_state = Arc::clone(&state);
        let guardian_request_id = request_id.to_owned();
        let task = joins::spawn_reserved(
            services,
            reservation,
            Box::pin(async move {
                task_state.started.future().await;
                let Some(work) = task_state
                    .work
                    .lock()
                    .expect("SDK session-guardian work lock poisoned")
                    .take()
                else {
                    // Abandoned without a session: nothing was ever handed
                    // over, so the task simply ends and its reservation
                    // settles.
                    return;
                };
                let bounded = HostBound::new(time, work.deadline);
                let finished = cleanup::run(
                    work.owned,
                    &task_services,
                    &bounded,
                    &guardian_request_id,
                    work.cooperative,
                )
                .await;
                *task_state
                    .report
                    .lock()
                    .expect("SDK session-guardian report lock poisoned") = Some(finished);
                task_state.cleaned.trigger();
            }),
        )?;
        Ok(Self {
            scope,
            execution_host_id: services.execution_host_id().clone(),
            services: services.clone(),
            state: Arc::clone(&state),
            task: Mutex::new(Some(task)),
        })
    }

    /// Hands the guardian the whole owned set. Infallible by construction.
    pub(crate) fn activate(&self, owned: Owned, cooperative: Cooperative, deadline: Deadline) {
        *self
            .state
            .work
            .lock()
            .expect("SDK session-guardian work lock poisoned") = Some(CleanupWork {
            owned,
            cooperative,
            deadline,
        });
        self.state.started.trigger();
    }

    /// Waits for the ordered continuation inside the caller's bound.
    ///
    /// `Some(report)` means every stage ran, in order, and the outcome may be
    /// decided from what the host actually observed. `None` means the caller's
    /// deadline arrived first: the guardian was handed to its owning host and
    /// still owns the process, the pump, and both leases, so the caller reports
    /// unconfirmed cleanup without waiting.
    pub(crate) async fn settle(&self, bounded: &HostBound) -> Option<CleanupReport> {
        let completed = bounded.run(self.state.cleaned.future()).await.is_some();
        let task = self
            .task
            .lock()
            .expect("SDK session-guardian task lock poisoned")
            .take();
        if let Some(task) = task {
            let owner = TaskOwner::new(&self.services, &self.execution_host_id, &self.scope);
            bounded_join(bounded, &owner, task).await;
        }
        if completed {
            self.state
                .report
                .lock()
                .expect("SDK session-guardian report lock poisoned")
                .take()
        } else {
            None
        }
    }
}

/// Dropping a guardian is a handoff, never a synchronous join.
///
/// This is the path a cancelled or rejected public cleanup takes: the runtime
/// can refuse an already-elapsed deadline, a missing time service, or the wrong
/// host before the cleanup future is ever polled, and a caller can drop that
/// future after one pending poll. Either way the guardian is already activated
/// and already owns the whole continuation, so ownership must move to the host
/// rather than be joined on the dropping thread.
impl Drop for SessionGuardian {
    fn drop(&mut self) {
        // A guardian that was never activated must still be released, or its
        // reservation would never settle and outer shutdown would wait forever.
        self.state.started.trigger();
        let mut task = self
            .task
            .lock()
            .expect("SDK session-guardian task lock poisoned");
        if task.is_none() {
            return;
        }
        if let Some(service) = self.services.task() {
            let _ = swallowtail_runtime::ScopedTaskService::relinquish(
                service.as_ref(),
                &self.scope,
                &mut task,
            );
        }
    }
}
