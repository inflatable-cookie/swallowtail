mod host;
mod server;

pub use host::FixtureHost;
pub use server::FixtureServer;

pub fn close_session(
    session: Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    services: swallowtail_runtime::HostServices,
) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::CleanupOutcome> {
    session.close(
        swallowtail_runtime::SessionCleanupRequest::new(swallowtail_runtime::Deadline::at(
            swallowtail_runtime::MonotonicInstant::from_ticks(10_000),
        )),
        services,
    )
}
