macro_rules! provider_run_reconciliation_driver_items {
    () => {
        /// Observes one admitted interrupted provider-owned run.
        fn reconcile_provider_run(
            &self,
            plan: ProviderRunReconciliationPlan,
            request: ProviderRunReconciliationRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderRunReconciliationOutcome, RuntimeFailure>>;
    };
}
