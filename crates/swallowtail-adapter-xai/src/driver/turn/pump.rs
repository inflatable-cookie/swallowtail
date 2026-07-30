use super::super::lifecycle::{CancelReason, TurnCancellation};
use crate::USD_TICKS_PER_USD;
use crate::protocol::{ProviderFailure, TurnUpdate};
use futures_channel::mpsc;
use futures_core::Stream;
use std::future::poll_fn;
use std::num::NonZeroU64;
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{AccessProfileId, ModelRouteId, SafeDiagnostic};
use swallowtail_runtime::{
    BilledCostObservation, BoxFuture, CleanupOutcome, Currency, DeadlineObservation,
    OperationContent, ProviderObservation, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeTurnId, TerminalOutcome, TerminalStatus,
};

pub(in crate::driver) struct PendingTurn {
    pub(in crate::driver) updates: mpsc::Receiver<TurnUpdate>,
    pub(in crate::driver) work: BoxFuture<'static, Result<(), RuntimeFailure>>,
}

pub(in crate::driver) struct TurnObservationContext {
    pub(in crate::driver) turn_id: RuntimeTurnId,
    pub(in crate::driver) activity_operation_id: swallowtail_runtime::ActivityOperationId,
    pub(in crate::driver) model_route_id: ModelRouteId,
    pub(in crate::driver) access_profile_id: AccessProfileId,
}

pub(in crate::driver) async fn pump_turn(
    mut pending: PendingTurn,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    observation_context: TurnObservationContext,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut activity =
        crate::activity::XaiActivityProjection::new(observation_context.activity_operation_id);
    loop {
        match next_signal(&mut pending, &mut deadline).await {
            TurnSignal::Deadline => {
                cancellation.timeout();
                let _ = pending.work.await;
                return finish(
                    &cancellation,
                    TerminalOutcome::new(TerminalStatus::TimedOut, CleanupOutcome::Clean),
                );
            }
            TurnSignal::Work(result) => {
                let status = match cancellation.reason() {
                    CancelReason::Cancelled => TerminalStatus::Cancelled,
                    CancelReason::TimedOut => TerminalStatus::TimedOut,
                    CancelReason::None | CancelReason::Finished => {
                        TerminalStatus::RuntimeFailed(result.map_or_else(
                            |error| error.diagnostic().clone(),
                            |_| {
                                SafeDiagnostic::new(
                                    "swallowtail.xai.turn_disconnected",
                                    "xAI WebSocket turn ended before a terminal response",
                                )
                            },
                        ))
                    }
                };
                return finish(
                    &cancellation,
                    TerminalOutcome::new(status, CleanupOutcome::Clean),
                );
            }
            TurnSignal::Update(TurnUpdate::None) => {}
            TurnSignal::Update(TurnUpdate::Started { response_id }) => {
                let observation = match activity.started(&response_id) {
                    Ok(observation) => observation,
                    Err(error) => {
                        cancellation.abort();
                        let _ = pending.work.await;
                        return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                    }
                };
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(observation),
                )) {
                    cancellation.abort();
                    let _ = pending.work.await;
                    return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                }
                sequence += 1;
            }
            TurnSignal::Update(TurnUpdate::Delta {
                response_id,
                content: delta,
                ..
            }) => {
                let content = match OperationContent::new(delta) {
                    Ok(content) => content,
                    Err(_) => continue,
                };
                let observation = match activity.delta(&response_id, content.as_str()) {
                    Ok(observation) => observation,
                    Err(error) => {
                        cancellation.abort();
                        let _ = pending.work.await;
                        return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                    }
                };
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(observation),
                )) {
                    cancellation.abort();
                    let _ = pending.work.await;
                    return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                }
                sequence += 1;
                if let Err(error) = events.send(RuntimeEvent::with_content(
                    sequence,
                    RuntimeEventKind::OutputDelta,
                    content,
                )) {
                    cancellation.abort();
                    let _ = pending.work.await;
                    return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                }
                sequence += 1;
            }
            TurnSignal::Update(TurnUpdate::ProviderFailed(kind)) => {
                let outcome = match pending.work.await {
                    Ok(()) => TerminalOutcome::new(provider_status(kind), CleanupOutcome::Clean),
                    Err(error) => runtime_failure(error.diagnostic().clone()),
                };
                return finish(&cancellation, outcome);
            }
            TurnSignal::Update(TurnUpdate::Complete {
                continuation,
                output,
                usage,
                cost_in_usd_ticks,
                ..
            }) => {
                let output = OperationContent::new(output).expect("completed output is non-empty");
                let observation = match activity.completed(&continuation, output.as_str()) {
                    Ok(observation) => observation,
                    Err(error) => {
                        cancellation.abort();
                        let _ = pending.work.await;
                        return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                    }
                };
                if let Err(error) = events.send(RuntimeEvent::new(
                    sequence,
                    RuntimeEventKind::Activity(observation),
                )) {
                    cancellation.abort();
                    let _ = pending.work.await;
                    return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                }
                sequence += 1;
                let observations = [
                    RuntimeEventKind::OutputAvailable,
                    RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                    RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(
                        BilledCostObservation::provider_reported(
                            cost_in_usd_ticks,
                            Currency::Usd,
                            NonZeroU64::new(USD_TICKS_PER_USD).expect("USD tick scale is nonzero"),
                            observation_context.turn_id.clone(),
                            observation_context.model_route_id.clone(),
                            observation_context.access_profile_id.clone(),
                            NonZeroU64::new(1).expect("one provider attempt is nonzero"),
                        ),
                    )),
                ];
                for kind in observations {
                    let event = if kind == RuntimeEventKind::OutputAvailable {
                        RuntimeEvent::with_content(sequence, kind, output.clone())
                    } else {
                        RuntimeEvent::new(sequence, kind)
                    };
                    if let Err(error) = events.send(event) {
                        cancellation.abort();
                        let _ = pending.work.await;
                        return finish(&cancellation, runtime_failure(error.diagnostic().clone()));
                    }
                    sequence += 1;
                }
                let outcome = match pending.work.await {
                    Ok(()) => {
                        TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
                            .with_output(output)
                    }
                    Err(error) => runtime_failure(error.diagnostic().clone()),
                };
                return finish(&cancellation, outcome);
            }
        }
    }
}

enum TurnSignal {
    Update(TurnUpdate),
    Work(Result<(), RuntimeFailure>),
    Deadline,
}

async fn next_signal(
    pending: &mut PendingTurn,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> TurnSignal {
    poll_fn(|context| {
        if let Poll::Ready(Some(update)) = Pin::new(&mut pending.updates).poll_next(context) {
            return Poll::Ready(TurnSignal::Update(update));
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(TurnSignal::Deadline);
        }
        pending.work.as_mut().poll(context).map(TurnSignal::Work)
    })
    .await
}

fn provider_status(failure: ProviderFailure) -> TerminalStatus {
    let (code, message) = match failure {
        ProviderFailure::PreviousResponseNotFound => (
            "swallowtail.xai.previous_response_not_found",
            "xAI rejected the connection-local continuation",
        ),
        ProviderFailure::ConnectionLimitReached => (
            "swallowtail.xai.connection_limit_reached",
            "xAI closed the session at its connection lifetime limit",
        ),
        ProviderFailure::Other => (
            "swallowtail.xai.provider_failed",
            "xAI reported a provider failure",
        ),
    };
    TerminalStatus::ProviderFailed(SafeDiagnostic::new(code, message))
}

fn runtime_failure(diagnostic: SafeDiagnostic) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(diagnostic),
        CleanupOutcome::Clean,
    )
}

fn finish(cancellation: &TurnCancellation, outcome: TerminalOutcome) -> TerminalOutcome {
    match cancellation.finish() {
        CancelReason::Cancelled => {
            TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean)
        }
        CancelReason::TimedOut => {
            TerminalOutcome::new(TerminalStatus::TimedOut, CleanupOutcome::Clean)
        }
        CancelReason::None | CancelReason::Finished => outcome,
    }
}
