use crate::{
    ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture, ProfilePreflightFixture,
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SyntheticProfile,
    assert_common_contract, poll_immediate,
};
use std::num::NonZeroUsize;
use swallowtail_core::{
    CredentialMechanism, DriverRole, HarnessIsolation, HostServiceKind, ResourceAccess, SessionRef,
};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, CredentialRef, ExecutableRef, LoadSessionRequest,
    OperationContent, ProcessRequest, RequestId, ResourceRepresentation, ResumeSessionRequest,
    ScopeId, SessionAccessPolicy, SessionReplayItem, SessionReplayKind, SessionResumeBinding,
    WorkingResourceLocator, WorkingResourceRef, WorkingResourceText, WorkingResourceWriteRequest,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::PersistentAcpHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture
        .preflight()
        .expect("persistent ACP preflight succeeds");
    let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite);
    assert_eq!(plan.requirements().session_access_policy(), Some(&policy));
    assert_eq!(
        policy.harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(
        plan.credential_mechanism(),
        &CredentialMechanism::InteractiveOauth
    );

    let required = fixture
        .driver()
        .required_host_services(DriverRole::InteractiveSession)
        .collect::<Vec<_>>();
    for service in [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ] {
        assert!(required.contains(&service));
    }
    assert!(!required.contains(&HostServiceKind::Network));

    let provider_session =
        SessionRef::new("provider/private/persistent-session").expect("provider session is valid");
    let resource =
        WorkingResourceRef::new("persistent-acp-resource").expect("resource reference is valid");
    let binding = SessionResumeBinding::new(
        provider_session.clone(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id()
            .expect("persistent ACP plan has a route")
            .clone(),
        plan.model_id()
            .expect("persistent ACP plan has a model")
            .clone(),
        resource.clone(),
        policy.clone(),
    );
    assert!(binding.matches_attachment(&plan, &resource, &policy));
    assert!(!format!("{binding:?}").contains(provider_session.as_provider_value()));

    let load = LoadSessionRequest::new(
        RequestId::new("persistent-acp-load").expect("request is valid"),
        binding.clone(),
        resource.clone(),
        None,
    )
    .with_access_policy(policy.clone());
    let resume = ResumeSessionRequest::new(
        RequestId::new("persistent-acp-resume").expect("request is valid"),
        binding,
        resource.clone(),
        None,
    )
    .with_access_policy(policy.clone());
    assert_eq!(load.provider_session_ref(), resume.provider_session_ref());
    assert_eq!(load.access_policy(), &policy);
    assert_eq!(resume.access_policy(), &policy);

    let replay = [
        SessionReplayItem::with_content(
            provider_session.clone(),
            0,
            SessionReplayKind::UserMessage,
            OperationContent::new("private historical prompt").expect("content is valid"),
        ),
        SessionReplayItem::with_content(
            provider_session,
            1,
            SessionReplayKind::AgentMessage,
            OperationContent::new("private historical response").expect("content is valid"),
        ),
    ];
    assert_eq!(
        replay
            .iter()
            .map(SessionReplayItem::sequence)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    assert!(!format!("{replay:?}").contains("private historical"));

    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let recording = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let services = recording.services();
        services
            .require_execution_host(topology.execution_host_id())
            .expect("topology authority is retained");
        let scope = ScopeId::new("persistent-acp-scope").expect("scope is valid");
        let credential = poll_immediate(
            services
                .credential()
                .expect("credential service exists")
                .acquire(
                    scope.clone(),
                    CredentialRef::new("persistent-acp-delegated-auth")
                        .expect("credential reference is valid"),
                    plan.endpoint_audience().clone(),
                ),
        )
        .expect("delegated credential resolves");
        assert!(matches!(credential, CredentialLease::Delegated(_)));
        let lease = poll_immediate(
            services
                .working_resource()
                .expect("resource service exists")
                .resolve(
                    scope.clone(),
                    resource.clone(),
                    ResourceAccess::ReadWrite,
                    ResourceRepresentation::Filesystem,
                ),
        )
        .expect("write-capable resource resolves");
        let content = WorkingResourceText::new(
            "bounded replacement".to_owned(),
            NonZeroUsize::new(1024 * 1024).expect("limit is non-zero"),
        )
        .expect("replacement is bounded");
        poll_immediate(
            services
                .working_resource_io()
                .expect("resource I/O service exists")
                .write_text(
                    &lease,
                    WorkingResourceWriteRequest::new(
                        WorkingResourceLocator::new("src/generated.rs").expect("locator is valid"),
                        content,
                    ),
                ),
        )
        .expect("bounded write succeeds");
        let process = poll_immediate(
            services.process().expect("process service exists").start(
                scope,
                ProcessRequest::new(
                    ExecutableRef::new("persistent-acp-agent")
                        .expect("executable reference is valid"),
                )
                .with_working_resource(resource.clone()),
            ),
        )
        .expect("ACP process starts");
        poll_immediate(process.request_stop()).expect("ACP process stops");
        poll_immediate(process.wait()).expect("ACP process joins");
        assert_eq!(
            poll_immediate(
                services
                    .working_resource()
                    .expect("resource service exists")
                    .release(lease),
            ),
            CleanupOutcome::NotApplicable
        );
        assert_eq!(
            poll_immediate(
                services
                    .credential()
                    .expect("credential service exists")
                    .release(credential),
            ),
            CleanupOutcome::Clean
        );
        let calls = recording.calls();
        let process_wait = position(&calls, RecordedHostCall::ProcessWait);
        let resource_release = position(&calls, RecordedHostCall::WorkingResourceRelease);
        let credential_release = position(&calls, RecordedHostCall::CredentialRelease);
        assert!(process_wait < resource_release);
        assert!(resource_release < credential_release);
        assert_eq!(
            recording.count(RecordedHostCall::WorkingResourceWriteText),
            1
        );
    }

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::ProcessLifecycle);
    report.record(ConformanceAssertion::PersistentSessionLifecycle);
    report.record(ConformanceAssertion::ReplayPhase);
    report.record(ConformanceAssertion::WorkingResourceWriteCallback);
    report.record(ConformanceAssertion::AmbientHarnessAuthority);
    report.record(ConformanceAssertion::DelegatedAuthentication);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report
}

fn position(calls: &[RecordedHostCall], expected: RecordedHostCall) -> usize {
    calls
        .iter()
        .position(|call| *call == expected)
        .expect("expected host call was recorded")
}
