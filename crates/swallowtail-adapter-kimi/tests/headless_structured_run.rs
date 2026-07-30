#[allow(dead_code)]
use crate::discovery_support as support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use support::{FakeProcessService, ImmediateTime, services, services_with_time};
use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, KimiHeadlessPreparationInput, KimiHeadlessPreparationProbe,
    KimiHeadlessPreparedIntegration, KimiHeadlessRunInput, KimiModelSelection,
    prepare_kimi_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, HarnessConfigurationPosture, HarnessIsolation,
    InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CancellationAcknowledgement, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, InstalledExecutableTarget, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, ProcessExit, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RequestId, RuntimeEvent, RuntimeEventKind, ScopeId, StructuredRunDriver, TerminalOutcome,
    TerminalStatus, WorkingResourceRef,
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
    let topology = ExecutionTopologyFixture::local();
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

    let (process, state) = FakeProcessService::held_open();
    let mut run =
        block_on(profile.start_run(services(topology.execution_host_id().clone(), process)))
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

    let (process, state) = FakeProcessService::held_open();
    let mut run = block_on(profile.start_run(services_with_time(
        topology.execution_host_id().clone(),
        process,
        Arc::new(ImmediateTime),
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
    let topology = ExecutionTopologyFixture::local();
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
    let (process, state) = FakeProcessService::completed("");
    let result = block_on(profile.low_level_driver().start_run(
        profile.plan().clone(),
        request,
        services(topology.execution_host_id().clone(), process),
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

struct RunEvidence {
    events: Vec<RuntimeEvent>,
    outcome: TerminalOutcome,
    request: support::ObservedProcessRequest,
    stdin_closed: bool,
}

fn execute(
    profile: &swallowtail_adapter_kimi::KimiHeadlessPreparedRun,
    host: ExecutionHostId,
    output: &str,
    exit: ProcessExit,
) -> RunEvidence {
    let (process, state) = FakeProcessService::with_exit(output, exit);
    let mut run = block_on(profile.start_run(services(host, process))).expect("run starts");
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
    RunEvidence {
        events,
        outcome,
        request: state.request(),
        stdin_closed: state.stdin_closed(),
    }
}

fn prepared(host: ExecutionHostId) -> KimiHeadlessPreparedIntegration {
    let access = access_profile();
    let (process, state) = FakeProcessService::completed("0.29.2\n");
    let prepared = block_on(prepare_kimi_headless(
        KimiHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("kimi.headless.fixture").expect("instance is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            host.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new("kimi.fixture.executable").expect("executable is valid"),
                InterfaceVersionAxis::new(KIMI_CODE_AXIS).expect("axis is valid"),
            ),
            EnvironmentRef::new("kimi.fixture.default-v1-environment")
                .expect("environment is valid"),
            access.clone(),
            PreparedAccessEvidence::caller_asserted(access_status(&access)),
        ),
        KimiHeadlessPreparationProbe::new(
            RequestId::new("kimi-headless-probe").expect("request is valid"),
            ScopeId::new("kimi-headless-probe").expect("scope is valid"),
            Deadline::at(MonotonicInstant::from_ticks(1000)),
            DiscoveryCancellation::new(),
        ),
        services(host, process),
    ))
    .expect("Kimi headless prepares");
    assert_eq!(state.request().arguments, ["--version"]);
    assert_eq!(
        prepared.observation().version().version().as_str(),
        "0.29.2"
    );
    prepared
}

fn profile(
    prepared: &KimiHeadlessPreparedIntegration,
    resource: WorkingResourceRef,
    id: &str,
) -> swallowtail_adapter_kimi::KimiHeadlessPreparedRun {
    prepared
        .prepare_run(
            KimiHeadlessRunInput::new(
                RequestId::new(format!("kimi-headless-{id}")).expect("request is valid"),
                KimiModelSelection::new(
                    ModelRouteId::new(format!("kimi.headless.{id}")).expect("route is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ModelId::new("kimi-coder").expect("model is valid"),
                ),
                OperationContent::new("private Kimi fixture prompt").expect("content is valid"),
                resource,
                Deadline::at(MonotonicInstant::from_ticks(1000)),
            )
            .accept_managed_recovery(),
        )
        .expect("run prepares")
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("kimi.headless.membership").expect("access id is valid"),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("kimi-code-membership").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    )
    .with_credential_reference(
        CredentialRef::new("kimi.fixture.delegated-auth").expect("credential is valid"),
    )
}

fn access_status(access: &AccessProfile) -> AccessStatus {
    AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn assert_status(outcome: &TerminalOutcome, code: &str, provider: bool) {
    let diagnostic = match outcome.status() {
        TerminalStatus::ProviderFailed(diagnostic) if provider => diagnostic,
        TerminalStatus::RuntimeFailed(diagnostic) if !provider => diagnostic,
        status => panic!("unexpected status {status:?}"),
    };
    assert_eq!(diagnostic.code(), code);
}
