use futures_executor::block_on;
use super::support::{
    assert_status, execute, local_topology, prepared, profile,
};
use swallowtail_core::{
    HarnessConfigurationPosture, HarnessIsolation, ObservableActivityAvailability,
};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, OperationContent, ProcessExit,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeEventKind, StructuredRunDriver,
    TerminalStatus,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    assert_prepared_operation_evidence_matches_plan, run_one_shot_structured_cli_profile,
    run_structured_harness_native_boundary_assertions,
};

#[test]
fn prepared_route_executes_exact_argv_and_bounded_corpus_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let prepared = prepared(topology.execution_host_id().clone());
        let base_profile = profile(&prepared, topology.working_resource().clone(), "complete");
        assert_eq!(
            base_profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            base_profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            base_profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::DurableAllowed
        );
        assert_eq!(
            base_profile.request().policy().provider_recovery(),
            ProviderRecoveryPolicy::ManagedAllowed
        );
        assert_prepared_operation_evidence_matches_plan(
            base_profile.evidence().operation(),
            base_profile.plan(),
        );
        assert_eq!(
            base_profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let evidence = execute(
            &base_profile,
            topology.execution_host_id().clone(),
            include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            evidence.outcome.output().map(OperationContent::as_str),
            Some("fixture result")
        );
        assert!(evidence.events.iter().any(|event| {
            event.kind() == &RuntimeEventKind::OutputAvailable
                && event
                    .content()
                    .is_some_and(|content| content.as_str() == "fixture result")
        }));
        assert_eq!(
            evidence.request.arguments,
            [
                "--model",
                "kimi-coder",
                "--prompt",
                "private Kimi fixture prompt",
                "--output-format",
                "stream-json",
            ]
        );
        assert_eq!(
            evidence.request.environments,
            ["kimi.fixture.default-v1-environment"]
        );
        assert_eq!(
            evidence.request.working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert!(evidence.stdin_closed);
        assert!(!format!("{base_profile:?}{:?}", evidence.outcome).contains("private Kimi"));

        let tools = execute(
            &profile(&prepared, topology.working_resource().clone(), "tools"),
            topology.execution_host_id().clone(),
            include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-tools.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        assert_eq!(tools.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            tools.outcome.output().map(OperationContent::as_str),
            Some("checkingdone")
        );
        assert!(tools.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind()
                    == &swallowtail_runtime::ActivityKind::ProviderOwnedTool
        )));

        let retry = execute(
            &profile(&prepared, topology.working_resource().clone(), "retry"),
            topology.execution_host_id().clone(),
            include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-retry.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        assert_eq!(retry.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            retry.outcome.output().map(OperationContent::as_str),
            Some("final answer")
        );
        assert!(!format!("{:?}", retry.events).contains("fixture retry"));
    }
}

#[test]
fn process_failure_malformed_incomplete_cancellation_and_timeout_remain_distinct() {
    let topology = local_topology();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(&prepared, topology.working_resource().clone(), "failure");

    for (output, exit, code, provider) in [
        (
            "",
            ProcessExit::new(false, Some(1)),
            "swallowtail.kimi.headless.process_failed",
            true,
        ),
        (
            "",
            ProcessExit::new(false, Some(130)),
            "swallowtail.kimi.headless.process_interrupted",
            true,
        ),
        (
            "{\"role\":\"assistant\"}\n",
            ProcessExit::new(true, Some(0)),
            "swallowtail.kimi.headless.malformed_stream",
            false,
        ),
        (
            "",
            ProcessExit::new(true, Some(0)),
            "swallowtail.kimi.headless.incomplete_stream",
            false,
        ),
    ] {
        let evidence = execute(&profile, topology.execution_host_id().clone(), output, exit);
        assert_status(&evidence.outcome, code, provider);
    }

    let (process, state) = super::discovery_support::FakeProcessService::held_open();
    let mut run = block_on(
        profile.start_run(super::support::services(topology.execution_host_id().clone(), process)),
    )
            .expect("cancellable run starts");
    assert_eq!(
        block_on(run.cancellation().request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        block_on(run.cancellation().request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );
    let cancelled = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(cancelled.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());

    let (process, state) = super::discovery_support::FakeProcessService::held_open();
    let mut run = block_on(profile.start_run(super::support::services_with_time(
        topology.execution_host_id().clone(),
        process,
        std::sync::Arc::new(super::discovery_support::ImmediateTime),
    )))
    .expect("deadline-bound run starts");
    let timed_out = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(timed_out.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn unsupported_input_fails_before_process_start() {
    let topology = local_topology();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "unsupported",
    );
    let request =
        profile
            .request()
            .clone()
            .with_tools([swallowtail_runtime::ToolDeclaration::new(
                "consumer-tool",
                swallowtail_runtime::SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024)
                    .expect("schema is valid"),
                "application/schema+json",
                "json-schema-2020-12",
            )
            .expect("tool is valid")]);
    let (process, state) = super::discovery_support::FakeProcessService::completed("");
    let result = block_on(profile.low_level_driver().start_run(
        profile.plan().clone(),
        request,
        super::support::services(topology.execution_host_id().clone(), process),
    ));
    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn provider_neutral_one_shot_and_native_profiles_cover_headless_boundaries() {
    let one_shot = run_one_shot_structured_cli_profile();
    assert_eq!(one_shot.profile(), SyntheticProfile::OneShotStructuredCli);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::ProcessLifecycle,
    ] {
        assert!(one_shot.covers(assertion), "missing {assertion:?}");
    }
    let native = run_structured_harness_native_boundary_assertions();
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(native.covers(assertion), "missing {assertion:?}");
    }
}
