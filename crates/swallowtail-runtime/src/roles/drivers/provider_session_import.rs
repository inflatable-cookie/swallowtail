macro_rules! provider_session_import_driver_items {
    () => {
        fn import_provider_session(
            &self,
            plan: ProviderSessionImportPlan,
            request: ProviderSessionImportRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>>;
    };
}
