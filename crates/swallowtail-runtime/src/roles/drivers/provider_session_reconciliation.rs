macro_rules! provider_session_reconciliation_driver_items {
    () => {
        fn reconcile_provider_session(
            &self,
            plan: ProviderSessionReconciliationPlan,
            request: ProviderSessionReconciliationRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>>;
    };
}
