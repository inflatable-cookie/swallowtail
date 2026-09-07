// Contract 063 route evidence for Codex dynamic native tool binding.
//
// Every case here drives the mounted wiring: a real prepared selection, the
// real local registered-tool bridge port, the real Codex app-server driver,
// and a scripted provider that issues `item/tool/call` and reads the exact
// response bytes back off the wire.

use support::registered::{
    REGISTERED_TOOL_WIRE_NAME, RouteDispatcher, registered_binding, registered_options,
    registered_plan, registered_services, registered_tool_id, result_text, text_result,
    tool_response, wait_until,
};
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_host_local::LocalHostServices;
use swallowtail_runtime::{
    RegisteredToolDispatcher, RegisteredToolExecutionDisposition, RegisteredToolExecutionKind,
    RegisteredToolFailure, RegisteredToolFailureKind, RegisteredToolOutcome,
};
use swallowtail_testkit::ScriptedAdmissionPort;

fn run_registered_turn(
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
    admission: &Arc<ScriptedAdmissionPort>,
    label: &str,
) -> (
    serde_json::Value,
    Arc<support::app_server::AppServerState>,
    LocalHostServices,
    CleanupOutcome,
) {
    let (process, state) = ScriptedAppServer::new(AppServerMode::RegisteredToolCall);
    let (services, local) = registered_services(process, dispatcher);
    let binding = registered_binding(admission);
    let mut session = block_on(
        driver()
            .with_registered_tools(binding.clone())
            .open_session(
                registered_plan(),
                read_only_open_request(
                    RequestId::new(format!("session-{label}")).expect("request id is valid"),
                    working_resource(),
                    None,
                )
                .with_options(registered_options(&binding)),
                services.clone(),
            ),
    )
    .expect("registered session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new(format!("turn-{label}")).expect("turn id is valid"),
            OperationContent::new("use the registered tool").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    assert!(
        turn.take_callbacks().is_none(),
        "registered tools expose no consumer callback exchange"
    );
    let response = tool_response(&state);
    let _terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let turn_cleanup = block_on(turn.close());
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    (response, state, local, turn_cleanup)
}

#[test]
fn a_registered_native_tool_is_declared_dispatched_and_returns_its_real_result() {
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let dispatcher = RouteDispatcher::new(|call| {
        assert_eq!(call.tool(), &registered_tool_id());
        assert_eq!(call.kind(), RegisteredToolExecutionKind::NativeClient);
        assert_eq!(
            std::str::from_utf8(call.arguments().expose_for_execution())
                .expect("arguments are text"),
            r#"{"operation":"list"}"#
        );
        Ok(text_result(call, "three open tasks"))
    });
    let (response, state, local, turn_cleanup) =
        run_registered_turn(dispatcher.clone(), &admission, "registered-allow");

    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/start")
        .expect("thread/start was sent");
    assert_eq!(
        thread_start["params"]["dynamicTools"][0]["name"],
        REGISTERED_TOOL_WIRE_NAME
    );
    assert_eq!(response["result"]["success"], true);
    assert_eq!(result_text(&response), "three open tasks");
    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(
        admission.observed_phases(),
        vec![
            swallowtail_runtime::AdmissionPhase::BeforeDispatch,
            swallowtail_runtime::AdmissionPhase::BeforeDelivery
        ]
    );
    assert_eq!(turn_cleanup, CleanupOutcome::Clean);
    assert_eq!(local.registered_tool_lease_count(), 0);
}

#[test]
fn a_consumer_denial_reaches_the_provider_as_an_exact_tool_failure() {
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let dispatcher = RouteDispatcher::new(|call| {
        Ok(RegisteredToolOutcome::failed(
            call,
            RegisteredToolFailure::new(RegisteredToolFailureKind::ConsumerDenied),
            RegisteredToolExecutionDisposition::NotExecuted,
        ))
    });
    let (response, _state, local, turn_cleanup) =
        run_registered_turn(dispatcher.clone(), &admission, "registered-deny");

    assert_eq!(response["result"]["success"], false);
    assert_eq!(
        result_text(&response),
        RegisteredToolFailureKind::ConsumerDenied.message()
    );
    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(turn_cleanup, CleanupOutcome::Clean);
    assert_eq!(local.registered_tool_lease_count(), 0);
}

#[test]
fn an_unknown_execution_outcome_is_reported_honestly_and_never_replayed() {
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let dispatcher = RouteDispatcher::new(|call| {
        Ok(RegisteredToolOutcome::failed(
            call,
            RegisteredToolFailure::new(RegisteredToolFailureKind::UnknownExecutionOutcome),
            RegisteredToolExecutionDisposition::Unknown,
        ))
    });
    let (response, _state, _local, _cleanup) =
        run_registered_turn(dispatcher.clone(), &admission, "registered-unknown");

    assert_eq!(response["result"]["success"], false);
    assert!(
        result_text(&response).contains("execution outcome unknown and not retried"),
        "the provider is told the outcome is unknown"
    );
    assert_eq!(dispatcher.dispatches(), 1);
}

#[test]
fn revoked_admission_refuses_the_call_before_any_dispatch() {
    let admission = Arc::new(ScriptedAdmissionPort::current());
    admission.revoke_now();
    let dispatcher = RouteDispatcher::new(|call| Ok(text_result(call, "never dispatched")));
    let (response, _state, local, turn_cleanup) =
        run_registered_turn(dispatcher.clone(), &admission, "registered-revoked");

    assert_eq!(response["result"]["success"], false);
    assert_eq!(
        result_text(&response),
        RegisteredToolFailureKind::Revoked.message()
    );
    assert_eq!(dispatcher.dispatches(), 0);
    assert_eq!(turn_cleanup, CleanupOutcome::Clean);
    assert_eq!(local.registered_tool_lease_count(), 0);
}

#[test]
fn an_unsettled_dispatch_keeps_its_lease_and_never_reports_a_clean_close() {
    let released = Arc::new(AtomicBool::new(false));
    let dispatcher_gate = Arc::clone(&released);
    let admission = Arc::new(ScriptedAdmissionPort::current());
    let dispatcher = RouteDispatcher::new(move |call| {
        while !dispatcher_gate.load(Ordering::SeqCst) {
            std::thread::yield_now();
        }
        Ok(text_result(call, "late result"))
    });
    let (process, state) = ScriptedAppServer::new(AppServerMode::RegisteredToolCall);
    let (services, local) = registered_services(process, dispatcher.clone());
    let binding = registered_binding(&admission);
    let mut session = block_on(
        driver()
            .with_registered_tools(binding.clone())
            .open_session(
                registered_plan(),
                read_only_open_request(
                    RequestId::new("session-registered-retained").expect("request id is valid"),
                    working_resource(),
                    None,
                )
                .with_options(registered_options(&binding)),
                services.clone(),
            ),
    )
    .expect("registered session opens");
    let turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-registered-retained").expect("turn id is valid"),
            OperationContent::new("use the registered tool").expect("content is valid"),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    // The dispatch is committed and unsettled: the kernel is holding the call.
    wait_until(|| dispatcher.dispatches() == 1);

    let cleanup = block_on(turn.close());

    assert_eq!(
        cleanup,
        CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.codex.app_server.registered_lease_retained",
            "Codex registered tool lease is still held by an unsettled dispatch",
        ))
    );
    assert_eq!(
        local.registered_tool_lease_count(),
        1,
        "the unjoined lease stays owned by its host registry"
    );

    // Releasing the dispatcher settles the call; the retained lease is then
    // released by its own defensive cleanup, never by the failed close.
    released.store(true, Ordering::SeqCst);
    let response = tool_response(&state);
    assert_eq!(response["result"]["success"], true);
    wait_until(|| local.registered_tool_lease_count() == 0);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}
