use super::common::{
    CREDIT_FAILURE, NO_TOOL_SUCCESS, TOOL_SUCCESS, UNKNOWN_EVENT, host_id, model, prepare,
    prepare_current, run_input,
};
use super::support;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
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
    assert!(!events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(
                    activity.kind(),
                    ActivityKind::Unknown(namespace)
                        if namespace.as_str() == "command-code.headless.event.model_request_start"
                )
        )
    }));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
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
    assert!(
        !process
            .arguments
            .iter()
            .any(|argument| argument == "--yolo")
    );
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
fn plan_lane_model_observation_reports_requested_and_cli_selected_ids() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../fixtures/command-code-1.79.1/plan-model-selection.json"
    ))
    .expect("conflicting model configuration fixture");
    let host_id = host_id();
    let prepared = prepare_current(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "model-selection"))
        .expect("run prepares");
    let stdout = fixture["stdout_ndjson"]
        .as_array()
        .expect("captured output records")
        .iter()
        .map(Value::to_string)
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let host = support::FixtureHost::completed([support::stdout_chunk(stdout.into_bytes())]);
    let (services, observations) = host.services_with_debug_observer(host_id);
    let mut handle = block_on(run.start_run(services)).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let observations = observations
        .lock()
        .expect("debug observations lock is available");
    assert_eq!(observations.len(), 1);
    let observation = &observations[0];
    assert_eq!(
        observation.kind(),
        swallowtail_runtime::DebugObservationKind::InterfaceVersion
    );
    assert_eq!(observation.route(), Some("command-code.headless"));
    assert_eq!(observation.stage(), Some("model-selection"));
    assert_eq!(
        observation.request_id().map(|request| request.as_str()),
        Some("command-code.fixture.run.model-selection")
    );
    let detail: Value = serde_json::from_str(observation.detail()).expect("bounded JSON detail");
    assert!(!events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(
                    activity.kind(),
                    ActivityKind::Unknown(namespace)
                        if namespace.as_str() == "command-code.headless.event.model_request_start"
                )
        )
    }));
    assert_eq!(detail["requested_model_id"], fixture["requested_model_id"]);
    assert_eq!(
        detail["effective_model_id"],
        fixture["configured_feature_models"]["planning"]
    );
    assert_eq!(
        detail["effective_source"],
        "command-code.model_request_start"
    );
    assert_eq!(detail.as_object().unwrap().len(), 4);
    assert!(!format!("{events:?}").contains("configured/planning-model"));
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
    for role in [
        DriverRole::ModelCatalog,
        DriverRole::ProviderSessionCatalogue,
    ] {
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
