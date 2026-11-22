use crate::exec_events::ExecEventParser;
use crate::exec_handle::ProcessCancellation;
use crate::exec_input::SharedExecMaterializations;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::task::Poll;
use std::time::{Duration, Instant};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, DebugObservationKind, HostServices,
    MonotonicInstant, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "codex.exec";
/// Dedicated teardown budget for reaping a stopped exec child during forced
/// cleanup so a refused `force_stop()` or hung `wait()` cannot block forever.
const CLEANUP_WAIT_BUDGET_TICKS: u64 = 500;
/// Bounded wall-clock fallback grace for `force_cleanup` when no `TimeService`
/// is supplied by the host.
const CLOCKLESS_TEARDOWN_GRACE: Duration = Duration::from_millis(100);

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<ProcessCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    materializations: SharedExecMaterializations,
    parser: ExecEventParser,
    services: HostServices,
) -> TerminalOutcome {
    let outcome = {
        let mut body = std::pin::pin!(pump_process(
            &process,
            &events,
            &cancellation,
            &mut deadline,
            parser,
            &services,
        ));
        poll_fn(|context| {
            if cancellation.poll_pump_aborted(context).is_ready() {
                return Poll::Ready(TerminalOutcome::new(
                    TerminalStatus::Cancelled,
                    process_cleanup_failed(),
                ));
            }
            body.as_mut().poll(context)
        })
        .await
    };
    with_cleanup(outcome, materializations.release().await)
}

async fn pump_process(
    process: &Arc<dyn ProcessHandle>,
    events: &RuntimeEventSender,
    cancellation: &Arc<ProcessCancellation>,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
    mut parser: ExecEventParser,
    services: &HostServices,
) -> TerminalOutcome {
    loop {
        match next_output(process.as_ref(), cancellation, deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref(), services).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        for event in parsed {
                            if events.send(event).is_err() {
                                let cleanup = force_cleanup(process.as_ref(), services).await;
                                return TerminalOutcome::new(
                                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                        "swallowtail.codex.exec.event_delivery_failed",
                                        "Codex exec event delivery failed",
                                    )),
                                    cleanup,
                                );
                            }
                        }
                    }
                    Err(failure) => {
                        emit_protocol_debug(services, &failure, "exec.pump.decode");
                        let cleanup = force_cleanup(process.as_ref(), services).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(failure)) => {
                emit_host_process_debug(services, &failure, "exec.pump.read");
                let cleanup = force_cleanup(process.as_ref(), services).await;
                return TerminalOutcome::new(
                    TerminalStatus::HostFailed(failure.diagnostic().clone()),
                    cleanup,
                );
            }
        }
    }
    let exit = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit));
    }
    match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            for event in trailing {
                if events.send(event).is_err() {
                    return TerminalOutcome::new(
                        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                            "swallowtail.codex.exec.event_delivery_failed",
                            "Codex exec event delivery failed",
                        )),
                        CleanupOutcome::Clean,
                    );
                }
            }
            parsed.outcome(exit.success())
        }
        (Err(failure), exit) => {
            emit_protocol_debug(services, &failure, "exec.pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                cleanup_from_wait(&exit),
            )
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.codex.exec.process_wait_failed",
                "Codex exec process wait failed",
            );
            services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "exec.pump.wait",
                diagnostic.code(),
                diagnostic.message(),
            );
            TerminalOutcome::new(
                TerminalStatus::HostFailed(diagnostic),
                process_cleanup_failed(),
            )
        }
    }
}

fn emit_protocol_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::ProtocolParse,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

fn emit_host_process_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &ProcessCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| {
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextOutput::Deadline);
        }
        read.as_mut().poll(context).map(NextOutput::Process)
    })
    .await
}

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle, services: &HostServices) {
    let _ = force_cleanup(process, services).await;
}

struct TeardownTimer<'a> {
    services: &'a HostServices,
    timer: Option<BoxFuture<'static, DeadlineObservation>>,
    clockless_deadline: Option<Instant>,
    exhausted: bool,
}

impl<'a> TeardownTimer<'a> {
    fn new(services: &'a HostServices) -> Self {
        Self {
            services,
            timer: None,
            clockless_deadline: None,
            exhausted: false,
        }
    }

    async fn race<T>(&mut self, mut step: BoxFuture<'_, T>) -> Option<T> {
        poll_fn(|context| {
            if let Poll::Ready(result) = step.as_mut().poll(context) {
                return Poll::Ready(Some(result));
            }
            if self.exhausted {
                return Poll::Ready(None);
            }
            if let Some(time) = self.services.time() {
                let timer = self.timer.get_or_insert_with(|| {
                    let deadline = Deadline::at(MonotonicInstant::from_ticks(
                        time.now().ticks().saturating_add(CLEANUP_WAIT_BUDGET_TICKS),
                    ));
                    time.wait_until(deadline)
                });
                if timer.as_mut().poll(context).is_ready() {
                    self.timer = None;
                    self.exhausted = true;
                    return Poll::Ready(None);
                }
                return Poll::Pending;
            }
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
}

async fn force_cleanup(process: &dyn ProcessHandle, services: &HostServices) -> CleanupOutcome {
    let mut timer = TeardownTimer::new(services);
    let Some(force) = timer.race(process.force_stop()).await else {
        return process_cleanup_failed();
    };
    if force.is_err() && services.time().is_none() {
        // Without a time service to bound `wait()`, a refused `force_stop()`
        // leaves the child running; do not hang indefinitely waiting for exit.
        return process_cleanup_failed();
    }
    let Some(wait) = timer.race(process.wait()).await else {
        return process_cleanup_failed();
    };
    if force.is_err() || wait.is_err() {
        process_cleanup_failed()
    } else {
        CleanupOutcome::Clean
    }
}

fn cleanup_from_wait(
    exit: &Result<swallowtail_runtime::ProcessExit, RuntimeFailure>,
) -> CleanupOutcome {
    if exit.is_ok() {
        CleanupOutcome::Clean
    } else {
        process_cleanup_failed()
    }
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.codex.exec.process_cleanup_failed",
        "Codex exec process cleanup failed",
    ))
}

fn with_cleanup(outcome: TerminalOutcome, materialization: CleanupOutcome) -> TerminalOutcome {
    let cleanup = CleanupOutcome::merge(outcome.cleanup().clone(), materialization);
    let mut combined = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output() {
        combined = combined.with_output(output.clone());
    }
    combined
}

#[cfg(test)]
mod tests {
    use super::with_cleanup;
    use swallowtail_core::SafeDiagnostic;
    use swallowtail_runtime::{CleanupOutcome, TerminalOutcome, TerminalStatus};

    fn diagnostic(code: &'static str) -> SafeDiagnostic {
        SafeDiagnostic::new(code, "message")
    }

    #[test]
    fn failed_materialization_cleanup_beats_degraded_process_cleanup() {
        let outcome = TerminalOutcome::new(
            TerminalStatus::Cancelled,
            CleanupOutcome::Degraded(diagnostic("process.degraded")),
        );
        let combined = with_cleanup(
            outcome,
            CleanupOutcome::Failed(diagnostic("materialization.failed")),
        );
        assert!(matches!(
            combined.cleanup(),
            CleanupOutcome::Failed(found) if found.code() == "materialization.failed"
        ));
    }

    #[test]
    fn degraded_materialization_cleanup_survives_clean_process_cleanup() {
        let outcome = TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean);
        let combined = with_cleanup(
            outcome,
            CleanupOutcome::Degraded(diagnostic("materialization.degraded")),
        );
        assert!(matches!(
            combined.cleanup(),
            CleanupOutcome::Degraded(found) if found.code() == "materialization.degraded"
        ));
    }

    #[test]
    fn force_cleanup_bounds_refused_force_stop_with_teardown_budget() {
        use super::force_cleanup;
        use std::sync::Arc;
        use swallowtail_core::ExecutionHostId;
        use swallowtail_runtime::{
            BoxFuture, Deadline, DeadlineObservation, HostServices, MonotonicInstant, ProcessExit,
            ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, TimeService,
        };

        struct RefusingHungWaitProcess;

        impl ProcessHandle for RefusingHungWaitProcess {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "test.force_stop_refused",
                        "force stop refused",
                    )))
                })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
        }

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

        let services_with_time = HostServices::new(
            ExecutionHostId::new("codex.exec.cleanup-budget-test").expect("host id"),
        )
        .with_time(Arc::new(PastClock));
        let outcome = futures_executor::block_on(force_cleanup(
            &RefusingHungWaitProcess,
            &services_with_time,
        ));
        assert!(matches!(
            outcome,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));

        let services_without_time = HostServices::new(
            ExecutionHostId::new("codex.exec.cleanup-notime-test").expect("host id"),
        );
        let outcome_without_time = futures_executor::block_on(force_cleanup(
            &RefusingHungWaitProcess,
            &services_without_time,
        ));
        assert!(matches!(
            outcome_without_time,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));

        struct PendingStopProcess;

        impl ProcessHandle for PendingStopProcess {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
        }

        struct AcceptedStopHungWaitProcess;

        impl ProcessHandle for AcceptedStopHungWaitProcess {
            fn write_stdin(
                &self,
                _: ProcessInputChunk,
            ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn read_output(
                &self,
            ) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
            fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
                Box::pin(async { Ok(()) })
            }
            fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
                Box::pin(std::future::pending())
            }
        }

        let pending_stop_outcome =
            futures_executor::block_on(force_cleanup(&PendingStopProcess, &services_with_time));
        assert!(matches!(
            pending_stop_outcome,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));

        let accepted_stop_hung_wait = futures_executor::block_on(force_cleanup(
            &AcceptedStopHungWaitProcess,
            &services_with_time,
        ));
        assert!(matches!(
            accepted_stop_hung_wait,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));

        let pending_stop_without_time =
            futures_executor::block_on(force_cleanup(&PendingStopProcess, &services_without_time));
        assert!(matches!(
            pending_stop_without_time,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));

        let accepted_stop_hung_wait_without_time = futures_executor::block_on(force_cleanup(
            &AcceptedStopHungWaitProcess,
            &services_without_time,
        ));
        assert!(matches!(
            accepted_stop_hung_wait_without_time,
            CleanupOutcome::Failed(ref diag) if diag.code() == "swallowtail.codex.exec.process_cleanup_failed"
        ));
    }
}
