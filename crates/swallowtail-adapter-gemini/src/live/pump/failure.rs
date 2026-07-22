use super::PumpContext;
use super::outcome::finish_terminal;
use crate::failure::failure;
use crate::live::lifecycle::{CancelReason, ResponseCancellation};
use futures_channel::mpsc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ProviderCancellationOutcome, RealtimeMediaEvent, RealtimeMediaResponseStatus, RuntimeFailure,
    TerminalOutcome,
};

pub(super) fn output_rejected(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
) -> TerminalOutcome {
    finish_terminal(
        context,
        events,
        RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
            "swallowtail.gemini.live_output_rejected",
            "Gemini Live output violated preflight media bounds",
        )),
    )
}

pub(super) fn protocol_failed(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
) -> TerminalOutcome {
    finish_terminal(
        context,
        events,
        RealtimeMediaResponseStatus::Failed(
            failure(
                "swallowtail.gemini.live_protocol_failed",
                "Gemini Live response ordering violated the frozen protocol",
            )
            .diagnostic()
            .clone(),
        ),
    )
}

pub(super) fn rollover_exhausted(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
) -> TerminalOutcome {
    finish_terminal(
        context,
        events,
        RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
            "swallowtail.gemini.live_rollover_exhausted",
            "Gemini Live requested more planned rollovers than preflight permitted",
        )),
    )
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
