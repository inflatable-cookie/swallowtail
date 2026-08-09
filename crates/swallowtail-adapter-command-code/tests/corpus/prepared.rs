use super::common::{
    CREDIT_FAILURE, NO_TOOL_SUCCESS, TOOL_SUCCESS, UNKNOWN_EVENT, host_id, model, prepare,
    run_input,
};
use super::support;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_command_code::COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE;
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, FailureKind, HarnessIsolation,
    ObservableActivityAvailability, ResourceAccess,
};
use swallowtail_runtime::{
    ActivityKind, CleanupOutcome, ProcessExit, RuntimeEventKind, TerminalStatus,
};
use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, assert_prepared_operation_evidence_matches_plan,
    run_one_shot_structured_cli_profile,
};

#[test]
fn prepared_run_uses_local_account_ambient_host_and_exact_read_only_cli_binding() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "no-tool"))
        .expect("run prepares");
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        COMMAND_CODE_LOCAL_ACCOUNT_AUDIENCE
    );
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(
        run.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    assert_eq!(
        run.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(
        run.plan()
            .requirements()
            .capabilities()
            .find(|requirement| requirement.capability() == Capability::WorkingResource)
            .expect("working resource")
            .constraints()
            .find_map(|constraint| match constraint {
                CapabilityConstraint::ResourceAccess(access) => Some(*access),
                _ => None,
            }),
        Some(ResourceAccess::Read)
    );

    let host = support::FixtureHost::scripted([NO_TOOL_SUCCESS]);
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal.output().map(swallowtail_runtime::OperationContent::as_str),
        Some("pong")
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let observed = host.observations();
    assert_eq!(observed.len(), 1);
    let process = &observed[0];
    assert!(process.executable.ends_with("command-code"));
    assert_eq!(process.environments, ["command-code.fixture.environment"]);
    assert_eq!(
        process.working_resource.as_deref(),
        Some("command-code.fixture.workspace")
    );
    assert_eq!(
        process.arguments,
        [
            "-p",
            "--output-format",
            "json",
            "--permission-mode",
            "plan",
            "--skip-onboarding",
            "--no-session",
            "--no-auto-update",
            "--trust",
            "--no-skills",
            "--max-turns",
            "8",
            "-m",
            "fixture-model",
        ]
    );
    assert!(!process.arguments.iter().any(|argument| argument == "--yolo"));
    assert!(!process.arguments.iter().any(|argument| {
        argument.contains("token") || argument.contains("credential") || argument.contains("auth")
    }));

    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ReasoningSummary
        )
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::AssistantMessage
        )
    }));
}

#[test]
fn prepared_run_projects_tool_activity_by_id_without_input_or_result_bodies() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "tool"))
        .expect("run prepares");
    let host = support::FixtureHost::scripted([TOOL_SUCCESS]);
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ProviderOwnedTool
                    && activity
                        .provider_activity_ref()
                        .is_some_and(|value| value.as_provider_value() == "call-1")
        )
    }));
    let public = format!("{events:?}{terminal:?}");
    assert!(!public.contains("private prompt"));
    assert!(!public.contains("read_file"));
}

#[test]
fn prepared_run_classifies_credit_exhaustion_at_process_exit_ten() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "credit-failure"))
        .expect("run prepares");
    let host = support::FixtureHost::with_exit(
        [support::stdout_chunk(CREDIT_FAILURE.as_bytes().to_vec())],
        ProcessExit::new(false, Some(10)),
    );
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let TerminalStatus::ProviderFailed(diagnostic) = terminal.status() else {
        panic!("credit exhaustion must classify as a provider failure");
    };
    assert_eq!(
        diagnostic.failure_classification().kind(),
        FailureKind::QuotaExhausted
    );
    let public = format!("{terminal:?}");
    assert!(!public.contains("run_error"));
}

#[test]
fn prepared_run_projects_unknown_event_and_completes() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "unknown-event"))
        .expect("run prepares");
    let host = support::FixtureHost::scripted([UNKNOWN_EVENT]);
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(activity.kind(), ActivityKind::Unknown(namespace)
                    if namespace.as_str() == "command-code.headless.event.future_experimental_event")
        )
    }));
}

#[test]
fn descriptor_and_common_profile_keep_unsupported_surfaces_unavailable() {
    let descriptor = swallowtail_adapter_command_code::command_code_headless_descriptor();
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    assert!(descriptor.supports_role(DriverRole::InteractiveSession));
    for role in [DriverRole::ModelCatalog, DriverRole::ProviderSessionCatalogue] {
        assert!(!descriptor.supports_role(role));
    }
    let prepared = prepare(host_id());
    for capability in [
        Capability::ModelCatalog,
        Capability::InteractiveSession,
        Capability::ToolCalls,
        Capability::ReasoningSelection,
        Capability::ProviderManagedRecovery,
        Capability::ProviderSessionCatalogue,
        Capability::ProviderSessionReconciliation,
    ] {
        assert!(
            prepared
                .instance()
                .capabilities()
                .iter()
                .all(|(advertised, _)| advertised != capability),
            "unexpected {capability:?}"
        );
    }
    let report = run_one_shot_structured_cli_profile();
    assert_eq!(report.profile(), SyntheticProfile::OneShotStructuredCli);
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
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}
