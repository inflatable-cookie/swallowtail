use crate::claude_code_events::ClaudeCodeEventParser;
use crate::claude_code_handle::ClaudeCodeCancellation;
use crate::claude_code_watcher::WatcherBinding;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, SafeDiagnostic, WatcherCleanupCause};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "claude-code.headless";

/// Host services and optional watcher binding consumed by one pump.
pub(crate) struct PumpHost {
    pub(crate) services: HostServices,
    pub(crate) watcher_binding: Option<WatcherBinding>,
}

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<ClaudeCodeCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    operation_id: ActivityOperationId,
    mut host: PumpHost,
) -> TerminalOutcome {
    let mut parser = ClaudeCodeEventParser::new(model, operation_id);
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return finish_with_watchers(
                    TerminalOutcome::new(TerminalStatus::TimedOut, cleanup),
                    host.watcher_binding.take(),
                )
                .await;
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return finish_with_watchers(
                                event_delivery_failed(cleanup),
                                host.watcher_binding.take(),
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        emit_protocol_debug(&host.services, &error, "headless.pump.decode");
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return finish_with_watchers(
                            TerminalOutcome::new(
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup,
                            ),
                            host.watcher_binding.take(),
                        )
                        .await;
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(error)) => {
                emit_host_process_debug(&host.services, &error, "headless.pump.read");
                let cleanup = force_cleanup(process.as_ref()).await;
                return finish_with_watchers(
                    TerminalOutcome::new(
                        TerminalStatus::HostFailed(error.diagnostic().clone()),
                        cleanup,
                    ),
                    host.watcher_binding.take(),
                )
                .await;
            }
        }
    }
    let exit = process.wait().await;
    if cancellation.is_requested() {
        return finish_with_watchers(
            TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit)),
            host.watcher_binding.take(),
        )
        .await;
    }
    let outcome = match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                event_delivery_failed(CleanupOutcome::Clean)
            } else {
                parsed.outcome(exit)
            }
        }
        (Err(error), exit) => {
            emit_protocol_debug(&host.services, &error, "headless.pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup_from_wait(&exit),
            )
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.claude_code.headless.process_wait_failed",
                "Claude Code headless process wait failed",
            );
            host.services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "headless.pump.wait",
                diagnostic.code(),
                diagnostic.message(),
            );
            TerminalOutcome::new(
                TerminalStatus::HostFailed(diagnostic),
                process_cleanup_failed(),
            )
        }
    };
    finish_with_watchers(outcome, host.watcher_binding.take()).await
}

async fn finish_with_watchers(
    mut outcome: TerminalOutcome,
    binding: Option<WatcherBinding>,
) -> TerminalOutcome {
    let Some(mut binding) = binding else {
        return outcome;
    };
    if matches!(outcome.status(), TerminalStatus::Completed) {
        match binding.completion_gate() {
            Ok(state) if state.allows_successful_completion() => {}
            Ok(_) => {
                outcome = TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                        "swallowtail.claude_code.headless.watcher_completion_blocked",
                        "Claude Code headless cannot complete while host watchers remain active or unjoined",
                    )),
                    outcome.cleanup().clone(),
                );
            }
            Err(error) => {
                outcome = TerminalOutcome::new(
                    TerminalStatus::HostFailed(error.diagnostic().clone()),
                    outcome.cleanup().clone(),
                );
            }
        }
    }
    let watcher_cleanup = binding.close(cleanup_cause(outcome.status()));
    replace_cleanup(outcome, watcher_cleanup)
}

fn cleanup_cause(status: &TerminalStatus) -> WatcherCleanupCause {
    match status {
        TerminalStatus::Cancelled => WatcherCleanupCause::Cancelled,
        TerminalStatus::TimedOut => WatcherCleanupCause::TimedOut,
        TerminalStatus::Completed => WatcherCleanupCause::Stopped,
        _ => WatcherCleanupCause::Failed,
    }
}

fn replace_cleanup(outcome: TerminalOutcome, watcher_cleanup: CleanupOutcome) -> TerminalOutcome {
    let cleanup = merge_cleanup(outcome.cleanup().clone(), watcher_cleanup);
    let rebuilt = TerminalOutcome::new(outcome.status().clone(), cleanup);
    match outcome.output().cloned() {
        Some(output) => rebuilt.with_output(output),
        None => rebuilt,
    }
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(diagnostic), _) | (_, CleanupOutcome::Failed(diagnostic)) => {
            CleanupOutcome::Failed(diagnostic)
        }
        (CleanupOutcome::Degraded(diagnostic), _) | (_, CleanupOutcome::Degraded(diagnostic)) => {
            CleanupOutcome::Degraded(diagnostic)
        }
        (CleanupOutcome::Clean, _) | (_, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
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
    cancellation: &ClaudeCodeCancellation,
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
            "swallowtail.claude_code.headless.event_delivery_failed",
            "Claude Code headless event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.claude_code.headless.process_cleanup_failed",
        "Claude Code headless process cleanup failed",
    ))
}
