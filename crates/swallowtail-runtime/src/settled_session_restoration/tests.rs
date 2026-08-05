use super::{
    PreparedSettledSessionRestoration, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionAttachmentOperation, SettledSessionReconciliationOperation,
    SettledSessionRestorationFailurePhase, SettledSessionRestorationOutcome,
};
use crate::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, ImmediateCancellation,
    InteractiveSessionHandle, InterruptedTurnState, LoadedSession,
    ProviderSessionReconciliationOutcome, RequestId, RuntimeFailure, RuntimeSessionId, TurnHandle,
    TurnRequest,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{CancellationScope, ExecutionHostId, SafeDiagnostic, SessionRef};

enum ReconciliationBehavior {
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

enum AttachmentBehavior {
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

#[test]
fn only_settled_or_inactive_state_dispatches_attachment() {
    for (state, eligible) in [
        (InterruptedTurnState::Active, false),
        (InterruptedTurnState::WaitingForProviderInput, false),
        (InterruptedTurnState::Completed, true),
        (InterruptedTurnState::Failed, true),
        (InterruptedTurnState::Cancelled, true),
        (InterruptedTurnState::InactiveUnresolved, true),
        (InterruptedTurnState::Unknown, false),
    ] {
        let reconciliation_calls = Arc::new(AtomicUsize::new(0));
        let attachment_calls = Arc::new(AtomicUsize::new(0));
        let order = Arc::new(Mutex::new(Vec::new()));
        let prepared = fixture_sequence(
            ReconciliationBehavior::State(state),
            AttachmentBehavior::Fail,
            SettledSessionAttachmentKind::Load,
            Arc::clone(&reconciliation_calls),
            Arc::clone(&attachment_calls),
            Arc::clone(&order),
        );
        let result = resolve(prepared.restore(services()));

        assert_eq!(reconciliation_calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            attachment_calls.load(Ordering::SeqCst),
            usize::from(eligible)
        );
        if eligible {
            let failure = match result {
                Ok(_) => panic!("eligible fixture attachment must fail"),
                Err(failure) => failure,
            };
            assert_eq!(
                failure.phase(),
                SettledSessionRestorationFailurePhase::Attachment
            );
            assert_eq!(
                failure.reconciliation().expect("observation").state(),
                state
            );
        } else {
            let SettledSessionRestorationOutcome::Observed(observation) =
                result.expect("ineligible state is an observed success")
            else {
                panic!("ineligible state cannot attach");
            };
            assert_eq!(observation.state(), state);
        }
    }
}

#[test]
fn reconciliation_failure_starts_no_attachment() {
    let reconciliation_calls = Arc::new(AtomicUsize::new(0));
    let attachment_calls = Arc::new(AtomicUsize::new(0));
    let order = Arc::new(Mutex::new(Vec::new()));
    let prepared = fixture_sequence(
        ReconciliationBehavior::Fail,
        AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Load),
        SettledSessionAttachmentKind::Load,
        Arc::clone(&reconciliation_calls),
        Arc::clone(&attachment_calls),
        Arc::clone(&order),
    );

    let failure = match resolve(prepared.restore(services())) {
        Ok(_) => panic!("reconciliation must fail"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.phase(),
        SettledSessionRestorationFailurePhase::Reconciliation
    );
    assert!(failure.reconciliation().is_none());
    assert_eq!(reconciliation_calls.load(Ordering::SeqCst), 1);
    assert_eq!(attachment_calls.load(Ordering::SeqCst), 0);
    assert_eq!(*order.lock().expect("order lock poisoned"), ["reconcile"]);
}

#[test]
fn loaded_and_resumed_success_preserve_order_and_distinct_truth() {
    for kind in [
        SettledSessionAttachmentKind::Load,
        SettledSessionAttachmentKind::Resume,
    ] {
        let reconciliation_calls = Arc::new(AtomicUsize::new(0));
        let attachment_calls = Arc::new(AtomicUsize::new(0));
        let order = Arc::new(Mutex::new(Vec::new()));
        let prepared = fixture_sequence(
            ReconciliationBehavior::State(InterruptedTurnState::InactiveUnresolved),
            AttachmentBehavior::Succeed(kind),
            kind,
            reconciliation_calls,
            attachment_calls,
            Arc::clone(&order),
        );
        assert_eq!(prepared.attachment_kind(), kind);

        let SettledSessionRestorationOutcome::Attached(attached) =
            resolve(prepared.restore(services())).expect("attachment succeeds")
        else {
            panic!("eligible state attaches");
        };
        assert_eq!(
            attached.reconciliation().state(),
            InterruptedTurnState::InactiveUnresolved
        );
        assert_eq!(attached.attachment().kind(), kind);
        assert_eq!(
            *order.lock().expect("order lock poisoned"),
            ["reconcile", "attach"]
        );
        let (_, attachment) = attached.into_parts();
        let session = match attachment {
            SettledSessionAttachment::Loaded(loaded) => {
                assert_eq!(loaded.replay().len(), 0);
                loaded.into_parts().1
            }
            SettledSessionAttachment::Resumed(session) => session,
        };
        assert_eq!(resolve(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn attachment_kind_mismatch_preserves_reconciliation_and_returns_no_handle() {
    let prepared = fixture_sequence(
        ReconciliationBehavior::State(InterruptedTurnState::InactiveUnresolved),
        AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Resume),
        SettledSessionAttachmentKind::Load,
        Arc::new(AtomicUsize::new(0)),
        Arc::new(AtomicUsize::new(0)),
        Arc::new(Mutex::new(Vec::new())),
    );

    let failure = match resolve(prepared.restore(services())) {
        Ok(_) => panic!("kind mismatch must fail"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.phase(),
        SettledSessionRestorationFailurePhase::Attachment
    );
    assert_eq!(
        failure.reconciliation().expect("observation").state(),
        InterruptedTurnState::InactiveUnresolved
    );
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.settled_session_restoration.attachment_kind_mismatch"
    );
}

fn fixture_sequence(
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

fn services() -> HostServices {
    HostServices::new(ExecutionHostId::new("fixture.host").expect("host id is valid"))
}

fn failure(code: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, "Fixture operation failed"))
}

fn resolve<T>(mut future: Pin<Box<dyn Future<Output = T> + Send + '_>>) -> T {
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("fixture future must resolve immediately"),
    }
}

struct FixtureSession {
    request_id: RequestId,
    session_id: RuntimeSessionId,
    cancellation: ImmediateCancellation,
}

impl FixtureSession {
    fn new() -> Self {
        Self {
            request_id: RequestId::new("fixture-request").expect("request id is valid"),
            session_id: RuntimeSessionId::new("fixture-session").expect("session id is valid"),
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

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}
