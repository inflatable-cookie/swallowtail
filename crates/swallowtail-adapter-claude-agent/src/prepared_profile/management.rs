use crate::ClaudeAgentAcpDriver;
use swallowtail_runtime::{
    BoxFuture, DeleteProviderSessionRequest, EnvironmentRef, HostServices,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RuntimeFailure,
};

#[path = "management/preparation.rs"]
mod preparation;

#[derive(Clone, Debug)]
pub struct ClaudeAgentPreparedDelete {
    environment: EnvironmentRef,
    credential: swallowtail_core::CredentialRef,
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl ClaudeAgentPreparedDelete {
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
        let driver = ClaudeAgentAcpDriver::new(self.environment.clone(), self.credential.clone());
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}
