#![allow(dead_code)]

#[path = "support/discovery.rs"]
pub(crate) mod discovery_support;
#[path = "support/driver.rs"]
pub(crate) mod support;

use crate::{OPENHANDS_PACKAGE_AXIS, OPENHANDS_PACKAGE_VERSION, OpenHandsAgentServerDriver};
use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::sync::Arc;
use support::{FixtureHost, ImmediateTime};
use swallowtail_core::{
    DiscoveryStatus, ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation,
    InterfaceVersionAxis,
};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, DiscoveryCancellation, DiscoveryDriver,
    EnvironmentRef, ExecutableRef, InstalledExecutableDiscoveryRequest, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, OperationPolicy, ProcessExit, ProviderRetentionPolicy,
    RequestId, RuntimeEventKind, ScopeId, StructuredRunDriver, StructuredRunRequest,
    TerminalStatus,
};

const ACTIVITY: &str = include_str!("fixtures/openhands-agent-server-1.42.1/activity.jsonl");
const ABORT: &str = include_str!("fixtures/openhands-agent-server-1.42.1/abort.jsonl");
const LIMIT: &str = include_str!("fixtures/openhands-agent-server-1.42.1/limit.jsonl");

#[test]
fn success_run_uses_module_loopback_host_and_joins_cleanup() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([""]);
    let mut handle = block_on(scripted(ACTIVITY).start_run(
        selected.plan,
        request("success", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().map(OperationContent::as_str),
        Some("OpenHands display text.")
    );
    let mut events = handle.take_events().expect("events are available");
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
    assert!(!format!("{outcome:?}").contains("OpenHands display text"));
    let observed = host.observed();
    assert_eq!(
        observed.arguments,
        [
            "-m",
            "openhands.agent_server",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
        ]
    );
    for forbidden in [
        "0.0.0.0",
        "::",
        "[::]",
        "--cors-origins",
        "acp",
        "NeverConfirm",
    ] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden),
            "{forbidden} must not be selected"
        );
    }
    assert!(
        observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--host", "127.0.0.1"])
    );
    assert!(host.stdin().is_empty());
    assert!(host.stdin_closed());
    assert!(host.waited());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.joined());
}

#[test]
fn abort_stream_cancels_without_selecting_acp() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([""]);
    let mut handle = block_on(scripted(ABORT).start_run(
        selected.plan,
        request("abort", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn stuck_is_bounded_failure_not_end_turn() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([""]);
    let mut handle = block_on(scripted(LIMIT).start_run(
        selected.plan,
        request("limit", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(
                diagnostic.code(),
                "swallowtail.openhands.agent_server.max_iterations"
            );
        }
        other => panic!("expected provider failed, got {other:?}"),
    }
}

#[test]
fn cancellation_force_stops_and_joins() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::held_open();
    let mut handle = block_on(scripted("").start_run(
        selected.plan,
        request("cancel", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(handle.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.force_stopped());
    assert!(host.waited());
}

#[test]
fn host_deadline_times_out() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::held_open();
    let mut handle = block_on(
        OpenHandsAgentServerDriver::with_scripted_events(
            EnvironmentRef::new("openhands.fixture.isolated").expect("environment"),
            vec![serde_json::json!({"kind":"MessageEvent","source":"user"})],
        )
        .start_run(
            selected.plan,
            request("timeout", selected.resource),
            host.services_with_time(host_id, Arc::new(ImmediateTime)),
        ),
    )
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(host.force_stopped());
}

#[test]
fn malformed_and_wrong_wire_fail_before_useful_work() {
    let cases = [
        ("array", serde_json::json!([])),
        (
            "acp",
            serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
        ),
        ("unknown", serde_json::json!({"kind":"NotARealEventKind"})),
        ("v0", serde_json::json!({"action":"init","args":{}})),
        ("missing-kind", serde_json::json!({"id":"opaque"})),
    ];
    for (id, body) in cases {
        let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
        let selected = support::selection(host_id.clone());
        let host = FixtureHost::scripted([""]);
        let mut handle = block_on(
            OpenHandsAgentServerDriver::with_scripted_events(
                EnvironmentRef::new("openhands.fixture.isolated").expect("environment"),
                vec![body],
            )
            .start_run(
                selected.plan,
                request(id, selected.resource),
                host.services(host_id),
            ),
        )
        .expect("run starts");
        let outcome = block_on(
            handle
                .take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert!(
            matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)),
            "{id}"
        );
    }
}

#[test]
fn process_failure_without_terminal_status_is_provider_failed() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::with_exit([], ProcessExit::new(false, Some(1)));
    let mut handle = block_on(scripted("").start_run(
        selected.plan,
        request("process-fail", selected.resource),
        host.services(host_id),
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
}

#[test]
fn missing_deadline_fails_before_process_start() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([""]);
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let request = StructuredRunRequest::new(
        RequestId::new("openhands-agent-server-missing-deadline").expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(selected.resource);
    let result =
        block_on(scripted(ACTIVITY).start_run(selected.plan, request, host.services(host_id)));
    assert!(result.is_err());
    assert!(!host.started());
}

#[test]
fn live_http_stays_unwired_without_scripted_corpus() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let selected = support::selection(host_id.clone());
    let host = FixtureHost::scripted([""]);
    let result = block_on(OpenHandsAgentServerDriver::new(environment()).start_run(
        selected.plan,
        request("live", selected.resource),
        host.services(host_id),
    ));
    match result {
        Err(failure) => {
            assert_eq!(
                failure.diagnostic().code(),
                "swallowtail.openhands.agent_server.live_http_unwired"
            );
        }
        Ok(_) => panic!("live HTTP must stay unwired"),
    }
    assert!(!host.started());
}

#[test]
fn discovery_probes_package_metadata_not_python_version() {
    let host_id = ExecutionHostId::new("fixture.host.local").expect("host");
    let host = DiscoveryHost::new(OPENHANDS_PACKAGE_VERSION);
    let request = InstalledExecutableDiscoveryRequest::new(
        RequestId::new("openhands.fixture.probe").expect("request"),
        ScopeId::new("openhands.fixture.probe").expect("scope"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("openhands.fixture.interpreter").expect("executable"),
            InterfaceVersionAxis::new(OPENHANDS_PACKAGE_AXIS).expect("axis"),
        ),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    );
    let outcome = block_on(
        OpenHandsAgentServerDriver::new(environment())
            .discover_installed_executable(request, host.services(host_id)),
    )
    .expect("probe");
    assert_eq!(outcome.status(), DiscoveryStatus::Discovered);
    assert_eq!(
        outcome
            .installed_executable_observation()
            .expect("observation")
            .version()
            .version()
            .as_str(),
        OPENHANDS_PACKAGE_VERSION
    );
    assert_eq!(
        host.observed_process().expect("probe ran").arguments,
        [
            "-c",
            "from importlib.metadata import version; print(version('openhands-agent-server'))"
        ]
    );
}

fn scripted(body: &str) -> OpenHandsAgentServerDriver {
    OpenHandsAgentServerDriver::with_scripted_events(environment(), json_lines(body))
}

fn environment() -> EnvironmentRef {
    EnvironmentRef::new("openhands.fixture.isolated").expect("environment")
}

fn request(id: &str, resource: swallowtail_runtime::WorkingResourceRef) -> StructuredRunRequest {
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    StructuredRunRequest::new(
        RequestId::new(format!("openhands-agent-server-{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(resource)
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

fn json_lines(body: &str) -> Vec<Value> {
    body.lines()
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_str(line).expect("jsonl"))
        .collect()
}
