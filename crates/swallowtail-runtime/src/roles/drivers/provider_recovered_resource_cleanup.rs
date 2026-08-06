macro_rules! provider_recovered_resource_cleanup_driver_items {
    () => {
        /// Cleans up the exact inactive resources retained by one recovered run.
        fn cleanup_recovered_resources(
            &self,
            plan: ProviderRecoveredResourceCleanupPlan,
            request: ProviderRecoveredResourceCleanupRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderRecoveredResourceCleanupOutcome, RuntimeFailure>>;
    };
}
