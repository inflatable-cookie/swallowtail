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

#[test]
fn malformed_duplicate_and_non_loopback_startup_fail_before_publication() {
    for (label, startup, expected) in [
        (
            "malformed",
            STARTUP_MALFORMED,
            "swallowtail.llama_cpp.serving_endpoint_invalid",
        ),
        (
            "duplicate",
            STARTUP_DUPLICATE,
            "swallowtail.llama_cpp.serving_endpoint_duplicate",
        ),
        (
            "non-loopback",
            STARTUP_NON_LOOPBACK,
            "swallowtail.llama_cpp.serving_endpoint_invalid",
        ),
    ] {
        let fixture = OwnedFixture::new(
            FixtureServer::start(),
            ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
        );
        let error = start(&fixture).err().expect(label);
        assert_eq!(error.diagnostic().code(), expected);
        assert!(!format!("{error:?}").contains("49152"));
        assert_eq!(
            fixture
                .owned
                .calls()
                .iter()
                .filter(|call| **call == OwnedCall::EndpointPublish)
                .count(),
            0
        );
        assert_order(
            &fixture.owned.calls(),
            &[
                OwnedCall::ProcessStart,
                OwnedCall::GracefulStop,
                OwnedCall::ProcessWait,
                OwnedCall::ArtifactRelease,
            ],
        );
    }
}

#[test]
fn duplicate_reported_during_http_readiness_still_prevents_a_handle() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let first = format!("srv listening on {}\n", server.endpoint()).into_bytes();
    let second = format!("srv listening on {}\n", server.endpoint()).into_bytes();
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::with_chunks([first, second], ProcessStop::Graceful),
    );
    let error = start(&fixture).err().expect("late duplicate fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_endpoint_duplicate"
    );
    assert_order(
        &fixture.owned.calls(),
        &[
            OwnedCall::EndpointPublish,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}

#[test]
fn early_exit_and_build_mismatch_take_the_same_joined_cleanup_path() {
    let exited = OwnedFixture::new(FixtureServer::start(), ScriptedOwnedServices::exited());
    let error = start(&exited).err().expect("early exit fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_process_exited"
    );
    assert_order(
        &exited.owned.calls(),
        &[
            OwnedCall::ProcessStart,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::ArtifactRelease,
        ],
    );

    let server = FixtureServer::start_with(PropertiesFixture::Expected, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let mismatch = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let error = start(&mismatch).err().expect("wrong build fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_build_mismatch"
    );
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
