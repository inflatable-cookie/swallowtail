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
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalStatus, WatcherActivityProjection,
    WatcherLifecycleSubscription, WatcherSnapshot, project_watcher_activity,
};

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
                return finish_with_watchers(
                    TerminalOutcome::new(TerminalStatus::TimedOut, cleanup),
                    host.watcher_binding.take(),
                )
                .await;
            }
            NextWork::Lifecycle(snapshot) => {
                if let Err(error) =
                    emit_lifecycle(&mut parser, &events, host.watcher_turn.as_ref(), snapshot)
                {
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
            NextWork::FeedFailed(error) => {
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
            NextWork::Process(Ok(Some(chunk))) if chunk.stream() == ProcessOutputStream::Stdout => {
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
            NextWork::Process(Ok(Some(_))) => {}
            NextWork::Process(Ok(None)) => break,
            NextWork::Process(Err(error)) => {
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
    if let Err(error) = drain_lifecycle(
        &mut parser,
        &events,
        host.watcher_feed.as_mut(),
        host.watcher_turn.as_ref(),
    ) {
        return finish_with_watchers(
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup_from_wait(&exit),
            ),
            host.watcher_binding.take(),
        )
        .await;
    }
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

fn emit_lifecycle(
    parser: &mut ClaudeCodeEventParser,
    events: &RuntimeEventSender,
    turn: Option<&RuntimeTurnId>,
    snapshot: WatcherSnapshot,
) -> Result<(), RuntimeFailure> {
    let Some(turn) = turn else {
        return Ok(());
    };
    match project_watcher_activity(turn, &snapshot) {
        Ok(WatcherActivityProjection::Activity(observation)) => {
            events.send(parser.activity_event(*observation))
        }
        Ok(WatcherActivityProjection::Joined { .. }) => Ok(()),
        Err(error) => Err(RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.claude_code.headless.watcher_activity_projection_failed",
            error.to_string(),
        ))),
    }
}

fn drain_lifecycle(
    parser: &mut ClaudeCodeEventParser,
    events: &RuntimeEventSender,
    feed: Option<&mut WatcherLifecycleSubscription>,
    turn: Option<&RuntimeTurnId>,
) -> Result<(), RuntimeFailure> {
    let Some(feed) = feed else {
        return Ok(());
    };
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match feed.poll_snapshot(&mut context) {
            Poll::Ready(Some(Ok(snapshot))) => {
                emit_lifecycle(parser, events, turn, snapshot)?;
            }
            Poll::Ready(Some(Err(error))) => return Err(error),
            Poll::Ready(None) | Poll::Pending => return Ok(()),
        }
    }
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
