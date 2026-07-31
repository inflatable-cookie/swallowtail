use crate::headless_events::AntigravityEventParser;
use crate::headless_handle::AntigravityHeadlessCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, ProcessHandle,
    ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender, RuntimeFailure, TerminalOutcome,
    TerminalStatus,
};

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<AntigravityHeadlessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    schema_expected: bool,
    operation_id: ActivityOperationId,
) -> TerminalOutcome {
    pump_with_conversation(
        process,
        events,
        cancellation,
        deadline,
        model,
        schema_expected,
        None,
        operation_id,
    )
    .await
    .outcome
}

pub(crate) struct AntigravityPumpResult {
    pub(crate) outcome: TerminalOutcome,
    pub(crate) conversation_id: Option<String>,
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn pump_with_conversation(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<AntigravityHeadlessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    schema_expected: bool,
    expected_conversation_id: Option<String>,
    operation_id: ActivityOperationId,
) -> AntigravityPumpResult {
    let mut parser = AntigravityEventParser::with_expected_conversation(
        operation_id,
        model,
        schema_expected,
        expected_conversation_id,
    );
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(TerminalStatus::TimedOut, cleanup));
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return result(event_delivery_failed(cleanup));
                        }
                    }
                    Err(failure) => {
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return result(TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                            cleanup,
                        ));
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(failure)) => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(
                    TerminalStatus::HostFailed(failure.diagnostic().clone()),
                    cleanup,
                ));
            }
        }
    }

    let exit = process.wait().await;
    if cancellation.is_requested() {
        return result(TerminalOutcome::new(
            TerminalStatus::Cancelled,
            cleanup_from_wait(&exit),
        ));
    }
    match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                result(event_delivery_failed(CleanupOutcome::Clean))
            } else {
                let conversation_id = parsed.conversation_id().map(str::to_owned);
                AntigravityPumpResult {
                    outcome: parsed.outcome(exit),
                    conversation_id,
                }
            }
        }
        (Err(failure), exit) => result(TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
            cleanup_from_wait(&exit),
        )),
        (_, Err(_)) => result(TerminalOutcome::new(
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.antigravity.headless.process_wait_failed",
                "Antigravity headless process wait failed",
            )),
            process_cleanup_failed(),
        )),
    }
}

fn result(outcome: TerminalOutcome) -> AntigravityPumpResult {
    AntigravityPumpResult {
        outcome,
        conversation_id: None,
    }
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &AntigravityHeadlessCancellation,
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
            "swallowtail.antigravity.headless.event_delivery_failed",
            "Antigravity headless event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.antigravity.headless.process_cleanup_failed",
        "Antigravity headless process cleanup failed",
    ))
}
