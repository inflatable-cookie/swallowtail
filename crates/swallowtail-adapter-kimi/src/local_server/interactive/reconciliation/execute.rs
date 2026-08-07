use super::super::access::SessionAccess;
use super::super::checkpoint::decode;
use super::failure::{checkpoint_required, cleanup_failure, protocol_failure};
use super::observation::{control, observe};
use crate::failure::failure;
use std::sync::Arc;
use swallowtail_core::{ResourceAccess, SessionAccessPolicy};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, ProviderSessionReconciliationDriver,
    ProviderSessionReconciliationOutcome, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest, RuntimeFailure,
    validate_provider_session_reconciliation_execution,
};

impl ProviderSessionReconciliationDriver for crate::KimiLocalServerDriver {
    fn reconcile_provider_session(
        &self,
        plan: ProviderSessionReconciliationPlan,
        request: ProviderSessionReconciliationRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_reconciliation_execution(&plan, &request, &services)?;
            execute(self, plan, request, services).await
        })
    }
}

async fn execute(
    driver: &crate::KimiLocalServerDriver,
    plan: ProviderSessionReconciliationPlan,
    request: ProviderSessionReconciliationRequest,
    services: HostServices,
) -> Result<ProviderSessionReconciliationOutcome, RuntimeFailure> {
    let agreement = plan.agreement();
    let checkpoint = agreement.checkpoint().ok_or_else(checkpoint_required)?;
    let cursor = decode(checkpoint.cursor())?;
    control(&request, &services)?;
    let scope = swallowtail_runtime::ScopeId::new(format!(
        "kimi-local:reconciliation:{}",
        request.request_id().as_str()
    ))
    .map_err(|_| protocol_failure())?;
    let access_policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
    let working_resource = agreement.binding().working_resource().ok_or_else(|| {
        failure(
            "swallowtail.kimi.local_server.reconciliation.resource_invalid",
            "Kimi local-server reconciliation requires a filesystem working resource",
        )
    })?;
    let mut access = SessionAccess::acquire(
        plan.preflight(),
        scope.clone(),
        &services,
        working_resource,
        &access_policy,
    )
    .await?;
    let result = observe(
        driver,
        &plan,
        &request,
        &services,
        &scope,
        access.endpoint.clone(),
        access.directory.clone(),
        Arc::clone(&access.secret),
        cursor,
    )
    .await;
    let cleanup = access.release(&services).await;
    let observation = match result {
        Ok(observation)
            if matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) =>
        {
            observation
        }
        Ok(_) => return Err(cleanup_failure(cleanup)),
        Err(error) => {
            if matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) {
                return Err(error);
            }
            return Err(cleanup_failure(cleanup));
        }
    };
    ProviderSessionReconciliationOutcome::new(&plan, &request, observation, cleanup)
}
