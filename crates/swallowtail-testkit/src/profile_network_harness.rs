use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, SyntheticProfile, assert_common_contract, poll_immediate,
};
use swallowtail_core::{DriverRole, EndpointAudience, HostServiceKind};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, CredentialRef, EndpointRef, ResourceAccess,
    ResourceRepresentation, ScopeId, WorkingResourceRef,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::AttachedNetworkHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    fixture
        .preflight()
        .expect("attached network harness preflight succeeds");
    let required: Vec<_> = fixture
        .driver()
        .required_host_services(DriverRole::InteractiveSession)
        .collect();
    for service in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
    ] {
        assert!(required.contains(&service));
    }
    assert!(!required.contains(&HostServiceKind::Process));

    let recording = RecordingHostServices::default();
    let services = recording.services();
    let scope = ScopeId::new("network-harness-session").expect("scope is valid");
    let audience = EndpointAudience::new("network-harness").expect("audience is valid");
    poll_immediate(
        services
            .network()
            .expect("network service is available")
            .authorize(
                scope.clone(),
                EndpointRef::new("network-harness-endpoint").expect("endpoint is valid"),
                audience.clone(),
            ),
    )
    .expect("endpoint authorization succeeds");
    let credential = poll_immediate(
        services
            .credential()
            .expect("credential service is available")
            .acquire(
                scope.clone(),
                CredentialRef::new("network-harness-credential").expect("credential is valid"),
                audience,
            ),
    )
    .expect("delegated credential succeeds");
    assert!(matches!(credential, CredentialLease::Delegated(_)));
    let resource = poll_immediate(
        services
            .working_resource()
            .expect("working-resource service is available")
            .resolve(
                scope.clone(),
                WorkingResourceRef::new("network-harness-resource").expect("resource is valid"),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            ),
    )
    .expect("resource resolution succeeds");
    poll_immediate(
        services
            .blocking_work()
            .expect("blocking-work service is available")
            .run(scope.clone(), Box::new(|| Ok(()))),
    )
    .expect("blocking transport work succeeds");
    let task = services
        .task()
        .expect("task service is available")
        .spawn(scope, Box::pin(async {}))
        .expect("stream task starts");
    poll_immediate(task.join()).expect("stream task joins");
    assert_eq!(
        poll_immediate(
            services
                .working_resource()
                .expect("working-resource service is available")
                .release(resource),
        ),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(
        poll_immediate(
            services
                .credential()
                .expect("credential service is available")
                .release(credential),
        ),
        CleanupOutcome::Clean
    );
    for call in [
        RecordedHostCall::NetworkAuthorize,
        RecordedHostCall::CredentialAcquire,
        RecordedHostCall::WorkingResourceResolve,
        RecordedHostCall::BlockingWork,
        RecordedHostCall::TaskSpawn,
        RecordedHostCall::TaskJoin,
        RecordedHostCall::WorkingResourceRelease,
        RecordedHostCall::CredentialRelease,
    ] {
        assert_eq!(recording.count(call), 1);
    }
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
    report.record(ConformanceAssertion::AttachedNetworkHarnessLifecycle);
    report
}
