use crate::ClaudeAgentAcpDriver;
use swallowtail_runtime::{
    BoxFuture, DeleteProviderSessionRequest, EnvironmentRef, HostServices,
    PreparedProviderSessionManagementEvidence, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RuntimeFailure,
};

#[path = "management/preparation.rs"]
mod preparation;

#[derive(Clone, Debug)]
/// Prepared delete operation for one inactive Claude Agent session.
pub struct ClaudeAgentPreparedDelete {
    environment: EnvironmentRef,
    credential: Option<swallowtail_core::CredentialRef>,
    evidence: PreparedProviderSessionManagementEvidence,
    request: DeleteProviderSessionRequest,
}

impl ClaudeAgentPreparedDelete {
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

    /// Executes the prepared session deletion.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        let driver = match self.credential.as_ref() {
            Some(credential) => {
                ClaudeAgentAcpDriver::new(self.environment.clone(), credential.clone())
            }
            None => ClaudeAgentAcpDriver::with_local_auth(self.environment.clone()),
        };
        let plan = self.plan().clone();
        let request = self.request.clone();
        Box::pin(async move { driver.delete_session(plan, request, services).await })
    }
}
