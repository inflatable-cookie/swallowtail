mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::host_services_for;
use support::topology::{driver, open_request, plan_for, tool_capability, tool_options};
use swallowtail_runtime::{
    CallbackRequestKind, CancellationAcknowledgement, CleanupOutcome, InteractiveSessionDriver,
    OperationContent, RuntimeTurnId, TerminalStatus, TurnRequest,
};
use swallowtail_testkit::ExecutionTopologyFixture;

#[test]
fn callback_wait_cancellation_and_close_join_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let plan = plan_for(&topology, [tool_capability()]);
        let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
        let services = host_services_for(topology.execution_host_id().clone(), process);
        let mut session = block_on(driver().open_session(
            plan,
            open_request("callback", &topology).with_options(tool_options()),
            services.clone(),
        ))
        .expect("tool-enabled session opens");
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("turn:{}", topology.execution_host_id().as_str()))
                        .expect("turn id is valid"),
                    OperationContent::new("wait for callback").expect("content is valid"),
                ),
                services,
            ),
        )
        .expect("turn starts");
        let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
        let mut requests = callbacks.take_requests().expect("request stream exists");
        let callback = block_on(requests.next())
            .expect("callback arrives")
            .expect("callback is valid");
        assert!(matches!(
            callback.kind(),
            CallbackRequestKind::ToolCall { .. }
        ));

        assert_eq!(
            block_on(turn.cancellation().request()).expect("turn cancellation succeeds"),
            CancellationAcknowledgement::Requested
        );
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
        assert!(block_on(requests.next()).is_none());
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert!(state.waited());
        assert!(state.messages().iter().any(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
                && message.get("error").is_some()
        }));
    }
}

#[test]
fn session_close_abandons_active_callbacks_and_joins_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let plan = plan_for(&topology, [tool_capability()]);
        let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
        let services = host_services_for(topology.execution_host_id().clone(), process);
        let mut session = block_on(driver().open_session(
            plan,
            open_request("close-callback", &topology).with_options(tool_options()),
            services.clone(),
        ))
        .expect("tool-enabled session opens");
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!(
                        "close-turn:{}",
                        topology.execution_host_id().as_str()
                    ))
                    .expect("turn id is valid"),
                    OperationContent::new("wait while closing").expect("content is valid"),
                ),
                services,
            ),
        )
        .expect("turn starts");
        let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
        let mut requests = callbacks.take_requests().expect("request stream exists");
        block_on(requests.next())
            .expect("callback arrives")
            .expect("callback is valid");

        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
        assert!(block_on(requests.next()).is_none());
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert!(state.waited());
    }
}

#[test]
fn foreign_session_events_and_unexpected_disconnects_fail_without_recovery_guessing() {
    let topology = ExecutionTopologyFixture::local();
    for (mode, expected_status, forced) in [
        (AppServerMode::MismatchedTurnSession, "runtime-failed", true),
        (AppServerMode::DisconnectTurn, "host-failed", false),
    ] {
        let (process, state) = ScriptedAppServer::new(mode);
        let services = host_services_for(topology.execution_host_id().clone(), process);
        let mut session = block_on(driver().open_session(
            plan_for(&topology, []),
            open_request("recovery-boundary", &topology),
            services.clone(),
        ))
        .expect("session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("turn-{expected_status}")).expect("turn id is valid"),
                OperationContent::new("observe failure").expect("content is valid"),
            ),
            services,
        ))
        .expect("turn handle is returned");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        match expected_status {
            "runtime-failed" => {
                assert!(matches!(
                    terminal.status(),
                    TerminalStatus::RuntimeFailed(_)
                ));
            }
            "host-failed" => {
                assert!(matches!(terminal.status(), TerminalStatus::HostFailed(_)));
            }
            _ => unreachable!(),
        }
        assert_eq!(state.forced(), forced);
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert!(state.waited());
    }
}
