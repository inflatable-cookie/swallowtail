use crate::events::MuseEventParser;
use crate::handle::MuseCancellation;
use std::future::poll_fn;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_core::{ModelId, SafeDiagnostic};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, CleanupOutcome, DeadlineObservation, DebugObservationKind,
    HostServices, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, RuntimeEventSender,
    RuntimeFailure, TerminalOutcome, TerminalStatus,
};

const ROUTE: &str = "muse-code.headless";

pub(crate) async fn pump(
    process: Arc<dyn ProcessHandle>,
    events: RuntimeEventSender,
    cancellation: Arc<MuseCancellation>,
    deadline: BoxFuture<'static, DeadlineObservation>,
    model: ModelId,
    operation_id: ActivityOperationId,
    services: HostServices,
) -> TerminalOutcome {
    let mut parser = MuseEventParser::new(operation_id, model);
    let mut deadline = Some(deadline);
    loop {
        match next_output(process.as_ref(), cancellation.as_ref(), &mut deadline).await {
            NextOutput::Deadline => {
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(TerminalStatus::TimedOut, cleanup);
            }
            NextOutput::Process(Ok(Some(chunk)))
                if chunk.stream() == ProcessOutputStream::Stdout =>
            {
                match parser.push(chunk.bytes()) {
                    Ok(parsed) => {
                        if send_all(&events, parsed).is_err() {
                            let cleanup = force_cleanup(process.as_ref()).await;
                            return event_delivery_failed(cleanup);
                        }
                    }
                    Err(failure) => {
                        emit_protocol_debug(&services, &failure, "headless.pump.decode");
                        let cleanup = force_cleanup(process.as_ref()).await;
                        return TerminalOutcome::new(
                            TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                            cleanup,
                        );
                    }
                }
            }
            NextOutput::Process(Ok(Some(_))) => {}
            NextOutput::Process(Ok(None)) => break,
            NextOutput::Process(Err(failure)) => {
                emit_host_process_debug(&services, &failure, "headless.pump.read");
                let cleanup = force_cleanup(process.as_ref()).await;
                return TerminalOutcome::new(
                    TerminalStatus::HostFailed(failure.diagnostic().clone()),
                    cleanup,
                );
            }
        }
    }

    let exit = process.wait().await;
    if cancellation.is_requested() {
        return TerminalOutcome::new(TerminalStatus::Cancelled, cleanup_from_wait(&exit));
    }
    match (parser.finish(), exit) {
        (Ok((trailing, parsed)), Ok(exit)) => {
            if send_all(&events, trailing).is_err() {
                event_delivery_failed(CleanupOutcome::Clean)
            } else {
                parsed.outcome(exit)
            }
        }
        (Err(failure), exit) => {
            emit_protocol_debug(&services, &failure, "headless.pump.finish");
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(failure.diagnostic().clone()),
                cleanup_from_wait(&exit),
            )
        }
        (_, Err(_)) => {
            let diagnostic = SafeDiagnostic::new(
                "swallowtail.muse_code.headless.process_wait_failed",
                "Muse Code process wait failed",
            );
            services.emit_failure_debug(
                DebugObservationKind::HostProcess,
                ROUTE,
                "headless.pump.wait",
                diagnostic.code(),
                diagnostic.message(),
            );
            TerminalOutcome::new(
                TerminalStatus::HostFailed(diagnostic),
                process_cleanup_failed(),
            )
        }
    }
}

fn emit_protocol_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::ProtocolParse,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

fn emit_host_process_debug(services: &HostServices, error: &RuntimeFailure, stage: &'static str) {
    let diagnostic = error.diagnostic();
    services.emit_failure_debug(
        DebugObservationKind::HostProcess,
        ROUTE,
        stage,
        diagnostic.code(),
        diagnostic.message(),
    );
}

enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &MuseCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| {
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextOutput::Deadline);
        }
        read.as_mut().poll(context).map(NextOutput::Process)
    })
    .await
}

fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = swallowtail_runtime::RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}

pub(crate) async fn cleanup_failed_start(process: &dyn ProcessHandle) {
    let _ = force_cleanup(process).await;
}

async fn force_cleanup(process: &dyn ProcessHandle) -> CleanupOutcome {
    let stop = process.force_stop().await;
    let wait = process.wait().await;
    if stop.is_err() || wait.is_err() {
        process_cleanup_failed()
    } else {
        CleanupOutcome::Clean
    }
}

fn cleanup_from_wait(
    exit: &Result<swallowtail_runtime::ProcessExit, RuntimeFailure>,
) -> CleanupOutcome {
    if exit.is_ok() {
        CleanupOutcome::Clean
    } else {
        process_cleanup_failed()
    }
}

fn event_delivery_failed(cleanup: CleanupOutcome) -> TerminalOutcome {
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
            "swallowtail.muse_code.headless.event_delivery_failed",
            "Muse Code event delivery failed",
        )),
        cleanup,
    )
}

fn process_cleanup_failed() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.muse_code.headless.process_cleanup_failed",
        "Muse Code process cleanup failed",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_executor::block_on;
    use futures_util::future::pending;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicBool, Ordering};
    use swallowtail_core::ExecutionHostId;
    use swallowtail_runtime::{
        ProcessExit, ProcessInputChunk, RuntimeRunId, runtime_event_channel,
    };

    const META: &str = include_str!("../tests/fixtures/muse-code-0.1.0-R708.1/meta-success.jsonl");

    struct FixtureProcess {
        output: Mutex<VecDeque<Result<Option<ProcessOutputChunk>, RuntimeFailure>>>,
        exit: Result<ProcessExit, RuntimeFailure>,
        forced: AtomicBool,
    }

    impl FixtureProcess {
        fn new(output: impl IntoIterator<Item = Vec<u8>>, exit: ProcessExit) -> Self {
            Self {
                output: Mutex::new(
                    output
                        .into_iter()
                        .map(|bytes| {
                            Ok(Some(ProcessOutputChunk::new(
                                ProcessOutputStream::Stdout,
                                bytes,
                            )))
                        })
                        .chain([Ok(None)])
                        .collect(),
                ),
                exit: Ok(exit),
                forced: AtomicBool::new(false),
            }
        }
    }

    impl ProcessHandle for FixtureProcess {
        fn write_stdin(
            &self,
            _chunk: ProcessInputChunk,
        ) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            let next = self.output.lock().expect("output lock").pop_front();
            Box::pin(async move { next.unwrap_or(Ok(None)) })
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.forced.store(true, Ordering::SeqCst);
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            let exit = self.exit.clone();
            Box::pin(async move { exit })
        }
    }

    fn operation() -> ActivityOperationId {
        ActivityOperationId::Run(RuntimeRunId::new("muse-pump-fixture").unwrap())
    }

    fn no_deadline() -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(pending())
    }

    fn services() -> HostServices {
        HostServices::new(ExecutionHostId::new("muse.pump.fixture").unwrap())
    }

    #[test]
    fn pump_completes_exact_stream_and_preserves_nonzero_exit_truth() {
        for (exit, expected) in [
            (ProcessExit::new(true, Some(0)), TerminalStatus::Completed),
            (
                ProcessExit::new(false, Some(7)),
                TerminalStatus::ProviderFailed(
                    SafeDiagnostic::new(
                        "swallowtail.muse_code.headless.process_failed",
                        "Muse Code exited with status 7",
                    )
                    .with_failure_classification(
                        swallowtail_core::FailureClassification::new(
                            swallowtail_core::FailureOrigin::Harness,
                            swallowtail_core::FailureKind::Unknown,
                            swallowtail_core::FailureRecovery::Unknown,
                        ),
                    ),
                ),
            ),
        ] {
            let process = Arc::new(FixtureProcess::new(
                META.as_bytes().chunks(79).map(<[u8]>::to_vec),
                exit,
            ));
            let (sender, events) = runtime_event_channel(128).unwrap();
            sender
                .send(swallowtail_runtime::RuntimeEvent::new(
                    0,
                    swallowtail_runtime::RuntimeEventKind::Started,
                ))
                .unwrap();
            let cancellation = Arc::new(MuseCancellation::new(process.clone()));
            let outcome = block_on(pump(
                process,
                sender,
                cancellation,
                no_deadline(),
                ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
                operation(),
                services(),
            ));
            assert_eq!(outcome.status(), &expected);
            drop(events);
        }
    }

    #[test]
    fn malformed_stream_is_runtime_failure_and_forces_joined_cleanup() {
        let process = Arc::new(FixtureProcess::new(
            [b"not-json\n".to_vec()],
            ProcessExit::new(true, Some(0)),
        ));
        let (sender, events) = runtime_event_channel(16).unwrap();
        sender
            .send(swallowtail_runtime::RuntimeEvent::new(
                0,
                swallowtail_runtime::RuntimeEventKind::Started,
            ))
            .unwrap();
        let cancellation = Arc::new(MuseCancellation::new(process.clone()));
        let outcome = block_on(pump(
            process.clone(),
            sender,
            cancellation,
            no_deadline(),
            ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
            operation(),
            services(),
        ));
        assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
        assert!(process.forced.load(Ordering::SeqCst));
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        drop(events);
    }

    #[test]
    fn cancellation_stays_distinct_from_provider_and_runtime_failure() {
        let process = Arc::new(FixtureProcess::new([], ProcessExit::new(false, None)));
        let (sender, events) = runtime_event_channel(16).unwrap();
        sender
            .send(swallowtail_runtime::RuntimeEvent::new(
                0,
                swallowtail_runtime::RuntimeEventKind::Started,
            ))
            .unwrap();
        let cancellation = Arc::new(MuseCancellation::new(process.clone()));
        block_on(swallowtail_runtime::CancellationControl::request(
            cancellation.as_ref(),
        ))
        .expect("cancellation requested");
        let outcome = block_on(pump(
            process,
            sender,
            cancellation,
            no_deadline(),
            ModelId::new(crate::MUSE_SPARK_MODEL_ID).unwrap(),
            operation(),
            services(),
        ));
        assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
        drop(events);
    }
}
