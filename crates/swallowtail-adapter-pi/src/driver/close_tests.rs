use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, TimeService,
};

/// A process that accepts the abort write and never sends an acknowledgement.
#[derive(Default)]
pub(crate) struct SilentAbort {
    pub(crate) writes: AtomicUsize,
    pub(crate) force_stopped: AtomicBool,
}

impl ProcessHandle for SilentAbort {
    fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.writes.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.force_stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

pub(crate) struct AbortGraceExpires;

impl TimeService for AbortGraceExpires {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(10)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        assert_eq!(
            deadline.instant().ticks(),
            510,
            "close gets a fresh 500-tick grace"
        );
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}

pub(crate) mod shutdown {
    use std::future::poll_fn;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Arc, Mutex};
    use std::task::{Context, Poll, Waker};
    use swallowtail_core::{
        CredentialRef, EndpointAudience, ExecutionHostId, ResourceAccess, ResourceRepresentation,
    };
    use swallowtail_runtime::{
        BoxFuture, CleanupOutcome, CredentialLease, Deadline, DeadlineObservation,
        DelegatedCredential, HostServices, InteractiveSessionHandle, JoinedTask, MonotonicInstant,
        ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, ResourceLease,
        RuntimeFailure, ScopeId, SessionCleanupRequest, TimeService, WorkingResourceRef,
    };

    use swallowtail_testkit::{RecordedHostCall, RecordingHostServices, RecordingOutcome};

    pub(crate) struct WedgedShutdown {
        host: RecordingHostServices,
        ticks: Arc<AtomicU64>,
        stages: Mutex<Vec<&'static str>>,
    }

    impl WedgedShutdown {
        pub(crate) fn new(host: ExecutionHostId) -> Self {
            Self {
                host: RecordingHostServices::for_host(host, RecordingOutcome::Succeed),
                ticks: Arc::default(),
                stages: Mutex::default(),
            }
        }

        pub(crate) fn services(self: &Arc<Self>) -> HostServices {
            self.host.services().clone().with_time(self.clone())
        }

        pub(crate) fn leases() -> (Option<ResourceLease>, Option<CredentialLease>) {
            let scope = ScopeId::new("pi.close.fixture").expect("scope is valid");
            (
                Some(ResourceLease::consumer_owned(
                    scope.clone(),
                    WorkingResourceRef::new("pi.fixture.workspace").expect("resource ref is valid"),
                    ResourceAccess::Read,
                    ResourceRepresentation::Filesystem,
                )),
                Some(CredentialLease::Delegated(DelegatedCredential::new(
                    scope,
                    CredentialRef::new("pi.fixture.credential").expect("credential ref is valid"),
                    EndpointAudience::new("pi.fixture.endpoint").expect("audience is valid"),
                ))),
            )
        }

        pub(crate) fn assert_escalation(
            &self,
            session: impl InteractiveSessionHandle + 'static,
            services: HostServices,
            native_close: bool,
        ) {
            let deadline = Deadline::at(MonotonicInstant::from_ticks(1000));
            let mut close = Box::new(session).close(SessionCleanupRequest::new(deadline), services);
            let mut context = Context::from_waker(Waker::noop());
            assert!(close.as_mut().poll(&mut context).is_pending());
            let expected = if native_close {
                "native close"
            } else {
                "request stop"
            };
            assert_eq!(
                self.stages.lock().expect("stage lock").as_slice(),
                [expected]
            );

            self.ticks.store(500, Ordering::SeqCst);
            let Poll::Ready(outcome) = close.as_mut().poll(&mut context) else {
                panic!("forced shutdown must complete the pump join and release both leases");
            };
            assert!(matches!(
                outcome,
                CleanupOutcome::Degraded(diagnostic)
                    if diagnostic.code() == "swallowtail.pi.shutdown_grace_expired"
            ));
            let stages = self.stages.lock().expect("stage lock");
            let stages = if native_close {
                &stages[1..]
            } else {
                &stages[..]
            };
            assert_eq!(stages, ["request stop", "force stop", "reap"]);
            assert!(self.host.calls().ends_with(&[
                RecordedHostCall::WorkingResourceRelease,
                RecordedHostCall::CredentialRelease,
            ]));
        }

        fn record(&self, stage: &'static str) {
            self.stages.lock().expect("stage lock").push(stage);
        }
    }

    pub(crate) struct Pump(pub(crate) BoxFuture<'static, ()>);

    impl JoinedTask for Pump {
        fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
            Box::pin(async move {
                self.0.await;
                Ok(())
            })
        }
    }

    impl ProcessHandle for WedgedShutdown {
        fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.record("native close");
            Box::pin(async { Ok(()) })
        }

        fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            Box::pin(async { Ok(()) })
        }

        fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
            Box::pin(poll_fn(|_| {
                if self
                    .stages
                    .lock()
                    .expect("stage lock")
                    .contains(&"force stop")
                {
                    Poll::Ready(Ok(None))
                } else {
                    Poll::Pending
                }
            }))
        }

        fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.record("request stop");
            Box::pin(async { Ok(()) })
        }

        fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
            self.record("force stop");
            Box::pin(async { Ok(()) })
        }

        fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
            assert!(!self.host.calls().iter().any(|call| matches!(
                call,
                RecordedHostCall::WorkingResourceRelease | RecordedHostCall::CredentialRelease
            )));
            self.record("reap");
            Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
        }
    }

    impl TimeService for WedgedShutdown {
        fn now(&self) -> MonotonicInstant {
            MonotonicInstant::from_ticks(self.ticks.load(Ordering::SeqCst))
        }

        fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
            let ticks = Arc::clone(&self.ticks);
            Box::pin(poll_fn(move |_| {
                let now = MonotonicInstant::from_ticks(ticks.load(Ordering::SeqCst));
                if now >= deadline.instant() {
                    Poll::Ready(DeadlineObservation::new(deadline, now))
                } else {
                    Poll::Pending
                }
            }))
        }
    }
}
