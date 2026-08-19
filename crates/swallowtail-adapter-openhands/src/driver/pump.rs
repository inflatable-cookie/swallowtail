use super::{events::OpenHandsEventParser, handle::OpenHandsCancellation};
use serde_json::Value;
use std::{future::poll_fn, sync::Arc, task::Poll};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessOutputChunk, RuntimeEventSender, RuntimeFailure,
    TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "openhands.agent-server";

pub(super) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<OpenHandsCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    operation_id: ActivityOperationId,
    services: HostServices,
    scripted: Vec<Value>,
) -> TerminalOutcome {
    let mut parser = OpenHandsEventParser::new(operation_id);
    let mut deadline = Some(deadline);
    for record in scripted {
        if cancellation.is_requested() {
            let cleanup = force_cleanup(process.as_ref()).await;
            return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup);
        }
        if deadline_elapsed(&mut deadline).await {
            let cleanup = force_cleanup(process.as_ref()).await;
            return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
        }
        match parser.push_event(&record) {
            Ok(parsed) => {
                if send_all(&events, parsed).is_err() {
                    let cleanup = force_cleanup(process.as_ref()).await;
                    return event_delivery_failed(cleanup);
                }
            }
            Err(failure) => {
                emit_protocol_debug(&services, &failure, "agent-server.pump.decode");
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                    cleanup,
                );
            }
        }
    }

    drain_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await;
    let exit = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit));
    }
    match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => match parsed.finalize(exit) {
            Ok((complete, outcome)) => {
                if send_all(&events, trailing).is_err() || send_all(&events, complete).is_err() {
                    event_delivery_failed(CleanupOutcome::Clean)
                } else {
                    outcome
                }
            }
            Err(failure) => {
                emit_protocol_debug(&services, &failure, "agent-server.pump.activity");
                TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                    CleanupOutcome::Clean,
                )
            }
        },
        (Err(failure), exit) => {
            emit_protocol_debug(&services, &failure, "agent-server.pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                cleanup_from_wait(&exit),
            )
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.openhands.agent_server.process_wait_failed",
                "OpenHands Agent Server process wait failed",
            );
            services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "agent-server.pump.wait",
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

async fn drain_output(
    process: &dyn ProcessHandle,
    cancellation: &OpenHandsCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) {
    loop {
        match next_output(process, cancellation, deadline).await {
            NextOutput::Deadline | NextOutput::Process(Ok(None)) | NextOutput::Process(Err(_)) => {
                break;
            }
            NextOutput::Process(Ok(Some(_))) => {}
        }
    }
}

async fn deadline_elapsed(deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>) -> bool {
    poll_fn(|context| {
        if let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            Poll::Ready(true)
        } else {
            Poll::Ready(false)
        }
    })
    .await
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

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &OpenHandsCancellation,
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

fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = swallowtail_runtime::RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}

pub(super) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
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

fn event_delivery_failed(cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.openhands.agent_server.event_delivery_failed",
            "OpenHands Agent Server event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.openhands.agent_server.process_cleanup_failed",
        "OpenHands Agent Server process cleanup failed",
    ))
}
