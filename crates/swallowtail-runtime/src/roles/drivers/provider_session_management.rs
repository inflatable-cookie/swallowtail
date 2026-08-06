macro_rules! provider_session_management_driver_items {
    () => {
            /// Archives the exact inactive provider session admitted by the plan.
            fn archive_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: ArchiveProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
        
            /// Restores the exact archived provider session admitted by the plan.
            fn restore_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: RestoreProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
        
            /// Deletes provider-session data at the strength admitted by the plan.
            fn delete_session(
                &self,
                plan: ProviderSessionManagementPlan,
                request: DeleteProviderSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>>;
    };
}
