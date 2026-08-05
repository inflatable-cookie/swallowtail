use crate::{
    BoxFuture, HostServices, InteractiveSessionDriver, InteractiveSessionHandle, LoadedSession,
    OpenSessionRequest, ProviderRunReconciliationOutcome, ProviderSessionReconciliationOutcome,
    ResumeSessionRequest, RuntimeFailure, RuntimeTurnId, SessionReplayItem,
};

/// Route-qualified operation used by the common restart facade.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkingStateRestorationMethod {
    ProviderSessionReconciliation,
    ProviderRunReconciliation,
    ProviderSessionContinuationRecovery,
    ProviderSessionAttachmentRecovery,
    FreshSessionReplacement,
}

/// A live attachment to the exact provider session with no replay claim.
pub struct ProviderSessionAttachmentRecoveryOutcome {
    interrupted_turn_id: RuntimeTurnId,
    session: Box<dyn InteractiveSessionHandle>,
}

impl ProviderSessionAttachmentRecoveryOutcome {
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
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
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
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
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
    #[must_use]
    pub const fn new(interrupted_turn_id: RuntimeTurnId, loaded: LoadedSession) -> Self {
        Self {
            interrupted_turn_id,
            loaded,
        }
    }

    #[must_use]
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.loaded.replay()
    }

    #[must_use]
    pub fn into_parts(self) -> (RuntimeTurnId, LoadedSession) {
        (self.interrupted_turn_id, self.loaded)
    }
}

/// Truth-preserving result of the route-selected restoration method.
pub enum WorkingStateRestorationOutcome {
    SessionReconciled(ProviderSessionReconciliationOutcome),
    RunReconciled(ProviderRunReconciliationOutcome),
    SessionRecovered(ProviderSessionContinuationRecoveryOutcome),
    SessionReattached(ProviderSessionAttachmentRecoveryOutcome),
    SessionReplaced(FreshSessionReplacementOutcome),
}

impl WorkingStateRestorationOutcome {
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
        }
    }
}

/// One already prepared, route-qualified restoration operation.
///
/// The consuming receiver makes the common facade exact-once. Implementations
/// must not widen authority or select another method after execution begins.
pub trait WorkingStateRestorationOperation: Send + Sync {
    fn method(&self) -> WorkingStateRestorationMethod;

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
    #[must_use]
    pub fn new(operation: impl WorkingStateRestorationOperation + 'static) -> Self {
        Self {
            operation: Box::new(operation),
        }
    }

    #[must_use]
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
    pub fn method(&self) -> WorkingStateRestorationMethod {
        self.operation.method()
    }

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
