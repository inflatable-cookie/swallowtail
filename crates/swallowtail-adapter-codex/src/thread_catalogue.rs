use crate::CodexAppServerDriver;
use swallowtail_runtime::{
    BoxFuture, HostServices, ProviderSessionCatalogueDriver, ProviderSessionCatalogueOutcome,
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionImportDriver,
    ProviderSessionImportOutcome, ProviderSessionImportPlan, ProviderSessionImportRequest,
    ProviderSessionOperationFailure, validate_provider_session_catalogue_execution,
    validate_provider_session_import_execution,
};

#[path = "thread_catalogue/execution.rs"]
mod execution;
#[path = "thread_catalogue/protocol.rs"]
mod protocol;

impl ProviderSessionCatalogueDriver for CodexAppServerDriver {
    fn list_provider_sessions(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        Box::pin(async move {
            validate_provider_session_catalogue_execution(&plan, &request, &services)?;
            self.execute_thread_catalogue(plan, request, services).await
        })
    }
}

impl ProviderSessionImportDriver for CodexAppServerDriver {
    fn import_provider_session(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>> {
        Box::pin(async move {
            validate_provider_session_import_execution(&plan, &request, &services)?;
            self.execute_thread_import(plan, request, services).await
        })
    }
}
