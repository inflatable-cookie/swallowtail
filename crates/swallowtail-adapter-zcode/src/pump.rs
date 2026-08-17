use crate::handle::ZcodeCancellation;
use crate::protocol::AppServerParser;
use futures_util::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream,
    RuntimeEventSender, RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "zcode.app-server";

#[allow(clippy::too_many_arguments)]
pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<ZcodeCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    operation_id: ActivityOperationId,
    cwd: String,
    provider: String,
    model: String,
    prompt: String,
    mode: String,
    services: HostServices,
) -> TerminalOutcome {
    let mut parser = AppServerParser::new(operation_id, cwd, provider, model, prompt, mode);
    let create = match parser.create_request() {
        Ok(request) => request,
        Err(error) => return runtime_failure_cleanup(process.as_ref(), &services, error).await,
    };
    if let Err(error) = process.write_stdin(ProcessInputChunk::new(create)).await {
        return host_failure_cleanup(process.as_ref(), &services, error).await;
    }
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
                let parsed = match parser.push(chunk.bytes()) {
                    Ok(parsed) => parsed,
                    Err(error) => {
                        emit_protocol_debug(&services, &error, "pump.decode");
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        );
                    }
                };
                if let Err(error) = deliver(process.as_ref(), &events, parsed).await {
                    let cleanup = force_cleanup(process.as_ref()).await;
                    return TerminalOutcome::new(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        cleanup,
                    );
                }
                if parser.is_complete() {
                    return finish_after_terminal(
                        process.as_ref(),
                        parser,
                        cancellation.as_ref(),
                        &services,
                    )
                    .await;
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(error)) => {
                emit_host_debug(&services, &error, "pump.read");
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
    let exit = match exit {
        Ok(exit) => exit,
        Err(error) => {
            emit_host_debug(&services, &error, "pump.wait");
            return TerminalOutcome::new(
                TerminalStatus::HostFailed(error.diagnostic().clone()),
                process_cleanup_failed(),
            );
        }
    };
    match parser.finish(exit) {
        Ok(parsed) => parsed.outcome(CleanupOutcome::Clean),
        Err(error) => {
            emit_protocol_debug(&services, &error, "pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                CleanupOutcome::Clean,
            )
        }
    }
}

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = force_cleanup(process).await;
}

async fn finish_after_terminal(
    process: &dyn ProcessHandle,
    parser: AppServerParser,
    cancellation: &ZcodeCancellation,
    services: &HostServices,
) -> TerminalOutcome {
    let force = process.force_stop().await;
    let wait = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&wait));
    }
    let cleanup = if force.is_err() || wait.is_err() {
        process_cleanup_failed()
    } else {
        CleanupOutcome::Clean
    };
    let exit = match wait {
        Ok(exit) => exit,
        Err(error) => {
            emit_host_debug(services, &error, "pump.wait");
            return TerminalOutcome::new(
                TerminalStatus::HostFailed(error.diagnostic().clone()),
                process_cleanup_failed(),
            );
        }
    };
    match parser.finish(exit) {
        Ok(parsed) => parsed.outcome(cleanup),
        Err(error) => {
            emit_protocol_debug(services, &error, "pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
    }
}

async fn deliver(
    process: &dyn ProcessHandle,
    events: &RuntimeEventSender,
    parsed: crate::protocol::ParserOutput,
) -> Result<(), RuntimeFailure> {
    for event in parsed.events {
        events.send(event)?;
    }
    for write in parsed.writes {
        process.write_stdin(ProcessInputChunk::new(write)).await?;
    }
    Ok(())
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &ZcodeCancellation,
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

async fn runtime_failure_cleanup(
    process: &dyn ProcessHandle,
    services: &HostServices,
    error: RuntimeFailure,
) -> TerminalOutcome {
    emit_protocol_debug(services, &error, "pump.create");
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
        force_cleanup(process).await,
    )
}

async fn host_failure_cleanup(
    process: &dyn ProcessHandle,
    services: &HostServices,
    error: RuntimeFailure,
) -> TerminalOutcome {
    emit_host_debug(services, &error, "pump.create");
    TerminalOutcome::new(
        TerminalStatus::HostFailed(error.diagnostic().clone()),
        force_cleanup(process).await,
    )
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

fn emit_host_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
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
        "swallowtail.zcode.app_server.process_cleanup_failed",
        "ZCode app-server process cleanup failed",
    ))
}
