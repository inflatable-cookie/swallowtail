use crate::{
    CallbackExchangeFixture, ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture,
    ProfilePreflightFixture, RecordedHostCall, RecordingHostServices, RecordingOutcome,
    SyntheticProfile, assert_common_contract, poll_immediate, remote_acp_requirements,
    successful_callback_response,
};
use swallowtail_core::{
    CredentialMechanism, DriverRole, EndpointAudience, HostServiceKind, RemoteAcpTransport,
    SupportAuthority,
};
use swallowtail_runtime::{
    CallbackId, CallbackPayload, CallbackRequest, CallbackWaitState, EndpointRef, RuntimeTurnId,
    ScopeId,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::RemoteAcpHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    assert_exact_preflight(profile);
    assert_transport_records_are_closed();
    assert_callback_exchange();
    assert_topology_and_joined_work(profile);

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::CallbackExchange);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report.record(ConformanceAssertion::RemoteAcpConnectionLifecycle);
    report.record(ConformanceAssertion::RemoteAcpAffinityScoped);
    report.record(ConformanceAssertion::RemoteAcpNoRecovery);
    report.record(ConformanceAssertion::RemoteAcpVersionAxesSeparate);
    report
}

fn assert_exact_preflight(profile: SyntheticProfile) {
    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture.preflight().expect("remote ACP preflight succeeds");
    assert_eq!(
        plan.credential_mechanism(),
        &CredentialMechanism::Unauthenticated
    );
    assert!(plan.credential_reference().is_none());
    assert!(
        plan.requirements()
            .access()
            .accepts_support_authority(SupportAuthority::ExperimentalObserved)
    );
    assert!(
        !plan
            .requirements()
            .access()
            .accepts_support_authority(SupportAuthority::ProviderSupported)
    );
    let required = fixture
        .driver()
        .required_host_services(DriverRole::InteractiveSession)
        .collect::<Vec<_>>();
    for service in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
    ] {
        assert!(required.contains(&service));
    }
    for service in [
        HostServiceKind::Process,
        HostServiceKind::Credential,
        HostServiceKind::WorkingResource,
        HostServiceKind::WorkingResourceIo,
    ] {
        assert!(!required.contains(&service));
    }
    assert_eq!(plan.interface_versions().count(), 0);
    assert_eq!(
        plan.requirements()
            .remote_acp()
            .expect("remote requirements are bound")
            .transport(),
        RemoteAcpTransport::StreamableHttpSse
    );
}

fn assert_transport_records_are_closed() {
    for transport in [
        RemoteAcpTransport::StreamableHttpSse,
        RemoteAcpTransport::WebSocket,
    ] {
        let requirements = remote_acp_requirements(transport);
        assert_eq!(requirements.transport(), transport);
        assert!(requirements.affinity().maximum_cookie_count().is_some());
        assert!(requirements.affinity().maximum_cookie_bytes().is_some());
        assert_eq!(requirements.maximum_connections().get(), 1);
        assert_eq!(requirements.maximum_active_sessions().get(), 1);
        assert!(!requirements.permits_redirect());
        assert!(!requirements.permits_retry());
        assert!(!requirements.permits_reconnect());
        assert!(!requirements.permits_replay_or_resumption());
        assert!(!requirements.permits_transport_fallback());
        assert!(!requirements.permits_pooling_or_multiplexing());
    }
}

fn assert_callback_exchange() {
    let mut callback = CallbackExchangeFixture::new(
        CallbackRequest::tool_call(
            CallbackId::new("remote-acp-callback").expect("callback id is valid"),
            RuntimeTurnId::new("remote-acp-turn").expect("turn id is valid"),
            2,
            None,
            "fixture_tool",
            CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
        )
        .expect("callback request is valid"),
    );
    let response = successful_callback_response(callback.request());
    callback.respond(response).expect("callback is correlated");
    assert_eq!(callback.state(), CallbackWaitState::Responded);
}

fn assert_topology_and_joined_work(profile: SyntheticProfile) {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for transport in [
            RemoteAcpTransport::StreamableHttpSse,
            RemoteAcpTransport::WebSocket,
        ] {
            assert_topology_transport(profile, &topology, transport);
        }
    }
}

fn assert_topology_transport(
    profile: SyntheticProfile,
    topology: &ExecutionTopologyFixture,
    transport: RemoteAcpTransport,
) {
    assert_eq!(profile, SyntheticProfile::RemoteAcpHarness);
    let fixture =
        ProfilePreflightFixture::for_remote_acp(transport, topology.execution_host_id().clone());
    let plan = fixture
        .preflight()
        .expect("remote ACP topology preflight succeeds");
    assert_eq!(plan.execution_host_id(), topology.execution_host_id());
    assert_eq!(
        plan.requirements()
            .remote_acp()
            .expect("remote ACP requirements are bound")
            .transport(),
        transport
    );

    let recording = RecordingHostServices::for_host(
        topology.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let scope = ScopeId::new("remote-acp-operation").expect("scope is valid");
    poll_immediate(
        recording
            .services()
            .network()
            .expect("network service exists")
            .authorize(
                scope.clone(),
                EndpointRef::new("remote-acp-endpoint").expect("endpoint is valid"),
                EndpointAudience::new("remote-acp-audience").expect("audience is valid"),
            ),
    )
    .expect("endpoint authorization succeeds");
    let _now = recording
        .services()
        .time()
        .expect("time service exists")
        .now();
    let task = recording
        .services()
        .task()
        .expect("task service exists")
        .spawn(scope, Box::pin(async {}))
        .expect("connection task starts");
    poll_immediate(task.join()).expect("connection task joins");

    assert_eq!(recording.count(RecordedHostCall::NetworkAuthorize), 1);
    assert_eq!(recording.count(RecordedHostCall::TimeNow), 1);
    assert_eq!(recording.count(RecordedHostCall::TaskSpawn), 1);
    assert_eq!(recording.count(RecordedHostCall::TaskJoin), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialAcquire), 0);
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
}
