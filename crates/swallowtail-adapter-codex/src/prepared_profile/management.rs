use crate::CodexAppServerDriver;
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, DeleteProviderSessionRequest, HostServices,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RestoreProviderSessionRequest,
    RuntimeFailure,
};

#[path = "management/preparation.rs"]
mod preparation;

#[derive(Clone, Debug)]
/// Prepared archive operation for one inactive Codex thread.
pub struct CodexPreparedArchive {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: ArchiveProviderSessionRequest,
}

impl CodexPreparedArchive {
    /// Returns portable evidence for the prepared management operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the exact management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the bound archive request.
    #[must_use]
    pub const fn request(&self) -> &ArchiveProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared archive operation.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = CodexAppServerDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.archive_session(plan, request, services).await })
    }
}

#[derive(Clone, Debug)]
/// Prepared restore operation for one inactive Codex thread.
pub struct CodexPreparedRestore {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: RestoreProviderSessionRequest,
}

impl CodexPreparedRestore {
    /// Returns portable evidence for the prepared management operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the exact management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the bound restore request.
    #[must_use]
    pub const fn request(&self) -> &RestoreProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared restore operation.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = CodexAppServerDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.restore_session(plan, request, services).await })
    }
}

#[derive(Clone, Debug)]
/// Prepared delete operation for one inactive Codex thread.
pub struct CodexPreparedDelete {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl CodexPreparedDelete {
    /// Returns portable evidence for the prepared management operation.
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    /// Returns the exact management plan.
    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    /// Returns the bound delete request.
    #[must_use]
    pub const fn request(&self) -> &DeleteProviderSessionRequest {
        &self.request
    }

    /// Executes the prepared delete operation.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = CodexAppServerDriver::new(self.environment.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}
