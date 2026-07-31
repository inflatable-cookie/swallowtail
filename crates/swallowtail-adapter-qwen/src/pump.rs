use crate::events::QwenEventParser;
use crate::handle::QwenProcessCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{InterfaceVersion, ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, ProcessHandle,
    ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender, RuntimeFailure, TerminalOutcome,
    TerminalStatus,
};

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<QwenProcessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    expected_version: InterfaceVersion,
    operation_id: ActivityOperationId,
) -> TerminalOutcome {
    pump_with_session(
        process,
        events,
        cancellation,
        deadline,
        QwenPumpContext::new(model, expected_version, None, operation_id),
    )
    .await
    .outcome
}

pub(crate) struct QwenPumpResult {
    pub(crate) outcome: TerminalOutcome,
    pub(crate) session_id: Option<String>,
}

pub(crate) struct QwenPumpContext {
    model: ModelId,
    expected_version: InterfaceVersion,
    expected_session_id: Option<String>,
    operation_id: ActivityOperationId,
}

impl QwenPumpContext {
    pub(crate) const fn new(
        model: ModelId,
        expected_version: InterfaceVersion,
        expected_session_id: Option<String>,
        operation_id: ActivityOperationId,
    ) -> Self {
        Self {
            model,
            expected_version,
            expected_session_id,
            operation_id,
        }
    }
}

pub(crate) async fn pump_with_session(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<QwenProcessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    context: QwenPumpContext,
) -> QwenPumpResult {
    let mut parser = QwenEventParser::with_expected_session(
        context.model,
        context.expected_version,
        context.expected_session_id,
        context.operation_id,
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
        (Ok((trailing, parsed, session_id)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                result(event_delivery_failed(CleanupOutcome::Clean))
            } else {
                QwenPumpResult {
                    outcome: parsed.outcome(exit),
                    session_id,
                }
            }
        }
        (Err(failure), exit) => result(TerminalOutcome::new(
            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
            cleanup_from_wait(&exit),
        )),
        (_, Err(_)) => result(TerminalOutcome::new(
            TerminalStatus::HostFailed(SafeDiagnostic::new(
                "swallowtail.qwen.headless.process_wait_failed",
                "Qwen headless process wait failed",
            )),
            process_cleanup_failed(),
        )),
    }
}

fn result(outcome: TerminalOutcome) -> QwenPumpResult {
    QwenPumpResult {
        outcome,
        session_id: None,
    }
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &QwenProcessCancellation,
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
            "swallowtail.qwen.headless.event_delivery_failed",
            "Qwen headless event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.qwen.headless.process_cleanup_failed",
        "Qwen headless process cleanup failed",
    ))
}
