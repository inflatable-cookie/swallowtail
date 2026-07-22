use super::super::lifecycle::TurnCancellation;
use crate::protocol::{ProviderEvent, ResponseStream};
use crate::transport::{StreamItem, Subscription};
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{OwnedRemoteResourceKind, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DeadlineObservation, OperationContent, ProviderCancellationOutcome,
    ProviderObservation, RemoteResourceDeletionOutcome, RuntimeEvent, RuntimeEventKind,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

pub(super) async fn pump_turn(
    mut subscription: Subscription,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> TerminalOutcome {
    let mut provider = ResponseStream::default();
    let mut sequence = 1;
    let mut completed_output = None;
    loop {
        match next_signal(&mut subscription, &mut deadline).await {
            Signal::Deadline => {
                cancellation.timeout();
                let _ = subscription.close().await;
                return uncertain(TerminalStatus::TimedOut, true);
            }
            Signal::Closed(result) => {
                if let Some(output) = completed_output {
                    return match result {
                        Ok(()) => {
                            TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
                                .with_output(output)
                        }
                        Err(error) => {
                            cancellation.fail_remote_uncertain();
                            uncertain(
                                TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                                false,
                            )
                        }
                    };
                }
                let reason = cancellation.reason();
                if reason == 1 {
                    return uncertain(TerminalStatus::Cancelled, true);
                }
                if reason == 2 {
                    return uncertain(TerminalStatus::TimedOut, true);
                }
                cancellation.fail_remote_uncertain();
                return uncertain(provider_status(result), false);
            }
            Signal::Item(Err(error)) => {
                cancellation.fail_remote_uncertain();
                let _ = subscription.close().await;
                return uncertain(
                    TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                    false,
                );
            }
            Signal::Item(Ok(StreamItem::Correlation(reference))) => {
                if let Err(error) = emit(
                    &events,
                    &mut sequence,
                    RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                        reference,
                    )),
                ) {
                    cancellation.fail_remote_uncertain();
                    let _ = subscription.close().await;
                    return runtime_failure(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Frame(frame))) => match provider.apply(&frame) {
                Err(error) => {
                    cancellation.fail_remote_uncertain();
                    let _ = subscription.close().await;
                    return uncertain(
                        TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                        false,
                    );
                }
                Ok(
                    ProviderEvent::Created(_)
                    | ProviderEvent::Progress(_)
                    | ProviderEvent::Unknown(_),
                ) => {
                    if let Err(error) = emit(&events, &mut sequence, RuntimeEventKind::Progress) {
                        cancellation.fail_remote_uncertain();
                        let _ = subscription.close().await;
                        return runtime_failure(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::TextDelta(content)) => {
                    if let Err(error) = emit_content(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::OutputDelta,
                        content,
                    ) {
                        cancellation.fail_remote_uncertain();
                        let _ = subscription.close().await;
                        return runtime_failure(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::TextDone(_)) => {}
                Ok(ProviderEvent::Completed { output, usage, .. }) => {
                    for kind in [
                        RuntimeEventKind::OutputAvailable,
                        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                    ] {
                        let result = if kind == RuntimeEventKind::OutputAvailable {
                            emit_content(&events, &mut sequence, kind, output.clone())
                        } else {
                            emit(&events, &mut sequence, kind)
                        };
                        if let Err(error) = result {
                            cancellation.fail_remote_uncertain();
                            let _ = subscription.close().await;
                            return runtime_failure(error.diagnostic().clone());
                        }
                    }
                    completed_output = Some(output);
                }
            },
        }
    }
}

enum Signal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed(Result<(), RuntimeFailure>),
    Deadline,
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Signal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(item) => Signal::Item(item),
                None => Signal::Closed(Ok(())),
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

fn provider_status(result: Result<(), RuntimeFailure>) -> TerminalStatus {
    result.map_or_else(
        |error| TerminalStatus::ProviderFailed(error.diagnostic().clone()),
        |_| {
            TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                "swallowtail.alibaba_model_studio.stream_disconnected",
                "Alibaba Model Studio stream closed before completion",
            ))
        },
    )
}

fn uncertain(status: TerminalStatus, cancelled: bool) -> TerminalOutcome {
    let mut outcome = TerminalOutcome::new(status, CleanupOutcome::Clean)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::ConversationItems,
            RemoteResourceDeletionOutcome::Unconfirmed,
        )
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Conversation,
            RemoteResourceDeletionOutcome::Unconfirmed,
        );
    if cancelled {
        outcome = outcome.with_provider_cancellation(ProviderCancellationOutcome::Unconfirmed);
    }
    outcome
}

fn runtime_failure(diagnostic: SafeDiagnostic) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(diagnostic),
        CleanupOutcome::Clean,
    )
}

fn emit_content(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    content: OperationContent,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}
