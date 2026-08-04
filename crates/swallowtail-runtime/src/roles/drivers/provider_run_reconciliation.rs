macro_rules! provider_run_reconciliation_driver_items {
    () => {
        fn reconcile_provider_run(
            &self,
            plan: ProviderRunReconciliationPlan,
            request: ProviderRunReconciliationRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderRunReconciliationOutcome, RuntimeFailure>>;
    };
}
