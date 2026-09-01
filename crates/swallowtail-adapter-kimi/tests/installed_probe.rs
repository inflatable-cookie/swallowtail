use crate::discovery_support as support;

use futures_executor::block_on;
use support::{FakeProcessService, services};
use swallowtail_adapter_kimi::{KIMI_CODE_AXIS, KimiAcpDriver, kimi_acp_claim};
use swallowtail_core::{
    DiscoveryStatus, ExecutionHostId, InstalledExecutableCompatibility, InterfaceVersionAxis,
};
use swallowtail_runtime::{
    CancellationControl, Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef,
    ExecutableRef, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    MonotonicInstant, RequestId, ScopeId,
};

#[test]
fn exact_versions_probe_only_the_host_approved_target() {
    for topology in [
        swallowtail_testkit::ExecutionTopologyFixture::local(),
        swallowtail_testkit::ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for (version, qualified) in [
            ("0.28.1", true),
            ("0.29.0", true),
            ("0.29.1", true),
            ("0.29.2", true),
            ("0.30.0", true),
            ("0.31.0", true),
            ("0.31.1", true),
            ("0.32.0", true),
            ("0.36.1", true),
            ("0.37.2", true),
            ("0.38.0", true),
            ("0.39.2", false),
        ] {
            let host = topology.execution_host_id().clone();
            let executable = ExecutableRef::from_instance_target(topology.instance_target());
            let (process, state) = FakeProcessService::completed(&format!("{version}\n"));
            let outcome = block_on(driver().discover_installed_executable(
                request(
                    host.clone(),
                    executable.clone(),
                    DiscoveryCancellation::new(),
                ),
                services(host.clone(), process),
            ))
            .expect("discovery completes");
            assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
            let observation = outcome
                .installed_executable_observation()
                .expect("exact observation is present");
            assert_eq!(observation.execution_host_id(), &host);
            assert_eq!(observation.version().version().as_str(), version);
            assert_eq!(observation.claim_id(), kimi_acp_claim().id());
            assert_eq!(observation.is_qualified(), qualified);
            if !qualified {
                let InstalledExecutableCompatibility::UnverifiedNewer(unverified) =
                    observation.compatibility()
                else {
                    panic!("newer stable version remains unverified");
                };
                assert_eq!(unverified.latest_qualified().as_str(), "0.38.0");
            }
            let captured = state.request();
            assert_eq!(captured.executable, executable.as_host_value());
            assert_eq!(captured.arguments, ["--version"]);
            assert!(captured.environments.is_empty());
            assert!(captured.working_resource.is_none());
            assert!(state.stdin_closed());
            assert!(state.waited());
        }
    }
}

#[test]
fn incompatible_malformed_and_precancelled_observations_stay_distinct_and_safe() {
    let host = ExecutionHostId::new("fixture.host.classification").expect("valid host");
    let (process, _) = FakeProcessService::completed("0.28.2\n");
    let incompatible = block_on(driver().discover_installed_executable(
        request(
            host.clone(),
            fixture_executable(),
            DiscoveryCancellation::new(),
        ),
        services(host.clone(), process),
    ))
    .expect("incompatible discovery completes");
    assert_eq!(incompatible.status(), DiscoveryStatus::Incompatible);
    assert!(incompatible.installed_executable_observation().is_some());

    let private = "private malformed output";
    let (process, _) = FakeProcessService::completed(private);
    let malformed = block_on(driver().discover_installed_executable(
        request(
            host.clone(),
            fixture_executable(),
            DiscoveryCancellation::new(),
        ),
        services(host.clone(), process),
    ))
    .expect("malformed discovery completes");
    assert_eq!(malformed.status(), DiscoveryStatus::Malformed);
    assert!(malformed.installed_executable_observation().is_none());
    assert!(!format!("{malformed:?}").contains(private));

    let cancellation = DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("cancellation is accepted");
    let (process, state) = FakeProcessService::completed("0.29.0\n");
    let cancelled = block_on(driver().discover_installed_executable(
        request(host.clone(), fixture_executable(), cancellation),
        services(host, process),
    ))
    .expect("cancelled discovery completes");
    assert_eq!(cancelled.status(), DiscoveryStatus::Cancelled);
    assert!(!state.started());
}

fn request(
    host: ExecutionHostId,
    executable: ExecutableRef,
    cancellation: DiscoveryCancellation,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("kimi-version-probe").expect("valid request"),
        ScopeId::new("kimi-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation,
    )
}

fn fixture_executable() -> ExecutableRef {
    ExecutableRef::new("kimi.fixture.approved").expect("valid executable")
}

fn driver() -> KimiAcpDriver {
    KimiAcpDriver::new(
        EnvironmentRef::new("kimi.fixture.isolated-state").expect("valid environment"),
        swallowtail_core::CredentialRef::new("kimi.fixture.credential").expect("valid credential"),
    )
}
