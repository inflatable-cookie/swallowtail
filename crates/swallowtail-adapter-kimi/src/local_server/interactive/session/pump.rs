#[path = "pump/frame.rs"]
mod frame;

use super::super::access::SecretMaterial;
use super::super::callbacks::CallbackHub;
use super::super::websocket::{Subscription, SubscriptionInput};
use super::{CursorState, TurnCancellation};
use crate::local_server::activity::KimiLocalActivityProjection;
use crate::local_server::protocol::TurnEndReason;
use crate::local_server::transport::CurlTransport;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};
use std::task::Poll;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CallbackAbandonment, CancellationControl, CleanupOutcome, Deadline,
    DeadlineObservation, HostServices, OperationContent, ProviderCancellationOutcome,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TerminalOutcomeSender, TerminalStatus,
};

pub(super) struct PumpInput {
    pub(super) subscription: Option<Subscription>,
    pub(super) scope: swallowtail_runtime::ScopeId,
    pub(super) session_id: String,
    pub(super) runtime_turn_id: RuntimeTurnId,
    pub(super) deadline: Option<Deadline>,
    pub(super) services: HostServices,
    pub(super) transport: CurlTransport,
    pub(super) endpoint: String,
    pub(super) secret: Weak<SecretMaterial>,
    pub(super) cursor: Arc<Mutex<CursorState>>,
    pub(super) cancellation: Arc<TurnCancellation>,
    pub(super) callbacks: Option<CallbackHub>,
    pub(super) events: swallowtail_runtime::RuntimeEventSender,
    pub(super) terminal: TerminalOutcomeSender,
    pub(super) terminal_flag: Arc<AtomicBool>,
    pub(super) remaining_reattachments: u32,
}

enum Signal {
    Frame(Vec<u8>),
    Failure(RuntimeFailure),
    Closed,
    Deadline,
}

pub(super) async fn run(mut input: PumpInput) {
    let mut deadline = input
        .deadline
        .and_then(|deadline| input.services.time().map(|time| time.wait_until(deadline)));
    let mut sequence = 2;
    let mut output = String::new();
    let mut reasoning_len = 0usize;
    let mut provider_turn = None;
    let mut recovery = None;
    let mut activity = KimiLocalActivityProjection::new(input.runtime_turn_id.clone());
    let mut provider_cancellation = None;
    let (status, operation_cleanup) = loop {
        match next_signal(
            input.subscription.as_mut().expect("active subscription"),
            &mut deadline,
        )
        .await
        {
            Signal::Deadline => {
                let cleanup = cleanup_from_result(input.cancellation.request().await.map(|_| ()));
                break (TerminalStatus::TimedOut, cleanup);
            }
            Signal::Closed => {
                if input.cancellation.requested.load(Ordering::SeqCst) {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
                }
                if reattach(&mut input).await.is_ok() {
                    continue;
                }
                break (
                    TerminalStatus::RuntimeFailed(disconnected().diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            Signal::Failure(error) => {
                if input.cancellation.requested.load(Ordering::SeqCst) {
                    break (TerminalStatus::Cancelled, CleanupOutcome::Clean);
                }
                if reattach(&mut input).await.is_ok() {
                    continue;
                }
                break (
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            Signal::Frame(frame) => {
                let result = frame::apply_frame(
                    &mut input,
                    &frame,
                    &mut sequence,
                    &mut output,
                    &mut reasoning_len,
                    &mut provider_turn,
                    &mut recovery,
                    &mut activity,
                )
                .await;
                match result {
                    Ok(Some(reason)) => {
                        let status = match reason {
                            TurnEndReason::Completed => TerminalStatus::Completed,
                            TurnEndReason::Cancelled => {
                                if input.cancellation.requested.load(Ordering::SeqCst) {
                                    provider_cancellation =
                                        Some(ProviderCancellationOutcome::Confirmed);
                                }
                                TerminalStatus::Cancelled
                            }
                            TurnEndReason::Failed | TurnEndReason::Blocked => {
                                TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                                    "swallowtail.kimi.local_server.turn_failed",
                                    "Kimi local server reported a failed turn",
                                ))
                            }
                        };
                        break (status, CleanupOutcome::Clean);
                    }
                    Ok(None) => {}
                    Err(error) => {
                        let cleanup =
                            cleanup_from_result(input.cancellation.request().await.map(|_| ()));
                        break (
                            TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
        }
    };
    let activity_status = match &status {
        TerminalStatus::Completed => swallowtail_runtime::ActivityStatus::Completed,
        TerminalStatus::Cancelled | TerminalStatus::TimedOut => {
            swallowtail_runtime::ActivityStatus::Cancelled
        }
        TerminalStatus::ProviderFailed(_)
        | TerminalStatus::HostFailed(_)
        | TerminalStatus::RuntimeFailed(_)
        | TerminalStatus::ProviderRequestObserved(_) => swallowtail_runtime::ActivityStatus::Failed,
    };
    if let Ok(observations) = activity.complete(activity_status) {
        for observation in observations {
            if input
                .events
                .send(swallowtail_runtime::RuntimeEvent::new(
                    sequence,
                    swallowtail_runtime::RuntimeEventKind::Activity(observation),
                ))
                .is_err()
            {
                break;
            }
            sequence += 1;
        }
    }
    if let Some(callbacks) = &input.callbacks {
        callbacks.abandon(match status {
            TerminalStatus::Cancelled => CallbackAbandonment::TurnCancelled,
            TerminalStatus::TimedOut => CallbackAbandonment::TimedOut,
            _ => CallbackAbandonment::TurnTerminated,
        });
    }
    let stream_cleanup = match input.subscription.take() {
        Some(subscription) => cleanup_from_result(subscription.close().await),
        None => CleanupOutcome::NotApplicable,
    };
    let cleanup = super::super::access::merge(operation_cleanup, stream_cleanup);
    input.events.mark_terminal();
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if let Ok(output) = OperationContent::new(output) {
        outcome = outcome.with_output(output);
    }
    if let Some(cancellation) = provider_cancellation {
        outcome = outcome.with_provider_cancellation(cancellation);
    }
    let _ = input.terminal.complete(outcome);
    input.terminal_flag.store(true, Ordering::SeqCst);
}

async fn reattach(input: &mut PumpInput) -> Result<(), RuntimeFailure> {
    if input.remaining_reattachments == 0 {
        return Err(disconnected());
    }
    input.remaining_reattachments -= 1;
    if let Some(subscription) = input.subscription.take() {
        subscription.close().await?;
    }
    let secret = input.secret.upgrade().ok_or_else(disconnected)?;
    let (cursor_seq, cursor_epoch) = {
        let cursor = input.cursor.lock().expect("cursor lock poisoned");
        (cursor.seq, cursor.epoch.clone())
    };
    let subscription = Subscription::open(
        SubscriptionInput {
            scope: input.scope.clone(),
            endpoint: input.endpoint.clone(),
            secret: secret.copy(),
            session_id: input.session_id.clone(),
            cursor_seq,
            cursor_epoch,
            deadline: input.deadline,
        },
        &input.services,
    )
    .await?;
    *input
        .cancellation
        .control
        .lock()
        .expect("subscription control lock poisoned") = subscription.control();
    input.subscription = Some(subscription);
    Ok(())
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Signal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(Ok(frame)) => Signal::Frame(frame),
                Some(Err(error)) => Signal::Failure(error),
                None => Signal::Closed,
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Signal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn cleanup_from_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn disconnected() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.kimi.local_server.websocket_disconnected",
        "Kimi local-server WebSocket disconnected before terminal truth",
    )
}
