#![allow(dead_code)]

mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::selection::run_selection;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_claude_agent::{ClaudeAgentAcpDriver, claude_agent_acp_descriptor};
use swallowtail_core::{
    CancellationScope, ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation,
};
use swallowtail_runtime::{
    CleanupOutcome, DriverRegistration, EnvironmentRef, InteractiveSessionDriver, MonotonicInstant,
    OperationContent, OperationPolicy, ProviderObservation, ProviderRetentionPolicy, RequestId,
    RunHandle, RuntimeEvent, RuntimeEventKind, SchemaDocument, StructuredRunDriver,
    StructuredRunRequest, TerminalOutcome, TerminalStatus, ToolDeclaration,
};

#[test]
fn descriptor_registers_structured_and_interactive_roles_independently() {
    let credential =
        swallowtail_core::CredentialRef::new("claude-agent.fixture.api-key").expect("credential");
    let driver = Arc::new(ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("environment"),
        credential,
    ));
    let registration = DriverRegistration::new(claude_agent_acp_descriptor())
        .with_interactive_session(Arc::clone(&driver) as Arc<dyn InteractiveSessionDriver>)
        .expect("interactive role registers")
        .with_structured_run(driver as Arc<dyn StructuredRunDriver>)
        .expect("structured role registers");
    assert!(registration.interactive_session().is_some());
    assert!(registration.structured_run().is_some());
}

#[test]
fn one_prompt_run_preserves_version_topology_retention_and_native_close() {
    for host_name in ["fixture.run.local", "fixture.run.remote-authoritative"] {
        for version in ["0.53.0", "0.54.1", "0.60.0", "0.61.0", "0.62.0", "0.63.0"] {
            let host_id = ExecutionHostId::new(host_name).expect("host id");
            let selected = run_selection(host_id.clone(), version);
            let host = FixtureHost::new(Scenario::Success, version);
            let services = host.services(host_id);
            let mut run = block_on(driver(selected.credential).start_run(
                selected.plan,
                request(
                    &format!("run-{host_name}-{version}"),
                    selected.resource,
                    None,
                ),
                services,
            ))
            .expect("structured run starts");
            assert!(run.provider_run_ref().is_none());
            assert!(run.take_callbacks().is_none());
            assert_eq!(run.cancellation().scope(), CancellationScope::StructuredRun);
            let (events, outcome) = complete(&mut run);
            assert_eq!(outcome.status(), &TerminalStatus::Completed);
            assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
            assert_eq!(
                outcome.output().expect("output").as_str(),
                "fixture response."
            );
            assert!(events.iter().any(|event| {
                event.kind() == &swallowtail_runtime::RuntimeEventKind::OutputDelta
            }));
            assert!(events.iter().any(|event| {
                matches!(
                    event.kind(),
                    RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                        if usage.input_tokens() == Some(12)
                            && usage.output_tokens() == Some(4)
                            && usage.reasoning_tokens().is_none()
                            && usage.cache_read_input_tokens() == Some(3)
                            && usage.cache_write_input_tokens() == Some(2)
                )
            }));
            assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

            let writes = host.writes();
            let new_session = writes
                .iter()
                .find(|message| {
                    message.get("method").and_then(serde_json::Value::as_str) == Some("session/new")
                })
                .expect("session creation is written");
            assert_eq!(
                new_session["params"]["_meta"]["claudeCode"]["options"]["tools"],
                serde_json::json!(["Read", "Glob", "Grep", "Edit", "Write"])
            );
            assert!(writes.iter().any(|message| {
                message.get("method").and_then(serde_json::Value::as_str)
                    == Some("session/set_mode")
                    && message["params"]["modeId"] == "acceptEdits"
            }));
            assert_eq!(
                writes
                    .iter()
                    .filter(|message| {
                        message.get("method").and_then(serde_json::Value::as_str)
                            == Some("session/prompt")
                    })
                    .count(),
                1
            );
            assert_eq!(
                writes.iter().any(|message| {
                    message.get("method").and_then(serde_json::Value::as_str)
                        == Some("session/close")
                }),
                !matches!(version, "0.62.0" | "0.63.0")
            );
            assert!(!writes.iter().any(|message| {
                message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
            }));
            assert_eq!(host.credential_acquires(), 1);
            assert_eq!(host.credential_releases(), 1);
            assert_eq!(host.resource_releases(), 1);
        }
    }
}

#[test]
fn claude_bridge_tool_update_above_shared_frame_default_completes() {
    let host_id = ExecutionHostId::new("fixture.run.large-tool-update").expect("host id");
    let selected = run_selection(host_id.clone(), "0.63.0");
    let host = FixtureHost::new(Scenario::LargeToolUpdate, "0.63.0");
    let mut run = block_on(driver(selected.credential).start_run(
        selected.plan,
        request("run-large-tool-update", selected.resource, None),
        host.services(host_id),
    ))
    .expect("structured run starts");

    let (events, outcome) = complete(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "fixture response."
    );
    assert!(
        events
            .iter()
            .any(|event| { event.kind() == &swallowtail_runtime::RuntimeEventKind::Progress })
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn malformed_prompt_usage_fails_closed_without_emitting_usage() {
    let host_id = ExecutionHostId::new("fixture.run.malformed-usage").expect("host id");
    let selected = run_selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::MalformedUsage, "0.61.0");
    let mut run = block_on(driver(selected.credential).start_run(
        selected.plan,
        request("run-malformed-usage", selected.resource, None),
        host.services(host_id),
    ))
    .expect("structured run starts");

    let (events, outcome) = complete(&mut run);
    let TerminalStatus::RuntimeFailed(diagnostic) = outcome.status() else {
        panic!("malformed usage must fail: {:?}", outcome.status());
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude_agent.acp.malformed_response"
    );
    assert!(!events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
        )
    }));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn permission_stops_as_provider_request_without_auto_approval_or_callback() {
    let host_id = ExecutionHostId::new("fixture.run.permission").expect("host id");
    let selected = run_selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::Permission, "0.61.0");
    let mut run = block_on(driver(selected.credential).start_run(
        selected.plan,
        request("run-permission", selected.resource, None),
        host.services(host_id),
    ))
    .expect("structured run starts");
    assert!(run.take_callbacks().is_none());
    let (_, outcome) = complete(&mut run);
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let writes = host.writes();
    assert!(writes.iter().any(|message| {
        message.get("id").and_then(serde_json::Value::as_u64) == Some(900)
            && message["result"]["outcome"]["optionId"] == "reject-once"
    }));
    assert!(!writes.iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
    }));
}

#[test]
fn cancellation_and_deadline_stop_the_turn_then_join_operation_cleanup() {
    let cancel_host_id = ExecutionHostId::new("fixture.run.cancel").expect("host id");
    let selected = run_selection(cancel_host_id.clone(), "0.61.0");
    let cancel_host = FixtureHost::new(Scenario::Cancellation, "0.61.0");
    let mut run = block_on(driver(selected.credential).start_run(
        selected.plan,
        request("run-cancel", selected.resource, None),
        cancel_host.services(cancel_host_id),
    ))
    .expect("structured run starts");
    cancel_host.wait_for_write("session/prompt");
    block_on(run.cancellation().request()).expect("cancellation requests");
    let (_, outcome) = complete(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let deadline_host_id = ExecutionHostId::new("fixture.run.deadline").expect("host id");
    let selected = run_selection(deadline_host_id.clone(), "0.61.0");
    let deadline_host =
        FixtureHost::new(Scenario::Cancellation, "0.61.0").with_immediate_deadline();
    let mut run = block_on(driver(selected.credential).start_run(
        selected.plan,
        request(
            "run-deadline",
            selected.resource,
            Some(swallowtail_runtime::Deadline::at(
                MonotonicInstant::from_ticks(1),
            )),
        ),
        deadline_host.services(deadline_host_id),
    ))
    .expect("deadline run starts");
    let (_, outcome) = complete(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn unsupported_consumer_tools_reject_before_process_or_lease_effects() {
    let host_id = ExecutionHostId::new("fixture.run.unsupported").expect("host id");
    let selected = run_selection(host_id.clone(), "0.61.0");
    let host = FixtureHost::new(Scenario::Success, "0.61.0");
    let request = request("run-tools", selected.resource, None).with_tools([ToolDeclaration::new(
        "consumer_tool",
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024).expect("schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool")]);
    let error = block_on(driver(selected.credential).start_run(
        selected.plan,
        request,
        host.services(host_id),
    ))
    .err()
    .expect("tools reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude_agent.acp.unsupported"
    );
    assert_eq!(host.credential_acquires(), 0);
    assert!(host.writes().is_empty());
}

fn driver(credential: swallowtail_core::CredentialRef) -> ClaudeAgentAcpDriver {
    ClaudeAgentAcpDriver::new(
        EnvironmentRef::new("claude-agent.fixture.environment").expect("environment"),
        credential,
    )
}

fn request(
    id: &str,
    resource: swallowtail_runtime::WorkingResourceRef,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> StructuredRunRequest {
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let mut request = StructuredRunRequest::new(
        RequestId::new(id).expect("request id"),
        OperationContent::new("private fixture prompt").expect("content"),
        policy,
    )
    .with_working_resource(resource);
    if let Some(deadline) = deadline {
        request = request.with_deadline(deadline);
    }
    request
}

fn complete(run: &mut Box<dyn RunHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event succeeds"));
        }
        (collected, terminal.await)
    })
}
