macro_rules! provider_recovered_resource_cleanup_driver_items {
    () => {
        fn cleanup_recovered_resources(
            &self,
            plan: ProviderRecoveredResourceCleanupPlan,
            request: ProviderRecoveredResourceCleanupRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderRecoveredResourceCleanupOutcome, RuntimeFailure>>;
    };
}
