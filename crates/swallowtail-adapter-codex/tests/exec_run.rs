mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{
    FakeProcessService, host_services, host_services_with, plan, plan_with, working_resource,
};
use swallowtail_adapter_codex::CodexExecDriver;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, HostServiceKind, ReasoningMode,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CancellationAcknowledgement, Deadline,
    EnvironmentRef, ExternalNetworkPolicy, ExternalSearchPolicy, MonotonicInstant,
    OperationContent, OperationPolicy, RequestId, SchemaDocument, StructuredOutputDescriptor,
    StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};
use swallowtail_testkit::{RecordedHostCall, RecordingHostServices};

const COMPLETED_JSONL: &str = concat!(
    "{\"type\":\"thread.started\",\"thread_id\":\"private-thread\"}\n",
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.completed\",\"item\":{\"type\":\"agent_message\",\"text\":\"finished\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn structured_run_translates_request_and_normalizes_jsonl() {
    let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
    let driver = CodexExecDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    );
    let request = StructuredRunRequest::new(
        RequestId::new("request-1").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        working_resource(),
        OperationPolicy::offline(),
    );
    let mut handle =
        block_on(driver.start_run(plan(), request, host_services(process))).expect("run starts");
    let events = block_on(
        handle
            .take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    );
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let cleanup = block_on(handle.close());

    assert!(events.iter().all(Result::is_ok));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(|content| content.as_str()),
        Some("finished")
    );
    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::Clean);
    let observed = state.request();
    assert_eq!(observed.executable, "codex-executable");
    assert_eq!(
        observed.arguments,
        [
            "exec",
            "--json",
            "--ephemeral",
            "--color",
            "never",
            "--ignore-user-config",
            "--ignore-rules",
            "--skip-git-repo-check",
            "--sandbox",
            "read-only",
            "--model",
            "gpt-5.4-mini",
            "--config",
            "approval_policy=\"never\"",
            "--config",
            "shell_environment_policy.inherit=\"none\"",
            "--config",
            "hide_agent_reasoning=false",
            "--config",
            "show_raw_agent_reasoning=false",
            "--config",
            "web_search=\"disabled\"",
            "-"
        ]
    );
    assert_eq!(observed.environments, ["codex-saved-login"]);
    assert_eq!(observed.working_resource.as_deref(), Some("workspace.main"));
    assert_eq!(state.stdin(), b"private prompt");
    assert!(state.stdin_closed());
    assert!(state.waited());
    assert!(!format!("{terminal:?}").contains("finished"));
}

#[test]
fn structured_inputs_use_only_host_materializations_and_explicit_policy() {
    let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
    let recording = RecordingHostServices::default();
    let reasoning = ReasoningMode::new("low").expect("reasoning mode is valid");
    let capabilities = [
        CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("image/png")
                    .expect("media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(1024),
                CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        ),
        CapabilityRequirement::new(
            Capability::StructuredOutput,
            [CapabilityConstraint::schema_dialect("json-schema-2020-12")
                .expect("schema dialect is valid")],
        ),
        CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(reasoning.clone())],
        ),
        CapabilityRequirement::new(Capability::ExternalSearch, []),
    ];
    let optional_services = [
        HostServiceKind::Attachment,
        HostServiceKind::Schema,
        HostServiceKind::Network,
    ];
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("screenshot.main").expect("attachment reference is valid"),
        "image/png",
        AttachmentRole::Input,
    )
    .expect("attachment descriptor is valid")
    .with_known_length(512);
    let schema = StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("structured output is valid");
    let policy = OperationPolicy::new(
        ExternalNetworkPolicy::HostApproved,
        ExternalSearchPolicy::Enabled,
    )
    .expect("search policy is explicit")
    .with_reasoning_mode(reasoning);
    let request = StructuredRunRequest::new(
        RequestId::new("request-structured").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        working_resource(),
        policy,
    )
    .with_attachments([attachment])
    .with_structured_output(schema);
    let mut handle = block_on(driver().start_run(
        plan_with(capabilities, optional_services),
        request,
        host_services_with(process, &recording, optional_services),
    ))
    .expect("structured run starts");
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );

    assert_eq!(
        state.request().arguments,
        [
            "exec",
            "--json",
            "--ephemeral",
            "--color",
            "never",
            "--ignore-user-config",
            "--ignore-rules",
            "--skip-git-repo-check",
            "--sandbox",
            "read-only",
            "--model",
            "gpt-5.4-mini",
            "--config",
            "approval_policy=\"never\"",
            "--config",
            "shell_environment_policy.inherit=\"none\"",
            "--config",
            "hide_agent_reasoning=false",
            "--config",
            "show_raw_agent_reasoning=false",
            "--config",
            "web_search=\"live\"",
            "--config",
            "model_reasoning_effort=\"low\"",
            "--image",
            "/private/recording/attachment.png",
            "--output-schema",
            "/private/recording/schema.json",
            "-"
        ]
    );
    assert_eq!(
        recording.count(RecordedHostCall::AttachmentMaterializeFile),
        1
    );
    assert_eq!(recording.count(RecordedHostCall::AttachmentFileRelease), 1);
    assert_eq!(recording.count(RecordedHostCall::SchemaMaterializeFile), 1);
    assert_eq!(recording.count(RecordedHostCall::SchemaFileRelease), 1);
    assert!(!format!("{terminal:?}").contains("/private/recording"));
}

#[test]
fn unsupported_inputs_fail_before_process_side_effects() {
    let (process, state) = FakeProcessService::completed("");
    let driver = CodexExecDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    );
    let request = StructuredRunRequest::new(
        RequestId::new("request-deadline").expect("request id is valid"),
        OperationContent::new("bounded prompt").expect("content is valid"),
        working_resource(),
        OperationPolicy::offline(),
    )
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10)));

    let result = block_on(driver.start_run(plan(), request, host_services(process)));

    assert!(result.is_err());
    assert!(!state.started());

    let request = StructuredRunRequest::new(
        RequestId::new("request-search").expect("request id is valid"),
        OperationContent::new("bounded prompt").expect("content is valid"),
        working_resource(),
        OperationPolicy::new(
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        )
        .expect("search policy is explicit"),
    );
    let (process, state) = FakeProcessService::completed("");
    let result = block_on(driver.start_run(plan(), request, host_services(process)));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn cancellation_force_stops_and_joins_the_owned_process() {
    let (process, state) = FakeProcessService::held_open();
    let driver = CodexExecDriver::new(
        EnvironmentRef::new("codex-saved-login").expect("environment is valid"),
    );
    let request = StructuredRunRequest::new(
        RequestId::new("request-cancel").expect("request id is valid"),
        OperationContent::new("wait indefinitely").expect("content is valid"),
        working_resource(),
        OperationPolicy::offline(),
    );
    let mut handle =
        block_on(driver.start_run(plan(), request, host_services(process))).expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(handle.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let terminal = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    let cleanup = block_on(handle.close());

    assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
    assert_eq!(cleanup, swallowtail_runtime::CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}

fn driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").expect("environment is valid"))
}
