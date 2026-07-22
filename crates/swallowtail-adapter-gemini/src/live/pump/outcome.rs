use super::PumpContext;
use crate::failure::failure;
use futures_channel::mpsc;
use std::num::NonZeroU64;
use std::sync::atomic::Ordering;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaResponseStatus,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
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
            "swallowtail.gemini.live_event_overflow",
            "Gemini Live output exceeded its bounded event channel",
        )
    })
}

pub(super) fn finish_terminal(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    status: RealtimeMediaResponseStatus,
) -> TerminalOutcome {
    if status.ends_session() {
        context.reusable.store(false, Ordering::SeqCst);
        context.connections.abort_all();
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
                "swallowtail.gemini.live_disconnected",
                "Gemini Live disconnected before terminal response truth",
            ))
        }
    };
    if emit(
        context,
        events,
        RealtimeMediaEventKind::ResponseTerminal(status),
    )
    .is_err()
    {
        context.reusable.store(false, Ordering::SeqCst);
        return runtime_failed();
    }
    context.cancellation.finish();
    let outcome = TerminalOutcome::new(terminal_status, CleanupOutcome::Clean);
    cancellation.map_or(outcome.clone(), |truth| {
        outcome.with_provider_cancellation(truth)
    })
}

pub(super) fn runtime_failed() -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.gemini.live_runtime_failed",
            "Gemini Live response lifecycle failed",
        )),
        CleanupOutcome::Clean,
    )
}
