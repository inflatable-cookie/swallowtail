#[path = "acp_support/mod.rs"]
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario, selection, selection_with_access};
use swallowtail_adapter_opencode::{
    OPENCODE_ACP_MCP_SERVER_NAME, OpenCodeAcpDriver, OpenCodeAcpRemoteMcpPlacement,
    OpenCodeAcpStdioMcpServer,
};
use swallowtail_core::{ExecutionHostId, ProviderRequestHandling, ResourceAccess};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, InteractiveSessionDriver, MonotonicInstant, OpenSessionRequest,
    OperationContent, RequestId, RuntimeEventKind, RuntimeTurnId, SessionPlanAgreement,
    TerminalStatus, TurnRequest,
};

#[test]
fn success_turn_uses_acp_only_and_joins_cleanup() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-open").expect("valid request"),
            selected.resource.clone(),
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider session exists")
            .as_provider_value(),
        "opaque-fixture-session"
    );
    assert!(session.negotiated_model_options().is_none());
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("opencode-turn").expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "fixture response."
    );
    let mut events = turn.take_events().expect("events are available");
    let events = block_on(async move {
        let mut seen = Vec::new();
        while let Some(event) = events.next().await {
            seen.push(event.expect("event is valid"));
        }
        seen
    });
    assert!(
        events
            .iter()
            .any(|event| matches!(event.kind(), RuntimeEventKind::OutputDelta))
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{outcome:?}").contains("fixture response"));
    let observed = host.observed_process();
    assert_eq!(observed.arguments, ["acp", "--pure"]);
    assert!(
        !observed
            .arguments
            .iter()
            .any(|argument| argument == "--port" || argument == "--hostname")
    );
    assert!(
        !observed
            .arguments
            .iter()
            .any(|argument| argument == "--yolo")
    );
    assert_eq!(observed.environment_count, 1);
    assert_eq!(observed.working_resource, Some(selected.resource));
    assert!(!host.writes().iter().any(|message| {
        matches!(
            message.get("method").and_then(serde_json::Value::as_str),
            Some("authenticate" | "session/load")
        )
    }));
    assert!(host.writes().iter().any(|message| {
        message["method"] == "initialize"
            && message["params"]["protocolVersion"] == 1
            && !format!("{message}").contains("negotiated")
    }));
    assert!(host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"] == serde_json::json!([])
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
    }));
    assert_eq!(host.releases(), 1);
}

#[test]
fn read_write_plan_still_launches_acp_without_port_or_yolo() {
    let host_id = ExecutionHostId::new("fixture.host.write").expect("valid host id");
    let selected = selection_with_access(host_id.clone(), ResourceAccess::ReadWrite);
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-write-open").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("write session opens");
    let mut turn = start(&mut *session, services.clone(), "write-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(host.observed_process().arguments, ["acp", "--pure"]);
    assert!(
        !host
            .observed_process()
            .arguments
            .iter()
            .any(|argument| argument == "--port"
                || argument == "--hostname"
                || argument == "--yolo")
    );
    assert!(host.writes().iter().any(|message| {
        message["method"] == "initialize"
            && message["params"]["clientCapabilities"]["fs"]["readTextFile"] == false
            && message["params"]["clientCapabilities"]["fs"]["writeTextFile"] == false
            && message["params"]["clientCapabilities"]["terminal"] == false
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn unexpected_write_callback_is_rejected_before_host_mutation() {
    let (host, mut session, services) = open(Scenario::UnexpectedWrite, "unexpected-write");
    let mut turn = start(&mut *session, services.clone(), "unexpected-write-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert!(host.writes().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(702)
            && message["error"]["code"] == -32601
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn permission_observes_without_selecting_allow_always() {
    let (host, mut session, services) = open(Scenario::Permission, "permission");
    let mut turn = start(&mut *session, services.clone(), "permission-turn");
    assert!(turn.take_callbacks().is_none());
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    assert!(host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/cancel")
    }));
    assert!(host.writes().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["outcome"] == "cancelled"
            && message["result"]["outcome"].get("optionId")
                != Some(&serde_json::json!("allow_always"))
    }));
    assert_eq!(
        ProviderRequestHandling::Reject,
        swallowtail_core::ProviderRequestPolicy::reject_all().handling_for(
            &swallowtail_core::ExtensionNamespace::new("acp/session/request-permission")
                .expect("valid namespace")
        )
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn active_turn_cancellation_waits_for_cancelled_prompt_result() {
    let (host, mut session, services) = open(Scenario::Cancellation, "cancellation");
    let mut turn = start(&mut *session, services.clone(), "cancel-turn");
    block_on(turn.cancellation().request()).expect("cancellation is sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn disconnect_fails_the_turn_and_session_close_still_joins_cleanup() {
    let (host, mut session, services) = open(Scenario::Disconnect, "disconnect");
    let mut turn = start(&mut *session, services.clone(), "disconnect-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn missing_host_auth_fails_closed_without_login() {
    let host_id = ExecutionHostId::new("fixture.host.auth").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::AuthRequired);
    let services = host.services(host_id);
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-auth").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services,
    )) {
        Err(error) => error,
        Ok(_) => panic!("auth gate must fail closed"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.acp.host_auth_required"
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn malformed_and_version_mismatch_fail_before_prompt() {
    for (scenario, suffix) in [
        (Scenario::Malformed, "malformed"),
        (Scenario::ProtocolMismatch, "protocol"),
    ] {
        let host_id =
            ExecutionHostId::new(format!("fixture.host.{suffix}")).expect("valid host id");
        let selected = selection(host_id.clone());
        let host = FixtureHost::new(scenario);
        let services = host.services(host_id);
        let driver = OpenCodeAcpDriver::new(
            swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
                .expect("valid environment"),
        );
        let error = match block_on(driver.open_session(
            selected.plan,
            OpenSessionRequest::new(
                RequestId::new(format!("opencode-{suffix}")).expect("valid request"),
                selected.resource,
                None,
                SessionPlanAgreement::explicit(
                    swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                    Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                    Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
                ),
            ),
            services,
        )) {
            Err(error) => error,
            Ok(_) => panic!("drift must be rejected"),
        };
        assert!(!format!("{error:?}").contains("private fixture prompt"));
        assert_eq!(host.releases(), 1);
    }
}

#[test]
fn oversized_update_fails_closed_and_still_joins() {
    let (host, mut session, services) = open(Scenario::Oversized, "oversized");
    let mut turn = start(&mut *session, services.clone(), "oversized-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn session_deadline_is_rejected_before_spawn() {
    let host_id = ExecutionHostId::new("fixture.host.deadline").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-deadline").expect("valid request"),
            selected.resource,
            Some(Deadline::at(MonotonicInstant::from_ticks(1_000))),
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services,
    )) {
        Err(error) => error,
        Ok(_) => panic!("deadline must be rejected before spawn"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.acp.unsupported"
    );
    assert!(!host.process_started());
    assert_eq!(host.releases(), 0);
}

#[test]
fn stdio_mcp_is_admitted_on_session_new_and_foreign_names_fail() {
    let host_id = ExecutionHostId::new("fixture.host.mcp").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let admitted = OpenCodeAcpStdioMcpServer::new(
        OPENCODE_ACP_MCP_SERVER_NAME,
        "/usr/bin/echo-mcp",
        vec!["--stdio".to_owned()],
        Vec::<(String, String)>::new(),
    );
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    )
    .with_stdio_mcp_server(admitted)
    .expect("route-owned stdio name is admitted");
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("opencode-mcp").expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    assert!(host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"][0]["name"] == OPENCODE_ACP_MCP_SERVER_NAME
            && message["params"]["mcpServers"][0]["command"] == "/usr/bin/echo-mcp"
            && message["params"]["mcpServers"][0].get("url").is_none()
    }));
    assert_eq!(
        block_on(session.close(host.cleanup_request(), services)),
        CleanupOutcome::Clean
    );

    let collision = OpenCodeAcpStdioMcpServer::new(
        "other-server",
        "/usr/bin/echo-mcp",
        Vec::<String>::new(),
        Vec::<(String, String)>::new(),
    );
    let error = match OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    )
    .with_stdio_mcp_server(collision)
    {
        Err(error) => error,
        Ok(_) => panic!("foreign MCP names collide"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.acp.mcp_name_collision"
    );
}

#[test]
fn url_plus_header_mcp_cannot_reach_production() {
    let remote = OpenCodeAcpRemoteMcpPlacement::new(
        OPENCODE_ACP_MCP_SERVER_NAME,
        "http://127.0.0.1:9/mcp",
        vec![("Authorization".to_owned(), "Bearer secret".to_owned())],
    );
    let error = remote
        .to_production_mcp_servers()
        .expect_err("remote placement stays gated");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.opencode.acp.mcp_remote_not_admitted"
    );
    assert!(!error.diagnostic().message().contains("Bearer secret"));
}

fn open(
    scenario: Scenario,
    suffix: &str,
) -> (
    FixtureHost,
    Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
    swallowtail_runtime::HostServices,
) {
    let host_id = ExecutionHostId::new(format!("fixture.host.{suffix}")).expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(scenario);
    let services = host.services(host_id);
    let driver = OpenCodeAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("opencode.acp.fixture.isolated")
            .expect("valid environment"),
    );
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new(format!("opencode-{suffix}")).expect("valid request"),
            selected.resource,
            None,
            SessionPlanAgreement::explicit(
                swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                Some(swallowtail_core::SessionProviderStatePolicy::Prohibited),
                Some(swallowtail_core::HarnessConfigurationPosture::Ambient),
            ),
        ),
        services.clone(),
    ))
    .expect("session opens");
    (host, session, services)
}

fn start(
    session: &mut dyn swallowtail_runtime::InteractiveSessionHandle,
    services: swallowtail_runtime::HostServices,
    turn_id: &str,
) -> Box<dyn swallowtail_runtime::TurnHandle> {
    block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new(turn_id).expect("valid turn"),
            OperationContent::new("private fixture prompt").expect("valid prompt"),
        ),
        services,
    ))
    .expect("turn starts")
}
