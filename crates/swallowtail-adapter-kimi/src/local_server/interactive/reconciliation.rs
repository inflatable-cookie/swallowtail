use failure::preparation_failure;
use swallowtail_runtime::{
    BoxFuture, HostServices, PersistedProviderOperationCheckpoint, PreparationFailure,
    PreparedProviderSessionReconciliationEvidence, PreparedSettledSessionRestoration,
    ProviderSessionReconciliationBounds, ProviderSessionReconciliationDriver,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, RequestId, ResumeSessionRequest, RuntimeFailure,
    SessionResumeBinding, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionAttachmentOperation, SettledSessionReconciliationOperation,
    WorkingStateRestorationMethod, WorkingStateRestorationOperation,
    WorkingStateRestorationOutcome, settled_session_plans_share_binding,
};

mod execute;
mod failure;
mod observation;
mod prepare;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Inputs for read-only reconciliation of one interrupted local-server turn.
pub struct KimiLocalServerReconciliationInput {
    request_id: RequestId,
    model: crate::KimiModelSelection,
    binding: SessionResumeBinding,
    checkpoint: PersistedProviderOperationCheckpoint,
    bounds: ProviderSessionReconciliationBounds,
    deadline: Option<swallowtail_runtime::Deadline>,
}

impl KimiLocalServerReconciliationInput {
    /// Creates bounded reconciliation input from exact binding and checkpoint evidence.
    #[must_use]
    pub const fn new(
        request_id: RequestId,
        model: crate::KimiModelSelection,
        binding: SessionResumeBinding,
        checkpoint: PersistedProviderOperationCheckpoint,
        bounds: ProviderSessionReconciliationBounds,
    ) -> Self {
        Self {
            request_id,
            model,
            binding,
            checkpoint,
            bounds,
            deadline: None,
        }
    }

    /// Adds a reconciliation deadline.
    #[must_use]
    pub const fn with_deadline(mut self, deadline: swallowtail_runtime::Deadline) -> Self {
        self.deadline = Some(deadline);
        self
    }
}

#[derive(Clone, Debug)]
/// Prepared read-only reconciliation of one interrupted local-server turn.
pub struct KimiLocalServerPreparedReconciliation {
    evidence: PreparedProviderSessionReconciliationEvidence,
    request: ProviderSessionReconciliationRequest,
}

impl KimiLocalServerPreparedReconciliation {
    /// Returns portable evidence for the prepared reconciliation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionReconciliationEvidence {
        &self.evidence
    }

    /// Returns the exact reconciliation plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        self.evidence.plan()
    }

    /// Returns the bound reconciliation request.
    #[must_use]
    pub const fn request(&self) -> &ProviderSessionReconciliationRequest {
        &self.request
    }

    /// Observes retained provider truth for the interrupted turn.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        let driver = crate::KimiLocalServerDriver::new();
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move {
            driver
                .reconcile_provider_session(plan, request, services)
                .await
        })
    }

    /// Composes reconciliation with a separately prepared, binding-equal resume.
    pub fn prepare_settled_session_restoration(
        self,
        session: super::KimiLocalServerPreparedSession,
        attachment_request_id: RequestId,
    ) -> Result<PreparedSettledSessionRestoration, PreparationFailure> {
        if !settled_session_plans_share_binding(self.plan().preflight(), session.plan()) {
            return Err(preparation_failure(
                "swallowtail.kimi.local_server.preparation.settled_session_binding_mismatch",
                "Kimi reconciliation and attachment do not share one prepared route binding",
            ));
        }
        let request = session.resume_request(
            attachment_request_id,
            self.plan().agreement().binding().clone(),
        )?;
        Ok(PreparedSettledSessionRestoration::new(
            self,
            KimiSettledSessionResume { session, request },
        ))
    }
}

impl SettledSessionReconciliationOperation for KimiLocalServerPreparedReconciliation {
    fn reconcile(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        KimiLocalServerPreparedReconciliation::execute(&self, services)
    }
}

struct KimiSettledSessionResume {
    session: super::KimiLocalServerPreparedSession,
    request: ResumeSessionRequest,
}

impl SettledSessionAttachmentOperation for KimiSettledSessionResume {
    fn kind(&self) -> SettledSessionAttachmentKind {
        SettledSessionAttachmentKind::Resume
    }

    fn attach(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<SettledSessionAttachment, RuntimeFailure>> {
        let future = self.session.resume_prepared_session(self.request, services);
        Box::pin(async move { future.await.map(SettledSessionAttachment::Resumed) })
    }
}

impl WorkingStateRestorationOperation for KimiLocalServerPreparedReconciliation {
    fn method(&self) -> WorkingStateRestorationMethod {
        WorkingStateRestorationMethod::ProviderSessionReconciliation
    }

    fn restore(
        self: Box<Self>,
        services: HostServices,
    ) -> BoxFuture<'static, Result<WorkingStateRestorationOutcome, RuntimeFailure>> {
        let future = self.execute(services);
        Box::pin(async move {
            future
                .await
                .map(WorkingStateRestorationOutcome::SessionReconciled)
        })
    }
}
