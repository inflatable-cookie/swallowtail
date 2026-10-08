use super::*;
use futures_util::StreamExt;
use swallowtail_runtime::{
    CancellationAcknowledgement, RuntimeEventKind, TerminalStatus, WorkingResourceRef,
};

fn exec_input(
    resource: &str,
    network: ExternalNetworkPolicy,
    search: ExternalSearchPolicy,
) -> CodexExecProfileInput {
    CodexExecProfileInput::new(
        RequestId::new(format!("policy-{resource}")).unwrap(),
        OperationContent::new("inspect the selected read-only workspace").unwrap(),
        model(),
        WorkingResourceRef::new(resource).unwrap(),
        network,
        search,
    )
}

fn assert_fixed_exec_policy(arguments: &[String]) {
    for required in [
        "exec",
        "--json",
        "--ephemeral",
        "--ignore-user-config",
        "--ignore-rules",
        "--skip-git-repo-check",
        "--sandbox",
        "read-only",
        "approval_policy=\"never\"",
        "shell_environment_policy.inherit=\"none\"",
    ] {
        assert!(
            arguments.iter().any(|argument| argument == required),
            "missing {required}"
        );
    }
    let all = arguments.join(" ").to_ascii_lowercase();
    for forbidden in [
        "ignore_managed_requirements",
        "--trust",
        "--dangerously-bypass-approvals-and-sandbox",
        "--yolo",
        "--full-auto",
    ] {
        assert!(
            !all.contains(forbidden),
            "unexpected policy override {forbidden}"
        );
    }
}

#[test]
fn explicit_projectless_and_marked_resources_use_the_selected_prepared_exec() {
    for resource in ["workspace.projectless", "workspace.marked"] {
        let recording = RecordingHostServices::default();
        let integration = prepared(
            CodexPreparedDriver::StructuredExec,
            "0.161.0",
            &recording,
            true,
        );
        let profile = integration
            .prepare_structured_exec(exec_input(
                resource,
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            ))
            .expect("current exec prepares without restoring persisted project trust");
        let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
        let mut run = block_on(profile.start_run(host_services_with(
            process,
            &recording,
            [HostServiceKind::WorkingResource],
        )))
        .expect("selected prepared exec starts");
        let terminal = block_on(run.take_terminal_outcome().unwrap());
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

        let request = state.request();
        assert_eq!(request.working_resource.as_deref(), Some(resource));
        assert_eq!(request.environments, ["saved-login"]);
        assert_fixed_exec_policy(&request.arguments);
        assert!(
            !request
                .arguments
                .iter()
                .any(|argument| argument == "--trust")
        );
        assert!(state.stdin_closed());
        assert!(state.waited());
    }
}

#[test]
fn managed_network_allowed_and_refreshed_denial_keep_bounded_exec_projection() {
    let recording = RecordingHostServices::default();
    let integration = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.161.0",
        &recording,
        true,
    );

    let allowed_jsonl = concat!(
        r#"{"type":"item.completed","item":{"id":"search-1","type":"web_search","query":"official manual","action":{"type":"search","query":"official manual"},"results":[{"title":"private result title","url":"https://private.invalid/path","snippet":"private result snippet"}]}}"#,
        "\n",
        r#"{"type":"turn.completed","usage":{"input_tokens":3,"output_tokens":2}}"#,
        "\n"
    );
    let profile = integration
        .prepare_structured_exec(exec_input(
            "workspace.managed-network",
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        ))
        .expect("host-approved search prepares");
    let (process, state) = FakeProcessService::completed(allowed_jsonl);
    let mut run = block_on(profile.start_run(host_services_with(
        process,
        &recording,
        [HostServiceKind::Network, HostServiceKind::WorkingResource],
    )))
    .expect("prepared exec selects the provider-managed network path");
    let events = block_on(run.take_events().unwrap().collect::<Vec<_>>());
    let terminal = block_on(run.take_terminal_outcome().unwrap());
    assert!(events.iter().all(Result::is_ok));
    assert!(events.iter().any(|event| {
        event.as_ref().is_ok_and(|event| {
            event.kind() == &RuntimeEventKind::ExternalSearchProgress
                && event
                    .content()
                    .is_some_and(|content| content.as_str() == "official manual")
        })
    }));
    let projected = format!("{events:?}");
    assert!(!projected.contains("private result title"));
    assert!(!projected.contains("private result snippet"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let request = state.request();
    assert!(
        request
            .arguments
            .iter()
            .any(|argument| argument == "web_search=\"live\"")
    );
    assert_fixed_exec_policy(&request.arguments);
    assert!(state.stdin_closed());
    assert!(state.waited());

    let denied_profile = integration
        .prepare_structured_exec(exec_input(
            "workspace.managed-network-refreshed",
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        ))
        .expect("managed policy remains an upstream authority after refresh");
    let denied_jsonl = concat!(
        r#"{"type":"item.completed","item":{"id":"search-2","type":"web_search","query":"restricted destination"}}"#,
        "\n",
        r#"{"type":"turn.failed","message":"private host rule, path, and refresh details"}"#,
        "\n"
    );
    let (process, state) = FakeProcessService::completed(denied_jsonl);
    let mut run = block_on(denied_profile.start_run(host_services_with(
        process,
        &recording,
        [HostServiceKind::Network, HostServiceKind::WorkingResource],
    )))
    .expect("provider-managed denial stays on the selected exec path");
    let events = block_on(run.take_events().unwrap().collect::<Vec<_>>());
    let terminal = block_on(run.take_terminal_outcome().unwrap());
    let TerminalStatus::ProviderFailed(diagnostic) = terminal.status() else {
        panic!("a provider denial is not a successful terminal");
    };
    assert_eq!(diagnostic.code(), "swallowtail.codex.exec.provider_failed");
    assert_eq!(
        diagnostic.message(),
        "Codex exec reported a provider failure"
    );
    assert!(!format!("{events:?}").contains("private host rule"));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_fixed_exec_policy(&state.request().arguments);
    assert!(state.stdin_closed());
    assert!(state.waited());
}

#[test]
fn selected_prepared_exec_cancellation_reaches_terminal_and_joins_process() {
    let recording = RecordingHostServices::default();
    let integration = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.161.0",
        &recording,
        true,
    );
    let profile = integration
        .prepare_structured_exec(exec_input(
            "workspace.projectless-cancel",
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .expect("current exec prepares");
    let (process, state) = FakeProcessService::held_open();
    let mut run = block_on(profile.start_run(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("prepared run starts");
    assert_eq!(
        block_on(run.cancellation().request()).unwrap(),
        CancellationAcknowledgement::Requested
    );
    let terminal = block_on(run.take_terminal_outcome().unwrap());
    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}
