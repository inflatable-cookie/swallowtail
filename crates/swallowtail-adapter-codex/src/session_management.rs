use crate::CodexAppServerDriver;
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, BoxFuture, DeleteProviderSessionRequest, HostServices,
    ProviderSessionManagementDriver, ProviderSessionManagementOutcome,
    ProviderSessionManagementPlan, RestoreProviderSessionRequest, RuntimeFailure,
    validate_provider_session_management_request,
};

mod execution;
mod protocol;

impl ProviderSessionManagementDriver for CodexAppServerDriver {
    fn archive_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: ArchiveProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.manage(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id().as_str(),
                services,
            )
            .await
        })
    }

    fn restore_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: RestoreProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.manage(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id().as_str(),
                services,
            )
            .await
        })
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.manage(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id().as_str(),
                services,
            )
            .await
        })
    }
}
