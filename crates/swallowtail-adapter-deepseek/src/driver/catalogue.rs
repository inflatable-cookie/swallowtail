use super::DeepSeekDirectDriver;
use super::access::AccessLeases;
use super::lifecycle::complete_before_deadline;
use crate::failure::{failure, protocol, provider};
use crate::protocol::{HttpRequest, parse_models, require_success};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{DriverRole, ModelCatalogEntry, PreflightPlan};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ModelCatalogDriver, ModelCatalogRequest,
    RuntimeFailure, ScopeId,
};

impl ModelCatalogDriver for DeepSeekDirectDriver {
    fn list_models(
        &self,
        plan: PreflightPlan,
        request: ModelCatalogRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<ModelCatalogEntry>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            if plan.requirements().driver_role() != DriverRole::ModelCatalog {
                return Err(failure(
                    "swallowtail.deepseek.role_mismatch",
                    "DeepSeek catalogue requires a catalogue preflight plan",
                ));
            }
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, false)?;
            let scope = operation_scope("catalog", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let response = complete_before_deadline(
                self.transport.request(
                    scope,
                    access.endpoint.clone(),
                    access.secret()?,
                    HttpRequest::models(),
                    &services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                &services,
                cancelled,
            )
            .await;
            let result = response.and_then(|response| {
                require_success(&response).map_err(provider)?;
                parse_models(&response.body).map_err(protocol)
            });
            let cleanup = access.release(&services).await;
            match (result, cleanup) {
                (Ok(models), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(models),
                (Err(error), _) => Err(error),
                (Ok(_), _) => Err(failure(
                    "swallowtail.deepseek.catalogue_cleanup_failed",
                    "DeepSeek catalogue credential cleanup failed",
                )),
            }
        })
    }
}

pub(super) fn require_services(
    services: &HostServices,
    session: bool,
) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
        || (session && services.task().is_none())
    {
        Err(failure(
            "swallowtail.deepseek.host_services_missing",
            "DeepSeek required host services are unavailable",
        ))
    } else {
        Ok(())
    }
}

pub(super) fn operation_scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("deepseek-direct:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.deepseek.scope_invalid",
            "DeepSeek operation scope was invalid",
        )
    })
}
