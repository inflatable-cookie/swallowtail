use super::KimiAcpDriver;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProviderSessionCatalogueDriver,
    ProviderSessionCatalogueOutcome, ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest,
    ProviderSessionImportDriver, ProviderSessionImportOutcome, ProviderSessionImportPlan,
    ProviderSessionImportRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, validate_provider_session_catalogue_execution,
    validate_provider_session_import_execution,
};

mod control;
mod projection;

use control::{Controlled, deadline_wait, wait_controlled};
use projection::{find_candidate, limits, negotiated_capabilities, project_page};

impl ProviderSessionCatalogueDriver for KimiAcpDriver {
    fn list_provider_sessions(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>>
    {
        Box::pin(async move {
            validate_provider_session_catalogue_execution(&plan, &request, &services)?;
            self.execute_session_catalogue(plan, request, services)
                .await
        })
    }
}

impl ProviderSessionImportDriver for KimiAcpDriver {
    fn import_provider_session(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>> {
        Box::pin(async move {
            validate_provider_session_import_execution(&plan, &request, &services)?;
            self.execute_session_import(plan, request, services).await
        })
    }
}

impl KimiAcpDriver {
    async fn execute_session_catalogue(
        &self,
        plan: ProviderSessionCataloguePlan,
        request: ProviderSessionCatalogueRequest,
        services: HostServices,
    ) -> Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure> {
        let selected = self
            .validate_plan(plan.preflight())
            .map_err(before_dispatch)?;
        check_control(
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        )?;
        let mut attachment = self
            .start_catalogue_attachment(
                plan.preflight(),
                request.request_id(),
                plan.agreement().scope().working_resource_ref().clone(),
                &services,
            )
            .await
            .map_err(catalogue_dispatch)?;
        let projected = async {
            let initialize = attachment
                .connection
                .initialize_catalogue()
                .await
                .map_err(catalogue_dispatch)?;
            super::validate_initialize(&initialize, selected.version().as_str())
                .map_err(before_dispatch)?;
            let capabilities = negotiated_capabilities(
                &initialize,
                ProviderSessionOperationFailureStage::BeforeDispatch,
            )?;
            let page = wait_controlled(
                attachment.connection.list_sessions(
                    capabilities,
                    attachment.cwd.clone(),
                    request
                        .cursor()
                        .map(|cursor| cursor.as_provider_value().to_owned()),
                    limits(&plan),
                ),
                request.cancellation().as_ref(),
                deadline_wait(request.agreement().deadline(), &services)?,
            )
            .await;
            let page = match page {
                Controlled::Completed(result) => result.map_err(catalogue_list_failure)?,
                Controlled::Cancelled => return Err(cancelled()),
                Controlled::Deadline => return Err(timed_out()),
            };
            project_page(
                &plan,
                &page,
                request
                    .cursor()
                    .map_or(0, |cursor| cursor.observed_candidates()),
            )
        }
        .await;
        let cleanup = attachment.abort(&services).await;
        let (candidates, next_cursor) = finish(projected, cleanup.clone())?;
        ProviderSessionCatalogueOutcome::new(&plan, &request, candidates, next_cursor, cleanup)
    }

    async fn execute_session_import(
        &self,
        plan: ProviderSessionImportPlan,
        request: ProviderSessionImportRequest,
        services: HostServices,
    ) -> Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure> {
        let selected = self
            .validate_plan(plan.preflight())
            .map_err(before_dispatch)?;
        check_control(
            request.cancellation().as_ref(),
            request.agreement().deadline(),
            &services,
        )?;
        let mut attachment = self
            .start_catalogue_attachment(
                plan.preflight(),
                request.request_id(),
                request.agreement().working_resource().clone(),
                &services,
            )
            .await
            .map_err(import_revalidation)?;
        let revalidated = async {
            let initialize = attachment
                .connection
                .initialize_catalogue()
                .await
                .map_err(import_revalidation)?;
            super::validate_initialize(&initialize, selected.version().as_str())
                .map_err(import_revalidation)?;
            let capabilities = negotiated_capabilities(
                &initialize,
                ProviderSessionOperationFailureStage::ImportRevalidation,
            )?;
            match wait_controlled(
                find_candidate(
                    &plan,
                    &request,
                    Arc::clone(&attachment.connection),
                    attachment.cwd.clone(),
                    capabilities,
                ),
                request.cancellation().as_ref(),
                deadline_wait(request.agreement().deadline(), &services)?,
            )
            .await
            {
                Controlled::Completed(result) => result,
                Controlled::Cancelled => Err(cancelled()),
                Controlled::Deadline => Err(timed_out()),
            }
        }
        .await;
        let cleanup = attachment.abort(&services).await;
        let revalidation = finish(revalidated, cleanup.clone())?;
        ProviderSessionImportOutcome::new(&plan, &request, revalidation, cleanup)
    }
}

fn check_control(
    cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    if cancellation.is_requested() {
        return Err(cancelled());
    }
    if deadline.is_some_and(|deadline| {
        services
            .time()
            .is_some_and(|time| time.now() >= deadline.instant())
    }) {
        return Err(timed_out());
    }
    Ok(())
}

fn cancelled() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::Cancelled,
        "swallowtail.kimi.provider_session.cancelled",
        "Kimi provider-session operation was cancelled",
    )
}

fn timed_out() -> ProviderSessionOperationFailure {
    operation_failure(
        ProviderSessionOperationFailureStage::TimedOut,
        "swallowtail.kimi.provider_session.timed_out",
        "Kimi provider-session operation timed out",
    )
}

fn finish<T>(
    result: Result<T, ProviderSessionOperationFailure>,
    cleanup: CleanupOutcome,
) -> Result<T, ProviderSessionOperationFailure> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => result,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            Err(ProviderSessionOperationFailure::new(
                ProviderSessionOperationFailureStage::Cleanup,
                diagnostic,
            ))
        }
    }
}

fn catalogue_dispatch(
    error: swallowtail_runtime::RuntimeFailure,
) -> ProviderSessionOperationFailure {
    from_runtime(
        ProviderSessionOperationFailureStage::CatalogueDispatch,
        error,
    )
}

fn catalogue_projection(
    error: swallowtail_runtime::RuntimeFailure,
) -> ProviderSessionOperationFailure {
    from_runtime(
        ProviderSessionOperationFailureStage::CatalogueProjection,
        error,
    )
}

fn catalogue_list_failure(
    error: swallowtail_runtime::RuntimeFailure,
) -> ProviderSessionOperationFailure {
    if error.diagnostic().code() == "swallowtail.kimi.acp.session_list_response_invalid" {
        catalogue_projection(error)
    } else {
        catalogue_dispatch(error)
    }
}

fn import_revalidation(
    error: swallowtail_runtime::RuntimeFailure,
) -> ProviderSessionOperationFailure {
    from_runtime(
        ProviderSessionOperationFailureStage::ImportRevalidation,
        error,
    )
}

fn before_dispatch(error: swallowtail_runtime::RuntimeFailure) -> ProviderSessionOperationFailure {
    from_runtime(ProviderSessionOperationFailureStage::BeforeDispatch, error)
}

fn from_runtime(
    stage: ProviderSessionOperationFailureStage,
    error: swallowtail_runtime::RuntimeFailure,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, error.diagnostic().clone())
}

pub(super) fn operation_failure(
    stage: ProviderSessionOperationFailureStage,
    code: &'static str,
    message: &'static str,
) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::new(stage, SafeDiagnostic::new(code, message))
}
