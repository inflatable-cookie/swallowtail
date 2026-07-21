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

pub(super) struct PendingTurn {
    pub(super) updates: mpsc::Receiver<TurnUpdate>,
    pub(super) work: BoxFuture<'static, Result<(), RuntimeFailure>>,
}

pub(super) async fn pump_turn(
    mut pending: PendingTurn,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    turn_id: RuntimeTurnId,
    model_route_id: ModelRouteId,
    access_profile_id: AccessProfileId,
) -> TerminalOutcome {
    let mut sequence = 1;
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
            TurnSignal::Update(TurnUpdate::Delta(delta)) => {
                let content = match OperationContent::new(delta) {
                    Ok(content) => content,
                    Err(_) => continue,
                };
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
                output,
                usage,
                cost_in_usd_ticks,
                ..
            }) => {
                let output = OperationContent::new(output).expect("completed output is non-empty");
                let observations = [
                    RuntimeEventKind::OutputAvailable,
                    RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                    RuntimeEventKind::ProviderObservation(ProviderObservation::BilledCost(
                        BilledCostObservation::provider_reported(
                            cost_in_usd_ticks,
                            Currency::Usd,
                            NonZeroU64::new(USD_TICKS_PER_USD).expect("USD tick scale is nonzero"),
                            turn_id.clone(),
                            model_route_id.clone(),
                            access_profile_id.clone(),
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
