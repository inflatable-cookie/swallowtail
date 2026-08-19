#![allow(dead_code)]

use super::driver_suite::{discovery_support, support};

use crate::{
    OPENHANDS_LOCAL_ACCOUNT_AUDIENCE, OPENHANDS_PACKAGE_AXIS, OPENHANDS_PACKAGE_VERSION,
    OpenHandsAgentServerPreparationInput, OpenHandsAgentServerPreparationProbe,
    OpenHandsAgentServerRunProfileInput, openhands_local_config_access_profile,
    prepare_openhands_agent_server,
};
use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use support::FixtureHost;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const ACTIVITY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/activity.jsonl");

#[test]
fn prepared_run_names_openhands_and_package_then_drains_one_conversation() {
    let host_id = ExecutionHostId::new("fixture.prepared.agent-server.local").expect("host");
    let discovery = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([""]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_openhands_agent_server(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("OpenHands Agent Server prepares");
    assert_eq!(
        discovery
            .observed_process()
            .expect("version probe ran")
            .arguments,
        [
            "-c",
            "from importlib.metadata import version; print(version('openhands-agent-server'))"
        ]
    );
    assert_eq!(
        prepared.observation().version().axis().as_str(),
        OPENHANDS_PACKAGE_AXIS
    );
    assert_eq!(
        prepared.observation().version().version().as_str(),
        OPENHANDS_PACKAGE_VERSION
    );
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        OPENHANDS_LOCAL_ACCOUNT_AUDIENCE
    );
    assert!(prepared.access_profile().credential_reference().is_none());
    assert_eq!(
        prepared.instance().driver_id().as_str(),
        "swallowtail.openhands.agent-server"
    );
    assert!(
        prepared
            .instance()
            .capabilities()
            .iter()
            .all(|(capability, _)| capability != Capability::InteractiveSession)
    );

    let run = prepared
        .prepare_run(run_input("prompt"))
        .expect("run prepares");
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(
        run.plan().driver_identity().id().as_str(),
        "swallowtail.openhands.agent-server"
    );
    let versions: Vec<_> = run
        .plan()
        .interface_versions()
        .map(|binding| {
            (
                binding.axis().as_str().to_owned(),
                binding.version().as_str().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        versions,
        [(
            OPENHANDS_PACKAGE_AXIS.to_owned(),
            OPENHANDS_PACKAGE_VERSION.to_owned()
        )]
    );
    assert!(run.plan().credential_reference().is_none());
    assert!(run.plan().model_id().is_none());
    assert!(run.plan().model_route_id().is_none());
    assert!(run.request().deadline().is_some());

    let live = block_on(run.start_run(operation.services(host_id.clone())));
    match live {
        Err(failure) => {
            assert_eq!(
                failure.diagnostic().code(),
                "swallowtail.openhands.agent_server.live_http_unwired"
            );
        }
        Ok(_) => panic!("live HTTP must stay unwired"),
    }
    assert!(!operation.started());

    let mut handle =
        block_on(run.start_scripted_run(json_lines(ACTIVITY), operation.services(host_id)))
            .expect("scripted run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
        Some("OpenHands display text.")
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{terminal:?}").contains("OpenHands display text"));
    let observed = operation.observed();
    assert_eq!(
        observed.arguments,
        [
            "-m",
            "openhands.agent_server",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
        ]
    );
    for forbidden in [
        "0.0.0.0",
        "::",
        "[::]",
        "--cors-origins",
        "acp",
        "NeverConfirm",
    ] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    assert!(operation.stdin().is_empty());
    assert!(operation.stdin_closed());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(operation.joined());
}

#[test]
fn preparation_rejects_access_axis_and_package_drift_before_stream_work() {
    let host_id = ExecutionHostId::new("fixture.prepared.agent-server.reject").expect("host");
    let prepared = prepare(host_id.clone());
    let wrong_host = ExecutionHostId::new("fixture.prepared.agent-server.other").expect("host");
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );

    let discovery = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let wrong_access = AccessProfile::new(
        AccessProfileId::new("openhands.fixture.local-loopback").expect("access"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("wrong.audience").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let error = block_on(prepare_openhands_agent_server(
        OpenHandsAgentServerPreparationInput::new(
            ConfiguredInstanceId::new("openhands.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            target(),
            EnvironmentRef::new("openhands.fixture.isolated").expect("environment"),
            wrong_access,
            evidence(),
        ),
        probe(),
        discovery.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.openhands.agent_server.preparation.access_profile_rejected"
    );
    assert!(discovery.observed_process().is_none());

    let axis_host = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let error = block_on(prepare_openhands_agent_server(
        OpenHandsAgentServerPreparationInput::new(
            ConfiguredInstanceId::new("openhands.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new("openhands.fixture.interpreter").expect("executable"),
                InterfaceVersionAxis::new("openhands.acp").expect("axis"),
            ),
            EnvironmentRef::new("openhands.fixture.isolated").expect("environment"),
            openhands_local_config_access_profile(
                AccessProfileId::new("openhands.fixture.local-loopback").expect("access"),
            ),
            evidence(),
        ),
        probe(),
        axis_host.services(host_id.clone()),
    ))
    .expect_err("ACP axis is not this route");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.openhands.agent_server.preparation.target_axis_mismatch"
    );
    assert!(axis_host.observed_process().is_none());

    let newer_host = ExecutionHostId::new("fixture.prepared.agent-server.newer").expect("host");
    let newer = DiscoveryHost::new("1.42.2");
    let error = block_on(prepare_openhands_agent_server(
        preparation_input(newer_host.clone()),
        probe(),
        newer.services(newer_host),
    ))
    .expect_err("unqualified package fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(
        newer.observed_process().expect("probe ran").arguments,
        [
            "-c",
            "from importlib.metadata import version; print(version('openhands-agent-server'))"
        ]
    );
}

#[test]
fn run_prepare_fails_closed_without_working_resource_authority() {
    let host_id = ExecutionHostId::new("fixture.prepared.agent-server.no-resource").expect("host");
    let discovery = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let prepared = block_on(prepare_openhands_agent_server(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("discovery does not require a working resource");
    let error = prepared
        .prepare_run(run_input("missing-resource"))
        .expect_err("run preflight requires working-resource authority");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
}

fn prepare(host_id: ExecutionHostId) -> crate::OpenHandsAgentServerPreparedIntegration {
    let discovery = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([""]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_openhands_agent_server(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("OpenHands Agent Server prepares")
}

fn preparation_input(host_id: ExecutionHostId) -> OpenHandsAgentServerPreparationInput {
    OpenHandsAgentServerPreparationInput::new(
        ConfiguredInstanceId::new("openhands.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host_id,
        target(),
        EnvironmentRef::new("openhands.fixture.isolated").expect("environment"),
        openhands_local_config_access_profile(
            AccessProfileId::new("openhands.fixture.local-loopback").expect("access"),
        ),
        evidence(),
    )
}

fn run_input(id: &str) -> OpenHandsAgentServerRunProfileInput {
    OpenHandsAgentServerRunProfileInput::new(
        RequestId::new(format!("openhands.fixture.run.{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        WorkingResourceRef::new("openhands.fixture.workspace").expect("resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn probe() -> OpenHandsAgentServerPreparationProbe {
    OpenHandsAgentServerPreparationProbe::new(
        RequestId::new("openhands.fixture.agent-server.probe").expect("request"),
        ScopeId::new("openhands.fixture.agent-server.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new("openhands.fixture.interpreter").expect("executable"),
        InterfaceVersionAxis::new(OPENHANDS_PACKAGE_AXIS).expect("axis"),
    )
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("openhands.fixture.local-loopback").expect("access"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

fn json_lines(body: &str) -> Vec<Value> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).expect("jsonl"))
        .collect()
}
