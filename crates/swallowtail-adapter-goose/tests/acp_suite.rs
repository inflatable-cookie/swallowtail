mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario, close_session, selection, selection_with_access};
use swallowtail_adapter_goose::{
    GOOSE_ACP_MCP_SERVER_NAME, GooseAcpDriver, GooseAcpRemoteMcpPlacement,
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
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("goose-open").expect("valid request"),
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
            RuntimeTurnId::new("goose-turn").expect("valid turn"),
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
    assert_eq!(observed.arguments, ["acp"]);
    assert!(
        !observed
            .arguments
            .iter()
            .any(|argument| argument == "serve")
    );
    assert!(
        !observed
            .arguments
            .iter()
            .any(|argument| argument == "--with-builtin")
    );
    assert_eq!(observed.environment_count, 1);
    assert_eq!(observed.working_resource, Some(selected.resource));
    assert!(host.writes().iter().any(|message| {
        message["method"] == "session/new"
            && message["params"]["mcpServers"] == serde_json::json!([])
    }));
    assert!(!host.writes().iter().any(|message| {
        matches!(
            message.get("method").and_then(serde_json::Value::as_str),
            Some("authenticate" | "session/load")
        )
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn read_write_plan_still_launches_acp_without_with_builtin() {
    let host_id = ExecutionHostId::new("fixture.host.write").expect("valid host id");
    let selected = selection_with_access(host_id.clone(), ResourceAccess::ReadWrite);
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    );
    let mut session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("goose-write-open").expect("valid request"),
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
    assert_eq!(host.observed_process().arguments, ["acp"]);
    assert!(
        !host
            .observed_process()
            .arguments
            .iter()
            .any(|argument| argument == "--with-builtin")
    );
    assert!(host.writes().iter().any(|message| {
        message["method"] == "initialize"
            && message["params"]["clientCapabilities"]["fs"]["readTextFile"] == false
            && message["params"]["clientCapabilities"]["fs"]["writeTextFile"] == false
            && message["params"]["clientCapabilities"]["terminal"] == false
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
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
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn cancelled_permission_interrupts_tool_and_preserves_route_outcome() {
    let (host, mut session, services) = open(Scenario::Permission, "permission-cancel");
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
    let writes = host.writes();
    let cancel_index = writes
        .iter()
        .position(|message| message["method"] == "session/cancel")
        .expect("active permission request triggers session/cancel");
    let reply_index = writes
        .iter()
        .position(|message| message.get("id").and_then(serde_json::Value::as_u64) == Some(900))
        .expect("permission request receives its response");
    assert!(
        cancel_index < reply_index,
        "cancel precedes the permission reply"
    );
    let permission_reply = &writes[reply_index];
    assert_eq!(
        permission_reply["result"]["outcome"],
        serde_json::json!({"outcome": "cancelled"})
    );
    assert!(
        permission_reply["result"]["outcome"]
            .get("optionId")
            .is_none()
    );

    let server_messages = host.server_messages();
    let advertised_tool_index = server_messages
        .iter()
        .position(|message| {
            message["params"]["update"]["sessionUpdate"] == "tool_call"
                && message["params"]["update"]["toolCallId"] == "tool-goose"
                && message["params"]["update"]["status"] == "in_progress"
        })
        .expect("tool request is advertised before permission is requested");
    let permission_request_index = server_messages
        .iter()
        .position(|message| message["method"] == "session/request_permission")
        .expect("permission request is sent for the advertised tool");
    assert!(advertised_tool_index < permission_request_index);
    assert_eq!(
        server_messages[permission_request_index]["params"]["toolCall"]["toolCallId"],
        "tool-goose"
    );
    assert!(
        server_messages[permission_request_index]["params"]["options"]
            .as_array()
            .expect("permission options")
            .iter()
            .any(|option| option["optionId"] == "allow_once")
    );
    let interrupted_tool_index = server_messages
        .iter()
        .position(|message| {
            message["params"]["update"]["sessionUpdate"] == "tool_call_update"
                && message["params"]["update"]["toolCallId"] == "tool-goose"
                && message["params"]["update"]["status"] == "failed"
                && message["params"]["update"]["content"][0]["content"]["text"]
                    == "Tool call was interrupted before completing"
        })
        .expect("cancelled tool call produces an interrupted result update");
    let cancelled_prompt_index = server_messages
        .iter()
        .position(|message| message["result"]["stopReason"] == "cancelled")
        .expect("prompt finishes with its cancelled result");
    assert!(permission_request_index < interrupted_tool_index);
    assert!(interrupted_tool_index < cancelled_prompt_index);
    assert_eq!(
        ProviderRequestHandling::Reject,
        swallowtail_core::ProviderRequestPolicy::reject_all().handling_for(
            &swallowtail_core::ExtensionNamespace::new("acp/session/request-permission")
                .expect("valid namespace")
        )
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn active_turn_cancellation_waits_for_cancelled_prompt_result() {
    let (_host, mut session, services) = open(Scenario::Cancellation, "cancellation");
    let mut turn = start(&mut *session, services.clone(), "cancel-turn");
    block_on(turn.cancellation().request()).expect("cancellation is sent");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn disconnect_fails_the_turn_and_session_close_still_joins_cleanup() {
    let (_host, mut session, services) = open(Scenario::Disconnect, "disconnect");
    let mut turn = start(&mut *session, services.clone(), "disconnect-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn typed_session_new_auth_failure_fails_closed_without_configure() {
    let host_id = ExecutionHostId::new("fixture.host.auth").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::AuthRequired);
    let services = host.services(host_id);
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("goose-auth").expect("valid request"),
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
        "swallowtail.goose.acp.auth_required"
    );
    assert_eq!(host.releases(), 1);
}

#[test]
fn typed_prompt_auth_failure_fails_the_turn_as_auth_required() {
    let (host, mut session, services) = open(Scenario::AuthRequiredOnPrompt, "prompt-auth");
    let mut turn = start(&mut *session, services.clone(), "prompt-auth-turn");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let TerminalStatus::RuntimeFailed(error) = outcome.status() else {
        panic!("typed prompt auth failure must fail the turn");
    };
    assert_eq!(error.code(), "swallowtail.goose.acp.auth_required");
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
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
        let driver = GooseAcpDriver::new(
            swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
                .expect("valid environment"),
        );
        let error = match block_on(driver.open_session(
            selected.plan,
            OpenSessionRequest::new(
                RequestId::new(format!("goose-{suffix}")).expect("valid request"),
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
        block_on(close_session(session, services)),
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
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    );
    let error = match block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("goose-deadline").expect("valid request"),
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
        "swallowtail.goose.acp.unsupported"
    );
    assert!(!host.process_started());
    assert_eq!(host.releases(), 0);
}

#[test]
fn http_mcp_is_admitted_on_session_new_and_keeps_values_verbatim() {
    let host_id = ExecutionHostId::new("fixture.host.http-mcp").expect("valid host id");
    let selected = selection(host_id.clone());
    let host = FixtureHost::new(Scenario::Success);
    let services = host.services(host_id);
    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-036-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-036-redaction-canary";
    let admitted = GooseAcpRemoteMcpPlacement::new(
        GOOSE_ACP_MCP_SERVER_NAME,
        CANARY_URL,
        vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
    );
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    )
    .with_http_mcp_placement(admitted)
    .expect("route-owned http placement is admitted");
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new("goose-http-mcp").expect("valid request"),
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
            && message["params"]["mcpServers"][0]["type"] == "http"
            && message["params"]["mcpServers"][0]["name"] == GOOSE_ACP_MCP_SERVER_NAME
            && message["params"]["mcpServers"][0]["url"] == CANARY_URL
            && message["params"]["mcpServers"][0]["headers"][0]["value"] == CANARY_HEADER
    }));
    assert_eq!(
        block_on(close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn sse_mcp_is_refused_before_session_new() {
    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-036-redaction-canary";
    let remote = GooseAcpRemoteMcpPlacement::sse(
        GOOSE_ACP_MCP_SERVER_NAME,
        CANARY_URL,
        vec![(
            "Authorization".to_owned(),
            "Bearer g06-036-redaction-canary".to_owned(),
        )],
    );
    let error = match GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    )
    .with_http_mcp_placement(remote)
    {
        Err(error) => error,
        Ok(_) => panic!("sse must refuse"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.goose.acp.mcp_sse_unsupported"
    );
    assert!(!error.diagnostic().message().contains(CANARY_URL));
    assert!(!format!("{error:?}").contains("g06-036-redaction-canary"));
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
    let driver = GooseAcpDriver::new(
        swallowtail_runtime::EnvironmentRef::new("goose.fixture.isolated")
            .expect("valid environment"),
    );
    let session = block_on(driver.open_session(
        selected.plan,
        OpenSessionRequest::new(
            RequestId::new(format!("goose-{suffix}")).expect("valid request"),
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
