use super::*;
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, DeleteProviderSessionRequest, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan, RestoreProviderSessionRequest,
    validate_provider_session_management_request,
};

mod access;
mod control;
mod execution;

impl ProviderSessionManagementDriver for ClaudeAgentAcpDriver {
    fn archive_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: ArchiveProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session archive")) })
    }

    fn restore_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: RestoreProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session restore")) })
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.manage_delete(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id(),
                services,
            )
            .await
        })
    }
}
