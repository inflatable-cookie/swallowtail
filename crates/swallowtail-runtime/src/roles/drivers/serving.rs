macro_rules! serving_driver_items {
    () => {
            fn attach(
                &self,
                plan: PreflightPlan,
                request: AttachServingRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn AttachedServingHandle>, RuntimeFailure>>;
        
            fn start(
                &self,
                plan: PreflightPlan,
                request: StartServingRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>>;
    };
}
