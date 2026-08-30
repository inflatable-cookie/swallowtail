use crate::claude_code_events::ClaudeCodeEventParser;
use crate::claude_code_handle::ClaudeCodeCancellation;
use crate::claude_code_watcher::WatcherBinding;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalStatus, WatcherLifecycleSubscription,
    WatcherSnapshot,
};

#[path = "claude_code_pump_lifecycle.rs"]
mod lifecycle;

const ROUTE: &str = "claude-code.headless";

/// Host services and optional watcher binding consumed by one pump.
pub(crate) struct PumpHost {
    pub(crate) services: HostServices,
    pub(crate) watcher_binding: Option<WatcherBinding>,
    pub(crate) watcher_feed: Option<WatcherLifecycleSubscription>,
    pub(crate) watcher_turn: Option<RuntimeTurnId>,
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
        match next_work(
            process.as_ref(),
            cancellation.as_ref(),
            &mut deadline,
            host.watcher_feed.as_mut(),
        )
        .await
        {
            NextWork::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return lifecycle::finish_with_watchers(
                    TerminalOutcome::new(TerminalStatus::TimedOut, cleanup),
                    &mut parser,
                    &events,
                    host,
                )
                .await;
            }
            NextWork::Lifecycle(snapshot) => {
                if let Err(error) = lifecycle::emit_lifecycle(
                    &mut parser,
                    &events,
                    host.watcher_turn.as_ref(),
                    snapshot,
                ) {
                    let cleanup = force_cleanup(process.as_ref()).await;
                    return lifecycle::finish_with_watchers(
                        TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        ),
                        &mut parser,
                        &events,
                        host,
                    )
                    .await;
                }
            }
            NextWork::FeedFailed(error) => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return lifecycle::finish_with_watchers(
                    TerminalOutcome::new(
                        TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                        cleanup,
                    ),
                    &mut parser,
                    &events,
                    host,
                )
                .await;
            }
            NextWork::Process(Ok(Some(chunk))) if chunk.stream() == ProcessOutputStream::Stdout => {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return lifecycle::finish_with_watchers(
                                event_delivery_failed(cleanup),
                                &mut parser,
                                &events,
                                host,
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        emit_protocol_debug(&host.services, &error, "headless.pump.decode");
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return lifecycle::finish_with_watchers(
                            TerminalOutcome::new(
                                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                                cleanup,
                            ),
                            &mut parser,
                            &events,
                            host,
                        )
                        .await;
                    }
                }
            }
            NextWork::Process(Ok(Some(_))) => {}
            NextWork::Process(Ok(None)) => break,
            NextWork::Process(Err(error)) => {
                emit_host_process_debug(&host.services, &error, "headless.pump.read");
                let cleanup = force_cleanup(process.as_ref()).await;
                return lifecycle::finish_with_watchers(
                    TerminalOutcome::new(
                        TerminalStatus::HostFailed(error.diagnostic().clone()),
                        cleanup,
                    ),
                    &mut parser,
                    &events,
                    host,
                )
                .await;
            }
        }
    }
    let exit = process.wait().await;
    if let Err(error) = lifecycle::drain_lifecycle(
        &mut parser,
        &events,
        host.watcher_feed.as_mut(),
        host.watcher_turn.as_ref(),
    ) {
        return lifecycle::finish_with_watchers(
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup_from_wait(&exit),
            ),
            &mut parser,
            &events,
            host,
        )
        .await;
    }
    if cancellation.is_requested() {
        return lifecycle::finish_with_watchers(
            TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit)),
            &mut parser,
            &events,
            host,
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
    lifecycle::finish_with_watchers(outcome, &mut parser, &events, host).await
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

enum NextWork {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Lifecycle(WatcherSnapshot),
    FeedFailed(RuntimeFailure),
    Deadline,
}

async fn next_work(
    process: &dyn ProcessHandle,
    cancellation: &ClaudeCodeCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
    mut feed: Option<&mut WatcherLifecycleSubscription>,
) -> NextWork {
    let mut read = process.read_output();
    poll_fn(|context| {
        if let Some(feed) = feed.as_mut() {
            match feed.poll_snapshot(context) {
                Poll::Ready(Some(Ok(snapshot))) => {
                    return Poll::Ready(NextWork::Lifecycle(snapshot));
                }
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(NextWork::FeedFailed(error));
                }
                Poll::Ready(None) | Poll::Pending => {}
            }
        }
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextWork::Deadline);
        }
        read.as_mut().poll(context).map(NextWork::Process)
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
