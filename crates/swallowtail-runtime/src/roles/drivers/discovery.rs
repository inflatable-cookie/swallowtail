macro_rules! discovery_driver_items {
    () => {
            fn discover(
                &self,
                request: DiscoveryRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Vec<DiscoveryOutcome>, RuntimeFailure>>;
        
            fn discover_installed_executable(
                &self,
                _request: InstalledExecutableDiscoveryRequest,
                _services: HostServices,
            ) -> BoxFuture<'_, Result<DiscoveryOutcome, RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.installed_executable.discovery_unsupported",
                        "Driver does not support installed executable discovery",
                    )))
                })
            }
    };
}
