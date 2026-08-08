macro_rules! provider_session_history_driver_items {
    () => {
        /// Reads one newest-first page of provider-owned session history.
        ///
        /// Default implementations fail closed as unsupported. Routes that
        /// advertise `DriverRole::ProviderSessionHistory` must override this
        /// method. The role returns no live session handle.
        fn page_provider_session_history(
            &self,
            _plan: ProviderSessionHistoryPlan,
            _request: ProviderSessionHistoryRequest,
            _services: HostServices,
        ) -> BoxFuture<'_, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
            Box::pin(async {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.provider_session_history.unsupported",
                    "Driver does not support provider-session history pages",
                )))
            })
        }
    };
}
