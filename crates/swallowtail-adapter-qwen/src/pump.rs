use crate::events::QwenEventParser;
use crate::handle::QwenProcessCancellation;
use serde_json::Value;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{InterfaceVersion, ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "qwen.headless";

#[allow(clippy::too_many_arguments)]
pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<QwenProcessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    expected_version: InterfaceVersion,
    operation_id: ActivityOperationId,
    buffered_values: Vec<serde_json::Value>,
    services: HostServices,
) -> TerminalOutcome {
    pump_with_session(
        process,
        events,
        cancellation,
        deadline,
        QwenPumpContext::new(model, expected_version, None, operation_id)
            .with_buffered_values(buffered_values),
        services,
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
    buffered_values: Vec<Value>,
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
            buffered_values: Vec::new(),
        }
    }

    pub(crate) fn with_buffered_values(mut self, buffered_values: Vec<Value>) -> Self {
        self.buffered_values = buffered_values;
        self
    }
}

pub(crate) async fn pump_with_session(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<QwenProcessCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    context: QwenPumpContext,
    services: HostServices,
) -> QwenPumpResult {
    let mut parser = QwenEventParser::with_expected_session(
        context.model,
        context.expected_version,
        context.expected_session_id,
        context.operation_id,
    );
    for value in context.buffered_values {
        let mut bytes = match serde_json::to_vec(&value) {
            Ok(bytes) => bytes,
            Err(_) => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.qwen.headless.malformed_stream",
                        "Qwen Code emitted malformed stream output",
                    )),
                    cleanup,
                ));
            }
        };
        bytes.push(b'\n');
        match parser.push(&bytes) {
            Ok(parsed) => {
                if send_all(&events, parsed).is_err() {
                    let cleanup = force_cleanup(process.as_ref()).await;
                    return result(event_delivery_failed(cleanup));
                }
            }
            Err(failure) => {
                emit_protocol_debug(&services, &failure, "headless.pump.buffered_decode");
                let cleanup = force_cleanup(process.as_ref()).await;
                return result(TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                    cleanup,
                ));
            }
        }
    }
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
                        emit_protocol_debug(&services, &failure, "headless.pump.decode");
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
                emit_host_process_debug(&services, &failure, "headless.pump.read");
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
        (Err(failure), exit) => {
            emit_protocol_debug(&services, &failure, "headless.pump.finish");
            result(TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                cleanup_from_wait(&exit),
            ))
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.qwen.headless.process_wait_failed",
                "Qwen headless process wait failed",
            );
            services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "headless.pump.wait",
                diagnostic.code(),
                diagnostic.message(),
            );
            result(TerminalOutcome::new(
                TerminalStatus::HostFailed(diagnostic),
                process_cleanup_failed(),
            ))
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
