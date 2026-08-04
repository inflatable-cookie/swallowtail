const MAXIMUM_RECOVERY_PAGES: usize = 8;
const MAXIMUM_RECOVERY_EVENTS: usize = 2_048;

impl ProviderRunReconciliationDriver for AnthropicManagedAgentDriver {
    fn reconcile_provider_run(
        &self,
        plan: ProviderRunReconciliationPlan,
        request: ProviderRunReconciliationRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderRunReconciliationOutcome, RuntimeFailure>> {
        let transport = self.transport.clone();
        Box::pin(async move {
            Self::validate_plan(plan.preflight())?;
            require_recovery_services(&services)?;
            swallowtail_runtime::validate_provider_run_reconciliation_execution(
                &plan, &request, &services,
            )?;
            let resources =
                crate::managed_recovery::from_checkpoint(plan.agreement().checkpoint())?;
            let scope = operation_scope(request.request_id().as_str())?;
            let mut access =
                ManagedAccessLeases::acquire(plan.preflight(), scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let credential = access.secret()?.to_vec();
            let result = observe_recovered_run(
                &transport,
                &scope,
                &endpoint,
                &credential,
                plan.preflight(),
                &resources,
                &services,
                request.cancellation(),
                plan.agreement().deadline(),
            )
            .await;
            let cleanup = access.release(&services).await;
            let observation = result?;
            ProviderRunReconciliationOutcome::new(&plan, &request, observation, cleanup)
        })
    }
}

impl ProviderRecoveredResourceCleanupDriver for AnthropicManagedAgentDriver {
    fn cleanup_recovered_resources(
        &self,
        plan: ProviderRecoveredResourceCleanupPlan,
        request: ProviderRecoveredResourceCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderRecoveredResourceCleanupOutcome, RuntimeFailure>> {
        let transport = self.transport.clone();
        Box::pin(async move {
            Self::validate_plan(plan.preflight())?;
            require_recovery_services(&services)?;
            swallowtail_runtime::validate_provider_recovered_resource_cleanup_execution(
                &plan, &request, &services,
            )?;
            let resources =
                crate::managed_recovery::from_cleanup_binding(plan.agreement().binding())?;
            let scope = operation_scope(request.request_id().as_str())?;
            let mut access =
                ManagedAccessLeases::acquire(plan.preflight(), scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let credential = access.secret()?.to_vec();
            let result = cleanup_recovered_run(
                &transport,
                &scope,
                &endpoint,
                &credential,
                plan.preflight(),
                &resources,
                &services,
                request.cancellation(),
                plan.agreement().deadline(),
            )
            .await;
            let local_cleanup = access.release(&services).await;
            let (effect, diagnostic) = result?;
            let mut outcome =
                ProviderRecoveredResourceCleanupOutcome::new(&plan, &request, effect)?;
            if let Some(diagnostic) = diagnostic.or_else(|| cleanup_diagnostic(local_cleanup)) {
                outcome = outcome.with_diagnostic(diagnostic);
            }
            Ok(outcome)
        })
    }
}

include!("recovery/observe.rs");
include!("recovery/cleanup.rs");
include!("recovery/support.rs");

#[cfg(test)]
include!("recovery/tests.rs");
