macro_rules! structured_run_driver_items {
    () => {
            fn start_run(
                &self,
                plan: PreflightPlan,
                request: StructuredRunRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>>;
    };
}
