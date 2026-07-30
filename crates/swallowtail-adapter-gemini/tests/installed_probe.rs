use crate::discovery_support as support;

use futures_executor::block_on;
use support::DiscoveryHost;
use swallowtail_adapter_gemini::{
    GEMINI_CLI_ACP_AXIS, GEMINI_CLI_HEADLESS_AXIS, GeminiAcpDriver, GeminiHeadlessDriver,
    gemini_cli_acp_claim, gemini_cli_headless_claim,
};
use swallowtail_core::{
    DiscoveryStatus, ExecutionHostId, InstalledExecutableCompatibility, InterfaceVersionAxis,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef, ExecutableRef,
    InstalledExecutableDiscoveryRequest, InstalledExecutableTarget, MonotonicInstant, RequestId,
    ScopeId,
};

#[test]
fn qualified_and_unverified_newer_versions_probe_the_approved_target() {
    for (version, qualified) in [("0.51.0", true), ("0.52.0", false)] {
        let host_id = ExecutionHostId::new("fixture.host.discovery").expect("valid host");
        let host = DiscoveryHost::new(version);
        let outcome = block_on(driver().discover_installed_executable(
            request(host_id.clone()),
            host.services(host_id.clone()),
        ))
        .expect("discovery completes");
        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("observation exists");
        assert_eq!(observation.execution_host_id(), &host_id);
        assert_eq!(observation.version().version().as_str(), version);
        assert_eq!(observation.claim_id(), gemini_cli_acp_claim().id());
        assert_eq!(observation.is_qualified(), qualified);
        if !qualified {
            assert!(matches!(
                observation.compatibility(),
                InstalledExecutableCompatibility::UnverifiedNewer(_)
            ));
        }
        let process = host.observed_process();
        assert_eq!(process.executable, "gemini.fixture.executable");
        assert_eq!(process.arguments, ["--version"]);
    }
}

#[test]
fn older_version_is_incompatible() {
    let host_id = ExecutionHostId::new("fixture.host.incompatible").expect("valid host");
    let host = DiscoveryHost::new("0.50.0");
    let outcome = block_on(
        driver().discover_installed_executable(request(host_id.clone()), host.services(host_id)),
    )
    .expect("discovery completes");
    assert_eq!(outcome.status(), DiscoveryStatus::Incompatible);
    assert!(
        outcome
            .installed_executable_observation()
            .is_some_and(|observation| !observation.is_permitted())
    );
}

#[test]
fn headless_probe_qualifies_frozen_range_and_keeps_newer_visible() {
    for (version, qualified) in [("0.51.0", true), ("0.52.0", true), ("0.53.0", false)] {
        let host_id = ExecutionHostId::new("fixture.host.headless").expect("valid host");
        let host = DiscoveryHost::new(version);
        let outcome = block_on(headless_driver().discover_installed_executable(
            headless_request(host_id.clone()),
            host.services(host_id.clone()),
        ))
        .expect("headless discovery completes");
        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("observation exists");
        assert_eq!(observation.version().version().as_str(), version);
        assert_eq!(observation.claim_id(), gemini_cli_headless_claim().id());
        assert_eq!(observation.is_qualified(), qualified);
        if !qualified {
            assert!(matches!(
                observation.compatibility(),
                InstalledExecutableCompatibility::UnverifiedNewer(_)
            ));
        }
        assert_eq!(host.observed_process().arguments, ["--version"]);
    }
}

fn request(host: ExecutionHostId) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("gemini-version-probe").expect("valid request"),
        ScopeId::new("gemini-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.fixture.executable").expect("valid executable"),
            InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn headless_request(host: ExecutionHostId) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("gemini-headless-version-probe").expect("valid request"),
        ScopeId::new("gemini-headless-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("gemini.fixture.executable").expect("valid executable"),
            InterfaceVersionAxis::new(GEMINI_CLI_HEADLESS_AXIS).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn driver() -> GeminiAcpDriver {
    GeminiAcpDriver::new(
        EnvironmentRef::new("gemini.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("gemini.fixture.api-key").expect("valid credential"),
    )
}

fn headless_driver() -> GeminiHeadlessDriver {
    GeminiHeadlessDriver::new(
        EnvironmentRef::new("gemini.fixture.environment").expect("valid environment"),
        swallowtail_core::CredentialRef::new("gemini.fixture.api-key").expect("valid credential"),
    )
}
