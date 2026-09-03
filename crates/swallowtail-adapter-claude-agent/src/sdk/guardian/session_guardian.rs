//! The enclosing guardian for closing one ready session.
//!
//! Close hands this guardian the whole owned set at once — the connection, the
//! sidecar process, the pump, any remaining turn-deadline task, the
//! working-resource lease, and the credential lease — and the guardian runs the
//! single ordered continuation in [`super::cleanup`].
//!
//! The caller waits for that continuation to finish inside its own cleanup
//! deadline. When the deadline arrives first, the caller transfers *this
//! guardian* through the reap reservation the session pre-admitted at open. It
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

/// One host-owned task that owns everything a closing session still holds.
pub(crate) struct SessionGuardian {
    scope: ScopeId,
    execution_host_id: ExecutionHostId,
    cleaned: Arc<Signal>,
    report: Arc<Mutex<Option<CleanupReport>>>,
    task: Mutex<Option<Box<dyn JoinedTask>>>,
}

impl SessionGuardian {
    /// Starts the guardian under the reservation the session has held since
    /// before its first acquisition, so the later transfer cannot be refused
    /// while the continuation is unfinished.
    pub(crate) fn arm(
        services: &HostServices,
        reservation: Box<dyn TaskReapReservation>,
        scope: ScopeId,
        request_id: &str,
        deadline: Deadline,
        owned: Owned,
        turn_active: bool,
    ) -> Result<Self, RuntimeFailure> {
        let time = services
            .time()
            .cloned()
            .expect("validated sidecar time service");
        let task_services = services.clone();
        let cleaned = Arc::new(Signal::default());
        let task_cleaned = Arc::clone(&cleaned);
        let report: Arc<Mutex<Option<CleanupReport>>> = Arc::new(Mutex::new(None));
        let task_report = Arc::clone(&report);
        let guardian_request_id = request_id.to_owned();
        let task = joins::spawn_reserved(
            services,
            reservation,
            Box::pin(async move {
                let bounded = HostBound::new(time, deadline);
                let finished = cleanup::run(
                    owned,
                    &task_services,
                    &bounded,
                    &guardian_request_id,
                    Cooperative::Session { turn_active },
                )
                .await;
                *task_report
                    .lock()
                    .expect("SDK session-guardian report lock poisoned") = Some(finished);
                task_cleaned.trigger();
            }),
        )?;
        Ok(Self {
            scope,
            execution_host_id: services.execution_host_id().clone(),
            cleaned,
            report,
            task: Mutex::new(Some(task)),
        })
    }

    /// Waits for the ordered continuation inside the caller's bound.
    ///
    /// `Some(report)` means every stage ran, in order, and the outcome may be
    /// decided from what the host actually observed. `None` means the caller's
    /// deadline arrived first: the guardian was handed to its owning host and
    /// still owns the process, the pump, and both leases, so the caller reports
    /// unconfirmed cleanup without waiting.
    pub(crate) async fn settle(
        &self,
        bounded: &HostBound,
        services: &HostServices,
    ) -> Option<CleanupReport> {
        let completed = bounded.run(self.cleaned.future()).await.is_some();
        let task = self
            .task
            .lock()
            .expect("SDK session-guardian task lock poisoned")
            .take();
        if let Some(task) = task {
            let owner = TaskOwner::new(services, &self.execution_host_id, &self.scope);
            bounded_join(bounded, &owner, task).await;
        }
        if completed {
            self.report
                .lock()
                .expect("SDK session-guardian report lock poisoned")
                .take()
        } else {
            None
        }
    }
}
