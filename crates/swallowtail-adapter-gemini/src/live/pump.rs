use super::lifecycle::{ConnectionRegistry, ResponseCancellation};
use super::worker::WorkerUpdate;
use crate::live_protocol::{RolloverState, ServerEvent};
use futures_channel::mpsc;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::{Arc, Mutex};
use swallowtail_core::{MediaDirection, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, MediaChunk, MediaStreamId, MediaTranscript,
    ProviderCancellationOutcome, ProviderObservation, RealtimeMediaEvent, RealtimeMediaEventKind,
    RealtimeMediaResponseStatus, RealtimeMediaSessionState, RuntimeFailure, RuntimeSessionId,
    RuntimeTurnId, TerminalOutcome,
};

mod failure;
mod outcome;
mod signal;

use failure::{
    interrupted_or_disconnected, interrupted_or_failed, output_rejected, protocol_failed,
    rollover_exhausted,
};
use outcome::{emit, finish_terminal, runtime_failed};
use signal::{Signal, next_signal};

pub(super) struct PumpContext {
    pub(super) turn_id: RuntimeTurnId,
    pub(super) session_id: RuntimeSessionId,
    pub(super) config: swallowtail_core::RealtimeMediaConfig,
    pub(super) state: Arc<Mutex<RealtimeMediaSessionState>>,
    pub(super) rollover: Arc<Mutex<RolloverState>>,
    pub(super) reusable: Arc<AtomicBool>,
    pub(super) next_event_sequence: Arc<AtomicU64>,
    pub(super) cancellation: Arc<ResponseCancellation>,
    pub(super) connections: ConnectionRegistry,
}

pub(super) async fn pump_response(
    mut updates: mpsc::Receiver<WorkerUpdate>,
    mut events: mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    context: PumpContext,
) -> (TerminalOutcome, mpsc::Receiver<WorkerUpdate>) {
    let mut output_sequence = 1_u64;
    let mut started = false;
    let mut transcript = String::new();
    if ensure_started(&context, &mut events, &mut started).is_err() {
        context.cancellation.abort();
        return (runtime_failed(), updates);
    }
    loop {
        match next_signal(&mut updates, &mut deadline).await {
            Signal::Deadline => {
                deadline = None;
                if context.cancellation.timeout() {
                    let status = RealtimeMediaResponseStatus::TimedOut(
                        ProviderCancellationOutcome::Unconfirmed,
                    );
                    return (finish_terminal(&context, &mut events, status), updates);
                }
            }
            Signal::Closed | Signal::Update(WorkerUpdate::Disconnected) => {
                let status = interrupted_or_disconnected(&context.cancellation);
                return (finish_terminal(&context, &mut events, status), updates);
            }
            Signal::Update(WorkerUpdate::Failed(error)) => {
                let status = interrupted_or_failed(&context.cancellation, error.diagnostic());
                return (finish_terminal(&context, &mut events, status), updates);
            }
            Signal::Update(WorkerUpdate::Event(event)) => match event {
                ServerEvent::SetupComplete => {
                    return (protocol_failed(&context, &mut events), updates);
                }
                ServerEvent::ResumptionUpdate { resumable, handle } => context
                    .rollover
                    .lock()
                    .expect("rollover state poisoned")
                    .update(resumable, handle),
                ServerEvent::GoAway(time_left) => {
                    let mut rollover = context.rollover.lock().expect("rollover state poisoned");
                    if rollover.exhausted() {
                        drop(rollover);
                        return (rollover_exhausted(&context, &mut events), updates);
                    }
                    rollover.warn(time_left);
                }
                ServerEvent::Audio(audio) => {
                    if ensure_started(&context, &mut events, &mut started).is_err() {
                        return (runtime_failed(), updates);
                    }
                    let stream_id = match MediaStreamId::new(format!(
                        "gemini-live-output:{}",
                        context.turn_id.as_str()
                    )) {
                        Ok(stream) => stream,
                        Err(_) => return (runtime_failed(), updates),
                    };
                    let chunk = MediaChunk::new(
                        context.session_id.clone(),
                        stream_id,
                        NonZeroU64::new(output_sequence).unwrap(),
                        MediaDirection::Output,
                        context.config.output_format(),
                        audio.bytes().to_vec(),
                        &context.config,
                    );
                    let Ok(chunk) = chunk else {
                        return (output_rejected(&context, &mut events), updates);
                    };
                    output_sequence = output_sequence.saturating_add(1);
                    if emit(
                        &context,
                        &mut events,
                        RealtimeMediaEventKind::OutputAudio(chunk),
                    )
                    .is_err()
                    {
                        context.cancellation.abort();
                        return (runtime_failed(), updates);
                    }
                }
                ServerEvent::Transcript(delta) => {
                    if ensure_started(&context, &mut events, &mut started).is_err() {
                        return (runtime_failed(), updates);
                    }
                    transcript.push_str(&delta);
                    if !delta.is_empty()
                        && emit(
                            &context,
                            &mut events,
                            RealtimeMediaEventKind::TranscriptDelta(
                                MediaTranscript::new(delta).expect("non-empty transcript"),
                            ),
                        )
                        .is_err()
                    {
                        context.cancellation.abort();
                        return (runtime_failed(), updates);
                    }
                }
                ServerEvent::Usage(usage) => {
                    if ensure_started(&context, &mut events, &mut started).is_err()
                        || emit(
                            &context,
                            &mut events,
                            RealtimeMediaEventKind::ProviderObservation(
                                ProviderObservation::Usage(usage),
                            ),
                        )
                        .is_err()
                    {
                        context.cancellation.abort();
                        return (runtime_failed(), updates);
                    }
                }
                ServerEvent::TurnComplete => {
                    if ensure_started(&context, &mut events, &mut started).is_err() {
                        return (runtime_failed(), updates);
                    }
                    if !transcript.is_empty()
                        && emit(
                            &context,
                            &mut events,
                            RealtimeMediaEventKind::TranscriptCompleted(
                                MediaTranscript::new(transcript).expect("transcript is non-empty"),
                            ),
                        )
                        .is_err()
                    {
                        context.cancellation.abort();
                        return (runtime_failed(), updates);
                    }
                    return (
                        finish_terminal(
                            &context,
                            &mut events,
                            RealtimeMediaResponseStatus::Completed,
                        ),
                        updates,
                    );
                }
                ServerEvent::ProviderFailed => {
                    let status = RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
                        "swallowtail.gemini.live_provider_failed",
                        "Gemini Live reported a provider failure",
                    ));
                    return (finish_terminal(&context, &mut events, status), updates);
                }
                ServerEvent::GenerationComplete | ServerEvent::Structural => {}
            },
        }
    }
}

fn ensure_started(
    context: &PumpContext,
    events: &mut mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    started: &mut bool,
) -> Result<(), RuntimeFailure> {
    if !*started {
        emit(context, events, RealtimeMediaEventKind::ResponseStarted)?;
        *started = true;
    }
    Ok(())
}
