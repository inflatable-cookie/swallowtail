macro_rules! structured_run_driver_items {
    () => {
            /// Starts one structured run and returns its operation handle.
            fn start_run(
                &self,
                plan: PreflightPlan,
                request: StructuredRunRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>>;
    };
}
