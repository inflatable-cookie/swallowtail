mod plan;
mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::path::{Path, PathBuf};
use support::FixtureHost;
use swallowtail_adapter_cursor::{CursorHeadlessDriver, CursorHeadlessReadMode};
use swallowtail_core::{
    HarnessConfigurationPosture, HarnessIsolation, ReasoningMode, ResourceAccess,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, EnvironmentRef, MonotonicInstant, OperationContent, OperationPolicy,
    ProcessOutputChunk, ProcessOutputStream, ProviderRetentionPolicy, RequestId,
    StructuredRunDriver, StructuredRunRequest, TerminalStatus, WorkingResourceRef,
};

const FIXTURE: &str = "tests/fixtures/cursor-agent-2026.07.01-41b2de7/headless-success.jsonl";

#[test]
fn ask_selection_dispatches_exactly_one_canonical_ask_mode() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let mut handle = block_on(ask_driver().start_run(
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::Read,
        ),
        request("ask-success"),
        host.services(host_id),
    ))
    .expect("ask run starts");
    let _events = block_on(
        handle
            .take_events()
            .expect("event stream")
            .collect::<Vec<_>>(),
    );
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let observed = host.observed();
    assert_eq!(
        observed.arguments,
        [
            "--print",
            "--output-format",
            "stream-json",
            "--model",
            "fixture-model",
            "--trust",
            "--mode",
            "ask",
        ]
    );
    assert_eq!(
        observed
            .arguments
            .iter()
            .filter(|value| *value == "--mode")
            .count(),
        1
    );
    for rejected in ["plan", "--plan", "--force", "--yolo", "--sandbox"] {
        assert!(!observed.arguments.iter().any(|value| value == rejected));
    }
    assert_eq!(observed.environments, ["cursor.fixture.environment"]);
    assert_eq!(observed.working_resource.as_deref(), Some("workspace.main"));
}

#[test]
fn explicit_plan_selection_keeps_the_exact_read_default_argv() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let mut handle = block_on(
        CursorHeadlessDriver::new(
            EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        )
        .with_read_mode(CursorHeadlessReadMode::Plan)
        .start_run(
            plan::headless_plan(
                host_id.clone(),
                "cursor.fixture.executable",
                ResourceAccess::Read,
            ),
            request("explicit-plan"),
            host.services(host_id),
        ),
    )
    .expect("explicit plan run starts");
    let _events = block_on(
        handle
            .take_events()
            .expect("event stream")
            .collect::<Vec<_>>(),
    );
    let _terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(
        host.observed()
            .arguments
            .ends_with(&["--mode".to_owned(), "plan".to_owned()])
    );
}

#[test]
fn ask_selection_rejects_read_write_authority_before_process_work() {
    let host_id = local_host();
    let host = FixtureHost::completed([stdout(&fixture())]);
    let failure = match block_on(ask_driver().start_run(
        plan::headless_plan(
            host_id.clone(),
            "cursor.fixture.executable",
            ResourceAccess::ReadWrite,
        ),
        request("ask-write"),
        host.services(host_id),
    )) {
        Ok(_) => panic!("ask must reject read-write authority"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.cursor.headless.unsupported_input"
    );
    assert!(!host.started());
}

#[test]
fn ask_selection_rejects_unqualified_releases_before_process_work() {
    for release in ["2026.08.12-abcdef1", "2026.09.01-abcdef1"] {
        let host_id = local_host();
        let host = FixtureHost::completed([stdout(&fixture())]);
        let failure = block_on(ask_driver().start_run(
            plan::headless_plan_with_release(
                host_id.clone(),
                "cursor.fixture.executable",
                ResourceAccess::Read,
                release,
            ),
            request("ask-unqualified"),
            host.services(host_id),
        ))
        .err()
        .unwrap_or_else(|| panic!("ask rejects {release}"));
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.cursor.headless.ask_mode_unqualified",
            "{release}"
        );
        assert!(!host.started(), "{release}");
    }
}

#[test]
fn ask_selection_composes_with_qualified_model_parameters() {
    for model in [
        "claude-opus-4-8[context=1m]",
        "claude-opus-4-8[effort=high]",
        "claude-opus-4-8[fast=false]",
        "claude-opus-4-8[context=1m,effort=high,fast=false]",
        "claude-opus-5[context=300k]",
        "claude-opus-5[effort=high]",
        "composer-2.5[fast=false]",
    ] {
        let reasoning = model.contains("effort=high").then_some("high");
        let host_id = local_host();
        let host = FixtureHost::completed([stdout(&fixture())]);
        let mut handle = block_on(ask_driver().start_run(
            plan::headless_plan_with_model(
                host_id.clone(),
                "cursor.fixture.executable",
                ResourceAccess::Read,
                model,
                reasoning,
            ),
            parameterized_request("ask-parameters", reasoning),
            host.services(host_id),
        ))
        .unwrap_or_else(|_| panic!("ask composes with {model}"));
        let _events = block_on(
            handle
                .take_events()
                .expect("event stream")
                .collect::<Vec<_>>(),
        );
        let _terminal = block_on(handle.take_terminal_outcome().expect("terminal future"));
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

        let arguments = host.observed().arguments;
        assert!(
            arguments.windows(2).any(|pair| pair == ["--model", model]),
            "{model}"
        );
        assert_eq!(
            arguments.iter().filter(|value| *value == "--model").count(),
            1,
            "{model}"
        );
        assert!(
            arguments.ends_with(&["--mode".to_owned(), "ask".to_owned()]),
            "{model}"
        );
    }
}

fn ask_driver() -> CursorHeadlessDriver {
    CursorHeadlessDriver::new(
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
    )
    .with_read_mode(CursorHeadlessReadMode::Ask)
}

fn request(id: &str) -> StructuredRunRequest {
    parameterized_request(id, None)
}

fn parameterized_request(id: &str, reasoning: Option<&str>) -> StructuredRunRequest {
    let mut policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    if let Some(reasoning) = reasoning {
        policy = policy.with_reasoning_mode(ReasoningMode::new(reasoning).expect("reasoning"));
    }
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id"),
        OperationContent::new("fixture-private-prompt").expect("prompt"),
        policy,
    )
    .with_working_resource(WorkingResourceRef::new("workspace.main").expect("resource"))
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000)))
}

fn fixture() -> String {
    let path: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join(FIXTURE);
    std::fs::read_to_string(path).expect("headless fixture reads")
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}

fn local_host() -> swallowtail_core::ExecutionHostId {
    swallowtail_core::ExecutionHostId::new("host.local").expect("host id")
}
