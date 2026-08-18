mod plan;
mod support;

use futures_executor::block_on;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME, ANTIGRAVITY_RELEASE_AXIS, AntigravityCatalogueDriver,
    antigravity_catalogue_claim,
};
use swallowtail_core::{
    DiscoveryStatus, ExecutionHostId, InstalledExecutableCompatibility, InterfaceVersionAxis,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, ModelCatalogDriver,
    ModelCatalogRequest, MonotonicInstant, ProcessExit, ProcessOutputChunk, ProcessOutputStream,
    RequestId, ScopeId,
};

const VERSION: &str = include_str!("fixtures/antigravity-cli-1.1.9/version.txt");
const CATALOGUE: &str = include_str!("fixtures/antigravity-cli-1.1.9/models.txt");
const ARTIFACT: &str = include_str!("fixtures/antigravity-cli-1.1.9/artifact.json");
const HELP: &str = include_str!("fixtures/antigravity-cli-1.1.9/help.txt");

#[test]
fn exact_artifact_fixture_preserves_version_hash_source_and_surface() {
    assert_eq!(VERSION, "1.1.9\n");
    for evidence in [
        "a27bff8d7c47fe5407e6740f14ecef73e86fb65ec73fec77b0765f8849024383",
        "Developer ID Application: Google LLC (EQHXZ8M8AV)",
        "03e095ac3619462ecd0928f3f5470387dbda6a00",
        "\"source_tags\": [\"1.1.8\", \"1.1.9\"]",
    ] {
        assert!(ARTIFACT.contains(evidence));
    }
    for surface in ["--output-format", "--json-schema", "--sandbox", "models"] {
        assert!(HELP.contains(surface));
    }
}

#[test]
fn exact_and_newer_releases_probe_only_the_host_approved_target() {
    assert_eq!(ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME, "agy");
    assert_ne!(ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME, "gemini");

    for (host_value, release, qualified) in [
        ("fixture.antigravity.local", "1.1.9", true),
        ("fixture.antigravity.latest", "1.1.14", true),
        ("fixture.antigravity.remote", "1.1.15", false),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host id");
        let host = support::FixtureHost::completed([stdout(&format!("{release}\n"))]);
        let outcome = block_on(driver().discover_installed_executable(
            discovery_request(host_id.clone(), "opaque.host-approved.agy"),
            host.services(host_id.clone()),
        ))
        .expect("Antigravity discovery completes");

        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("installed observation exists");
        assert_eq!(observation.execution_host_id(), &host_id);
        assert_eq!(observation.claim_id(), antigravity_catalogue_claim().id());
        assert_eq!(observation.is_qualified(), qualified);
        if !qualified {
            assert!(matches!(
                observation.compatibility(),
                InstalledExecutableCompatibility::UnverifiedNewer(_)
            ));
        }
        assert_eq!(host.observed().executable, "opaque.host-approved.agy");
        assert_eq!(host.observed().arguments, ["--version"]);
        assert!(host.stdin_closed());
        assert!(host.waited());
        assert!(host.joined());
    }
}

#[test]
fn nonzero_version_probe_reports_status_and_sanitized_stderr() {
    let host_id = ExecutionHostId::new("fixture.antigravity.failed").expect("valid host id");
    let host = support::FixtureHost::with_exit(
        [stderr(
            "wrapper at /Users/private/bin/agy token=private user@example.com",
        )],
        ProcessExit::new(false, Some(126)),
    );
    let outcome = block_on(driver().discover_installed_executable(
        discovery_request(host_id.clone(), "approved.wrapper"),
        host.services(host_id),
    ))
    .expect("probe failure remains an outcome");

    assert_eq!(outcome.status(), DiscoveryStatus::Failed);
    let diagnostic = outcome.diagnostic().expect("failure is diagnosed");
    assert_eq!(
        diagnostic.code(),
        "swallowtail.antigravity.discovery_exit_failed"
    );
    assert!(diagnostic.message().contains("status 126"));
    assert!(diagnostic.message().contains("<path>"));
    assert!(diagnostic.message().contains("<redacted>"));
    for secret in ["/Users/private", "token=private", "user@example.com"] {
        assert!(!diagnostic.message().contains(secret));
    }
    assert!(host.waited());
    assert!(host.joined());
}

#[test]
fn authenticated_catalogue_is_bounded_identity_only_and_topology_neutral() {
    for host_value in [
        "fixture.antigravity.catalogue.local",
        "fixture.antigravity.catalogue.remote",
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host id");
        let plan = plan::catalogue_plan(host_id.clone(), "opaque.host-approved.agy", "1.1.9");
        assert!(plan.provider_id().is_none());
        assert!(plan.model_id().is_none());
        assert!(plan.model_route_id().is_none());
        let host = support::FixtureHost::completed([stdout(CATALOGUE)]);
        let models = block_on(driver().list_models(
            plan,
            ModelCatalogRequest::new(
                RequestId::new("antigravity-model-catalogue").expect("valid request id"),
            ),
            host.services(host_id),
        ))
        .expect("Antigravity catalogue loads");

        assert_eq!(models.len(), 11);
        assert_eq!(models[0].id().as_str(), "gemini-3.6-flash-high");
        assert_eq!(models[8].id().as_str(), "claude-sonnet-4-6");
        assert!(models.iter().all(|model| model.provider_id().is_none()));
        assert_eq!(host.observed().executable, "opaque.host-approved.agy");
        assert_eq!(host.observed().arguments, ["models"]);
        assert_eq!(
            host.observed().environments,
            ["antigravity.fixture.environment"]
        );
        assert!(host.stdin_closed());
        assert!(host.waited());
    }
}

#[test]
fn catalogue_failure_is_safe_and_host_mismatch_has_no_effect() {
    let host_id = ExecutionHostId::new("fixture.antigravity.exit").expect("valid host id");
    let plan = plan::catalogue_plan(host_id.clone(), "opaque.host-approved.agy", "1.1.9");
    let host = support::FixtureHost::with_exit(
        [stderr(
            "authentication failed for user@example.com token=private at /Users/private/keyring",
        )],
        ProcessExit::new(false, Some(1)),
    );
    let error = block_on(driver().list_models(
        plan,
        ModelCatalogRequest::new(
            RequestId::new("antigravity-catalogue-exit").expect("valid request id"),
        ),
        host.services(host_id),
    ))
    .expect_err("failed catalogue is diagnosed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.antigravity.catalogue_exit_failed"
    );
    assert!(error.diagnostic().message().contains("status 1"));
    for secret in ["user@example.com", "token=private", "/Users/private"] {
        assert!(!error.diagnostic().message().contains(secret));
    }

    let planned_host = ExecutionHostId::new("fixture.antigravity.planned").expect("valid host id");
    let actual_host = ExecutionHostId::new("fixture.antigravity.other").expect("valid host id");
    let plan = plan::catalogue_plan(planned_host, "opaque.host-approved.agy", "1.1.9");
    let host = support::FixtureHost::completed([stdout(CATALOGUE)]);
    let error = block_on(driver().list_models(
        plan,
        ModelCatalogRequest::new(
            RequestId::new("antigravity-host-mismatch").expect("valid request id"),
        ),
        host.services(actual_host),
    ))
    .expect_err("host mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!host.started());
}

fn driver() -> AntigravityCatalogueDriver {
    AntigravityCatalogueDriver::new(
        EnvironmentRef::new("antigravity.fixture.environment").expect("valid environment"),
    )
}

fn discovery_request(
    host: ExecutionHostId,
    executable: &str,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("antigravity-version-probe").expect("valid request id"),
        ScopeId::new("antigravity-version-probe").expect("valid scope id"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new(executable).expect("valid executable"),
            InterfaceVersionAxis::new(ANTIGRAVITY_RELEASE_AXIS).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}

fn stderr(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stderr, value.as_bytes().to_vec())
}
