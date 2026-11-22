use super::run_before_deadline;
use futures_channel::oneshot;
use std::future::Future;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, HostServices, JoinedTask, MonotonicInstant,
    RuntimeFailure, ScopeId,
};

const GRACEFUL_SHUTDOWN_TICKS: u64 = 500;

pub(crate) fn graceful_shutdown_deadline(
    services: &HostServices,
    hard_deadline: Option<Deadline>,
) -> Option<Deadline> {
    let now = services.time()?.now().ticks();
    // Leave time for force-stop, reap and lease release within a caller's
    // hard deadline, even when the caller supplies less than the usual grace.
    let grace = hard_deadline.map_or(GRACEFUL_SHUTDOWN_TICKS, |deadline| {
        GRACEFUL_SHUTDOWN_TICKS.min(deadline.instant().ticks().saturating_sub(now) / 2)
    });
    Some(Deadline::at(MonotonicInstant::from_ticks(
        now.saturating_add(grace),
    )))
}

pub(crate) fn spawn_session_pump(
    scope: ScopeId,
    services: &HostServices,
    pump: impl Future<Output = ()> + Send + 'static,
) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
    let (done, finished) = oneshot::channel();
    let task = services.task().expect("validated Pi task service").spawn(
        scope,
        Box::pin(async move {
            // Keep the sender alive until the pump returns or unwinds.
            let _done = done;
            pump.await;
        }),
    )?;
    Ok(Box::new(SessionPumpTask { task, finished }))
}

struct SessionPumpTask {
    task: Box<dyn JoinedTask>,
    finished: oneshot::Receiver<()>,
}

impl JoinedTask for SessionPumpTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            // Some hosts block inside join. Wait for the pump to finish
            // before calling it so graceful shutdown can still time out.
            let _ = self.finished.await;
            self.task.join().await
        })
    }
}

pub(crate) async fn join_session_pump(
    task: Box<dyn JoinedTask>,
    cleanup_outcome: impl FnOnce() -> CleanupOutcome,
) -> CleanupOutcome {
    match task.join().await {
        Ok(()) => cleanup_outcome(),
        Err(_) => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.pi.pump_join_failed",
            "Pi protocol task did not join cleanly",
        )),
    }
}

pub(crate) async fn stop_session_process(
    graceful_stop: impl Future<Output = ()>,
    pump_join: impl Future<Output = CleanupOutcome>,
    force_stop: impl Future<Output = Result<(), RuntimeFailure>>,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> CleanupOutcome {
    let mut pump_join = std::pin::pin!(pump_join);
    let graceful = run_before_deadline(
        async {
            graceful_stop.await;
            Ok(pump_join.as_mut().await)
        },
        deadline,
        services,
        "swallowtail.pi.shutdown_grace_expired",
        "Pi process did not finish during its graceful shutdown window",
    )
    .await;
    match graceful {
        Ok(outcome) => outcome,
        Err(timeout) => {
            let forced = force_stop.await;
            // The pump owns the process wait. Resume the same join so it reaps
            // the process before the session releases its leases.
            let reaped = pump_join.await;
            let escalation = match forced {
                Ok(()) => CleanupOutcome::Degraded(timeout.diagnostic().clone()),
                Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
            };
            CleanupOutcome::merge(escalation, reaped)
        }
    }
}
