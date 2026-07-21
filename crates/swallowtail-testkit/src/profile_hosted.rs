use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, SyntheticProfile, assert_common_contract, poll_immediate,
};
use std::num::NonZeroU64;
use swallowtail_core::{DriverRole, EndpointAudience, HostServiceKind};
use swallowtail_runtime::{
    CredentialRef, EndpointRef, OperationContent, OperationPolicy, ProviderObservation,
    QuotaObservation, QuotaState, RateLimitKind, RateLimitObservation, RequestId, ScopeId,
    StructuredRunRequest, TokenUsage,
};

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
    let scope = ScopeId::new("hosted-operation").expect("scope is valid");
    let audience = EndpointAudience::new("hosted-api").expect("audience is valid");
    let grant = poll_immediate(
        recording
            .services()
            .network()
            .expect("network service is available")
            .authorize(
                scope.clone(),
                EndpointRef::new("hosted-endpoint").expect("reference is valid"),
                audience.clone(),
            ),
    )
    .expect("endpoint authorization succeeds");
    assert_eq!(grant.scope(), &scope);
    assert_eq!(grant.audience(), &audience);
    assert!(!format!("{grant:?}").contains("recording.invalid"));
    let credential = poll_immediate(
        recording
            .services()
            .credential()
            .expect("credential service is available")
            .acquire(
                scope.clone(),
                CredentialRef::new("hosted-credential").expect("reference is valid"),
                audience.clone(),
            ),
    )
    .expect("credential acquisition succeeds");
    assert_eq!(credential.scope(), &scope);
    assert_eq!(credential.audience(), &audience);
    assert_eq!(
        poll_immediate(
            recording
                .services()
                .credential()
                .expect("credential service is available")
                .release(credential),
        ),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    assert_eq!(recording.count(RecordedHostCall::NetworkAuthorize), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialAcquire), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialRelease), 1);
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);

    let direct = StructuredRunRequest::new(
        RequestId::new("hosted-direct").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"));
    assert!(direct.working_resource().is_none());
    assert_eq!(
        direct.maximum_output_tokens().map(NonZeroU64::get),
        Some(64)
    );

    let observations = [
        ProviderObservation::Usage(TokenUsage::new(Some(12), Some(4))),
        ProviderObservation::RateLimit(RateLimitObservation::new(
            RateLimitKind::Requests,
            Some(100),
            Some(99),
            None,
        )),
        ProviderObservation::Quota(QuotaObservation::new(QuotaState::Available)),
    ];
    assert!(matches!(observations[0], ProviderObservation::Usage(_)));
    assert!(matches!(observations[1], ProviderObservation::RateLimit(_)));
    assert!(matches!(observations[2], ProviderObservation::Quota(_)));

    report.record(ConformanceAssertion::HostedApiNeedsNoProcess);
    report.record(ConformanceAssertion::HostedEndpointCredentialBinding);
    report.record(ConformanceAssertion::DirectRunNoResource);
    report.record(ConformanceAssertion::DirectRunOutputBound);
    report.record(ConformanceAssertion::ProviderEvidenceSeparated);
    report
}
