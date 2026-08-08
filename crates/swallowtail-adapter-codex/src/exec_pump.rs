use crate::exec_events::ExecEventParser;
use crate::exec_handle::ProcessCancellation;
use crate::exec_input::SharedExecMaterializations;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind, HostServices,
    ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender, RuntimeFailure,
    TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "codex.exec";

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<ProcessCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    materializations: SharedExecMaterializations,
    parser: ExecEventParser,
    services: HostServices,
) -> TerminalOutcome {
    let outcome =
        pump_process(&process, &events, &cancellation, &mut deadline, parser, &services).await;
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
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        for event in parsed {
                            if events.send(event).is_err() {
                                let cleanup = force_cleanup(process.as_ref()).await;
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
                        let cleanup = force_cleanup(process.as_ref()).await;
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
                let cleanup = force_cleanup(process.as_ref()).await;
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

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = force_cleanup(process).await;
}

async fn force_cleanup(process: &dyn ProcessHandle) -> CleanupOutcome {
    let force = process.force_stop().await;
    let wait = process.wait().await;
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
    let cleanup = match (outcome.cleanup(), &materialization) {
        (CleanupOutcome::Failed(_), _) | (CleanupOutcome::Degraded(_), _) => {
            outcome.cleanup().clone()
        }
        (_, CleanupOutcome::Failed(_)) | (_, CleanupOutcome::Degraded(_)) => materialization,
        _ => outcome.cleanup().clone(),
    };
    let mut combined = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output() {
        combined = combined.with_output(output.clone());
    }
    combined
}
