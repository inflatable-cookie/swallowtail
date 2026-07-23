mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    app_server_plan, app_server_plan_with, host_services, host_services_with,
    session_resume_binding, working_resource,
};
use swallowtail_adapter_codex::CodexAppServerDriver;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, DriverRole, HostServiceKind,
    ReasoningMode,
};
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult,
    CancellationAcknowledgement, CleanupOutcome, Deadline, EnvironmentRef,
    InteractiveSessionDriver, ModelCatalogDriver, ModelCatalogRequest, MonotonicInstant,
    OperationContent, RequestId, RuntimeEventKind, RuntimeTurnId, SchemaDocument,
    SessionAccessPolicy, SessionOptions, SessionResumeBinding, StructuredOutputDescriptor,
    TerminalStatus, ToolDeclaration, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::RecordingHostServices;

fn driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    )
}

fn read_only_open_request(
    request_id: RequestId,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
) -> swallowtail_runtime::OpenSessionRequest {
    swallowtail_runtime::OpenSessionRequest::new(request_id, working_resource, deadline)
        .with_access_policy(SessionAccessPolicy::read_only())
}

fn read_only_resume_request(
    request_id: RequestId,
    binding: SessionResumeBinding,
    working_resource: WorkingResourceRef,
    deadline: Option<Deadline>,
) -> swallowtail_runtime::ResumeSessionRequest {
    swallowtail_runtime::ResumeSessionRequest::new(request_id, binding, working_resource, deadline)
        .with_access_policy(SessionAccessPolicy::read_only())
}

fn tool_declaration(name: &str) -> ToolDeclaration {
    ToolDeclaration::new(
        name,
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"operation":{"type":"string"}}}"#.to_vec(),
            1024,
        )
        .expect("tool schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid")
    .with_description(OperationContent::new("Operate on tasks").expect("description is valid"))
}

fn tool_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ToolCalls,
        [
            CapabilityConstraint::ToolMaximumCount(4),
            CapabilityConstraint::ToolMaximumSchemaBytes(4096),
            CapabilityConstraint::tool_schema_dialect("json-schema-2020-12")
                .expect("dialect is valid"),
        ],
    )
}

fn reasoning_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::ReasoningSelection,
        [CapabilityConstraint::reasoning_mode(
            ReasoningMode::new("low").expect("reasoning mode is valid"),
        )],
    )
}

fn session_options(tool_name: &str) -> SessionOptions {
    SessionOptions::default()
        .with_developer_instructions(
            OperationContent::new("private session instructions").expect("instructions are valid"),
        )
        .with_reasoning_mode(ReasoningMode::new("low").expect("reasoning mode is valid"))
        .with_tools([tool_declaration(tool_name)])
}

#[test]
fn model_catalog_initializes_pages_and_cleans_up() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let models = block_on(driver().list_models(
        app_server_plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalog-1").expect("request id is valid")),
        host_services(process),
    ))
    .expect("catalog succeeds");

    assert_eq!(
        models
            .iter()
            .map(|model| model.id().as_str())
            .collect::<Vec<_>>(),
        ["gpt-5.4-mini", "gpt-5.4"]
    );
    assert_eq!(models[0].metadata().display_name(), Some("GPT-5.4 Mini"));
    assert_eq!(
        models[0].metadata().description(),
        Some("Fast structured work")
    );
    assert!(models[0].metadata().is_default());
    assert!(!models[1].metadata().is_default());
    let reasoning = models[0]
        .metadata()
        .reasoning()
        .expect("reasoning catalog evidence is present");
    assert_eq!(
        reasoning
            .supported_modes()
            .map(|mode| mode.as_str())
            .collect::<Vec<_>>(),
        ["low", "medium"]
    );
    assert_eq!(
        reasoning.default_mode().map(|mode| mode.as_str()),
        Some("medium")
    );
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "model/list", "model/list"]
    );
    let initialize = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("initialize")
        })
        .expect("initialize was sent");
    assert!(initialize["params"].get("capabilities").is_none());
    assert!(state.waited());
    let process_request = state.request();
    assert_eq!(process_request.executable, "codex-app-server-executable");
    assert_eq!(
        process_request.arguments,
        ["app-server", "--listen", "stdio://"]
    );
    assert!(process_request.working_resource.is_none());
}

#[test]
fn model_catalog_deadline_closes_and_joins_the_connection() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldCatalog);
    let failure = block_on(
        driver().list_models(
            app_server_plan_with(DriverRole::ModelCatalog, [], [HostServiceKind::Time]),
            ModelCatalogRequest::new(
                RequestId::new("catalog-timeout").expect("request id is valid"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(20))),
            host_services_with(process, &recording, [HostServiceKind::Time]),
        ),
    )
    .expect_err("catalog deadline expires");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.catalog_timed_out"
    );
    assert!(state.waited());
}

#[test]
fn session_turn_streams_output_and_preserves_provider_ids() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-1").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider session id is present")
            .as_provider_value(),
        "thread-provider-new"
    );
    assert_ne!(
        session.session_id().as_str(),
        session
            .provider_session_ref()
            .expect("provider session id is present")
            .as_provider_value()
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-runtime-1").expect("turn id is valid"),
            OperationContent::new("private prompt").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    assert_eq!(
        turn.provider_turn_ref()
            .expect("provider turn id is present")
            .as_provider_value(),
        "turn-provider-1"
    );
    let events = block_on(
        turn.take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(events.iter().all(Result::is_ok));
    assert!(events.iter().any(|event| {
        event
            .as_ref()
            .is_ok_and(|event| event.kind() == &RuntimeEventKind::OutputDelta)
    }));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(|output| output.as_str()),
        Some("final answer")
    );
    assert!(!format!("{terminal:?}").contains("final answer"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert_eq!(
        state.request().working_resource.as_deref(),
        Some("workspace.main")
    );
    assert!(state.methods().contains(&"turn/start".to_owned()));
}

#[test]
fn session_options_and_dynamic_tool_callback_round_trip() {
    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::DynamicToolCall);
    let services = host_services(process);
    let plan = app_server_plan_with(
        DriverRole::InteractiveSession,
        [reasoning_capability(), tool_capability()],
        [],
    );
    let mut session = block_on(
        driver().open_session(
            plan,
            read_only_open_request(
                RequestId::new("session-tools").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            services.clone(),
        ),
    )
    .expect("tool-enabled session opens");

    let messages = state.messages();
    let initialize = messages
        .iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("initialize")
        })
        .expect("initialize was sent");
    assert_eq!(
        initialize["params"]["capabilities"]["experimentalApi"],
        true
    );
    let thread_start = messages
        .iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread/start was sent");
    assert_eq!(
        thread_start["params"]["developerInstructions"],
        "private session instructions"
    );
    assert!(
        thread_start["params"]
            .get("allowProviderModelFallback")
            .is_none()
    );
    assert_eq!(
        thread_start["params"]["dynamicTools"][0]["type"],
        "function"
    );
    assert_eq!(
        thread_start["params"]["dynamicTools"][0]["name"],
        "task_ledger"
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-tools").expect("turn id is valid"),
            OperationContent::new("list tasks").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn
        .take_callbacks()
        .expect("tool-enabled turn exposes callbacks");
    let mut requests = callbacks
        .take_requests()
        .expect("callback request stream is available");
    let request = block_on(requests.next())
        .expect("callback request arrives")
        .expect("callback request is valid");
    match request.kind() {
        CallbackRequestKind::ToolCall {
            tool_name,
            arguments,
        } => {
            assert_eq!(tool_name, "task_ledger");
            assert_eq!(
                serde_json::from_slice::<serde_json::Value>(arguments.as_bytes())
                    .expect("arguments remain JSON"),
                serde_json::json!({"operation": "list"})
            );
        }
        CallbackRequestKind::Extension(_) => panic!("expected a dynamic tool callback"),
        CallbackRequestKind::HarnessUiDialog(_) => panic!("expected a dynamic tool callback"),
    }
    let response = CallbackResponse::new(
        request.callback_id().clone(),
        request
            .turn_id()
            .expect("callback belongs to a turn")
            .clone(),
        CallbackResult::Success(
            CallbackPayload::new(br#"{"tasks":[]}"#.to_vec(), 128)
                .expect("callback result is bounded"),
        ),
    );
    block_on(callbacks.responder().respond(response.clone()))
        .expect("callback response is accepted");
    assert!(block_on(callbacks.responder().respond(response)).is_err());

    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    let turn_start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("turn/start")
        })
        .expect("turn/start was sent");
    assert_eq!(turn_start["params"]["effort"], "low");
    let provider_response = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
                && message.get("result").is_some()
        })
        .expect("provider callback response was sent");
    assert_eq!(provider_response["result"]["success"], true);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn gate_fixture_accepts_stable_session_without_experimental_fields() {
    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-experimental-gate").expect("request id is valid"),
            working_resource(),
            None,
        ),
        host_services(process),
    ));

    let session = result.expect("stable session opens without experimental negotiation");
    let initialize = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "initialize")
        .expect("initialize was sent");
    assert!(
        initialize
            .pointer("/params/capabilities/experimentalApi")
            .is_none()
    );
    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/start")
        .expect("thread/start was sent");
    assert!(
        thread_start["params"]
            .get("allowProviderModelFallback")
            .is_none()
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.waited());
}

#[test]
fn undeclared_dynamic_tool_never_reaches_the_consumer() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::DynamicToolCall);
    let services = host_services(process);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [],
            ),
            read_only_open_request(
                RequestId::new("session-unknown-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("different_tool")),
            services.clone(),
        ),
    )
    .expect("declared session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-unknown-tool").expect("turn id is valid"),
            OperationContent::new("try a tool").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert!(matches!(
        terminal.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert!(block_on(requests.next()).is_none());
    assert!(state.forced());
    assert!(state.messages().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn resumed_turn_uses_native_interruption_without_stopping_session() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldTurn);
    let services = host_services(process);
    let plan = app_server_plan_with(DriverRole::InteractiveSession, [reasoning_capability()], []);
    let binding = session_resume_binding(&plan, "thread-provider-existing");
    let mut session = block_on(
        driver().resume_session(
            plan,
            read_only_resume_request(
                RequestId::new("session-resume").expect("request id is valid"),
                binding,
                working_resource(),
                None,
            )
            .with_options(
                SessionOptions::default()
                    .with_developer_instructions(
                        OperationContent::new("resumed instructions")
                            .expect("instructions are valid"),
                    )
                    .with_reasoning_mode(
                        ReasoningMode::new("low").expect("reasoning mode is valid"),
                    ),
            ),
            services.clone(),
        ),
    )
    .expect("session resumes");
    assert_eq!(
        session
            .provider_session_ref()
            .expect("provider ref is present")
            .as_provider_value(),
        "thread-provider-existing"
    );
    let resume = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/resume")
        })
        .expect("thread/resume was sent");
    assert_eq!(
        resume["params"]["developerInstructions"],
        "resumed instructions"
    );
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-runtime-cancel").expect("turn id is valid"),
            OperationContent::new("keep working").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let resumed_turn = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("turn/start")
        })
        .expect("turn/start was sent");
    assert_eq!(resumed_turn["params"]["effort"], "low");
    assert_eq!(
        block_on(turn.cancellation().request()).expect("turn cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert!(!state.forced());
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"turn/interrupt".to_owned()));
}

#[test]
fn cancellation_abandons_pending_callback_and_rejects_late_response() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
    let services = host_services(process);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [],
            ),
            read_only_open_request(
                RequestId::new("session-cancel-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            services.clone(),
        ),
    )
    .expect("tool-enabled session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-cancel-tool").expect("turn id is valid"),
            OperationContent::new("wait for tool").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let request = block_on(requests.next())
        .expect("callback request arrives")
        .expect("callback request is valid");

    assert_eq!(
        block_on(turn.cancellation().request()).expect("turn cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    let late = CallbackResponse::new(
        request.callback_id().clone(),
        request
            .turn_id()
            .expect("callback belongs to a turn")
            .clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"late".to_vec(), 16).expect("payload is bounded"),
        ),
    );
    assert!(block_on(callbacks.responder().respond(late)).is_err());
    assert!(state.messages().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn callback_wait_ends_when_the_host_deadline_is_observed() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldDynamicToolCall);
    let recording = RecordingHostServices::default();
    let services = host_services_with(process, &recording, [HostServiceKind::Time]);
    let mut session = block_on(
        driver().open_session(
            app_server_plan_with(
                DriverRole::InteractiveSession,
                [reasoning_capability(), tool_capability()],
                [HostServiceKind::Time],
            ),
            read_only_open_request(
                RequestId::new("session-timeout-tool").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            services.clone(),
        ),
    )
    .expect("deadline-capable session opens");
    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn-timeout-tool").expect("turn id is valid"),
                OperationContent::new("wait for tool").expect("content is valid"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(50))),
            services,
        ),
    )
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("request stream exists");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    assert!(block_on(requests.next()).is_none());
    assert!(state.methods().contains(&"turn/interrupt".to_owned()));
    assert!(state.messages().iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("error").is_some()
    }));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn whole_session_cancellation_force_stops_and_joins() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::HoldTurn);
    let session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-cancel").expect("request id is valid"),
            working_resource(),
            None,
        ),
        host_services(process),
    ))
    .expect("session opens");
    assert_eq!(
        block_on(session.cancellation().request()).expect("session cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(session.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn unsupported_server_request_fails_instead_of_hanging() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::RequestCallback);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-callback").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-callback").expect("turn id is valid"),
            OperationContent::new("trigger callback").expect("content is valid"),
        ),
        services,
    ))
    .expect("turn response remains correlated");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );

    assert!(matches!(
        terminal.status(),
        TerminalStatus::RuntimeFailed(_)
    ));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(state.forced());
    assert!(state.waited());
}

#[test]
fn unsupported_session_input_fails_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-deadline").expect("request id is valid"),
            working_resource(),
            Some(Deadline::at(MonotonicInstant::from_ticks(10))),
        ),
        host_services(process),
    ));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn session_options_without_matching_preflight_fail_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let request = read_only_open_request(
        RequestId::new("session-options").expect("request id is valid"),
        working_resource(),
        None,
    )
    .with_options(
        SessionOptions::default()
            .with_reasoning_mode(ReasoningMode::new("low").expect("reasoning mode is valid")),
    );
    let result = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        request,
        host_services(process),
    ));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn resumed_dynamic_tools_fail_before_process_start_when_schema_cannot_redeclare_them() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let plan = app_server_plan_with(
        DriverRole::InteractiveSession,
        [reasoning_capability(), tool_capability()],
        [],
    );
    let binding = session_resume_binding(&plan, "thread-provider-existing");
    let result = block_on(
        driver().resume_session(
            plan,
            read_only_resume_request(
                RequestId::new("resume-tools").expect("request id is valid"),
                binding,
                working_resource(),
                None,
            )
            .with_options(session_options("task_ledger")),
            host_services(process),
        ),
    );

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn structured_output_is_rejected_before_turn_provider_work() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        read_only_open_request(
            RequestId::new("session-structured-output").expect("request id is valid"),
            working_resource(),
            None,
        ),
        services.clone(),
    ))
    .expect("session opens");
    let methods_before_turn = state.methods();
    let schema = StructuredOutputDescriptor::new(
        SchemaDocument::inline(b"{}".to_vec(), 16).expect("schema is within bound"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor is valid");
    let result = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("turn-structured-output").expect("turn id is valid"),
                OperationContent::new("return structured output").expect("content is valid"),
            )
            .with_structured_output(schema),
            services,
        ),
    );

    assert!(result.is_err());
    assert_eq!(state.methods(), methods_before_turn);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}
