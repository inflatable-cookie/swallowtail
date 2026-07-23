use crate::{RecordedHostCall, RecordingHostServices, poll_immediate};
use swallowtail_core::{
    DiscoveryOutcome, DiscoveryStatus, ExecutionHostId, InstalledExecutableCompatibility,
    InstalledExecutableObservation, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, SafeDiagnostic,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, ExecutableRef, InstalledExecutableDiscoveryRequest,
    InstalledExecutableTarget, MonotonicInstant, ProcessRequest, RequestId, ScopeId,
    validate_installed_executable_discovery_services,
};

pub fn assert_installed_executable_observation_contract() {
    let claim = compatibility_claim();
    let local = ExecutionHostId::new("fixture.host.local").expect("host id is valid");
    let remote = ExecutionHostId::new("fixture.host.remote").expect("host id is valid");

    assert_topology_and_join(local, &claim);
    assert_topology_and_join(remote, &claim);
    assert_classification_and_safe_outcomes(&claim);
}

fn assert_topology_and_join(
    execution_host_id: ExecutionHostId,
    claim: &InterfaceCompatibilityClaim,
) {
    let recording = RecordingHostServices::for_host(
        execution_host_id.clone(),
        crate::RecordingOutcome::Succeed,
    );
    let request = request(execution_host_id.clone());
    validate_installed_executable_discovery_services(&request, recording.services())
        .expect("matching authoritative host and services are accepted");

    let process = poll_immediate(
        recording
            .services()
            .process()
            .expect("process service is present")
            .start(
                request.scope_id().clone(),
                ProcessRequest::new(request.target().executable().clone())
                    .with_arguments(["--version".to_owned()]),
            ),
    )
    .expect("approved fixture process starts");
    poll_immediate(process.close_stdin()).expect("probe stdin closes");
    assert!(
        poll_immediate(process.wait())
            .expect("probe process joins")
            .success()
    );

    let observation = InstalledExecutableObservation::classify(
        execution_host_id.clone(),
        version("1.2.0"),
        claim,
    )
    .expect("fixture claim axis matches");
    let outcome = DiscoveryOutcome::installed_executable(observation);
    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    assert_eq!(
        outcome
            .installed_executable_observation()
            .expect("exact observation is present")
            .execution_host_id(),
        &execution_host_id
    );
    assert_eq!(
        recording.calls(),
        [
            RecordedHostCall::ProcessStart,
            RecordedHostCall::ProcessWait
        ]
    );
}

fn assert_classification_and_safe_outcomes(claim: &InterfaceCompatibilityClaim) {
    let host = ExecutionHostId::new("fixture.host.local").expect("host id is valid");
    let incompatible = InstalledExecutableObservation::classify(host, version("2.0.0"), claim)
        .expect("fixture claim axis matches");
    assert_eq!(
        incompatible.compatibility(),
        &InstalledExecutableCompatibility::Incompatible
    );
    assert_eq!(
        DiscoveryOutcome::installed_executable(incompatible).status(),
        DiscoveryStatus::Incompatible
    );

    let diagnostic = SafeDiagnostic::new(
        "fixture.installed_executable",
        "Installed executable probe did not complete",
    );
    let terminal_states = [
        DiscoveryStatus::Absent,
        DiscoveryStatus::Malformed,
        DiscoveryStatus::TimedOut,
        DiscoveryStatus::Cancelled,
        DiscoveryStatus::Failed,
        DiscoveryStatus::CleanupFailed,
    ];
    for status in terminal_states {
        let outcome = DiscoveryOutcome::new(status, Some(diagnostic.clone()));
        assert_eq!(outcome.status(), status);
        assert!(outcome.installed_executable_observation().is_none());
        let debug = format!("{outcome:?}");
        assert!(!debug.contains("/private/"));
        assert!(!debug.contains("raw-version-output"));
    }
}

fn request(execution_host_id: ExecutionHostId) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("fixture.discovery.request").expect("request id is valid"),
        ScopeId::new("fixture.discovery.scope").expect("scope id is valid"),
        execution_host_id,
        InstalledExecutableTarget::new(
            ExecutableRef::new("fixture.executable").expect("reference is valid"),
            axis(),
        ),
        Deadline::at(MonotonicInstant::from_ticks(20)),
        DiscoveryCancellation::new(),
    )
}

fn compatibility_claim() -> InterfaceCompatibilityClaim {
    InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("fixture.installed.claim.v1")
            .expect("claim id is valid"),
        axis(),
        InterfaceVersionScheme::Semantic,
        [InterfaceVersionSegment::new(
            InterfaceVersion::new("1.0.0").expect("version is valid"),
            InterfaceVersion::new("1.5.0").expect("version is valid"),
            InterfaceBehaviorRevision::new("fixture.installed.behavior.v1")
                .expect("behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("claim is valid")
}

fn version(value: &str) -> InterfaceVersionBinding {
    InterfaceVersionBinding::new(
        axis(),
        InterfaceVersion::new(value).expect("version is valid"),
    )
}

fn axis() -> InterfaceVersionAxis {
    InterfaceVersionAxis::new("fixture.harness.package").expect("axis is valid")
}
