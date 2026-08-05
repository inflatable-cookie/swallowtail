#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use support::{
    CleanupEvent, FixtureHost, Scenario, allow_user_input_result, run_request,
    run_selection_for_topology,
};
use swallowtail_adapter_oh_my_pi::OhMyPiRpcDriver;
use swallowtail_core::{CancellationScope, DriverRole};
use swallowtail_runtime::{
    CallbackPayload, CallbackRequestKind, CallbackResponse, CallbackResult, CleanupOutcome,
    Deadline, EnvironmentRef, MonotonicInstant, ProviderObservation, ProviderRetentionPolicy,
    RuntimeEventKind, StructuredRunDriver, TerminalStatus,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture,
    run_structured_harness_native_boundary_assertions,
};

#[test]
fn provider_neutral_projection_pack_covers_pi_retention_and_lifecycle_truth() {
    let report = run_structured_harness_native_boundary_assertions();
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::NativeBudgetIndependent,
        ConformanceAssertion::NoTranscriptDeletionClaim,
        ConformanceAssertion::OwnedRemoteDeletionTruth,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn unsupported_structured_input_stops_before_pi_effects() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Complete);
    let selected = run_selection_for_topology(&topology);
    let driver = OhMyPiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
    );
    let request = run_request(
        "pi-unsupported-run",
        selected.resource,
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    )
    .with_maximum_output_tokens(NonZeroU64::new(8).expect("non-zero"));
    let error = block_on(driver.start_run(
        selected.plan,
        request,
        fixture.services(topology.execution_host_id().clone()),
    ))
    .err()
    .expect("unsupported run fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.oh_my_pi.rpc.unsupported_input"
    );
    assert!(!fixture.process_started());
}

#[test]
fn one_rpc_prompt_projects_as_a_private_structured_run_on_both_host_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = FixtureHost::new(Scenario::Complete);
        let selected = run_selection_for_topology(&topology);
        assert_eq!(
            selected.plan.requirements().driver_role(),
            DriverRole::StructuredRun
        );
        let services = fixture.services(topology.execution_host_id().clone());
        let driver = OhMyPiRpcDriver::new(
            EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
        );
        let request = run_request(
            "pi-structured-run",
            selected.resource,
            Deadline::at(MonotonicInstant::from_ticks(100_000)),
        );
        assert_eq!(
            request.policy().provider_retention(),
            ProviderRetentionPolicy::Prohibited
        );
        let mut handle =
            block_on(driver.start_run(selected.plan, request, services)).expect("run starts");
        assert!(handle.provider_run_ref().is_none());
        assert_eq!(
            handle.cancellation().scope(),
            CancellationScope::StructuredRun
        );
        assert!(handle.take_callbacks().is_some());
        let mut events = handle.take_events().expect("events are available");
        let terminal = handle
            .take_terminal_outcome()
            .expect("terminal outcome is available");
        let (observed, outcome) = block_on(async {
            let mut observed = Vec::new();
            while let Some(event) = events.next().await {
                observed.push(event.expect("runtime event succeeds").kind().clone());
            }
            (observed, terminal.await)
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert!(observed.iter().any(|kind| {
            matches!(
                kind,
                RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                    if usage.input_tokens() == Some(20)
                        && usage.output_tokens() == Some(10)
                        && usage.reasoning_tokens().is_none()
                        && usage.cache_read_input_tokens() == Some(4)
                        && usage.cache_write_input_tokens() == Some(2)
            )
        }));
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(
            fixture.cleanup_events(),
            [CleanupEvent::ProcessWait, CleanupEvent::ResourceRelease,]
        );
        assert_eq!(
            fixture
                .inputs()
                .iter()
                .filter(|value| value["type"] == "prompt")
                .count(),
            1
        );
        assert_eq!(
            fixture.process_executable(),
            topology.instance_target().as_host_value()
        );
    }
}

#[test]
fn structured_run_deadline_aborts_then_joins_the_rpc_process() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::Hold).with_immediate_time();
    let selected = run_selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let driver = OhMyPiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
    );
    let mut handle = block_on(driver.start_run(
        selected.plan,
        run_request(
            "pi-structured-timeout",
            selected.resource,
            Deadline::at(MonotonicInstant::from_ticks(1_001)),
        ),
        services,
    ))
    .expect("run starts");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["type"] == "abort")
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn structured_run_relays_the_qualified_pi_ui_callback() {
    let topology = ExecutionTopologyFixture::local();
    let fixture = FixtureHost::new(Scenario::PromptUi);
    let selected = run_selection_for_topology(&topology);
    let services = fixture.services(topology.execution_host_id().clone());
    let driver = OhMyPiRpcDriver::new(
        EnvironmentRef::new("pi.fixture.environment").expect("valid environment"),
    );
    let mut handle = block_on(driver.start_run(
        selected.plan,
        run_request(
            "pi-structured-callback",
            selected.resource,
            Deadline::at(MonotonicInstant::from_ticks(100_000)),
        ),
        services,
    ))
    .expect("run starts");
    let mut callbacks = handle.take_callbacks().expect("callback exchange exists");
    let mut requests = callbacks.take_requests().expect("callback stream exists");
    let callback = block_on(requests.next())
        .expect("callback arrives")
        .expect("callback is valid");
    assert!(matches!(
        callback.kind(),
        CallbackRequestKind::HarnessUserInput(_)
    ));
    block_on(callbacks.responder().respond(CallbackResponse::new(
        callback.callback_id().clone(),
        callback.turn_id().expect("callback turn").clone(),
        allow_user_input_result(&callback),
    )))
    .expect("callback response relays");
    let outcome = block_on(
        handle
            .take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}
