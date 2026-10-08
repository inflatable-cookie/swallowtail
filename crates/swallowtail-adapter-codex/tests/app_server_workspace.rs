use crate::support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::collections::BTreeSet;
use std::sync::Arc;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    app_server_plan, app_server_plan_for_version, app_server_session_agreement,
    bounded_workspace_plan, bounded_workspace_plan_for, bounded_workspace_plan_for_version,
    host_services, host_services_for, host_services_with, host_services_with_for, working_resource,
};
use swallowtail_adapter_codex::{
    CodexAppServerDriver, codex_approval_request_extension, codex_bounded_workspace_access_policy,
    codex_user_input_request_extension,
};
use swallowtail_core::{
    ConfiguredInstanceId, DriverRole, ExecutionHostId, HostServiceKind, InstanceTargetRef,
    SessionRef,
};
use swallowtail_runtime::{
    BoxFuture, CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult,
    CleanupOutcome, EnvironmentRef, HostServices, InteractiveSessionDriver,
    MaterializedResourceRef, ModelCatalogDriver, ModelCatalogRequest, OpenSessionRequest,
    OperationContent, ProviderRequestObservation, RequestId, ResourceAccess, ResourceLease,
    ResourceRepresentation, ResumeSessionRequest, RuntimeFailure, RuntimeTurnId, ScopeId,
    SessionAccessPolicy, SessionResumeBinding, TerminalStatus, TurnRequest, WorkingResourceRef,
    WorkingResourceService,
};
use swallowtail_testkit::{
    ExecutionTopologyFixture, RecordedHostCall, RecordingHostServices, RecordingOutcome,
};

fn driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    )
}

struct FixtureWorkingResource(&'static str);

impl WorkingResourceService for FixtureWorkingResource {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let root = self.0;
        Box::pin(async move {
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        MaterializedResourceRef::new(root)
                            .expect("fixture root is a valid materialized resource"),
                    ),
            )
        })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.temporary_resource_unsupported",
                "Fixture does not create temporary resources",
            )))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

fn host_services_with_root(process: Arc<ScriptedAppServer>, root: &'static str) -> HostServices {
    host_services_for(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        process,
    )
    .with_working_resource(Arc::new(FixtureWorkingResource(root)))
}

#[test]
fn bounded_workspace_maps_one_host_authorized_root_and_denies_network() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let mut session = block_on(driver().open_session(
        bounded_workspace_plan_for_version(
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            "0.161.0",
        ),
        OpenSessionRequest::new(
            RequestId::new("workspace-session").expect("request id is valid"),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        services.clone(),
    ))
    .expect("bounded workspace session opens");

    let thread = message(&state.messages(), "thread/start");
    assert_eq!(
        thread["params"]
            .as_object()
            .expect("thread start parameters are an object")
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "approvalPolicy",
            "cwd",
            "model",
            "runtimeWorkspaceRoots",
            "sandbox",
        ])
    );
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

    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("workspace-turn").expect("turn id is valid"),
                OperationContent::new("write an ordinary file inside the approved workspace")
                    .expect("content is valid"),
            ),
            services.clone(),
        ),
    )
    .expect("workspace turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);

    let turn_start = message(&state.messages(), "turn/start");
    let sandbox = &turn_start["params"]["sandboxPolicy"];
    assert_eq!(
        sandbox
            .as_object()
            .expect("workspace sandbox policy is an object")
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "excludeSlashTmp",
            "excludeTmpdirEnvVar",
            "networkAccess",
            "type",
            "writableRoots",
        ])
    );
    assert_eq!(sandbox["type"], "workspaceWrite");
    assert_eq!(sandbox["writableRoots"].as_array().map(Vec::len), Some(1));
    assert_eq!(sandbox["writableRoots"][0], "/private/recording/workspace");
    assert_eq!(sandbox["networkAccess"], false);
    assert_eq!(sandbox["excludeSlashTmp"], true);
    assert_eq!(sandbox["excludeTmpdirEnvVar"], true);
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "thread/start", "turn/start"]
    );
    assert_eq!(
        state.request().arguments,
        ["app-server", "--listen", "stdio://"]
    );
    assert_eq!(state.request().environments, ["codex-saved-login"]);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn failed_protected_aws_path_write_is_projected_as_provider_failed() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::FailedTurn);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let mut session = block_on(driver().open_session(
        bounded_workspace_plan_for_version(
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            "0.159.0",
        ),
        OpenSessionRequest::new(
            RequestId::new("workspace-failed-turn").expect("request id is valid"),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        services.clone(),
    ))
    .expect("bounded workspace session opens");

    let thread = message(&state.messages(), "thread/start");
    assert_eq!(thread["params"]["sandbox"], "workspace-write");
    assert_eq!(
        thread["params"]["runtimeWorkspaceRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );

    let mut turn = block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("workspace-failed-turn").expect("turn id is valid"),
                OperationContent::new("write to the protected workspace .aws/config path")
                    .expect("content is valid"),
            ),
            services.clone(),
        ),
    )
    .expect("workspace turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let TerminalStatus::ProviderFailed(diagnostic) = terminal.status() else {
        panic!("provider failed status must not be reported as success");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.codex.app_server.turn_failed"
    );
    assert_eq!(diagnostic.message(), "Codex app-server turn failed");

    let sandbox = &message(&state.messages(), "turn/start")["params"]["sandboxPolicy"];
    assert_eq!(
        sandbox
            .as_object()
            .expect("workspace sandbox policy is an object")
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "excludeSlashTmp",
            "excludeTmpdirEnvVar",
            "networkAccess",
            "type",
            "writableRoots",
        ])
    );
    assert_eq!(sandbox["type"], "workspaceWrite");
    assert_eq!(
        sandbox["writableRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );
    assert_eq!(sandbox["networkAccess"], false);
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn managed_model_list_refusal_is_generic_and_joins_without_retry() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::RejectModelList);
    let failure = block_on(driver().list_models(
        app_server_plan_for_version(
            DriverRole::ModelCatalog,
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            "0.156.0",
            [],
            [],
        ),
        ModelCatalogRequest::new(RequestId::new("managed-model-list").unwrap()),
        host_services(process),
    ))
    .expect_err("managed provider requirements can refuse model discovery");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.request_failed"
    );
    assert_eq!(
        failure.diagnostic().message(),
        "Codex app-server rejected a request"
    );
    assert_eq!(state.methods(), ["initialize", "initialized", "model/list"]);
    assert!(state.waited());
}

#[test]
fn managed_turn_start_refusal_keeps_the_fresh_workspace_bound_and_generic() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::RejectTurnStart);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let mut session = block_on(driver().open_session(
        bounded_workspace_plan_for_version(
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            "0.156.0",
        ),
        OpenSessionRequest::new(
            RequestId::new("managed-fresh-thread").unwrap(),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        services.clone(),
    ))
    .expect("bounded app-server session opens");

    let thread = message(&state.messages(), "thread/start");
    assert_eq!(
        thread["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "approvalPolicy",
            "cwd",
            "model",
            "runtimeWorkspaceRoots",
            "sandbox",
        ])
    );
    assert_eq!(
        thread["params"]["runtimeWorkspaceRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );

    let refusal = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("managed-fresh-turn").unwrap(),
            OperationContent::new("continue within the approved root").unwrap(),
        ),
        services.clone(),
    ));
    let error = match refusal {
        Ok(_) => panic!("managed provider requirements must refuse this turn"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.request_failed"
    );
    assert_eq!(
        error.diagnostic().message(),
        "Codex app-server rejected a request"
    );

    let turn_start = message(&state.messages(), "turn/start");
    assert_eq!(
        turn_start["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["input", "sandboxPolicy", "threadId"])
    );
    assert_eq!(
        turn_start["params"]["sandboxPolicy"]["writableRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );
    assert_eq!(
        turn_start["params"]["sandboxPolicy"]["networkAccess"],
        false
    );
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "thread/start", "turn/start"]
    );
    assert_eq!(
        state.request().arguments,
        ["app-server", "--listen", "stdio://"]
    );
    assert_eq!(state.request().environments, ["codex-saved-login"]);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn managed_turn_start_refusal_on_a_retained_thread_keeps_the_same_root() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::RejectTurnStart);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let plan = bounded_workspace_plan_for_version(
        ExecutionHostId::new("host.local").unwrap(),
        ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
        InstanceTargetRef::new("codex-app-server-executable").unwrap(),
        "0.156.0",
    );
    let working_resource = working_resource();
    let access = codex_bounded_workspace_access_policy();
    let binding = SessionResumeBinding::new(
        SessionRef::new("thread-provider-existing").unwrap(),
        plan.instance_id().clone(),
        plan.execution_host_id().clone(),
        plan.model_route_id().unwrap().clone(),
        plan.model_id().unwrap().clone(),
        working_resource.clone(),
        access.clone(),
    );
    let mut session = block_on(driver().resume_session(
        plan,
        ResumeSessionRequest::new(
            RequestId::new("managed-retained-thread").unwrap(),
            binding,
            working_resource,
            None,
            app_server_session_agreement(access),
        ),
        services.clone(),
    ))
    .expect("bounded retained thread opens");

    let resume = message(&state.messages(), "thread/resume");
    assert_eq!(
        resume["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "approvalPolicy",
            "cwd",
            "excludeTurns",
            "model",
            "runtimeWorkspaceRoots",
            "sandbox",
            "threadId",
        ])
    );
    assert_eq!(resume["params"]["threadId"], "thread-provider-existing");
    assert_eq!(
        resume["params"]["runtimeWorkspaceRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );

    let refusal = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("managed-retained-turn").unwrap(),
            OperationContent::new("continue the retained thread").unwrap(),
        ),
        services.clone(),
    ));
    let error = match refusal {
        Ok(_) => panic!("managed provider requirements must refuse the retained turn"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.codex.app_server.request_failed"
    );
    assert_eq!(
        error.diagnostic().message(),
        "Codex app-server rejected a request"
    );
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "thread/resume", "turn/start"]
    );
    assert_eq!(
        state.request().arguments,
        ["app-server", "--listen", "stdio://"]
    );
    assert_eq!(state.request().environments, ["codex-saved-login"]);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn failed_home_relative_permission_path_keeps_the_root_and_projects_a_generic_failure() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::FailedTurn);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let mut session = block_on(driver().open_session(
        bounded_workspace_plan_for_version(
            ExecutionHostId::new("host.local").unwrap(),
            ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
            InstanceTargetRef::new("codex-app-server-executable").unwrap(),
            "0.156.0",
        ),
        OpenSessionRequest::new(
            RequestId::new("managed-permission-denial").unwrap(),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        services.clone(),
    ))
    .expect("bounded app-server session opens");

    let thread = message(&state.messages(), "thread/start");
    assert_eq!(
        thread["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "approvalPolicy",
            "cwd",
            "model",
            "runtimeWorkspaceRoots",
            "sandbox",
        ])
    );
    assert_eq!(thread["params"]["sandbox"], "workspace-write");
    assert_eq!(thread["params"]["approvalPolicy"], "never");
    assert_eq!(
        thread["params"]["runtimeWorkspaceRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );

    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("managed-permission-denial-turn").unwrap(),
            OperationContent::new("attempt a path covered by a home-relative deny rule").unwrap(),
        ),
        services.clone(),
    ))
    .expect("turn starts before provider policy is applied");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let TerminalStatus::ProviderFailed(diagnostic) = terminal.status() else {
        panic!("a policy-denied write must remain a failed provider turn");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.codex.app_server.turn_failed"
    );
    assert_eq!(diagnostic.message(), "Codex app-server turn failed");

    let start = message(&state.messages(), "turn/start");
    assert_eq!(
        start["params"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from(["input", "sandboxPolicy", "threadId"])
    );
    assert_eq!(
        start["params"]["sandboxPolicy"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([
            "excludeSlashTmp",
            "excludeTmpdirEnvVar",
            "networkAccess",
            "type",
            "writableRoots",
        ])
    );
    assert_eq!(
        start["params"]["sandboxPolicy"]["writableRoots"],
        serde_json::json!(["/private/recording/workspace"])
    );
    assert_eq!(start["params"]["sandboxPolicy"]["networkAccess"], false);
    assert_eq!(
        state.methods(),
        ["initialize", "initialized", "thread/start", "turn/start"]
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
    assert!(state.waited());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 1);
}

#[test]
fn projectless_marked_and_alias_roots_do_not_expand_the_selected_sandbox() {
    let scenarios = [
        (
            "projectless-with-saved-trust",
            "/private/recording/projectless-with-saved-trust",
            "0.156.0",
        ),
        (
            "marked-project",
            "/private/recording/marked-project",
            "0.156.0",
        ),
        (
            "path-alias-root",
            "/private/recording/path-alias-root",
            "0.158.0",
        ),
        (
            "alias-with-linked-gitdir",
            "/private/recording/alias-with-linked-gitdir",
            "0.158.0",
        ),
    ];

    for (scenario, root, version) in scenarios {
        // These synthetic roots exercise only the adapter request boundary. The frozen
        // Codex source regressions bind project-marker and filesystem-policy behavior.
        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let services = host_services_with_root(Arc::clone(&process), root);
        let mut session = block_on(
            driver().open_session(
                bounded_workspace_plan_for_version(
                    ExecutionHostId::new("host.local").unwrap(),
                    ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
                    InstanceTargetRef::new("codex-app-server-executable").unwrap(),
                    version,
                ),
                OpenSessionRequest::new(
                    RequestId::new(format!("workspace-{scenario}")).expect("request id is valid"),
                    WorkingResourceRef::new(format!("workspace.{scenario}"))
                        .expect("resource reference is valid"),
                    None,
                    app_server_session_agreement(codex_bounded_workspace_access_policy()),
                ),
                services.clone(),
            ),
        )
        .expect("bounded workspace session opens for each fixture root");

        let thread = message(&state.messages(), "thread/start");
        assert_eq!(
            thread["params"]
                .as_object()
                .expect("thread start parameters are an object")
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                "approvalPolicy",
                "cwd",
                "model",
                "runtimeWorkspaceRoots",
                "sandbox",
            ]),
            "unexpected thread/start field for {scenario}"
        );
        assert_eq!(thread["params"]["sandbox"], "workspace-write");
        assert_eq!(thread["params"]["approvalPolicy"], "never");
        assert_eq!(thread["params"]["cwd"], root);
        assert_eq!(
            thread["params"]["runtimeWorkspaceRoots"],
            serde_json::json!([root])
        );

        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("turn-{scenario}")).expect("turn id is valid"),
                    OperationContent::new("perform ordinary work within this approved root")
                        .expect("content is valid"),
                ),
                services.clone(),
            ),
        )
        .expect("bounded workspace turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);

        let sandbox = &message(&state.messages(), "turn/start")["params"]["sandboxPolicy"];
        assert_eq!(
            sandbox
                .as_object()
                .expect("workspace sandbox policy is an object")
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                "excludeSlashTmp",
                "excludeTmpdirEnvVar",
                "networkAccess",
                "type",
                "writableRoots",
            ]),
            "unexpected turn/start field for {scenario}"
        );
        assert_eq!(sandbox["type"], "workspaceWrite");
        assert_eq!(sandbox["writableRoots"], serde_json::json!([root]));
        assert_eq!(sandbox["networkAccess"], false);
        assert_eq!(sandbox["excludeSlashTmp"], true);
        assert_eq!(sandbox["excludeTmpdirEnvVar"], true);

        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(support::close_session(session, services)),
            CleanupOutcome::Clean
        );
    }
}

#[test]
fn read_only_session_request_shape_remains_unchanged() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(driver().open_session(
        app_server_plan(DriverRole::InteractiveSession),
        OpenSessionRequest::new(
            RequestId::new("read-only-session").expect("request id is valid"),
            working_resource(),
            None,
            app_server_session_agreement(SessionAccessPolicy::read_only()),
        ),
        services.clone(),
    ))
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
        services.clone(),
    ))
    .expect("read-only turn starts");
    assert!(
        message(&state.messages(), "turn/start")["params"]
            .get("sandboxPolicy")
            .is_none()
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );
}

#[test]
fn writable_request_without_host_resource_service_fails_before_process_start() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let result = block_on(driver().open_session(
        bounded_workspace_plan(),
        OpenSessionRequest::new(
            RequestId::new("workspace-service-missing").expect("request id is valid"),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        host_services(process),
    ));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn pre_workspace_root_segment_rejects_bounded_access_before_host_or_process_work() {
    let recording = RecordingHostServices::default();
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
    let plan = bounded_workspace_plan_for_version(
        ExecutionHostId::new("host.local").unwrap(),
        ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
        InstanceTargetRef::new("codex-app-server-executable").unwrap(),
        "0.130.0",
    );
    let result = block_on(driver().open_session(
        plan,
        OpenSessionRequest::new(
            RequestId::new("workspace-before-milestone").unwrap(),
            working_resource(),
            None,
            app_server_session_agreement(codex_bounded_workspace_access_policy()),
        ),
        services,
    ));

    assert!(result.is_err());
    assert!(!state.started());
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 0);
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
                    app_server_session_agreement(codex_bounded_workspace_access_policy()),
                ),
                services.clone(),
            ),
        )
        .expect("bounded session opens on its authoritative host");

        assert_eq!(
            state.request().working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
        assert_eq!(
            block_on(support::close_session(session, services)),
            CleanupOutcome::Clean
        );
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
        let (process, state) = ScriptedAppServer::gate_enforcing(mode);
        let services = host_services_with(process, &recording, [HostServiceKind::WorkingResource]);
        let mut session = block_on(driver().open_session(
            bounded_workspace_plan(),
            OpenSessionRequest::new(
                RequestId::new(format!("observed-{provider_id}")).expect("request id is valid"),
                working_resource(),
                None,
                app_server_session_agreement(codex_bounded_workspace_access_policy()),
            ),
            services.clone(),
        ))
        .expect("observing session opens");
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("turn-{provider_id}")).expect("turn id is valid"),
                OperationContent::new("trigger provider request").expect("content is valid"),
            ),
            services.clone(),
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
            CallbackRequestKind::HarnessUserInput(_) => {
                panic!("expected a provider extension")
            }
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
        let refusal = block_on(
            callbacks.responder().respond(CallbackResponse::new(
                request.callback_id().clone(),
                request
                    .turn_id()
                    .expect("observed callback keeps its turn")
                    .clone(),
                CallbackResult::Success(
                    CallbackPayload::new(b"approved".to_vec(), 16).expect("payload is bounded"),
                ),
            )),
        )
        .expect_err("an observed provider request cannot be answered");
        assert_eq!(
            refusal.diagnostic().code(),
            "swallowtail.codex.app_server.callback_closed"
        );
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(support::close_session(session, services)),
            CleanupOutcome::Clean
        );
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
