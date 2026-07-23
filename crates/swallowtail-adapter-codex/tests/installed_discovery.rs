mod support;

use futures_executor::block_on;
use std::sync::Arc;
use support::{
    FakeProcessService, app_server_plan_for_version, host_services, host_services_for,
    host_services_with, plan_with_version,
};
use swallowtail_adapter_codex::{
    CODEX_CLI_AXIS, CodexAppServerDriver, CodexExecDriver, codex_app_server_claim, codex_exec_claim,
};
use swallowtail_core::{
    ConfiguredInstanceId, DiscoveryStatus, DriverRole, ExecutionHostId, InstanceTargetRef,
    InterfaceVersionAxis, PreflightPlan,
};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, Deadline, DeadlineObservation, DiscoveryCancellation,
    DiscoveryDriver, EnvironmentRef, ExecutableRef, InstalledExecutableDiscoveryRequest,
    InstalledExecutableTarget, MonotonicInstant, RequestId, ScopeId, TimeService,
};
use swallowtail_testkit::{ExecutionTopologyFixture, RecordingHostServices};

#[test]
fn both_drivers_probe_only_the_explicit_target_and_classify_independently() {
    for (driver, claim_id) in [
        (
            Driver::Exec(exec_driver()),
            codex_exec_claim().id().as_str().to_owned(),
        ),
        (
            Driver::App(app_driver()),
            codex_app_server_claim().id().as_str().to_owned(),
        ),
    ] {
        let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
        let outcome =
            block_on(driver.probe(request(DiscoveryCancellation::new()), services(process)))
                .expect("probe completes");
        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        let observation = outcome
            .installed_executable_observation()
            .expect("exact observation is present");
        assert_eq!(observation.version().version().as_str(), "0.145.0");
        assert_eq!(observation.claim_id().as_str(), claim_id);
        assert_eq!(observation.execution_host_id().as_str(), "host.local");
        let plan = driver.qualified_plan(observation.version().version().as_str());
        let bound = plan
            .interface_versions()
            .find(|binding| binding.axis().as_str() == CODEX_CLI_AXIS)
            .expect("preflight retains the promoted exact observation");
        assert_eq!(bound, observation.version());
        assert_eq!(state.request().arguments, ["--version"]);
        assert!(state.request().environments.is_empty());
        assert!(state.request().working_resource.is_none());
        assert!(state.stdin_closed());
        assert!(state.waited());
    }
}

#[test]
fn exact_observation_distinguishes_incompatible_malformed_and_cancelled() {
    let (process, _) = FakeProcessService::completed("codex-cli 0.108.0\n");
    let incompatible =
        block_on(exec_driver().discover_installed_executable(
            request(DiscoveryCancellation::new()),
            services(process),
        ))
        .expect("probe completes");
    assert_eq!(incompatible.status(), DiscoveryStatus::Incompatible);
    assert!(incompatible.installed_executable_observation().is_some());

    let secret = "private malformed output";
    let (process, _) = FakeProcessService::completed(&format!("{secret}\n"));
    let malformed =
        block_on(exec_driver().discover_installed_executable(
            request(DiscoveryCancellation::new()),
            services(process),
        ))
        .expect("probe completes");
    assert_eq!(malformed.status(), DiscoveryStatus::Malformed);
    assert!(malformed.installed_executable_observation().is_none());
    assert!(!format!("{malformed:?}").contains(secret));

    let cancellation = DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("cancellation is accepted");
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let cancelled = block_on(
        exec_driver().discover_installed_executable(request(cancellation), services(process)),
    )
    .expect("probe completes");
    assert_eq!(cancelled.status(), DiscoveryStatus::Cancelled);
    assert!(!state.started());
}

#[test]
fn host_deadline_wins_and_joins_the_probe_process() {
    let recording = RecordingHostServices::default();
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let outcome = block_on(exec_driver().discover_installed_executable(
        request(DiscoveryCancellation::new()),
        host_services_with(
            process,
            &recording,
            [swallowtail_core::HostServiceKind::Time],
        ),
    ))
    .expect("probe completes");

    assert_eq!(outcome.status(), DiscoveryStatus::TimedOut);
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn local_and_remote_authoritative_hosts_execute_their_own_probe() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
        let services = host_services_for(topology.execution_host_id().clone(), process)
            .with_time(Arc::new(PendingTime));
        let outcome = block_on(exec_driver().discover_installed_executable(
            request_for(
                topology.execution_host_id().clone(),
                ExecutableRef::from_instance_target(topology.instance_target()),
                DiscoveryCancellation::new(),
            ),
            services,
        ))
        .expect("authoritative probe completes");

        assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
        assert_eq!(
            outcome
                .installed_executable_observation()
                .unwrap()
                .execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(
            state.request().executable,
            topology.instance_target().as_host_value()
        );
        assert!(state.waited());
    }
}

enum Driver {
    Exec(CodexExecDriver),
    App(CodexAppServerDriver),
}

impl Driver {
    fn probe(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: swallowtail_runtime::HostServices,
    ) -> BoxFuture<
        '_,
        Result<swallowtail_core::DiscoveryOutcome, swallowtail_runtime::RuntimeFailure>,
    > {
        match self {
            Self::Exec(driver) => driver.discover_installed_executable(request, services),
            Self::App(driver) => driver.discover_installed_executable(request, services),
        }
    }

    fn qualified_plan(&self, version: &str) -> PreflightPlan {
        match self {
            Self::Exec(_) => plan_with_version(version, [], []),
            Self::App(_) => app_server_plan_for_version(
                DriverRole::ModelCatalog,
                ExecutionHostId::new("host.local").unwrap(),
                ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
                InstanceTargetRef::new("codex-app-server-executable").unwrap(),
                version,
                [],
                [],
            ),
        }
    }
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

fn services(
    process: Arc<dyn swallowtail_runtime::ProcessService>,
) -> swallowtail_runtime::HostServices {
    host_services(process).with_time(Arc::new(PendingTime))
}

fn request(cancellation: DiscoveryCancellation) -> InstalledExecutableDiscoveryRequest {
    request_for(
        ExecutionHostId::new("host.local").unwrap(),
        ExecutableRef::new("codex-executable").unwrap(),
        cancellation,
    )
}

fn request_for(
    host: ExecutionHostId,
    executable: ExecutableRef,
    cancellation: DiscoveryCancellation,
) -> InstalledExecutableDiscoveryRequest {
    InstalledExecutableDiscoveryRequest::new(
        RequestId::new("codex-version-probe").unwrap(),
        ScopeId::new("codex-version-probe").unwrap(),
        host,
        InstalledExecutableTarget::new(
            executable,
            InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
        ),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation,
    )
}

fn exec_driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}

fn app_driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}
