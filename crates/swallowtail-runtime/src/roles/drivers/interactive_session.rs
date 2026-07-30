macro_rules! interactive_session_driver_items {
    () => {
            fn open_session(
                &self,
                plan: PreflightPlan,
                request: OpenSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
        
            fn resume_session(
                &self,
                plan: PreflightPlan,
                request: ResumeSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>>;
        
            fn load_session(
                &self,
                _plan: PreflightPlan,
                _request: LoadSessionRequest,
                _services: HostServices,
            ) -> BoxFuture<'_, Result<LoadedSession, RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.session_load_unsupported",
                        "Driver does not support provider session load",
                    )))
                })
            }
        
            fn open_direct_continuation_session(
                &self,
                _plan: PreflightPlan,
                _request: OpenDirectContinuationSessionRequest,
                _services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
                Box::pin(async {
                    Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.direct_continuation.unsupported",
                        "Driver does not support locally continued direct sessions",
                    )))
                })
            }
    };
}
