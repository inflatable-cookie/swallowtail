mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    app_server_plan, bounded_workspace_plan, bounded_workspace_plan_for, host_services,
    host_services_with, host_services_with_for, working_resource,
};
use swallowtail_adapter_codex::{
    CodexAppServerDriver, codex_approval_request_extension, codex_bounded_workspace_access_policy,
    codex_user_input_request_extension,
};
use swallowtail_core::{DriverRole, HostServiceKind};
use swallowtail_runtime::{
    CallbackRequestKind, CleanupOutcome, EnvironmentRef, InteractiveSessionDriver,
    OpenSessionRequest, OperationContent, ProviderRequestObservation, RequestId, RuntimeTurnId,
    SessionAccessPolicy, TerminalStatus, TurnRequest,
};
use swallowtail_testkit::{
    ExecutionTopologyFixture, RecordedHostCall, RecordingHostServices, RecordingOutcome,
};

fn driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    )
}

#[test]
fn bounded_workspace_maps_one_host_authorized_root_and_denies_network() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let mut session = block_on(
        driver().open_session(
            bounded_workspace_plan(),
            OpenSessionRequest::new(
                RequestId::new("workspace-session").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_access_policy(codex_bounded_workspace_access_policy()),
            services.clone(),
        ),
    )
    .expect("bounded workspace session opens");

    let thread = message(&state.messages(), "thread/start");
    assert_eq!(thread["params"]["sandbox"], "workspace-write");
    assert_eq!(thread["params"]["approvalPolicy"], "never");
    assert_eq!(thread["params"]["cwd"], "/private/recording/workspace");
    assert_eq!(
        thread["params"]["runtimeWorkspaceRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );
    assert_eq!(
        message(&state.messages(), "initialize")["params"]["capabilities"]["experimentalApi"],
        true
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("workspace-turn").expect("turn id is valid"),
            OperationContent::new("perform bounded work").expect("content is valid"),
        ),
        services,
    ))
    .expect("workspace turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);

    let turn_start = message(&state.messages(), "turn/start");
    let sandbox = &turn_start["params"]["sandboxPolicy"];
    assert_eq!(sandbox["type"], "workspaceWrite");
    assert_eq!(sandbox["writableRoots"].as_array().map(Vec::len), Some(1));
    assert_eq!(sandbox["writableRoots"][0], "/private/recording/workspace");
    assert_eq!(sandbox["networkAccess"], false);
    assert_eq!(sandbox["excludeSlashTmp"], true);
    assert_eq!(sandbox["excludeTmpdirEnvVar"], true);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn read_only_session_request_shape_remains_unchanged() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(
        driver().open_session(
            app_server_plan(DriverRole::InteractiveSession),
            OpenSessionRequest::new(
                RequestId::new("read-only-session").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_access_policy(SessionAccessPolicy::read_only()),
            services.clone(),
        ),
    )
    .expect("read-only session opens");
    let thread = message(&state.messages(), "thread/start");
    assert_eq!(thread["params"]["sandbox"], "read-only");
    assert_eq!(thread["params"]["approvalPolicy"], "never");
    assert!(thread["params"].get("cwd").is_none());
    assert!(thread["params"].get("runtimeWorkspaceRoots").is_none());

    let turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("read-only-turn").expect("turn id is valid"),
            OperationContent::new("inspect only").expect("content is valid"),
        ),
        services,
    ))
    .expect("read-only turn starts");
    assert!(
        message(&state.messages(), "turn/start")["params"]
            .get("sandboxPolicy")
            .is_none()
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn writable_request_without_host_resource_service_fails_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let result = block_on(
        driver().open_session(
            bounded_workspace_plan(),
            OpenSessionRequest::new(
                RequestId::new("workspace-service-missing").expect("request id is valid"),
                working_resource(),
                None,
            )
            .with_access_policy(codex_bounded_workspace_access_policy()),
            host_services(process),
        ),
    );

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn bounded_workspace_open_retains_local_and_remote_authoritative_host_identity() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let recording = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let services = host_services_with_for(
            topology.execution_host_id().clone(),
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        );
        let plan = bounded_workspace_plan_for(
            topology.execution_host_id().clone(),
            topology.configured_instance_id().clone(),
            topology.instance_target().clone(),
        );
        let session = block_on(
            driver().open_session(
                plan,
                OpenSessionRequest::new(
                    RequestId::new(format!(
                        "workspace-open:{}",
                        topology.execution_host_id().as_str()
                    ))
                    .expect("request id is valid"),
                    topology.working_resource().clone(),
                    None,
                )
                .with_access_policy(codex_bounded_workspace_access_policy()),
                services,
            ),
        )
        .expect("bounded session opens on its authoritative host");

        assert_eq!(
            state.request().working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn declared_approval_and_user_input_requests_are_observed_then_stop() {
    for (mode, expected_namespace, provider_id) in [
        (
            AppServerMode::ObserveApproval,
            codex_approval_request_extension(),
            "approval-900",
        ),
        (
            AppServerMode::ObserveUserInput,
            codex_user_input_request_extension(),
            "input-900",
        ),
    ] {
        let recording = RecordingHostServices::default();
        let (process, state) = ScriptedAppServer::new(mode);
        let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
        let mut session = block_on(
            driver().open_session(
                bounded_workspace_plan(),
                OpenSessionRequest::new(
                    RequestId::new(format!("observed-{provider_id}")).expect("request id is valid"),
                    working_resource(),
                    None,
                )
                .with_access_policy(codex_bounded_workspace_access_policy()),
                services.clone(),
            ),
        )
        .expect("observing session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("turn-{provider_id}")).expect("turn id is valid"),
                OperationContent::new("trigger provider request").expect("content is valid"),
            ),
            services,
        ))
        .expect("turn starts");
        let mut callbacks = turn
            .take_callbacks()
            .expect("observing turn exposes callback observations");
        let mut requests = callbacks
            .take_requests()
            .expect("provider request stream is available");
        let request = block_on(requests.next())
            .expect("provider request is observed")
            .expect("provider request is valid");
        let extension = match request.kind() {
            CallbackRequestKind::Extension(extension) => extension,
            CallbackRequestKind::ToolCall { .. } => panic!("expected a provider extension"),
        };
        assert_eq!(extension.namespace(), &expected_namespace);
        assert_eq!(
            request
                .provider_request_ref()
                .expect("provider request ref is retained")
                .as_provider_value(),
            provider_id
        );
        assert!(!format!("{request:?}").contains("private"));
        assert!(block_on(requests.next()).is_none());

        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        let observation = match terminal.status() {
            TerminalStatus::ProviderRequestObserved(observation) => observation,
            status => panic!("expected observed provider request, got {status:?}"),
        };
        assert_observation(observation, &request, &expected_namespace, provider_id);
        assert!(state.methods().contains(&"turn/interrupt".to_owned()));
        assert!(state.messages().iter().any(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some(provider_id)
                && message.get("error").is_some()
        }));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

fn assert_observation(
    observation: &ProviderRequestObservation,
    request: &swallowtail_runtime::CallbackRequest,
    namespace: &swallowtail_core::ExtensionNamespace,
    provider_id: &str,
) {
    assert_eq!(observation.callback_id(), request.callback_id());
    assert_eq!(observation.namespace(), namespace);
    assert_eq!(
        observation.provider_request_ref().as_provider_value(),
        provider_id
    );
}

fn message(messages: &[serde_json::Value], method: &str) -> serde_json::Value {
    messages
        .iter()
        .find(|message| message.get("method").and_then(serde_json::Value::as_str) == Some(method))
        .cloned()
        .unwrap_or_else(|| panic!("{method} was sent"))
}
