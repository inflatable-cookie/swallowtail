mod headless_support;

use futures_executor::block_on;
use headless_support::{
    FakeProcessService, PendingTimeService, ScriptedProcessService, assert_redacted,
    assert_status_code, cancelled, completed, driver, fixture, host_services_for, plan_for,
    request_for, timed_out,
};
use std::sync::Arc;
use swallowtail_adapter_gemini::{
    GeminiCliPreparedDriver, GeminiCliPreparedIntegration, GeminiHeadlessModelSelection,
    GeminiHeadlessRunProfileInput, GeminiHeadlessSessionManagementInput, prepare_gemini_cli,
};
use swallowtail_core::{
    Capability, DriverRole, HarnessConfigurationPosture, HarnessIsolation, ModelId, ModelRouteId,
    ModelRouteRevision, ObservableActivityAvailability, OwnedRemoteResourceKind, ProviderId,
    ProviderSessionEffectTruth,
};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProcessExit,
    ProviderObservation, ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId,
    RuntimeEventKind, StructuredRunDriver, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    assert_prepared_operation_evidence_matches_plan, run_one_shot_structured_cli_profile,
    run_structured_harness_native_boundary_assertions,
};

#[test]
fn production_route_preserves_cli_and_host_truth_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let id = format!("gemini-success-{}", topology.execution_host_id().as_str());
        let evidence = completed(
            &topology,
            &fixture("success.jsonl", &id),
            ProcessExit::new(true, Some(0)),
            &id,
        );
        assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            evidence.outcome.output().map(|value| value.as_str()),
            Some("fixture answer")
        );
        assert!(evidence.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(2)
                    && usage.cache_read_input_tokens() == Some(3)
                    && usage.cache_miss_input_tokens() == Some(9)
        )));
        assert_redacted(&evidence.events, &evidence.outcome);

        let unknown = completed(
            &topology,
            &fixture("unknown-event.jsonl", "gemini-unknown"),
            ProcessExit::new(true, Some(0)),
            "gemini-unknown",
        );
        assert_eq!(unknown.outcome.status(), &TerminalStatus::Completed);
        assert!(unknown.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(activity.kind(), swallowtail_runtime::ActivityKind::Unknown(_))
        )));
        assert_redacted(&unknown.events, &unknown.outcome);

        let provider = completed(
            &topology,
            &fixture("provider-failure.jsonl", "gemini-provider-failure"),
            ProcessExit::new(true, Some(0)),
            "gemini-provider-failure",
        );
        assert_status_code(
            &provider.outcome,
            "swallowtail.gemini.headless.provider_failed",
            true,
        );

        let malformed = completed(
            &topology,
            &fixture("malformed.jsonl", "gemini-malformed"),
            ProcessExit::new(true, Some(0)),
            "gemini-malformed",
        );
        assert_status_code(
            &malformed.outcome,
            "swallowtail.gemini.headless.malformed_stream",
            false,
        );

        let incomplete = completed(
            &topology,
            "",
            ProcessExit::new(true, Some(0)),
            "gemini-incomplete",
        );
        assert_status_code(
            &incomplete.outcome,
            "swallowtail.gemini.headless.incomplete_stream",
            false,
        );

        for (exit, code) in [
            (41, "swallowtail.gemini.headless.native_authentication"),
            (42, "swallowtail.gemini.headless.native_input"),
            (44, "swallowtail.gemini.headless.native_sandbox"),
            (52, "swallowtail.gemini.headless.native_configuration"),
            (53, "swallowtail.gemini.headless.native_turn_limit"),
            (54, "swallowtail.gemini.headless.native_tool"),
            (55, "swallowtail.gemini.headless.native_trust"),
            (130, "swallowtail.gemini.headless.process_interrupted"),
            (1, "swallowtail.gemini.headless.process_failed"),
        ] {
            let native = completed(&topology, "", ProcessExit::new(false, Some(exit)), code);
            assert_status_code(&native.outcome, code, true);
        }

        assert_eq!(cancelled(&topology).status(), &TerminalStatus::Cancelled);
        assert_eq!(timed_out(&topology).status(), &TerminalStatus::TimedOut);
    }
}

#[test]
fn unsupported_inputs_fail_before_process_start() {
    let topology = ExecutionTopologyFixture::local();
    let (process, state) = FakeProcessService::completed("");
    let (services, _) = host_services_for(
        topology.execution_host_id().clone(),
        process,
        Arc::new(PendingTimeService),
    );
    let request = request_for("gemini-tools-rejected", topology.working_resource().clone())
        .with_tools([swallowtail_runtime::ToolDeclaration::new(
            "fixture-tool",
            swallowtail_runtime::SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1_024)
                .expect("schema is valid"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("tool is valid")]);
    let result = block_on(driver().start_run(plan_for(&topology), request, services));

    assert!(result.is_err());
    assert!(!state.started());
}

#[test]
fn prepared_facade_discovers_exact_version_and_starts_a_bound_run() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let (probe_process, probe_state) = FakeProcessService::completed("0.52.0\n");
        let (probe_services, _) = host_services_for(
            topology.execution_host_id().clone(),
            probe_process,
            Arc::new(PendingTimeService),
        );
        let selected = block_on(prepare_gemini_cli(
            headless_support::cli_preparation_input(topology.execution_host_id().clone()),
            headless_support::cli_probe(),
            probe_services,
        ))
        .expect("Gemini headless prepares");
        assert_eq!(selected.driver(), GeminiCliPreparedDriver::Headless);
        let GeminiCliPreparedIntegration::Headless(prepared) = selected else {
            panic!("headless route remains explicitly selected");
        };
        assert_eq!(probe_state.request().arguments, ["--version"]);
        assert_eq!(
            prepared.observation().version().version().as_str(),
            "0.52.0"
        );

        let profile = prepared
            .prepare_run(GeminiHeadlessRunProfileInput::new(
                RequestId::new("gemini-prepared-run").expect("request id is valid"),
                GeminiHeadlessModelSelection::new(
                    ModelRouteId::new("gemini.prepared.route").expect("route id is valid"),
                    ModelRouteRevision::new("1").expect("route revision is valid"),
                    ProviderId::new("gemini").expect("provider id is valid"),
                    ModelId::new("gemini-2.5-flash").expect("model id is valid"),
                ),
                OperationContent::new("prepared private prompt").expect("prompt is valid"),
                WorkingResourceRef::new(topology.working_resource().as_host_value())
                    .expect("working resource is valid"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("Gemini headless run prepares");
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let output = fixture("success.jsonl", "gemini-prepared-run");
        let (run_process, run_state) = FakeProcessService::completed(&output);
        let (run_services, task) = host_services_for(
            topology.execution_host_id().clone(),
            run_process,
            Arc::new(PendingTimeService),
        );
        let mut run = block_on(profile.start_run(run_services)).expect("prepared run starts");
        assert!(run.provider_run_ref().is_some());
        assert!(run.take_management_binding().is_none());
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        let binding = run
            .take_management_binding()
            .expect("successful durable run yields management authority");
        assert!(binding.supports(Capability::ProviderSessionDelete));
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let cancelled = prepared
            .prepare_delete_session(GeminiHeadlessSessionManagementInput::new(
                RequestId::new("gemini-prepared-delete-cancelled").expect("request id is valid"),
                binding.clone(),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("cancelled deletion prepares");
        block_on(cancelled.request().cancellation().request())
            .expect("pre-dispatch cancellation is accepted");
        let (cancelled_process, cancelled_state) = FakeProcessService::completed("");
        let (cancelled_services, _) = host_services_for(
            topology.execution_host_id().clone(),
            cancelled_process,
            Arc::new(PendingTimeService),
        );
        let cancelled_outcome = block_on(cancelled.execute(cancelled_services))
            .expect("cancelled deletion returns effect truth");
        assert_eq!(
            cancelled_outcome.effect().truth(),
            ProviderSessionEffectTruth::FailedBeforeEffect
        );
        assert!(!cancelled_state.started());
        let delete = prepared
            .prepare_delete_session(GeminiHeadlessSessionManagementInput::new(
                RequestId::new("gemini-prepared-delete").expect("request id is valid"),
                binding,
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("bound transcript deletion prepares");
        assert_eq!(
            delete.plan().preflight().requirements().driver_role(),
            DriverRole::ProviderSessionManagement
        );
        let (delete_process, delete_states) = ScriptedProcessService::new([
            (
                "Deleted session 1: private".to_owned(),
                String::new(),
                ProcessExit::new(true, Some(0)),
            ),
            (
                "No sessions found".to_owned(),
                String::new(),
                ProcessExit::new(true, Some(0)),
            ),
        ]);
        let (delete_services, _) = host_services_for(
            topology.execution_host_id().clone(),
            delete_process,
            Arc::new(PendingTimeService),
        );
        let deletion = block_on(delete.execute(delete_services))
            .expect("prepared transcript deletion executes");
        assert_eq!(
            deletion.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            delete_states[0].request().arguments,
            [
                "--delete-session",
                headless_support::session_id("gemini-prepared-run").as_str(),
            ]
        );
        assert_eq!(delete_states[1].request().arguments, ["--list-sessions"]);
        assert!(run_state.waited());
        assert!(task.joined());
        assert_eq!(run_state.stdin(), b"prepared private prompt");
    }
}

#[test]
fn prepared_temporary_profile_deletes_and_reconciles_its_owned_transcript() {
    for retained in [false, true] {
        let topology = ExecutionTopologyFixture::local();
        let (probe_process, _) = FakeProcessService::completed("0.52.0\n");
        let (probe_services, _) = host_services_for(
            topology.execution_host_id().clone(),
            probe_process,
            Arc::new(PendingTimeService),
        );
        let GeminiCliPreparedIntegration::Headless(prepared) = block_on(prepare_gemini_cli(
            headless_support::cli_preparation_input(topology.execution_host_id().clone()),
            headless_support::cli_probe(),
            probe_services,
        ))
        .expect("Gemini headless prepares") else {
            panic!("headless route remains selected");
        };
        let profile = prepared
            .prepare_run(
                GeminiHeadlessRunProfileInput::new(
                    RequestId::new("gemini-owned-cleanup").expect("request id is valid"),
                    GeminiHeadlessModelSelection::new(
                        ModelRouteId::new("gemini.prepared.route").expect("route id is valid"),
                        ModelRouteRevision::new("1").expect("route revision is valid"),
                        ProviderId::new("gemini").expect("provider id is valid"),
                        ModelId::new("gemini-2.5-flash").expect("model id is valid"),
                    ),
                    OperationContent::new("temporary private prompt").expect("prompt is valid"),
                    topology.working_resource().clone(),
                    Deadline::at(MonotonicInstant::from_ticks(1_000)),
                )
                .with_owned_transcript_cleanup(),
            )
            .expect("temporary run prepares");
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        let provider_id = headless_support::session_id("gemini-owned-cleanup");
        let list = if retained {
            format!("1 {provider_id} private")
        } else {
            "No sessions found".to_owned()
        };
        let (process, states) = ScriptedProcessService::new([
            (
                fixture("success.jsonl", "gemini-owned-cleanup"),
                String::new(),
                ProcessExit::new(true, Some(0)),
            ),
            (
                "Deleted session 1: private".to_owned(),
                String::new(),
                ProcessExit::new(true, Some(0)),
            ),
            (list, String::new(), ProcessExit::new(true, Some(0))),
        ]);
        let (services, task) = host_services_for(
            topology.execution_host_id().clone(),
            process,
            Arc::new(PendingTimeService),
        );
        let mut run = block_on(profile.start_run(services)).expect("temporary run starts");
        let outcome = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(if retained {
                RemoteResourceDeletionOutcome::Unconfirmed
            } else {
                RemoteResourceDeletionOutcome::Confirmed
            })
        );
        assert_eq!(
            matches!(outcome.cleanup(), CleanupOutcome::Degraded(_)),
            retained
        );
        assert!(run.take_management_binding().is_none());
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(task.joined());
        assert_eq!(
            states[1].request().arguments,
            ["--delete-session", provider_id.as_str()]
        );
        assert_eq!(states[2].request().arguments, ["--list-sessions"]);
        assert!(states.iter().all(|state| state.waited()));
    }
}

#[test]
fn provider_neutral_one_shot_and_native_profiles_cover_the_route_boundaries() {
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
    assert_eq!(native.profile(), SyntheticProfile::OneShotStructuredCli);
    for assertion in [
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::NativeBudgetIndependent,
        ConformanceAssertion::NoTranscriptDeletionClaim,
    ] {
        assert!(native.covers(assertion), "missing {assertion:?}");
    }
}
