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
pub struct CodexPreparedArchive {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: ArchiveProviderSessionRequest,
}

impl CodexPreparedArchive {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &ArchiveProviderSessionRequest {
        &self.request
    }

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
pub struct CodexPreparedRestore {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: RestoreProviderSessionRequest,
}

impl CodexPreparedRestore {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &RestoreProviderSessionRequest {
        &self.request
    }

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
pub struct CodexPreparedDelete {
    environment: swallowtail_runtime::EnvironmentRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl CodexPreparedDelete {
    #[must_use]
    pub const fn evidence(&self) -> &PreparedProviderSessionManagementEvidence {
        &self.evidence
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        self.evidence.plan()
    }

    #[must_use]
    pub const fn request(&self) -> &DeleteProviderSessionRequest {
        &self.request
    }

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
