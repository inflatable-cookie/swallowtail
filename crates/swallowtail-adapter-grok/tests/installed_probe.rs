use crate::discovery_support as support;

use futures_executor::block_on;
use support::{FakeProcessService, ImmediateTime, services, services_with_time};
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GrokAcpDriver, GrokModelSelection, GrokPreparationInput,
    GrokPreparationProbe, GrokRunProfileInput, GrokSessionProfileInput, grok_build_acp_claim,
    grok_build_acp_descriptor, grok_build_subscription_access_profile, prepare_grok_build,
};
use swallowtail_core::{
    AccessStatus, ConfiguredInstanceId, CredentialState, DiscoveryStatus, EndpointAuthorization,
    EntitlementState, ExecutionHostId, HarnessConfigurationPosture,
    InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SessionRef, SupportAuthority,
};
use swallowtail_runtime::{
    CancellationControl, Deadline, DiscoveryCancellation, DiscoveryDriver, EnvironmentRef,
    ExecutableRef, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, RequestId, RuntimeTurnId, ScopeId,
    SessionOptions, SessionResumeBinding, WorkingResourceRef, WorkingStateRestorationMethod,
};

#[test]
fn exact_and_unverified_versions_probe_only_the_approved_target_on_both_topologies() {
    for topology in [
        swallowtail_testkit::ExecutionTopologyFixture::local(),
        swallowtail_testkit::ExecutionTopologyFixture::remote_authoritative(),
    ] {
        for (output, version, qualified) in [
            ("grok 0.2.114 (0c785038798) [stable]\n", "0.2.114", true),
            ("grok 0.2.115 (dd16b5eb7d50) [stable]\n", "0.2.115", true),
            ("grok 0.2.116 (99b387d2cc0e) [stable]\n", "0.2.116", true),
            ("grok 0.2.117 (f1c06093089f) [stable]\n", "0.2.117", true),
            ("grok 1.0.4 (d846eb93d94d) [stable]\n", "1.0.4", true),
            ("grok 1.0.6 (abcdef123456) [stable]\n", "1.0.6", false),
        ] {
            let host = topology.execution_host_id().clone();
            let executable = ExecutableRef::from_instance_target(topology.instance_target());
            let (process, state) = FakeProcessService::completed(output);
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
                .expect("observation is present");
            assert_eq!(observation.execution_host_id(), &host);
            assert_eq!(observation.version().version().as_str(), version);
            assert_eq!(observation.claim_id(), grok_build_acp_claim().id());
            assert_eq!(observation.is_qualified(), qualified);
            if !qualified {
                let InstalledExecutableCompatibility::UnverifiedNewer(unverified) =
                    observation.compatibility()
                else {
                    panic!("later stable version remains unverified");
                };
                assert_eq!(unverified.latest_qualified().as_str(), "1.0.4");
            }
            let captured = state.request();
            assert_eq!(captured.executable, executable.as_host_value());
            assert_eq!(
                captured.arguments,
                ["--no-auto-update".to_owned(), "--version".to_owned()]
            );
            assert!(captured.environments.is_empty());
            assert!(captured.working_resource.is_none());
            assert!(state.stdin_closed());
            assert!(state.waited());
        }
    }
}

#[test]
fn incompatible_malformed_cancelled_and_timed_out_results_stay_distinct() {
    let host = ExecutionHostId::new("fixture.host.grok.classification").expect("valid host");
    let (process, _) = FakeProcessService::completed("grok 0.2.113 (123456789abc) [stable]\n");
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

    let private = "private malformed provider output";
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
    assert!(!format!("{malformed:?}").contains(private));

    let cancellation = DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("cancellation is accepted");
    let (process, state) = FakeProcessService::completed("grok 0.2.114 (0c785038798) [stable]\n");
    let cancelled = block_on(driver().discover_installed_executable(
        request(host.clone(), fixture_executable(), cancellation),
        services(host.clone(), process),
    ))
    .expect("cancelled discovery completes");
    assert_eq!(cancelled.status(), DiscoveryStatus::Cancelled);
    assert!(!state.started());

    let (process, state) = FakeProcessService::held_open();
    let timed_out = block_on(driver().discover_installed_executable(
        request(
            host.clone(),
            fixture_executable(),
            DiscoveryCancellation::new(),
        ),
        services_with_time(host, process, std::sync::Arc::new(ImmediateTime)),
    ))
    .expect("timed-out discovery completes");
    assert_eq!(timed_out.status(), DiscoveryStatus::TimedOut);
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn prepared_discovery_binds_exact_instance_access_and_ambient_posture() {
    for topology in [
        swallowtail_testkit::ExecutionTopologyFixture::local(),
        swallowtail_testkit::ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let host = topology.execution_host_id().clone();
        let executable = ExecutableRef::from_instance_target(topology.instance_target());
        let target = InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("valid axis"),
        );
        let credential =
            swallowtail_core::CredentialRef::new("grok.fixture.credential").expect("credential");
        let access = grok_build_subscription_access_profile(credential);
        let evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access.id().clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ));
        let input = GrokPreparationInput::new(
            ConfiguredInstanceId::new("grok.fixture.instance").expect("instance"),
            InstanceRevision::new("grok.fixture.instance-r1").expect("revision"),
            host.clone(),
            target.clone(),
            EnvironmentRef::new("grok.fixture.ambient-state").expect("environment"),
            access.clone(),
            evidence,
        );
        let probe = GrokPreparationProbe::new(
            RequestId::new("grok-preparation").expect("request"),
            ScopeId::new("grok-preparation").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        );
        let (process, _) = FakeProcessService::completed("grok 0.2.117 (f1c06093089f) [stable]\n");
        let prepared = block_on(prepare_grok_build(
            input,
            probe,
            services(host.clone(), process),
        ))
        .expect("preparation succeeds");

        assert_eq!(prepared.target(), &target);
        assert_eq!(prepared.observation().execution_host_id(), &host);
        assert_eq!(prepared.access_profile(), &access);
        assert_eq!(
            prepared.instance().driver_id(),
            grok_build_acp_descriptor().identity().id()
        );
        assert_eq!(prepared.instance().execution_host_id(), &host);
        assert_eq!(
            prepared.instance().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            prepared
                .instance()
                .interface_versions()
                .next()
                .expect("version")
                .version()
                .as_str(),
            "0.2.117"
        );
        prepared
            .validate_execution_binding(&host, &target)
            .expect("exact execution binding remains valid");
        let session = prepared
            .prepare_session(GrokSessionProfileInput::new(
                RequestId::new("grok-session").expect("request"),
                GrokModelSelection::new(
                    ModelRouteId::new("grok.fixture.route").expect("route"),
                    ModelRouteRevision::new("grok.fixture.route-r1").expect("route revision"),
                    ModelId::new("grok-4.5").expect("model"),
                ),
                WorkingResourceRef::new("grok.fixture.workspace").expect("workspace"),
                SessionOptions::default(),
            ))
            .expect("prepared session succeeds");
        assert_eq!(
            session.plan().model_id().expect("model").as_str(),
            "grok-4.5"
        );
        assert_eq!(
            session.request().provider_state_policy(),
            Some(swallowtail_core::SessionProviderStatePolicy::DurableProviderSessionPreserved)
        );
        assert_eq!(
            session.request().access_policy(),
            &swallowtail_core::SessionAccessPolicy::ambient_harness(
                swallowtail_core::ResourceAccess::ReadWrite
            )
        );
        assert_eq!(
            session
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            swallowtail_core::ObservableActivityAvailability::Available
        );
        let binding = SessionResumeBinding::new(
            SessionRef::new("grok-prepared-session").expect("session"),
            session.plan().instance_id().clone(),
            session.plan().execution_host_id().clone(),
            session.plan().model_route_id().expect("route").clone(),
            session.plan().model_id().expect("model").clone(),
            WorkingResourceRef::new("grok.fixture.workspace").expect("workspace"),
            session.request().access_policy().clone(),
        );
        assert_eq!(
            session
                .prepare_working_state_restoration(
                    RequestId::new("grok-recovery").expect("request"),
                    binding,
                    RuntimeTurnId::new("lost-grok-turn").expect("turn"),
                )
                .expect("attachment recovery prepares")
                .method(),
            WorkingStateRestorationMethod::ProviderSessionAttachmentRecovery
        );
        let run = prepared
            .prepare_run(GrokRunProfileInput::new(
                RequestId::new("grok-run").expect("request"),
                GrokModelSelection::new(
                    ModelRouteId::new("grok.fixture.run-route").expect("route"),
                    ModelRouteRevision::new("grok.fixture.run-route-r1").expect("route revision"),
                    ModelId::new("grok-4.5").expect("model"),
                ),
                OperationContent::new("fixture prompt").expect("content"),
                WorkingResourceRef::new("grok.fixture.workspace").expect("workspace"),
                Some(Deadline::at(MonotonicInstant::from_ticks(200))),
            ))
            .expect("prepared run succeeds");
        assert_eq!(
            run.plan().requirements().operation_shape(),
            swallowtail_core::OperationShape::StructuredRun
        );
        assert_eq!(
            run.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            swallowtail_runtime::ProviderRetentionPolicy::DurableAllowed
        );
        assert_eq!(
            run.request()
                .deadline()
                .expect("deadline")
                .instant()
                .ticks(),
            200
        );
    }
}

fn request(
    host: ExecutionHostId,
    executable: ExecutableRef,
    cancellation: DiscoveryCancellation,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("grok-version-probe").expect("valid request"),
        ScopeId::new("grok-version-probe").expect("valid scope"),
        host,
        InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("valid axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation,
    )
}

fn fixture_executable() -> ExecutableRef {
    ExecutableRef::new("grok.fixture.approved").expect("valid executable")
}

fn driver() -> GrokAcpDriver {
    GrokAcpDriver::new(
        EnvironmentRef::new("grok.fixture.ambient-state").expect("valid environment"),
        swallowtail_core::CredentialRef::new("grok.fixture.credential").expect("valid credential"),
    )
}
