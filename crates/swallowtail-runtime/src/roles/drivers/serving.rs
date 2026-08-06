macro_rules! serving_driver_items {
    () => {
            /// Attaches to an already-running serving instance.
            fn attach(
                &self,
                plan: PreflightPlan,
                request: AttachServingRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn AttachedServingHandle>, RuntimeFailure>>;
        
            /// Starts a host-owned serving instance.
            fn start(
                &self,
                plan: PreflightPlan,
                request: StartServingRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn OwnedServingHandle>, RuntimeFailure>>;
    };
}
