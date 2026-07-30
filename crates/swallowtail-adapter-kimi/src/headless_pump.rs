use crate::headless_events::KimiHeadlessEventParser;
use crate::headless_handle::KimiHeadlessCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, ProcessHandle,
    ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender, RuntimeFailure, TerminalOutcome,
    TerminalStatus,
};

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<KimiHeadlessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    operation_id: ActivityOperationId,
) -> TerminalOutcome {
    let mut parser = KimiHeadlessEventParser::new(operation_id);
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return event_delivery_failed(cleanup);
                        }
                    }
                    Err(error) => {
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(error)) => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(
                    TerminalStatus::HostFailed(error.diagnostic().clone()),
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
            if send_all(&events, trailing).is_err() {
                event_delivery_failed(CleanupOutcome::Clean)
            } else {
                parsed.outcome(exit)
            }
        }
        (Err(error), exit) => TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
            cleanup_from_wait(&exit),
        ),
        (_, Err(_)) => TerminalOutcome::new(
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.kimi.headless.process_wait_failed",
                "Kimi headless process wait failed",
            )),
            process_cleanup_failed(),
        ),
    }
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &KimiHeadlessCancellation,
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

fn event_delivery_failed(cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.kimi.headless.event_delivery_failed",
            "Kimi headless event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.kimi.headless.process_cleanup_failed",
        "Kimi headless process cleanup failed",
    ))
}
