use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, SyntheticProfile, assert_common_contract, poll_immediate,
};
use swallowtail_core::{DriverRole, EndpointAudience, HostServiceKind};
use swallowtail_runtime::{CredentialRef, EndpointRef};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::HostedDirectApi;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    fixture.preflight().expect("hosted API preflight succeeds");
    let required: Vec<_> = fixture
        .driver()
        .required_host_services(DriverRole::StructuredRun)
        .collect();
    assert!(required.contains(&HostServiceKind::Network));
    assert!(required.contains(&HostServiceKind::Credential));
    assert!(!required.contains(&HostServiceKind::Process));

    let recording = RecordingHostServices::default();
    poll_immediate(
        recording
            .services()
            .network()
            .expect("network service is available")
            .authorize(EndpointRef::new("hosted-endpoint").expect("reference is valid")),
    )
    .expect("endpoint authorization succeeds");
    poll_immediate(
        recording
            .services()
            .credential()
            .expect("credential service is available")
            .acquire(
                CredentialRef::new("hosted-credential").expect("reference is valid"),
                EndpointAudience::new("hosted-api").expect("audience is valid"),
            ),
    )
    .expect("credential acquisition succeeds");
    assert_eq!(recording.count(RecordedHostCall::NetworkAuthorize), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialAcquire), 1);
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);

    report.record(ConformanceAssertion::HostedApiNeedsNoProcess);
    report
}
