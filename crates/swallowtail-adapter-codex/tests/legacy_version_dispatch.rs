mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::app_server::{AppServerMode, ScriptedAppServer};
use support::{
    FakeProcessService, app_server_plan_for_version, bounded_workspace_plan_for_version,
    current_exec_policy, exec_policy_for_version, host_services, plan_with_version,
    working_resource,
};
use swallowtail_adapter_codex::{CodexAppServerDriver, CodexExecDriver};
use swallowtail_core::{
    ConfiguredInstanceId, DriverRole, ExecutionHostId, HarnessConfigurationPosture,
    InstanceTargetRef, SessionAccessPolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, EnvironmentRef, InteractiveSessionDriver, ModelCatalogDriver,
    ModelCatalogRequest, OpenSessionRequest, OperationContent, ProviderRetentionPolicy, RequestId,
    StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

const COMPLETED_JSONL: &str = concat!(
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"done\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn exec_runs_at_every_corpus_qualification_point() {
    for version in [
        "0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.98.0", "0.99.0", "0.100.0", "0.110.0",
        "0.121.0", "0.122.0", "0.130.0", "0.140.0", "0.144.6", "0.145.0",
    ] {
        let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
        let mut run = block_on(exec_driver().start_run(
            plan_with_version(version, [], []),
            run_request_for_version(&format!("exec-qualified-{version}"), version),
            host_services(process),
        ))
        .expect("qualified exec version starts");
        let events = block_on(run.take_events().unwrap().collect::<Vec<_>>());
        let terminal = block_on(run.take_terminal_outcome().unwrap());
        assert!(events.iter().all(Result::is_ok));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(state.waited());
        assert_exec_dispatch(version, &state.request().arguments);
    }
}

#[test]
fn app_server_catalogue_runs_at_every_corpus_qualification_point() {
    for version in [
        "0.80.0", "0.81.0", "0.84.0", "0.94.0", "0.99.0", "0.100.0", "0.107.0", "0.110.0",
        "0.120.0", "0.130.0", "0.131.0", "0.140.0", "0.144.6", "0.145.0",
    ] {
        let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
        let models = block_on(app_driver().list_models(
            app_server_plan_for_version(
                DriverRole::ModelCatalog,
                ExecutionHostId::new("host.local").unwrap(),
                ConfiguredInstanceId::new("codex.app-server.local").unwrap(),
                InstanceTargetRef::new("codex-app-server-executable").unwrap(),
                version,
                [],
                [],
            ),
            ModelCatalogRequest::new(
                RequestId::new(format!("app-catalog-qualified-{version}")).unwrap(),
            ),
            host_services(process),
        ))
        .expect("qualified app-server version lists models");
        assert_eq!(models.len(), 2);
        assert!(state.waited());
        let expected = if matches!(
            version,
            "0.80.0" | "0.81.0" | "0.84.0" | "0.94.0" | "0.99.0"
        ) {
            vec!["app-server"]
        } else {
            vec!["app-server", "--listen", "stdio://"]
        };
        assert_eq!(state.request().arguments, expected);
    }
}

#[test]
fn exec_policy_mismatch_rejects_before_process_work() {
    for (version, policy, expected_code) in [
        (
            "0.80.0",
            exec_policy_for_version("0.80.0")
                .with_provider_retention(ProviderRetentionPolicy::Prohibited),
            "swallowtail.codex.exec.unsupported_input",
        ),
        (
            "0.99.0",
            exec_policy_for_version("0.99.0")
                .with_provider_retention(ProviderRetentionPolicy::DurableAllowed),
            "swallowtail.codex.exec.unsupported_input",
        ),
        (
            "0.122.0",
            current_exec_policy()
                .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient),
            "swallowtail.codex.exec.request_plan_mismatch",
        ),
    ] {
        let (process, state) = FakeProcessService::completed("");
        let request = StructuredRunRequest::new(
            RequestId::new(format!("exec-policy-{version}")).unwrap(),
            OperationContent::new("private prompt").unwrap(),
            policy,
        )
        .with_working_resource(working_resource());
        let failure = block_on(exec_driver().start_run(
            plan_with_version(version, [], []),
            request,
            host_services(process),
        ))
        .err()
        .expect("policy mismatch fails");
        assert_eq!(failure.diagnostic().code(), expected_code);
        assert!(!state.started());
    }
}

#[test]
fn legacy_app_server_runs_only_the_stable_read_only_subset() {
    let host = ExecutionHostId::new("host.local").unwrap();
    let instance = ConfiguredInstanceId::new("codex.app-server.local").unwrap();
    let target = InstanceTargetRef::new("codex-app-server-executable").unwrap();
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let session = block_on(
        app_driver().open_session(
            app_server_plan_for_version(
                DriverRole::InteractiveSession,
                host.clone(),
                instance.clone(),
                target.clone(),
                "0.80.0",
                [],
                [],
            ),
            OpenSessionRequest::new(
                RequestId::new("legacy-read-only").unwrap(),
                working_resource(),
                None,
            )
            .with_access_policy(SessionAccessPolicy::read_only()),
            host_services(process),
        ),
    )
    .expect("legacy read-only session opens");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(state.request().arguments, ["app-server"]);
    let messages = state.messages();
    let initialize = messages
        .iter()
        .find(|message| message["method"] == "initialize")
        .expect("initialize is sent");
    assert!(
        initialize
            .pointer("/params/capabilities/experimentalApi")
            .is_none()
    );
    let start = messages
        .iter()
        .find(|message| message["method"] == "thread/start")
        .expect("thread start is sent");
    assert_eq!(start["params"]["sandbox"], "read-only");
    assert_eq!(start["params"]["approvalPolicy"], "never");
    for field in ["dynamicTools", "runtimeWorkspaceRoots", "cwd"] {
        assert!(start["params"].get(field).is_none());
    }

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let failure = block_on(
        app_driver().open_session(
            bounded_workspace_plan_for_version(host, instance, target, "0.107.0"),
            OpenSessionRequest::new(
                RequestId::new("legacy-workspace-rejected").unwrap(),
                working_resource(),
                None,
            )
            .with_access_policy(swallowtail_adapter_codex::codex_bounded_workspace_access_policy()),
            host_services(process),
        ),
    )
    .err()
    .expect("legacy workspace capability fails");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.unsupported_input"
    );
    assert!(!state.started());
}

fn exec_driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}

fn app_driver() -> CodexAppServerDriver {
    CodexAppServerDriver::new(EnvironmentRef::new("codex-saved-login").unwrap())
}

fn run_request_for_version(id: &str, version: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        exec_policy_for_version(version),
    )
    .with_working_resource(working_resource())
}

fn assert_exec_dispatch(version: &str, arguments: &[String]) {
    let contains = |value: &str| arguments.iter().any(|argument| argument == value);
    match version {
        "0.80.0" | "0.81.0" => {
            assert!(!contains("--ephemeral"));
            assert!(!contains("--ignore-user-config"));
            assert!(contains("features.web_search_request=false"));
            assert!(!contains("web_search=\"disabled\""));
        }
        "0.84.0" | "0.94.0" | "0.98.0" => {
            assert!(!contains("--ephemeral"));
            assert!(!contains("--ignore-user-config"));
            assert!(contains("web_search=\"disabled\""));
        }
        "0.99.0" | "0.100.0" | "0.110.0" | "0.121.0" => {
            assert!(contains("--ephemeral"));
            assert!(!contains("--ignore-user-config"));
            assert!(!contains("--ignore-rules"));
            assert!(contains("web_search=\"disabled\""));
        }
        _ => {
            assert!(contains("--ephemeral"));
            assert!(contains("--ignore-user-config"));
            assert!(contains("--ignore-rules"));
            assert!(contains("web_search=\"disabled\""));
        }
    }
}
