use super::PumpContext;
use crate::failure::failure;
use crate::realtime::lifecycle::{CancelReason, ResponseCancellation};
use futures_channel::mpsc;
use std::num::NonZeroU64;
use std::sync::atomic::Ordering;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, ProviderCancellationOutcome, RealtimeMediaEvent, RealtimeMediaEventKind,
    RealtimeMediaResponseStatus, RuntimeFailure, TerminalOutcome, TerminalStatus,
};

pub(super) fn emit(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    kind: RealtimeMediaEventKind,
) -> Result<(), RuntimeFailure> {
    let sequence = context.next_event_sequence.fetch_add(1, Ordering::SeqCst);
    let event = RealtimeMediaEvent::new(
        NonZeroU64::new(sequence).expect("event sequence is nonzero"),
        context.turn_id.clone(),
        kind,
    );
    context
        .state
        .lock()
        .expect("media state lock poisoned")
        .record_response_event(&event)
        .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))?;
    events.try_send(Ok(event)).map_err(|_| {
        failure(
            "swallowtail.openai.realtime_event_overflow",
            "OpenAI Realtime output exceeded its bounded event channel",
        )
    })
}

pub(super) async fn finish_terminal(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    status: RealtimeMediaResponseStatus,
) -> TerminalOutcome {
    let ends_session = status.ends_session();
    let outcome = terminal(context, events, status);
    if ends_session {
        let _ = context.worker.close().await;
    }
    outcome
}

fn terminal(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    status: RealtimeMediaResponseStatus,
) -> TerminalOutcome {
    if status.ends_session() {
        context.reusable.store(false, Ordering::SeqCst);
    }
    let cancellation = match &status {
        RealtimeMediaResponseStatus::Cancelled(outcome)
        | RealtimeMediaResponseStatus::TimedOut(outcome) => Some(*outcome),
        _ => None,
    };
    let terminal_status = match &status {
        RealtimeMediaResponseStatus::Completed => TerminalStatus::Completed,
        RealtimeMediaResponseStatus::Cancelled(_) => TerminalStatus::Cancelled,
        RealtimeMediaResponseStatus::TimedOut(_) => TerminalStatus::TimedOut,
        RealtimeMediaResponseStatus::Failed(diagnostic) => {
            TerminalStatus::ProviderFailed(diagnostic.clone())
        }
        RealtimeMediaResponseStatus::Disconnected => {
            TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                "swallowtail.openai.realtime_disconnected",
                "OpenAI Realtime disconnected before terminal response truth",
            ))
        }
    };
    if emit(
        context,
        events,
        RealtimeMediaEventKind::ResponseTerminal(status.clone()),
    )
    .is_err()
    {
        context.reusable.store(false, Ordering::SeqCst);
        return runtime_failed();
    }
    let _ = context.cancellation.finish();
    let outcome = TerminalOutcome::new(terminal_status, CleanupOutcome::Clean);
    cancellation.map_or(outcome.clone(), |cancellation| {
        outcome.with_provider_cancellation(cancellation)
    })
}

pub(super) fn completed_status(
    cancellation: &ResponseCancellation,
    provider: Option<ProviderCancellationOutcome>,
) -> RealtimeMediaResponseStatus {
    match (cancellation.reason(), provider) {
        (CancelReason::Cancelled, Some(outcome)) => RealtimeMediaResponseStatus::Cancelled(outcome),
        (CancelReason::Cancelled, None) => {
            RealtimeMediaResponseStatus::Cancelled(ProviderCancellationOutcome::RacedWithCompletion)
        }
        (CancelReason::TimedOut, Some(outcome)) => RealtimeMediaResponseStatus::TimedOut(outcome),
        (CancelReason::TimedOut, None) => {
            RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::RacedWithCompletion)
        }
        (_, Some(outcome)) => RealtimeMediaResponseStatus::Cancelled(outcome),
        _ => RealtimeMediaResponseStatus::Completed,
    }
}

pub(super) fn interrupted_or_failed(
    cancellation: &ResponseCancellation,
    diagnostic: &SafeDiagnostic,
) -> RealtimeMediaResponseStatus {
    match cancellation.reason() {
        CancelReason::Cancelled => {
            RealtimeMediaResponseStatus::Cancelled(ProviderCancellationOutcome::Unconfirmed)
        }
        CancelReason::TimedOut => {
            RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::Unconfirmed)
        }
        _ => RealtimeMediaResponseStatus::Failed(diagnostic.clone()),
    }
}

pub(super) fn interrupted_or_disconnected(
    cancellation: &ResponseCancellation,
) -> RealtimeMediaResponseStatus {
    match cancellation.reason() {
        CancelReason::Cancelled => {
            RealtimeMediaResponseStatus::Cancelled(ProviderCancellationOutcome::Unconfirmed)
        }
        CancelReason::TimedOut => {
            RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::Unconfirmed)
        }
        _ => RealtimeMediaResponseStatus::Disconnected,
    }
}

pub(super) async fn protocol_failed(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
) -> TerminalOutcome {
    finish_terminal(
        context,
        events,
        RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
            "swallowtail.openai.realtime_transcript_rejected",
            "OpenAI Realtime returned an invalid transcript event",
        )),
    )
    .await
}

pub(super) fn runtime_failed() -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.openai.realtime_runtime_failed",
            "OpenAI Realtime response lifecycle failed",
        )),
        CleanupOutcome::Clean,
    )
}
