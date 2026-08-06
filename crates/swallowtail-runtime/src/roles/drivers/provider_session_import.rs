macro_rules! provider_session_import_driver_items {
    () => {
        /// Revalidates and imports one explicitly selected provider session.
        fn import_provider_session(
            &self,
            plan: ProviderSessionImportPlan,
            request: ProviderSessionImportRequest,
            services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderSessionImportOutcome, ProviderSessionOperationFailure>>;
    };
}
