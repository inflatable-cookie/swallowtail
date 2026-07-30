mod claude_code_support;

use claude_code_support::{
    FakeProcessService, ImmediateTimeService, PendingTimeService, fixture, host_services,
    preparation_input, preparation_probe,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_claude_agent::{
    ClaudeCodeModelSelection, ClaudeCodePreparedIntegration, ClaudeCodePreparedRun,
    ClaudeCodeRunProfileInput, prepare_claude_code_headless,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, HarnessConfigurationPosture, HarnessIsolation, HarnessMode,
    ModelId, ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ReasoningMode,
};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, MonotonicInstant, OperationContent,
    ProcessExit, ProviderObservation, ProviderRetentionPolicy, RequestId, RuntimeEvent,
    RuntimeEventKind, StructuredRunDriver, TerminalOutcome, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    assert_prepared_operation_evidence_matches_plan, run_one_shot_structured_cli_profile,
    run_structured_harness_native_boundary_assertions,
};

#[test]
fn prepared_route_executes_exact_local_subscription_invocation_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let prepared = prepared(topology.execution_host_id().clone());
        let profile = profile(
            &prepared,
            topology.working_resource().clone(),
            "prepared",
            Some("high"),
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::Prohibited
        );
        assert_eq!(
            profile.request().policy().harness_mode(),
            Some(HarnessMode::Plan)
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| {
                    requirement.capability() == Capability::HarnessModeSelection
                        && requirement.constraints().any(|constraint| {
                            constraint == &CapabilityConstraint::HarnessMode(HarnessMode::Plan)
                        })
                })
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let evidence = execute(
            &profile,
            topology.execution_host_id().clone(),
            &fixture("headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            evidence.outcome.output().map(OperationContent::as_str),
            Some("fixture result")
        );
        assert!(evidence.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(3)
                    && usage.cache_read_input_tokens() == Some(4)
                    && usage.cache_write_input_tokens() == Some(1)
        )));
        assert_eq!(
            evidence.request.arguments,
            [
                "-p",
                "--input-format",
                "text",
                "--output-format",
                "stream-json",
                "--verbose",
                "--no-session-persistence",
                "--model",
                "claude-opus-5",
                "--effort",
                "high",
                "--permission-mode",
                "plan",
                "--tools",
                "Read,Glob,Grep",
                "--setting-sources",
                "user,project,local",
                "--mcp-config",
                r#"{"mcpServers":{}}"#,
                "--strict-mcp-config",
            ]
        );
        for forbidden in [
            "--bare",
            "--dangerously-skip-permissions",
            "--resume",
            "--continue",
        ] {
            assert!(
                !evidence
                    .request
                    .arguments
                    .iter()
                    .any(|argument| argument == forbidden)
            );
        }
        assert_eq!(
            evidence.request.environments,
            ["claude.fixture.local-subscription-environment"]
        );
        assert_eq!(
            evidence.request.working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert_eq!(evidence.stdin, b"private Claude fixture prompt");
        assert!(evidence.stdin_closed);
        assert!(
            !format!("{:?}{:?}", evidence.events, evidence.outcome)
                .contains("private Claude fixture prompt")
        );
    }
}

#[test]
fn tool_progress_provider_failure_and_malformed_stream_remain_distinct() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "outcomes",
        None,
    );

    let tools = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture("headless-tools.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(tools.outcome.status(), &TerminalStatus::Completed);
    assert!(tools.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if activity.kind() == &swallowtail_runtime::ActivityKind::ProviderOwnedTool
    )));
    assert!(!format!("{:?}", tools.events).contains("private fixture file content"));

    let provider = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture("headless-provider-failure.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &provider.outcome,
        "swallowtail.claude_code.headless.provider_failed",
        true,
    );

    let malformed = execute(
        &profile,
        topology.execution_host_id().clone(),
        "{\"type\":\"system\",\"subtype\":\"init\",\"session_id\":\"fixture-session\",\"model\":\"wrong\",\"permissionMode\":\"plan\"}\n",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &malformed.outcome,
        "swallowtail.claude_code.headless.malformed_stream",
        false,
    );

    for invalid_pre_init in [
        concat!(
            "{\"type\":\"system\",\"subtype\":\"hook_started\",",
            "\"session_id\":\"first-session\"}\n",
            "{\"type\":\"system\",\"subtype\":\"init\",",
            "\"session_id\":\"second-session\",\"model\":\"claude-opus-5\",",
            "\"permissionMode\":\"plan\"}\n",
        ),
        "{\"type\":\"rate_limit_event\",\"session_id\":\"fixture-session\"}\n",
    ] {
        let rejected = execute(
            &profile,
            topology.execution_host_id().clone(),
            invalid_pre_init,
            ProcessExit::new(true, Some(0)),
        );
        assert_status(
            &rejected.outcome,
            "swallowtail.claude_code.headless.malformed_stream",
            false,
        );
    }

    let incomplete = execute(
        &profile,
        topology.execution_host_id().clone(),
        "",
        ProcessExit::new(true, Some(0)),
    );
    assert_status(
        &incomplete.outcome,
        "swallowtail.claude_code.headless.incomplete_stream",
        false,
    );

    let failed = execute(
        &profile,
        topology.execution_host_id().clone(),
        "",
        ProcessExit::new(false, Some(1)),
    );
    assert_status(
        &failed.outcome,
        "swallowtail.claude_code.headless.process_failed",
        true,
    );
}

#[test]
fn unsupported_input_cancellation_and_timeout_are_bounded_before_cleanup() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "bounded",
        None,
    );

    let request =
        profile
            .request()
            .clone()
            .with_tools([swallowtail_runtime::ToolDeclaration::new(
                "consumer-tool",
                swallowtail_runtime::SchemaDocument::inline(
                    br#"{"type":"object"}"#.to_vec(),
                    1_024,
                )
                .expect("schema is valid"),
                "application/schema+json",
                "json-schema-2020-12",
            )
            .expect("tool is valid")]);
    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let result = block_on(profile.low_level_driver().start_run(
        profile.plan().clone(),
        request,
        services,
    ));
    assert!(result.is_err());
    assert!(!state.started());

    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let mut run = block_on(profile.start_run(services)).expect("cancellable run starts");
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
    assert!(task.joined());

    let (process, state) = FakeProcessService::held_open();
    let (services, task) = host_services(
        topology.execution_host_id().clone(),
        process,
        Arc::new(ImmediateTimeService),
    );
    let mut run = block_on(profile.start_run(services)).expect("deadline-bound run starts");
    let timed_out = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(timed_out.status(), &TerminalStatus::TimedOut);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.force_stopped());
    assert!(state.waited());
    assert!(task.joined());
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

struct RunEvidence {
    events: Vec<RuntimeEvent>,
    outcome: TerminalOutcome,
    request: claude_code_support::ObservedProcessRequest,
    stdin: Vec<u8>,
    stdin_closed: bool,
}

fn execute(
    profile: &ClaudeCodePreparedRun,
    host: swallowtail_core::ExecutionHostId,
    output: &str,
    exit: ProcessExit,
) -> RunEvidence {
    let (process, state) = FakeProcessService::with_exit(output, exit);
    let (services, task) = host_services(host, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("run starts");
    assert!(run.provider_run_ref().is_none());
    let events = block_on(
        run.take_events()
            .expect("event stream is available")
            .collect::<Vec<_>>(),
    )
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("events are valid");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert!(state.waited());
    assert!(task.joined());
    RunEvidence {
        events,
        outcome,
        request: state.request(),
        stdin: state.stdin(),
        stdin_closed: state.stdin_closed(),
    }
}

fn prepared(host: swallowtail_core::ExecutionHostId) -> ClaudeCodePreparedIntegration {
    let (process, state) = FakeProcessService::completed("2.1.220 (Claude Code)\n");
    let (services, task) = host_services(host.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_claude_code_headless(
        preparation_input(host),
        preparation_probe(),
        services,
    ))
    .expect("Claude Code headless prepares");
    assert_eq!(state.request().arguments, ["--version"]);
    assert!(state.waited());
    assert!(task.joined());
    assert_eq!(
        prepared.observation().version().version().as_str(),
        "2.1.220"
    );
    prepared
}

fn profile(
    prepared: &ClaudeCodePreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
    reasoning: Option<&str>,
) -> ClaudeCodePreparedRun {
    let input = ClaudeCodeRunProfileInput::new(
        RequestId::new(format!("claude-code-{id}")).expect("request is valid"),
        ClaudeCodeModelSelection::new(
            ModelRouteId::new(format!("claude-code.{id}")).expect("route is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            ModelId::new("claude-opus-5").expect("model is valid"),
        ),
        OperationContent::new("private Claude fixture prompt").expect("content is valid"),
        resource,
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    let input = match reasoning {
        Some(reasoning) => input
            .with_reasoning_mode(ReasoningMode::new(reasoning).expect("reasoning mode is valid")),
        None => input,
    };
    prepared
        .prepare_run(input)
        .expect("Claude Code run prepares")
}

fn assert_status(outcome: &TerminalOutcome, code: &str, provider: bool) {
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected status {status:?}"),
    };
    assert_eq!(diagnostic.code(), code);
}
