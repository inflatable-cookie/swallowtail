macro_rules! provider_session_catalogue_driver_items {
    () => {
        /// Lists provider-owned sessions without importing or mutating them.
        fn list_provider_sessions(
            &self,
            plan: ProviderSessionCataloguePlan,
            request: ProviderSessionCatalogueRequest,
            services: HostServices,
        ) -> BoxFuture<
            '_,
            Result<ProviderSessionCatalogueOutcome, ProviderSessionOperationFailure>,
        >;
    };
}
