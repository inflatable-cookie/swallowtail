#![deny(missing_docs)]

use crate::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle, LoadedSession,
    OpenSessionRequest, ProviderRunReconciliationOutcome, ProviderSessionReconciliationOutcome,
    ResumeSessionRequest, RuntimeFailure, RuntimeTurnId, SessionReplayItem,
};

mod realtime;

pub use realtime::FreshRealtimeSessionReplacementOutcome;

/// Route-qualified operation used by the common restart facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkingStateRestorationMethod {
    /// Read-only observation of an interrupted provider session turn.
    ProviderSessionReconciliation,
    /// Read-only observation of an interrupted provider-owned run.
    ProviderRunReconciliation,
    /// Stateful load with bounded replay and no interrupted-turn state claim.
    ProviderSessionContinuationRecovery,
    /// Live attachment to the exact session without a replay claim.
    ProviderSessionAttachmentRecovery,
    /// New interactive session with explicit provider-context loss.
    FreshSessionReplacement,
    /// New realtime session with explicit connection-context loss.
    FreshRealtimeSessionReplacement,
}

/// A live attachment to the exact provider session with no replay claim.
pub struct ProviderSessionAttachmentRecoveryOutcome {
    interrupted_turn_id: RuntimeTurnId,
    session: Box<dyn InteractiveSessionHandle>,
}

impl ProviderSessionAttachmentRecoveryOutcome {
    /// Creates an exact-session attachment outcome without replay evidence.
    #[must_use]
    pub const fn new(
        interrupted_turn_id: RuntimeTurnId,
        session: Box<dyn InteractiveSessionHandle>,
    ) -> Self {
        Self {
            interrupted_turn_id,
            session,
        }
    }

    #[must_use]
    /// Returns the unresolved interrupted consumer turn.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    /// Separates the interrupted turn identity from the live session.
    pub fn into_parts(self) -> (RuntimeTurnId, Box<dyn InteractiveSessionHandle>) {
        (self.interrupted_turn_id, self.session)
    }
}

/// A new usable session which carries no provider context from the lost one.
pub struct FreshSessionReplacementOutcome {
    interrupted_turn_id: RuntimeTurnId,
    session: Box<dyn InteractiveSessionHandle>,
}

impl FreshSessionReplacementOutcome {
    /// Creates a context-losing replacement outcome.
    #[must_use]
    pub const fn new(
        interrupted_turn_id: RuntimeTurnId,
        session: Box<dyn InteractiveSessionHandle>,
    ) -> Self {
        Self {
            interrupted_turn_id,
            session,
        }
    }

    #[must_use]
    /// Returns the unresolved interrupted consumer turn.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    /// Separates the interrupted turn identity from the new live session.
    pub fn into_parts(self) -> (RuntimeTurnId, Box<dyn InteractiveSessionHandle>) {
        (self.interrupted_turn_id, self.session)
    }
}

/// A stateful loaded session whose interrupted provider turn remains unresolved.
pub struct ProviderSessionContinuationRecoveryOutcome {
    interrupted_turn_id: RuntimeTurnId,
    loaded: LoadedSession,
}

impl ProviderSessionContinuationRecoveryOutcome {
    /// Creates a continuation-recovery outcome from one loaded session.
    #[must_use]
    pub const fn new(interrupted_turn_id: RuntimeTurnId, loaded: LoadedSession) -> Self {
        Self {
            interrupted_turn_id,
            loaded,
        }
    }

    #[must_use]
    /// Returns the unresolved interrupted consumer turn.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    /// Iterates over the bounded replay supplied by the load operation.
    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.loaded.replay()
    }

    #[must_use]
    /// Separates the interrupted turn identity from the loaded live session.
    pub fn into_parts(self) -> (RuntimeTurnId, LoadedSession) {
        (self.interrupted_turn_id, self.loaded)
    }
}

/// Truth-preserving result of the route-selected restoration method.
pub enum WorkingStateRestorationOutcome {
    /// Read-only provider-session observation.
    SessionReconciled(ProviderSessionReconciliationOutcome),
    /// Read-only provider-run observation.
    RunReconciled(ProviderRunReconciliationOutcome),
    /// Stateful provider-session continuation recovery.
    SessionRecovered(ProviderSessionContinuationRecoveryOutcome),
    /// Exact provider-session attachment without replay.
    SessionReattached(ProviderSessionAttachmentRecoveryOutcome),
    /// Fresh interactive session with context loss.
    SessionReplaced(FreshSessionReplacementOutcome),
    /// Fresh realtime session with context loss.
    RealtimeSessionReplaced(FreshRealtimeSessionReplacementOutcome),
}

impl WorkingStateRestorationOutcome {
    /// Returns the exact route-selected method that produced this outcome.
    #[must_use]
    pub const fn method(&self) -> WorkingStateRestorationMethod {
        match self {
            Self::SessionReconciled(_) => {
                WorkingStateRestorationMethod::ProviderSessionReconciliation
            }
            Self::RunReconciled(_) => WorkingStateRestorationMethod::ProviderRunReconciliation,
            Self::SessionRecovered(_) => {
                WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
            }
            Self::SessionReattached(_) => {
                WorkingStateRestorationMethod::ProviderSessionAttachmentRecovery
            }
            Self::SessionReplaced(_) => WorkingStateRestorationMethod::FreshSessionReplacement,
            Self::RealtimeSessionReplaced(_) => {
                WorkingStateRestorationMethod::FreshRealtimeSessionReplacement
            }
        }
    }
}

/// One already prepared, route-qualified restoration operation.
///
/// The consuming receiver makes the common facade exact-once. Implementations
/// must not widen authority or select another method after execution begins.
pub trait WorkingStateRestorationOperation: Send + Sync {
    /// Returns the prepared method without starting provider work.
    fn method(&self) -> WorkingStateRestorationMethod;

    /// Consumes and executes the single prepared restoration method.
    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>>;
}

/// Consumer-facing facade over one route-selected restoration operation.
pub struct PreparedWorkingStateRestoration {
    operation: Box<dyn WorkingStateRestorationOperation>,
}

impl PreparedWorkingStateRestoration {
    /// Wraps one already prepared route-qualified restoration operation.
    #[must_use]
    pub fn new(operation: impl WorkingStateRestorationOperation + 'static) -> Self {
        Self {
            operation: Box::new(operation),
        }
    }

    #[must_use]
    /// Prepares an exact-session attachment operation without replay.
    pub fn provider_session_attachment_recovery(
        interrupted_turn_id: RuntimeTurnId,
        driver: impl InteractiveSessionDriver + 'static,
        plan: swallowtail_core::PreflightPlan,
        request: ResumeSessionRequest,
    ) -> Self {
        Self::new(SessionAttachmentRecoveryOperation {
            interrupted_turn_id,
            driver: Box::new(driver),
            plan,
            request,
        })
    }

    #[must_use]
    /// Prepares a fresh interactive session with explicit context loss.
    pub fn fresh_session_replacement(
        interrupted_turn_id: RuntimeTurnId,
        driver: impl InteractiveSessionDriver + 'static,
        plan: swallowtail_core::PreflightPlan,
        request: OpenSessionRequest,
    ) -> Self {
        Self::new(FreshSessionReplacementOperation {
            interrupted_turn_id,
            driver: Box::new(driver),
            plan,
            request,
        })
    }

    #[must_use]
    /// Returns the selected method without executing it.
    pub fn method(&self) -> WorkingStateRestorationMethod {
        self.operation.method()
    }

    /// Consumes and executes the prepared restoration operation.
    pub fn restore(
        self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        self.operation.restore(services)
    }
}

struct SessionAttachmentRecoveryOperation {
    interrupted_turn_id: RuntimeTurnId,
    driver: Box<dyn InteractiveSessionDriver>,
    plan: swallowtail_core::PreflightPlan,
    request: ResumeSessionRequest,
}

impl WorkingStateRestorationOperation for SessionAttachmentRecoveryOperation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionAttachmentRecovery
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            let session = self
                .driver
                .recover_session_attachment(self.plan, self.request, services)
                .await?;
            Ok(WorkingStateRestorationOutcome::SessionReattached(
                ProviderSessionAttachmentRecoveryOutcome::new(self.interrupted_turn_id, session),
            ))
        })
    }
}

struct FreshSessionReplacementOperation {
    interrupted_turn_id: RuntimeTurnId,
    driver: Box<dyn InteractiveSessionDriver>,
    plan: swallowtail_core::PreflightPlan,
    request: OpenSessionRequest,
}

impl WorkingStateRestorationOperation for FreshSessionReplacementOperation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::FreshSessionReplacement
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            let session = self
                .driver
                .open_session(self.plan, self.request, services)
                .await?;
            Ok(WorkingStateRestorationOutcome::SessionReplaced(
                FreshSessionReplacementOutcome::new(self.interrupted_turn_id, session),
            ))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PreparedWorkingStateRestoration, WorkingStateRestorationMethod,
        WorkingStateRestorationOperation, WorkingStateRestorationOutcome,
    };
    use crate::{BoxFuture, HostServices, RuntimeFailure};
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};
    use swallowtail_core::ExecutionHostId;

    struct FailedSessionReconciliation;

    impl WorkingStateRestorationOperation for FailedSessionReconciliation {
        fn method(&self) -> WorkingStateRestorationMethod {
            WorkingStateRestorationMethod::ProviderSessionReconciliation
        }

        fn restore(
            self: Box<Self>,
            _services: HostServices,
        ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "fixture.reconciliation_failed",
                    "Fixture reconciliation failed without fallback",
                )))
            })
        }
    }

    #[test]
    fn prepared_method_is_visible_and_failure_is_returned_without_fallback() {
        let prepared = PreparedWorkingStateRestoration::new(FailedSessionReconciliation);
        assert_eq!(
            prepared.method(),
            WorkingStateRestorationMethod::ProviderSessionReconciliation
        );
        let services = HostServices::new(
            ExecutionHostId::new("fixture.host").expect("fixture host id is valid"),
        );
        let mut future = prepared.restore(services);
        let mut context = Context::from_waker(Waker::noop());
        let result = match Pin::new(&mut future).poll(&mut context) {
            Poll::Ready(result) => result,
            Poll::Pending => panic!("fixture restoration must resolve immediately"),
        };
        let error = match result {
            Ok(_) => panic!("fixture must fail"),
            Err(error) => error,
        };
        assert_eq!(error.diagnostic().code(), "fixture.reconciliation_failed");
    }
}
