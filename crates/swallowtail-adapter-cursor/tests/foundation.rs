mod plan;
mod support;

use futures_executor::block_on;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME, CURSOR_AGENT_RELEASE_AXIS, CursorCatalogueDriver,
    cursor_catalogue_claim,
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

const CATALOGUE: &str = "Available models\n\nauto - Auto (current, default)\ngpt-5.3-codex-high - Codex 5.3 High\nclaude-fable-5-high - Fable 5 1M (NO ZDR)\nglm-5.2-max - GLM 5.2 Max\n\nTip: use --model <id> to switch.\n";

#[test]
fn exact_and_newer_cursor_releases_probe_only_the_host_approved_target() {
    assert_eq!(CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME, "cursor-agent");
    assert_ne!(CURSOR_AGENT_AUTOMATIC_EXECUTABLE_NAME, "agent");

    for (host_value, release, qualified) in [
        ("fixture.cursor.local", "2026.07.01-41b2de7", true),
        ("fixture.cursor.remote", "2026.07.23-e383d2b", true),
        ("fixture.cursor.host", "2026.08.04-aaa8809", true),
        ("fixture.cursor.registry", "2026.08.11-e8db854", true),
        ("fixture.cursor.newer", "2026.08.12-a1b2c3d", false),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host id");
        let host = support::FixtureHost::completed([stdout(&format!("{release}\n"))]);
        let outcome = block_on(driver().discover_installed_executable(
            discovery_request(host_id.clone(), "opaque.host-approved.executable"),
            host.services(host_id.clone()),
        ))
        .expect("Cursor discovery completes");

        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("installed observation exists");
        assert_eq!(observation.execution_host_id(), &host_id);
        assert_eq!(observation.claim_id(), cursor_catalogue_claim().id());
        assert_eq!(observation.is_qualified(), qualified);
        if !qualified {
            assert!(matches!(
                observation.compatibility(),
                InstalledExecutableCompatibility::UnverifiedNewer(_)
            ));
        }
        assert_eq!(
            host.observed().executable,
            "opaque.host-approved.executable"
        );
        assert_eq!(host.observed().arguments, ["--version"]);
        assert!(host.stdin_closed());
        assert!(host.waited());
        assert!(host.joined());
    }
}

#[test]
fn nonzero_version_probe_reports_status_and_sanitized_stderr() {
    let host_id = ExecutionHostId::new("fixture.cursor.failed").expect("valid host id");
    let host = support::FixtureHost::with_exit(
        [stderr(
            "wrapper at /Users/private/bin/cursor-agent token=private user@example.com",
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
        "swallowtail.cursor.discovery_exit_failed"
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
fn auth_aware_catalogue_is_bounded_and_topology_neutral() {
    for (host_value, release) in [
        ("fixture.cursor.catalogue.local", "2026.07.01-41b2de7"),
        ("fixture.cursor.catalogue.remote", "2026.07.23-e383d2b"),
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host id");
        let plan = plan::catalogue_plan(host_id.clone(), "opaque.host-approved.cursor", release);
        assert!(plan.provider_id().is_none());
        assert!(plan.model_id().is_none());
        assert!(plan.model_route_id().is_none());
        let host = support::FixtureHost::completed([stdout(CATALOGUE)]);
        let models = block_on(driver().list_models(
            plan,
            ModelCatalogRequest::new(
                RequestId::new("cursor-model-catalogue").expect("valid request id"),
            ),
            host.services(host_id),
        ))
        .expect("Cursor catalogue loads");

        assert_eq!(models.len(), 4);
        assert_eq!(models[0].id().as_str(), "auto");
        assert_eq!(models[0].metadata().display_name(), Some("Auto"));
        assert!(models[0].metadata().is_default());
        assert_eq!(models[1].id().as_str(), "gpt-5.3-codex-high");
        assert!(models.iter().all(|model| model.provider_id().is_none()));
        assert_eq!(host.observed().executable, "opaque.host-approved.cursor");
        assert_eq!(host.observed().arguments, ["models"]);
        assert_eq!(host.observed().environments, ["cursor.fixture.environment"]);
        assert!(host.stdin_closed());
        assert!(host.waited());
    }
}

#[test]
fn catalogue_rejects_host_mismatch_before_starting_a_process() {
    let planned_host = ExecutionHostId::new("fixture.cursor.planned").expect("valid host id");
    let actual_host = ExecutionHostId::new("fixture.cursor.other").expect("valid host id");
    let plan = plan::catalogue_plan(
        planned_host,
        "opaque.host-approved.cursor",
        "2026.07.01-41b2de7",
    );
    let host = support::FixtureHost::completed([stdout(CATALOGUE)]);
    let error = block_on(driver().list_models(
        plan,
        ModelCatalogRequest::new(RequestId::new("cursor-host-mismatch").expect("valid request id")),
        host.services(actual_host),
    ))
    .expect_err("host mismatch fails");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!host.started());
}

fn driver() -> CursorCatalogueDriver {
    CursorCatalogueDriver::new(
        EnvironmentRef::new("cursor.fixture.environment").expect("valid environment"),
    )
}

fn discovery_request(
    host: ExecutionHostId,
    executable: &str,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("cursor-version-probe").expect("valid request id"),
        ScopeId::new("cursor-version-probe").expect("valid scope id"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new(executable).expect("valid executable"),
            InterfaceVersionAxis::new(CURSOR_AGENT_RELEASE_AXIS).expect("valid axis"),
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
