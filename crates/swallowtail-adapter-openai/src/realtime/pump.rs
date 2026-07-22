use super::lifecycle::ResponseCancellation;
use super::worker::{WorkerHandle, WorkerUpdate};
use crate::realtime_protocol::{RealtimeServerEvent, RealtimeServerStream};
use futures_channel::mpsc;
use std::num::NonZeroU64;
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::{Arc, Mutex};
use swallowtail_core::{MediaDirection, ProviderRequestRef, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, MediaChunk, MediaStreamId, MediaTranscript,
    ProviderCancellationOutcome, ProviderObservation, RealtimeMediaEvent, RealtimeMediaEventKind,
    RealtimeMediaResponseStatus, RealtimeMediaSessionState, RuntimeFailure, RuntimeSessionId,
    RuntimeTurnId, TerminalOutcome,
};

mod outcome;
mod signal;

use outcome::{
    completed_status, emit, finish_terminal, interrupted_or_disconnected, interrupted_or_failed,
    protocol_failed, runtime_failed,
};
use signal::{Signal, next_signal};

pub(super) struct PumpContext {
    pub(super) turn_id: RuntimeTurnId,
    pub(super) session_id: RuntimeSessionId,
    pub(super) config: swallowtail_core::RealtimeMediaConfig,
    pub(super) state: Arc<Mutex<RealtimeMediaSessionState>>,
    pub(super) reusable: Arc<AtomicBool>,
    pub(super) next_event_sequence: Arc<AtomicU64>,
    pub(super) cancellation: Arc<ResponseCancellation>,
    pub(super) worker: WorkerHandle,
    pub(super) request_ref: ProviderRequestRef,
}

pub(super) async fn pump_response(
    mut updates: mpsc::Receiver<WorkerUpdate>,
    mut events: mpsc::Sender<Result<RealtimeMediaEvent, RuntimeFailure>>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    context: PumpContext,
) -> (TerminalOutcome, mpsc::Receiver<WorkerUpdate>) {
    let mut provider_stream = RealtimeServerStream::new();
    let mut output_sequence = 1_u64;
    loop {
        let signal = next_signal(&mut updates, &mut deadline).await;
        if matches!(signal, Signal::Deadline) {
            deadline = None;
            if context.cancellation.timeout().await.is_err() {
                let result = finish_terminal(
                    &context,
                    &mut events,
                    RealtimeMediaResponseStatus::TimedOut(ProviderCancellationOutcome::Unconfirmed),
                )
                .await;
                return (result, updates);
            }
            continue;
        }
        let update = match signal {
            Signal::Update(update) => update,
            Signal::Deadline => unreachable!(),
            Signal::Closed => WorkerUpdate::Disconnected,
        };
        let event = match update {
            WorkerUpdate::Event(event) => event,
            WorkerUpdate::Failed(error) => {
                let status = interrupted_or_failed(&context.cancellation, error.diagnostic());
                return (
                    finish_terminal(&context, &mut events, status).await,
                    updates,
                );
            }
            WorkerUpdate::Disconnected => {
                let status = interrupted_or_disconnected(&context.cancellation);
                return (
                    finish_terminal(&context, &mut events, status).await,
                    updates,
                );
            }
        };
        if let Err(error) = provider_stream.apply(&event) {
            return (
                finish_terminal(
                    &context,
                    &mut events,
                    RealtimeMediaResponseStatus::Failed(error.diagnostic().clone()),
                )
                .await,
                updates,
            );
        }
        match event {
            RealtimeServerEvent::SessionConfigured
            | RealtimeServerEvent::InputCommitted
            | RealtimeServerEvent::Structural
            | RealtimeServerEvent::AudioCompleted { .. } => {}
            RealtimeServerEvent::ResponseStarted(_) => {
                if emit(
                    &context,
                    &mut events,
                    RealtimeMediaEventKind::ResponseStarted,
                )
                .is_err()
                    || emit(
                        &context,
                        &mut events,
                        RealtimeMediaEventKind::ProviderObservation(
                            ProviderObservation::RequestCorrelation(context.request_ref.clone()),
                        ),
                    )
                    .is_err()
                {
                    context.cancellation.abort();
                    return (runtime_failed(), updates);
                }
            }
            RealtimeServerEvent::AudioDelta { audio, .. } => {
                let stream_id = match MediaStreamId::new(format!(
                    "openai-realtime-output:{}",
                    context.turn_id.as_str()
                )) {
                    Ok(stream_id) => stream_id,
                    Err(_) => return (runtime_failed(), updates),
                };
                let chunk = MediaChunk::new(
                    context.session_id.clone(),
                    stream_id,
                    NonZeroU64::new(output_sequence).expect("output sequence is nonzero"),
                    MediaDirection::Output,
                    context.config.output_format(),
                    audio.bytes().to_vec(),
                    &context.config,
                );
                let Ok(chunk) = chunk else {
                    return (
                        finish_terminal(
                            &context,
                            &mut events,
                            RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
                                "swallowtail.openai.realtime_output_rejected",
                                "OpenAI Realtime output violated preflight media bounds",
                            )),
                        )
                        .await,
                        updates,
                    );
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
            RealtimeServerEvent::TranscriptDelta { transcript, .. } => {
                if !transcript.is_empty()
                    && emit(
                        &context,
                        &mut events,
                        RealtimeMediaEventKind::TranscriptDelta(
                            MediaTranscript::new(transcript).expect("non-empty transcript"),
                        ),
                    )
                    .is_err()
                {
                    context.cancellation.abort();
                    return (runtime_failed(), updates);
                }
            }
            RealtimeServerEvent::TranscriptCompleted { transcript, .. } => {
                let transcript = match MediaTranscript::new(transcript) {
                    Ok(transcript) => transcript,
                    Err(_) => return (protocol_failed(&context, &mut events).await, updates),
                };
                if emit(
                    &context,
                    &mut events,
                    RealtimeMediaEventKind::TranscriptCompleted(transcript),
                )
                .is_err()
                {
                    context.cancellation.abort();
                    return (runtime_failed(), updates);
                }
            }
            RealtimeServerEvent::RateLimits(observations) => {
                for observation in observations {
                    if emit(
                        &context,
                        &mut events,
                        RealtimeMediaEventKind::ProviderObservation(
                            ProviderObservation::RateLimit(observation),
                        ),
                    )
                    .is_err()
                    {
                        context.cancellation.abort();
                        return (runtime_failed(), updates);
                    }
                }
            }
            RealtimeServerEvent::Usage {
                usage, cancelled, ..
            } => {
                if emit(
                    &context,
                    &mut events,
                    RealtimeMediaEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                )
                .is_err()
                {
                    context.cancellation.abort();
                    return (runtime_failed(), updates);
                }
                let status = completed_status(&context.cancellation, cancelled);
                return (
                    finish_terminal(&context, &mut events, status).await,
                    updates,
                );
            }
            RealtimeServerEvent::ProviderFailed => {
                return (
                    finish_terminal(
                        &context,
                        &mut events,
                        RealtimeMediaResponseStatus::Failed(SafeDiagnostic::new(
                            "swallowtail.openai.realtime_provider_failed",
                            "OpenAI Realtime reported a provider failure",
                        )),
                    )
                    .await,
                    updates,
                );
            }
        }
    }
}
