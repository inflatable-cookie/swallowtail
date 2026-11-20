use crate::claude_code_events::ClaudeCodeEventParser;
use crate::claude_code_handle::ClaudeCodeCancellation;
use crate::claude_code_pump_common::{
    self as common, HEADLESS_PUMP_ROUTE, cleanup_from_wait, emit_host_process_debug,
    emit_protocol_debug, event_delivery_failed, force_cleanup, process_wait_failed, send_all,
};
use crate::claude_code_watcher::WatcherBinding;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::ModelId;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, HostServices,
    ProcessHandle, ProcessOutputStream, RuntimeEventSender, RuntimeFailure, RuntimeTurnId,
    TerminalOutcome, TerminalStatus, WatcherLifecycleSubscription, WatcherSnapshot,
};

#[path = "claude_code_pump_lifecycle.rs"]
mod lifecycle;

const ROUTE: common::PumpRoute = HEADLESS_PUMP_ROUTE;

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
                let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
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
                    let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
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
                let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
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
                            let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
                            return lifecycle::finish_with_watchers(
                                event_delivery_failed(&ROUTE, cleanup),
                                &mut parser,
                                &events,
                                host,
                            )
                            .await;
                        }
                    }
                    Err(error) => {
                        emit_protocol_debug(&ROUTE, &host.services, &error, ROUTE.decode_stage);
                        let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
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
                emit_host_process_debug(&ROUTE, &host.services, &error, ROUTE.read_stage);
                let cleanup = force_cleanup(&ROUTE, process.as_ref()).await;
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
                cleanup_from_wait(&ROUTE, &exit),
            ),
            &mut parser,
            &events,
            host,
        )
        .await;
    }
    if cancellation.is_requested() {
        return lifecycle::finish_with_watchers(
            TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&ROUTE, &exit)),
            &mut parser,
            &events,
            host,
        )
        .await;
    }
    let outcome = match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                event_delivery_failed(&ROUTE, CleanupOutcome::Clean)
            } else {
                parsed.outcome(exit)
            }
        }
        (Err(error), exit) => {
            emit_protocol_debug(&ROUTE, &host.services, &error, ROUTE.finish_stage);
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup_from_wait(&ROUTE, &exit),
            )
        }
        (_, Err(_)) => process_wait_failed(&ROUTE, &host.services),
    };
    lifecycle::finish_with_watchers(outcome, &mut parser, &events, host).await
}

enum NextWork {
    Process(Result<Option<swallowtail_runtime::ProcessOutputChunk>, RuntimeFailure>),
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
        match common::poll_output(&mut read, cancellation, deadline, context) {
            Poll::Ready(common::NextOutput::Deadline) => Poll::Ready(NextWork::Deadline),
            Poll::Ready(common::NextOutput::Process(result)) => {
                Poll::Ready(NextWork::Process(result))
            }
            Poll::Pending => Poll::Pending,
        }
    })
    .await
}
