macro_rules! realtime_driver_items {
    () => {
            /// Opens one realtime media session and returns its duplex handle.
            fn open_realtime_media_session(
                &self,
                plan: PreflightPlan,
                request: OpenRealtimeMediaSessionRequest,
                services: HostServices,
            ) -> BoxFuture<'_, Result<Box<dyn RealtimeMediaSessionHandle>, RuntimeFailure>>;
    };
}
