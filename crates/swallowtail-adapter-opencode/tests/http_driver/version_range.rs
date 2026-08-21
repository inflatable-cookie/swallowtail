const RANGE_CORPUS: &str =
    include_str!("../fixtures/opencode-v1.14.48-v1.18.10/compatibility.json");

#[test]
fn every_frozen_behavior_surface_executes_from_an_exact_plan() {
    let corpus: serde_json::Value = serde_json::from_str(RANGE_CORPUS).expect("corpus parses");
    let releases = corpus["releases"].as_array().expect("release array");
    let mut surfaces = std::collections::BTreeMap::new();
    for release in releases {
        surfaces
            .entry(release["surface"].as_str().expect("surface"))
            .or_insert(release["version"].as_str().expect("version"));
    }
    assert_eq!(surfaces.len(), 19);

    for (surface, version) in surfaces {
        let server = FixtureServer::start_with_version(StreamFixture::Success, version);
        let fixture = Fixture::new_with_version(server.endpoint(), "host.range", version);
        let models = block_on(OpenCodeHttpDriver::new().list_models(
            fixture.plan(DriverRole::ModelCatalog),
            ModelCatalogRequest::new(
                RequestId::new(format!("catalogue-{surface}")).expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .unwrap_or_else(|error| panic!("{version} ({surface}) failed: {error:?}"));
        assert_eq!(models.len(), 1);
        assert_eq!(
            server
                .requests()
                .iter()
                .filter(|request| request.contains("/global/health"))
                .count(),
            1
        );
    }
}

#[test]
fn range_boundaries_match_health_under_both_host_authorities() {
    for (host, version) in [
        ("host.local", "1.14.48"),
        ("host.local", "1.18.20"),
        ("host.remote-authoritative", "1.14.48"),
        ("host.remote-authoritative", "1.18.20"),
    ] {
        let server = FixtureServer::start_with_version(StreamFixture::Success, version);
        let fixture = Fixture::new_with_version(server.endpoint(), host, version);
        let models = block_on(OpenCodeHttpDriver::new().list_models(
            fixture.plan(DriverRole::ModelCatalog),
            ModelCatalogRequest::new(
                RequestId::new(format!("catalogue-{host}-{version}"))
                    .expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .unwrap_or_else(|error| panic!("{host} {version} failed: {error:?}"));
        assert_eq!(models.len(), 1);
        assert_eq!(server.requests().len(), 2);
    }
}

#[test]
fn health_drift_fails_before_catalogue_or_session_work() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.4");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.drift", "1.18.3");
    let error = block_on(OpenCodeHttpDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalogue-drift").expect("request id is valid")),
        fixture.services(),
    ))
    .expect_err("health drift rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.version_mismatch"
    );
    let requests = server.requests();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].contains("/global/health"));
}

#[test]
fn latest_qualified_session_uses_the_same_exact_version() {
    let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.20");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.latest", "1.18.20");
    let session = block_on(OpenCodeHttpDriver::new().open_session(
        fixture.plan(DriverRole::InteractiveSession),
        open_session_request("latest-session", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("latest session opens");
    assert!(
        server
            .requests()
            .iter()
            .any(|request| request.contains("/session?directory="))
    );
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
fn unverified_newer_catalogue_and_session_use_the_latest_qualified_surface() {
    for host in ["host.local", "host.remote-authoritative"] {
        let server = FixtureServer::start_with_version(StreamFixture::Success, "1.18.21");
        let fixture = Fixture::new_with_version(server.endpoint(), host, "1.18.21");
        let models = block_on(OpenCodeHttpDriver::new().list_models(
            fixture.plan(DriverRole::ModelCatalog),
            ModelCatalogRequest::new(
                RequestId::new(format!("newer-catalogue-{host}"))
                    .expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .expect("unverified newer catalogue is attempted");
        assert_eq!(models.len(), 1);

        let session = block_on(OpenCodeHttpDriver::new().open_session(
            fixture.plan(DriverRole::InteractiveSession),
            open_session_request(format!("newer-session-{host}"), fixture.resource.clone()),
            fixture.services(),
        ))
        .expect("unverified newer session is attempted");
        assert!(matches!(
            block_on(session.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
}

#[test]
fn missing_and_ambiguous_plan_versions_fail_before_network_work() {
    let server = FixtureServer::start(StreamFixture::Success);
    let fixture = Fixture::new(server.endpoint(), "host.closed-plan");
    let driver = OpenCodeHttpDriver::new();
    let cases = [
        (
            fixture.plan_with_versions(DriverRole::ModelCatalog, &[]),
            "swallowtail.opencode.version_missing",
        ),
        (
            fixture.plan_with_versions(DriverRole::ModelCatalog, &["1.14.48", "1.18.10"]),
            "swallowtail.opencode.version_ambiguous",
        ),
    ];
    for (plan, expected) in cases {
        let error = block_on(driver.list_models(
            plan,
            ModelCatalogRequest::new(
                RequestId::new(format!("catalogue-{expected}")).expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .expect_err("invalid version selection rejects");
        assert_eq!(error.diagnostic().code(), expected);
    }
    assert!(server.requests().is_empty());
}
