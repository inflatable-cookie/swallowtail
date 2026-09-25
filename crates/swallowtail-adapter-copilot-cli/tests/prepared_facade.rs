#![allow(dead_code)]

mod support;

#[path = "support/discovery.rs"]
mod discovery_support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_copilot_cli::{
    COPILOT_CLI_ACP_HTTP_MCP_PLACEMENT, COPILOT_CLI_ACP_MATURITY, COPILOT_CLI_ACP_MCP_SERVER_NAME,
    COPILOT_CLI_EXECUTABLE_NAME, COPILOT_CLI_HOST_ACCOUNT_AUDIENCE, COPILOT_CLI_PACKAGE_AXIS,
    COPILOT_CLI_PACKAGE_VERSION, CopilotCliAcpRemoteMcpPlacement, CopilotCliPreparationInput,
    CopilotCliPreparationProbe, CopilotCliSessionProfileInput,
    copilot_cli_host_account_access_profile, prepare_copilot_cli_acp,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest, WorkingResourceRef,
    WorkingStateRestorationOutcome,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_session_names_copilot_cli_acp_and_release_then_drains_one_prompt() {
    let host_id = ExecutionHostId::new("fixture.prepared.local").expect("host");
    let discovery = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_copilot_cli_acp(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Copilot CLI ACP prepares");
    assert_eq!(
        discovery
            .observed_process()
            .expect("version probe ran")
            .arguments,
        ["--version"]
    );
    assert_eq!(
        prepared.observation().version().axis().as_str(),
        COPILOT_CLI_PACKAGE_AXIS
    );
    assert_eq!(
        prepared.observation().version().version().as_str(),
        COPILOT_CLI_PACKAGE_VERSION
    );
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        COPILOT_CLI_HOST_ACCOUNT_AUDIENCE
    );
    assert!(prepared.access_profile().credential_reference().is_none());
    assert_eq!(
        prepared.access_profile().support_authority(),
        SupportAuthority::ExperimentalObserved
    );
    assert_eq!(COPILOT_CLI_ACP_MATURITY, "public-preview");
    assert_eq!(
        prepared.instance().driver_id().as_str(),
        "swallowtail.copilot-cli.acp"
    );

    let session = prepared
        .prepare_session(session_input("prompt"))
        .expect("session prepares");
    assert_prepared_operation_evidence_matches_plan(session.evidence(), session.plan());
    assert_eq!(
        session.plan().driver_identity().id().as_str(),
        "swallowtail.copilot-cli.acp"
    );
    let versions: Vec<_> = session
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
            COPILOT_CLI_PACKAGE_AXIS.to_owned(),
            COPILOT_CLI_PACKAGE_VERSION.to_owned()
        )]
    );
    assert!(session.plan().credential_reference().is_none());
    assert_eq!(
        session.plan().access_status().support_authority(),
        SupportAuthority::ExperimentalObserved
    );
    assert!(session.request().deadline().is_none());

    let operation_services = operation.services(host_id);
    let mut handle = block_on(session.open_session(operation_services.clone())).expect("opens");
    let mut turn = block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("copilot-prepared-turn").expect("turn"),
            OperationContent::new("private fixture prompt").expect("prompt"),
        ),
        operation_services.clone(),
    ))
    .expect("turn starts");
    let events = block_on(turn.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
        Some("fixture response.")
    );
    assert_eq!(operation.observed_process().arguments, ["--acp", "--stdio"]);
    assert!(
        !operation
            .observed_process()
            .arguments
            .iter()
            .any(|argument| argument == "--port" || argument == "--yolo")
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(handle.close(operation.cleanup_request(), operation_services)),
        CleanupOutcome::Clean
    );
    assert_eq!(operation.releases(), 1);
}

#[test]
fn preparation_rejects_access_axis_and_package_drift_before_acp_work() {
    let host_id = ExecutionHostId::new("fixture.prepared.reject").expect("host");
    let prepared = prepare(host_id.clone());
    let wrong_host = ExecutionHostId::new("fixture.prepared.other").expect("host");
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );

    let discovery = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let wrong_access = AccessProfile::new(
        AccessProfileId::new("copilot-cli.fixture.host-account").expect("access"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("wrong.audience").expect("audience"),
        SupportAuthority::ExperimentalObserved,
    );
    let error = block_on(prepare_copilot_cli_acp(
        CopilotCliPreparationInput::new(
            ConfiguredInstanceId::new("copilot-cli.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            target(),
            EnvironmentRef::new("copilot-cli.fixture.isolated").expect("environment"),
            wrong_access,
            evidence(),
        ),
        probe(),
        discovery.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.copilot-cli.acp.preparation.access_profile_rejected"
    );
    assert!(discovery.observed_process().is_none());

    let axis_host = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let error = block_on(prepare_copilot_cli_acp(
        CopilotCliPreparationInput::new(
            ConfiguredInstanceId::new("copilot-cli.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/bin/{COPILOT_CLI_EXECUTABLE_NAME}"))
                    .expect("executable"),
                InterfaceVersionAxis::new("copilot-cli.tcp-port").expect("axis"),
            ),
            EnvironmentRef::new("copilot-cli.fixture.isolated").expect("environment"),
            copilot_cli_host_account_access_profile(
                AccessProfileId::new("copilot-cli.fixture.host-account").expect("access"),
            ),
            evidence(),
        ),
        probe(),
        axis_host.services(host_id.clone()),
    ))
    .expect_err("tcp-port axis is not this route");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.copilot-cli.acp.preparation.target_axis_mismatch"
    );
    assert!(axis_host.observed_process().is_none());

    let newer_host = ExecutionHostId::new("fixture.prepared.newer").expect("host");
    let newer = DiscoveryHost::new("1.0.81");
    let error = block_on(prepare_copilot_cli_acp(
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
        ["--version"]
    );
}

#[test]
fn session_prepare_fails_closed_without_working_resource_authority() {
    let host_id = ExecutionHostId::new("fixture.prepared.no-resource").expect("host");
    let discovery = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let prepared = block_on(prepare_copilot_cli_acp(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("discovery does not require a working resource");
    let error = prepared
        .prepare_session(session_input("missing-resource"))
        .expect_err("session preflight requires working-resource authority");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
}

const HTTP_CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-034-redaction-canary";
const HTTP_CANARY_HEADER: &str = "Bearer g06-034-redaction-canary";

fn http_placement() -> CopilotCliAcpRemoteMcpPlacement {
    CopilotCliAcpRemoteMcpPlacement::new(
        COPILOT_CLI_ACP_MCP_SERVER_NAME,
        HTTP_CANARY_URL,
        vec![("Authorization".to_owned(), HTTP_CANARY_HEADER.to_owned())],
    )
}

fn assert_http_secrets_redacted(value: &impl std::fmt::Debug) {
    let rendered = format!("{value:?}");
    assert!(
        !rendered.contains(HTTP_CANARY_URL),
        "debug leaked the URL: {rendered}"
    );
    assert!(
        !rendered.contains(HTTP_CANARY_HEADER),
        "debug leaked a header value: {rendered}"
    );
}

#[test]
fn prepared_session_can_bind_the_admitted_http_mcp_placement() {
    let host_id = ExecutionHostId::new("fixture.prepared.http-mcp").expect("host");
    let prepared = prepare(host_id.clone());
    let session = prepared
        .prepare_session(session_input("http-mcp").with_http_mcp_placement(http_placement()))
        .expect("session prepares");
    assert_http_secrets_redacted(&session);
    assert_http_secrets_redacted(session.plan());
    assert_http_secrets_redacted(session.evidence());
    let contribution = session
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new(
                "copilot-cli.acp.prepared-session",
            )
            .expect("source"),
        )
        .expect("projects");
    assert_http_secrets_redacted(&contribution);
    let placement = contribution
        .selection_rows()
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id() == "mcp.placement")
        })
        .expect("HTTP placement is named");
    let values = match placement
        .control_value()
        .expect("placement names values")
        .domain()
    {
        swallowtail_runtime::ConsumerRouteValueDomain::Enumerated(values) => values,
        other => panic!("placement must be enumerated, got {other:?}"),
    };
    let texts: Vec<&str> = values.values().map(|value| value.as_str()).collect();
    assert!(texts.contains(&COPILOT_CLI_ACP_HTTP_MCP_PLACEMENT));
    assert!(texts.contains(&COPILOT_CLI_ACP_MCP_SERVER_NAME));
    assert!(
        !texts
            .iter()
            .any(|value| value.contains("g06-034-redaction-canary"))
    );
    let operation = FixtureHost::new(Scenario::Success);
    let handle =
        block_on(session.open_session(operation.services(host_id.clone()))).expect("opens");
    assert!(operation.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["url"] == HTTP_CANARY_URL
            && message["params"]["mcpServers"][0]["headers"][0]["value"] == HTTP_CANARY_HEADER
    }));
    assert_eq!(
        block_on(handle.close(operation.cleanup_request(), operation.services(host_id))),
        CleanupOutcome::Clean
    );
}

#[test]
fn working_state_restoration_carries_the_prepared_http_mcp_declaration() {
    let host_id = ExecutionHostId::new("fixture.prepared.restore-http-mcp").expect("host");
    let prepared = prepare(host_id.clone());
    let session = prepared
        .prepare_session(
            session_input("restore-http-mcp").with_http_mcp_placement(http_placement()),
        )
        .expect("session prepares");
    let operation = FixtureHost::new(Scenario::Success);
    let restoration = session.prepare_working_state_restoration(
        RuntimeTurnId::new("interrupted-http-mcp").expect("turn"),
    );
    let WorkingStateRestorationOutcome::SessionReplaced(replaced) =
        block_on(restoration.restore(operation.services(host_id.clone()))).expect("restores")
    else {
        panic!("Copilot CLI ACP restoration is a fresh session replacement");
    };
    assert!(operation.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["url"] == HTTP_CANARY_URL
    }));
    let (_, handle) = replaced.into_parts();
    assert_eq!(
        block_on(handle.close(operation.cleanup_request(), operation.services(host_id))),
        CleanupOutcome::Clean
    );
}

#[test]
fn restoration_refuses_invalid_http_mcp_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.prepared.restore-sse-mcp").expect("host");
    let prepared = prepare(host_id.clone());
    let session = prepared
        .prepare_session(session_input("restore-sse-mcp").with_http_mcp_placement(
            CopilotCliAcpRemoteMcpPlacement::sse(
                COPILOT_CLI_ACP_MCP_SERVER_NAME,
                HTTP_CANARY_URL,
                vec![("Authorization".to_owned(), HTTP_CANARY_HEADER.to_owned())],
            ),
        ))
        .expect("session prepares");
    let operation = FixtureHost::new(Scenario::Success);
    let restoration = session.prepare_working_state_restoration(
        RuntimeTurnId::new("interrupted-sse-mcp").expect("turn"),
    );
    let error = match block_on(restoration.restore(operation.services(host_id))) {
        Err(error) => error,
        Ok(_) => panic!("sse restoration must refuse before the ACP child starts"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.copilot-cli.acp.mcp_sse_not_emitted"
    );
    assert_http_secrets_redacted(&error);
    assert!(!operation.process_started());
}

fn prepare(
    host_id: ExecutionHostId,
) -> swallowtail_adapter_copilot_cli::CopilotCliPreparedIntegration {
    let discovery = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_copilot_cli_acp(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("Copilot CLI ACP prepares")
}

fn preparation_input(host_id: ExecutionHostId) -> CopilotCliPreparationInput {
    CopilotCliPreparationInput::new(
        ConfiguredInstanceId::new("copilot-cli.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host_id,
        target(),
        EnvironmentRef::new("copilot-cli.fixture.isolated").expect("environment"),
        copilot_cli_host_account_access_profile(
            AccessProfileId::new("copilot-cli.fixture.host-account").expect("access"),
        ),
        evidence(),
    )
}

fn session_input(id: &str) -> CopilotCliSessionProfileInput {
    CopilotCliSessionProfileInput::new(
        RequestId::new(format!("copilot-cli.fixture.session.{id}")).expect("request"),
        WorkingResourceRef::new("copilot-cli.fixture.workspace").expect("resource"),
    )
}

fn probe() -> CopilotCliPreparationProbe {
    CopilotCliPreparationProbe::new(
        RequestId::new("copilot-cli.fixture.probe").expect("request"),
        ScopeId::new("copilot-cli.fixture.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{COPILOT_CLI_EXECUTABLE_NAME}"))
            .expect("executable"),
        InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("axis"),
    )
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("copilot-cli.fixture.host-account").expect("access"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ExperimentalObserved,
    ))
}
