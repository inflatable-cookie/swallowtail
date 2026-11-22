use super::super::lifecycle::TurnCancellation;
use crate::protocol::{ProviderEvent, ResponseStream};
use crate::transport::{StreamItem, Subscription};
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{OwnedRemoteResourceKind, SafeDiagnostic};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind, HostServices,
    OperationContent, ProviderCancellationOutcome, ProviderObservation,
    RemoteResourceDeletionOutcome, RuntimeEvent, RuntimeEventKind, RuntimeFailure, TerminalOutcome,
    TerminalStatus,
};

const ROUTE: &str = "alibaba.conversations";

pub(super) async fn pump_turn(
    mut subscription: Subscription,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
    turn_id: swallowtail_runtime::RuntimeTurnId,
    services: HostServices,
) -> TerminalOutcome {
    let mut provider = ResponseStream::default();
    let mut sequence = 1;
    let mut completed_output = None;
    let mut activity = crate::activity::AlibabaActivityProjection::new(
        swallowtail_runtime::ActivityOperationId::Turn(turn_id),
    );
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
                if let Err(error) = &result {
                    emit_wire_debug(&services, error, "http.pump.transport");
                } else {
                    let diagnostic = SafeDiagnostic::new(
                        "swallowtail.alibaba_model_studio.stream_disconnected",
                        "Alibaba Model Studio stream closed before completion",
                    );
                    services.emit_failure_debug(
                        DebugObservationKind::WireInbound,
                        ROUTE,
                        "http.pump.transport",
                        diagnostic.code(),
                        diagnostic.message(),
                    );
                }
                return uncertain(provider_status(result), false);
            }
            Signal::Item(Err(error)) => {
                let _ = subscription.close().await;
                if error.diagnostic().code()
                    == "swallowtail.alibaba_model_studio.connection_cancelled"
                {
                    let reason = cancellation.reason();
                    if reason == 1 {
                        if let Some(output) = completed_output {
                            return TerminalOutcome::new(
                                TerminalStatus::Completed,
                                CleanupOutcome::Clean,
                            )
                            .with_output(output);
                        }
                        return uncertain(TerminalStatus::Cancelled, true);
                    }
                    if reason == 2 {
                        if let Some(output) = completed_output {
                            return TerminalOutcome::new(
                                TerminalStatus::Completed,
                                CleanupOutcome::Clean,
                            )
                            .with_output(output);
                        }
                        return uncertain(TerminalStatus::TimedOut, true);
                    }
                }
                emit_wire_debug(&services, &error, "http.pump.transport");
                cancellation.fail_remote_uncertain();
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
                    let diagnostic = error.diagnostic();
                    services.emit_failure_debug(
                        DebugObservationKind::ProtocolParse,
                        ROUTE,
                        "http.pump.decode",
                        diagnostic.code(),
                        diagnostic.message(),
                    );
                    cancellation.fail_remote_uncertain();
                    let _ = subscription.close().await;
                    return uncertain(TerminalStatus::ProviderFailed(diagnostic.clone()), false);
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
                Ok(ProviderEvent::AssistantStarted(item)) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(match activity.started(&item) {
                            Ok(observation) => observation,
                            Err(error) => {
                                cancellation.fail_remote_uncertain();
                                let _ = subscription.close().await;
                                return runtime_failure(error.diagnostic().clone());
                            }
                        }),
                    ) {
                        cancellation.fail_remote_uncertain();
                        let _ = subscription.close().await;
                        return runtime_failure(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::TextDelta { item, content }) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(match activity.delta(&item, content.as_str()) {
                            Ok(observation) => observation,
                            Err(error) => {
                                cancellation.fail_remote_uncertain();
                                let _ = subscription.close().await;
                                return runtime_failure(error.diagnostic().clone());
                            }
                        }),
                    ) {
                        cancellation.fail_remote_uncertain();
                        let _ = subscription.close().await;
                        return runtime_failure(error.diagnostic().clone());
                    }
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
                Ok(ProviderEvent::TextDone { .. }) => {}
                Ok(ProviderEvent::Completed {
                    item,
                    output,
                    usage,
                    ..
                }) => {
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::Activity(
                            match activity.completed(&item, output.as_str()) {
                                Ok(observation) => observation,
                                Err(error) => {
                                    cancellation.fail_remote_uncertain();
                                    let _ = subscription.close().await;
                                    return runtime_failure(error.diagnostic().clone());
                                }
                            },
                        ),
                    ) {
                        cancellation.fail_remote_uncertain();
                        let _ = subscription.close().await;
                        return runtime_failure(error.diagnostic().clone());
                    }
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
    swallowtail_runtime::emit(events, sequence, kind)
}

fn emit_wire_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::WireInbound,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

#[cfg(test)]
mod tests {
    use super::pump_turn;
    use crate::driver::lifecycle::TurnCancellation;
    use crate::protocol::{ConversationRef, TurnOptions, WireRequest};
    use crate::transport::CurlTransport;
    use futures_executor::block_on;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::atomic::AtomicBool;
    use std::sync::{Arc, mpsc};
    use std::thread;
    use swallowtail_core::{ExecutionHostId, SafeDiagnostic};
    use swallowtail_runtime::{
        BlockingJob, BlockingWorkService, BoxFuture, CancellationControl, HostServices,
        OperationContent, RuntimeFailure, RuntimeTurnId, ScopeId, TerminalStatus,
        runtime_event_channel,
    };

    const SUCCESS_SSE: &str =
        include_str!("../../../tests/fixtures/model-studio-2026-07-22/success.sse");

    struct SignalingBlocking {
        done: mpsc::Sender<()>,
    }

    impl BlockingWorkService for SignalingBlocking {
        fn run(
            &self,
            _scope: ScopeId,
            job: BlockingJob,
        ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            let done = self.done.clone();
            let (tx, rx) = futures_channel::oneshot::channel();
            thread::spawn(move || {
                let result = job();
                let _ = done.send(());
                let _ = tx.send(result);
            });
            Box::pin(async move {
                rx.await.unwrap_or_else(|_| {
                    Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "fixture.blocking_panicked",
                        "fixture blocking worker panicked",
                    )))
                })
            })
        }
    }

    #[test]
    fn cancellation_plus_decode_error_after_completed_reports_provider_failed() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener binds");
        let endpoint = format!("http://{}", listener.local_addr().expect("addr exists"));
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accepts");
            let mut buf = [0_u8; 4096];
            let _ = stream.read(&mut buf);
            let mut payload = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nX-Request-Id: req_decode_1\r\nConnection: close\r\n\r\n{SUCCESS_SSE}"
            )
            .into_bytes();
            payload.extend_from_slice(b"data: \xff\xfe\n\n");
            let _ = stream.write_all(&payload);
            let _ = stream.flush();
        });

        let (done_tx, done_rx) = mpsc::channel();
        let services = HostServices::new(ExecutionHostId::new("test-host").expect("valid host"))
            .with_blocking_work(Arc::new(SignalingBlocking { done: done_tx }));
        let cancelled = Arc::new(AtomicBool::new(false));
        let conversation = ConversationRef::new("conv_fixture_01").expect("valid conversation");
        let wire = WireRequest::response(
            &conversation,
            &OperationContent::new("hello").expect("valid content"),
            &TurnOptions::frozen(),
        )
        .expect("valid request");
        let subscription = CurlTransport
            .subscribe(
                ScopeId::new("alibaba:test").expect("valid scope"),
                endpoint,
                b"fixture-secret".to_vec(),
                wire,
                &services,
                Arc::clone(&cancelled),
            )
            .expect("subscription starts");

        // Wait until the worker has buffered all SUCCESS frames and recorded StreamError::Decode.
        done_rx
            .recv()
            .expect("blocking worker finished with decode failure");

        // Now request cancellation so cancellation.reason() == 1 before pump_turn runs.
        let cancellation = Arc::new(TurnCancellation::new(
            cancelled,
            Arc::new(AtomicBool::new(true)),
            Arc::new(AtomicBool::new(false)),
        ));
        block_on(cancellation.request()).expect("cancellation accepted");

        let (events, _stream) = runtime_event_channel(64).expect("event channel opens");
        events
            .send(swallowtail_runtime::RuntimeEvent::new(
                0,
                swallowtail_runtime::RuntimeEventKind::Started,
            ))
            .expect("started event sends");
        let outcome = block_on(pump_turn(
            subscription,
            events,
            cancellation,
            None,
            RuntimeTurnId::new("turn-decode-after-completed").expect("valid turn id"),
            services,
        ));
        server.join().expect("server joins");

        match outcome.status() {
            TerminalStatus::ProviderFailed(diagnostic) => {
                assert_ne!(
                    diagnostic.code(),
                    "swallowtail.alibaba_model_studio.connection_cancelled",
                    "decode failure must not be misclassified as cancellation"
                );
            }
            other => panic!(
                "decode error after response.completed must remain ProviderFailed even when cancelled, got {other:?}"
            ),
        }
    }
}
