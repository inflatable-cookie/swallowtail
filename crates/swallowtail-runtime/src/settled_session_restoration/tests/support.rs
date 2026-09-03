use super::super::{
    PreparedSettledSessionRestoration, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionAttachmentOperation, SettledSessionReconciliationOperation,
};
use crate::{
    BoxFuture, CancellationControl, CleanupOutcome, Deadline, DeadlineObservation, HostServices,
    ImmediateCancellation, InteractiveSessionHandle, InterruptedTurnState, LoadedSession,
    MonotonicInstant, ProviderSessionReconciliationOutcome, RequestId, RuntimeFailure,
    RuntimeSessionId, SessionCleanupRequest, TimeService, TurnHandle, TurnRequest,
    bound_session_cleanup,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{CancellationScope, ExecutionHostId, SafeDiagnostic, SessionRef};

pub(super) enum ReconciliationBehavior {
    State(InterruptedTurnState),
    Fail,
}

struct FixtureReconciliation {
    behavior: ReconciliationBehavior,
    calls: Arc<AtomicUsize>,
    order: Arc<Mutex<Vec<&'static str>>>,
}

impl SettledSessionReconciliationOperation for FixtureReconciliation {
    fn reconcile(
        self: Box<Self>,
        _services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.order
            .lock()
            .expect("order lock poisoned")
            .push("reconcile");
        Box::pin(async move {
            match self.behavior {
                ReconciliationBehavior::State(state) => {
                    Ok(ProviderSessionReconciliationOutcome::fixture(state))
                }
                ReconciliationBehavior::Fail => Err(failure("fixture.reconciliation_failed")),
            }
        })
    }
}

pub(super) enum AttachmentBehavior {
    Succeed(SettledSessionAttachmentKind),
    Fail,
}

struct FixtureAttachment {
    advertised: SettledSessionAttachmentKind,
    behavior: AttachmentBehavior,
    calls: Arc<AtomicUsize>,
    order: Arc<Mutex<Vec<&'static str>>>,
}

impl SettledSessionAttachmentOperation for FixtureAttachment {
    fn kind(&self) -> SettledSessionAttachmentKind {
        self.advertised
    }

    fn attach(
        self: Box<Self>,
        _services: HostServices,
    ) -> BoxFuture<'static, Result<SettledSessionAttachment, RuntimeFailure>> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        self.order
            .lock()
            .expect("order lock poisoned")
            .push("attach");
        Box::pin(async move {
            match self.behavior {
                AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Load) => {
                    Ok(SettledSessionAttachment::Loaded(LoadedSession::new(
                        Vec::new(),
                        Box::new(FixtureSession::new()),
                    )))
                }
                AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Resume) => Ok(
                    SettledSessionAttachment::Resumed(Box::new(FixtureSession::new())),
                ),
                AttachmentBehavior::Fail => Err(failure("fixture.attachment_failed")),
            }
        })
    }
}

pub(super) fn fixture_sequence(
    reconciliation: ReconciliationBehavior,
    attachment: AttachmentBehavior,
    advertised: SettledSessionAttachmentKind,
    reconciliation_calls: Arc<AtomicUsize>,
    attachment_calls: Arc<AtomicUsize>,
    order: Arc<Mutex<Vec<&'static str>>>,
) -> PreparedSettledSessionRestoration {
    PreparedSettledSessionRestoration::new(
        FixtureReconciliation {
            behavior: reconciliation,
            calls: reconciliation_calls,
            order: Arc::clone(&order),
        },
        FixtureAttachment {
            advertised,
            behavior: attachment,
            calls: attachment_calls,
            order,
        },
    )
}

pub(super) fn services() -> HostServices {
    HostServices::new(fixture_host_id()).with_time(Arc::new(NeverTime))
}

pub(super) fn cleanup_request() -> SessionCleanupRequest {
    SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(100)))
}

fn fixture_host_id() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host").expect("host id is valid")
}

struct NeverTime;

impl TimeService for NeverTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

pub(super) fn failure(code: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, "Fixture operation failed"))
}

pub(super) fn resolve<T>(mut future: Pin<Box<dyn Future<Output = T> + Send + '_>>) -> T {
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("fixture future must resolve immediately"),
    }
}

struct FixtureSession {
    request_id: RequestId,
    session_id: RuntimeSessionId,
    execution_host_id: ExecutionHostId,
    cancellation: ImmediateCancellation,
}

impl FixtureSession {
    fn new() -> Self {
        Self {
            request_id: RequestId::new("fixture-request").expect("request id is valid"),
            session_id: RuntimeSessionId::new("fixture-session").expect("session id is valid"),
            execution_host_id: fixture_host_id(),
            cancellation: ImmediateCancellation::new(CancellationScope::InteractiveSession),
        }
    }
}

impl InteractiveSessionHandle for FixtureSession {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.session_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&crate::SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        _request: TurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async { Err(failure("fixture.turn_unavailable")) })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(
        self: Box<Self>,
        request: SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        bound_session_cleanup(
            self.execution_host_id.clone(),
            request,
            services,
            Box::pin(async { CleanupOutcome::Clean }),
        )
    }
}
