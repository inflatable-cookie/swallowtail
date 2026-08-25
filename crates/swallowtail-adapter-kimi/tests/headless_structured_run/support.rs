use super::discovery_support as support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KimiHeadlessPreparationInput, KimiHeadlessPreparationProbe,
    KimiHeadlessPreparedIntegration, KimiHeadlessRunInput, KimiModelSelection,
    prepare_kimi_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    ProcessExit, RequestId, RuntimeEvent, ScopeId, TerminalOutcome, TerminalStatus,
    WorkingResourceRef,
};
use swallowtail_testkit::ExecutionTopologyFixture;

pub struct RunEvidence {
    pub events: Vec<RuntimeEvent>,
    pub outcome: TerminalOutcome,
    pub request: support::ObservedProcessRequest,
    pub stdin_closed: bool,
}

pub fn execute(
    profile: &swallowtail_adapter_kimi::KimiHeadlessPreparedRun,
    host: ExecutionHostId,
    output: &str,
    exit: ProcessExit,
) -> RunEvidence {
    let (process, state) = support::FakeProcessService::with_exit(output, exit);
    let mut run = block_on(profile.start_run(services(host, process))).expect("run starts");
    assert!(run.provider_run_ref().is_none());
    let events = block_on(
        run.take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    RunEvidence {
        events,
        outcome,
        request: state.request(),
        stdin_closed: state.stdin_closed(),
    }
}

pub fn prepared(host: ExecutionHostId) -> KimiHeadlessPreparedIntegration {
    prepared_with_version(host, "0.31.1")
}

pub fn prepared_with_version(
    host: ExecutionHostId,
    version: &str,
) -> KimiHeadlessPreparedIntegration {
    let access = access_profile();
    let (process, state) = support::FakeProcessService::completed(&format!("{version}\n"));
    let prepared = block_on(prepare_kimi_headless(
        KimiHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("kimi.headless.fixture").expect("instance is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            host.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new("kimi.fixture.executable").expect("executable is valid"),
                InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("axis is valid"),
            ),
            EnvironmentRef::new("kimi.fixture.default-v1-environment")
                .expect("environment is valid"),
            access.clone(),
            PreparedAccessEvidence::caller_asserted(access_status(&access)),
        ),
        KimiHeadlessPreparationProbe::new(
            RequestId::new("kimi-headless-probe").expect("request is valid"),
            ScopeId::new("kimi-headless-probe").expect("scope is valid"),
            Deadline::at(MonotonicInstant::from_ticks(1000)),
            DiscoveryCancellation::new(),
        ),
        services(host, process),
    ))
    .expect("Kimi headless prepares");
    assert_eq!(state.request().arguments, ["--version"]);
    assert_eq!(prepared.observation().version().version().as_str(), version);
    prepared
}

pub fn profile(
    prepared: &KimiHeadlessPreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
) -> swallowtail_adapter_kimi::KimiHeadlessPreparedRun {
    prepared
        .prepare_run(
            KimiHeadlessRunInput::new(
                RequestId::new(format!("kimi-headless-{id}")).expect("request is valid"),
                KimiModelSelection::new(
                    ModelRouteId::new(format!("kimi.headless.{id}")).expect("route is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ModelId::new("kimi-coder").expect("model is valid"),
                ),
                OperationContent::new("private Kimi fixture prompt").expect("content is valid"),
                resource,
                Deadline::at(MonotonicInstant::from_ticks(1000)),
            )
            .accept_managed_recovery(),
        )
        .expect("run prepares")
}

pub fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("kimi.headless.membership").expect("access id is valid"),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("kimi-code-membership").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(
        CredentialRef::new("kimi.fixture.delegated-auth").expect("credential is valid"),
    )
}

pub fn access_status(access: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

pub fn assert_status(outcome: &TerminalOutcome, code: &str, provider: bool) {
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected status {status:?}"),
    };
    assert_eq!(diagnostic.code(), code);
}

pub fn local_topology() -> ExecutionTopologyFixture {
    ExecutionTopologyFixture::local()
}

pub fn services(
    host: ExecutionHostId,
    process: Arc<support::FakeProcessService>,
) -> swallowtail_runtime::HostServices {
    support::services(host, process)
}

pub fn services_with_time(
    host: ExecutionHostId,
    process: Arc<support::FakeProcessService>,
    time: Arc<support::ImmediateTime>,
) -> swallowtail_runtime::HostServices {
    support::services_with_time(host, process, time)
}
