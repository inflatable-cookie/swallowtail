use super::*;

#[test]
fn readiness_timeout_and_route_mismatch_join_before_releasing_leases() {
    let timed_out = OwnedFixture::new(
        FixtureServer::start(),
        ScriptedOwnedServices::readiness_timeout(),
    );
    let error = start(&timed_out).err().expect("readiness timeout fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_readiness_timed_out"
    );
    assert_order(
        &timed_out.owned.calls(),
        &[
            OwnedCall::ProcessStart,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::ArtifactRelease,
        ],
    );

    let server =
        FixtureServer::start_with(PropertiesFixture::RouteMismatch, StreamFixture::Success);
    let endpoint = server.endpoint().to_owned();
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", &endpoint);
    let mismatch = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let error = start(&mismatch).err().expect("route mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_route_mismatch"
    );
    let diagnostic = format!("{error:?}");
    assert!(!diagnostic.contains(&endpoint));
    assert!(!diagnostic.contains("/private/models/fixture.gguf"));
    assert_order(
        &mismatch.owned.calls(),
        &[
            OwnedCall::EndpointPublish,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}
