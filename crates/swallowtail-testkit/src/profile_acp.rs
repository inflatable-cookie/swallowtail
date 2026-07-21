use crate::{
    ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture, ProfilePreflightFixture,
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SyntheticProfile,
    assert_common_contract, poll_immediate,
};
use std::num::NonZeroUsize;
use swallowtail_core::{CancellationScope, DriverRole, HostServiceKind};
use swallowtail_runtime::{
    CancellationControl, ExecutableRef, ImmediateCancellation, ProcessRequest, ResourceAccess,
    ResourceRepresentation, ScopeId, WorkingResourceLocator, WorkingResourceReadRequest,
    WorkingResourceRef,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::LongLivedAcpHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture.preflight().expect("ACP preflight succeeds");
    let required = fixture
        .driver()
        .required_host_services(DriverRole::InteractiveSession)
        .collect::<Vec<_>>();
    assert!(required.contains(&HostServiceKind::WorkingResourceIo));
    assert!(!required.contains(&HostServiceKind::Network));
    assert!(!required.contains(&HostServiceKind::Credential));

    let recording = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let scope = ScopeId::new("acp-session-scope").expect("scope is valid");
    let resource = WorkingResourceRef::new("acp-resource").expect("resource is valid");
    let lease = poll_immediate(
        recording
            .services()
            .working_resource()
            .expect("resource service exists")
            .resolve(
                scope.clone(),
                resource.clone(),
                ResourceAccess::Read,
                ResourceRepresentation::Filesystem,
            ),
    )
    .expect("filesystem resource resolves");
    let request = WorkingResourceReadRequest::new(
        WorkingResourceLocator::new("src/lib.rs").expect("locator is valid"),
        NonZeroUsize::new(1024).expect("limit is non-zero"),
    )
    .with_lines(Some(1), Some(32));
    let content = poll_immediate(
        recording
            .services()
            .working_resource_io()
            .expect("resource I/O service exists")
            .read_text(&lease, request),
    )
    .expect("bounded read succeeds");
    assert!(!format!("{content:?}").contains(content.as_driver_value()));
    assert_eq!(
        recording.count(RecordedHostCall::WorkingResourceReadText),
        1
    );

    let process = poll_immediate(
        recording
            .services()
            .process()
            .expect("process service exists")
            .start(
                scope,
                ProcessRequest::new(
                    ExecutableRef::new("acp-agent").expect("executable reference is valid"),
                )
                .with_working_resource(resource),
            ),
    )
    .expect("owned ACP process starts");
    poll_immediate(process.request_stop()).expect("owned process stops");
    poll_immediate(process.wait()).expect("owned process exit is observed");
    let cleanup = poll_immediate(
        recording
            .services()
            .working_resource()
            .expect("resource service exists")
            .release(lease),
    );
    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::NotApplicable);

    let cancellation = ImmediateCancellation::new(CancellationScope::ActiveTurn);
    poll_immediate(cancellation.request()).expect("active prompt cancellation succeeds");
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 1);
    assert_eq!(recording.count(RecordedHostCall::ProcessGracefulStop), 1);
    assert_eq!(recording.count(RecordedHostCall::ProcessWait), 1);

    let local = ExecutionTopologyFixture::local();
    let remote = ExecutionTopologyFixture::remote_authoritative();
    assert_ne!(local.execution_host_id(), remote.execution_host_id());
    for topology in [local, remote] {
        let services = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        services
            .services()
            .require_execution_host(topology.execution_host_id())
            .expect("topology authority is retained");
        assert!(services.services().working_resource_io().is_some());
    }

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::ProcessLifecycle);
    report.record(ConformanceAssertion::WorkingResourceCallback);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report
}
