macro_rules! provider_session_management_driver_items {
    () => {
            fn archive_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: ArchiveProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
        
            fn restore_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: RestoreProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
        
            fn delete_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: DeleteProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
    };
}
