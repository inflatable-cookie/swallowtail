#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use std::sync::Arc;
use support::{FakeProcessService, PendingTimeService, ScriptedProcessService, host_services_for};
use swallowtail_adapter_qwen::{
    QWEN_CODE_AXIS, QwenCatalogueProfileInput, QwenModelSelection, QwenPreparationInput,
    QwenPreparationProbe, QwenRunProfileInput, QwenSessionProfileInput, prepare_qwen_catalogue,
    prepare_qwen_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ProviderId,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_runs_preserve_qwen_stdin_budgets_and_ambient_truth_in_both_topologies() {
    for host_value in [
        "fixture.qwen.prepared.local",
        "fixture.qwen.prepared.remote",
    ] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let (discovery_process, discovery_state) = FakeProcessService::completed("0.19.11\n");
        let (discovery_services, _) = host_services_for(
            host_id.clone(),
            discovery_process,
            Arc::new(PendingTimeService),
        );
        let prepared = block_on(prepare_qwen_headless(
            preparation_input(host_id.clone()),
            probe(),
            discovery_services,
        ))
        .expect("Qwen prepares");
        assert_eq!(discovery_state.request().arguments, ["--version"]);

        let profile = prepared
            .prepare_run(QwenRunProfileInput::new(
                RequestId::new("qwen-prepared-run").expect("valid request"),
                QwenModelSelection::new(
                    ModelRouteId::new("qwen.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                    ModelId::new("qwen3-coder-plus").expect("valid model"),
                ),
                OperationContent::new("prepared private prompt").expect("valid prompt"),
                WorkingResourceRef::new("qwen.prepared.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(1_000)),
            ))
            .expect("Qwen run profile prepares");
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().provider_id().map(ProviderId::as_str),
            Some("alibaba-modelstudio")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let (operation_process, operation_state) = FakeProcessService::completed(include_str!(
            "fixtures/qwen-code-v0.19.11/success.jsonl"
        ));
        let (operation_services, _) =
            host_services_for(host_id, operation_process, Arc::new(PendingTimeService));
        let mut run = block_on(profile.start_run(operation_services)).expect("prepared run starts");
        let terminal = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert_eq!(operation_state.stdin(), b"prepared private prompt");
        let arguments = operation_state.request().arguments;
        for exact in [
            "--input-format",
            "text",
            "--output-format",
            "stream-json",
            "--max-wall-time",
            "60s",
            "--max-tool-calls",
            "16",
            "--max-session-turns",
            "24",
        ] {
            assert!(arguments.iter().any(|argument| argument == exact));
        }
        assert!(!arguments.iter().any(|argument| argument == "--sandbox"));
    }
}

#[test]
fn prepared_session_uses_only_the_exact_private_resume_id_on_later_turns() {
    let host_id = ExecutionHostId::new("fixture.qwen.interactive").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_session(QwenSessionProfileInput::new(
            RequestId::new("qwen-session").expect("valid request"),
            QwenModelSelection::new(
                ModelRouteId::new("qwen.session.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                ModelId::new("qwen3-coder-plus").expect("valid model"),
            ),
            WorkingResourceRef::new("qwen.session.workspace").expect("valid resource"),
        ))
        .expect("Qwen session prepares");
    assert_prepared_operation_evidence_matches_plan(profile.evidence().operation(), profile.plan());
    assert_eq!(
        profile.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );

    let (process, states) = ScriptedProcessService::completed(&[
        include_str!("fixtures/qwen-code-v0.19.11/interactive-first-turn.jsonl"),
        include_str!("fixtures/qwen-code-v0.19.11/interactive-continued-turn.jsonl"),
    ]);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());

    for (index, content) in ["first prompt", "second prompt"].into_iter().enumerate() {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-turn-{}", index + 1)).expect("valid turn"),
                    OperationContent::new(content).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(
            turn.take_terminal_outcome()
                .expect("turn terminal is available"),
        );
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    let first = states[0].request().arguments;
    assert!(!first.iter().any(|argument| argument == "--resume"));
    assert!(!first.iter().any(|argument| argument == "--continue"));
    let second = states[1].request().arguments;
    assert_eq!(
        second
            .windows(2)
            .find(|arguments| arguments[0] == "--resume"),
        Some(
            [
                "--resume".to_owned(),
                "123e4567-e89b-12d3-a456-426614174000".to_owned()
            ]
            .as_slice()
        )
    );
    assert!(!second.iter().any(|argument| argument == "--continue"));
    assert_eq!(states[0].stdin(), b"first prompt");
    assert_eq!(states[1].stdin(), b"second prompt");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn qwen_session_mismatch_fails_closed_without_starting_another_child() {
    let host_id = ExecutionHostId::new("fixture.qwen.mismatch").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let profile = prepared
        .prepare_session(QwenSessionProfileInput::new(
            RequestId::new("qwen-mismatch").expect("valid request"),
            QwenModelSelection::new(
                ModelRouteId::new("qwen.mismatch.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                ModelId::new("qwen3-coder-plus").expect("valid model"),
            ),
            WorkingResourceRef::new("qwen.mismatch.workspace").expect("valid resource"),
        ))
        .expect("Qwen session prepares");
    let (process, states) = ScriptedProcessService::completed(&[
        include_str!("fixtures/qwen-code-v0.19.11/interactive-first-turn.jsonl"),
        include_str!("fixtures/qwen-code-v0.19.11/interactive-session-mismatch.jsonl"),
    ]);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    for (index, expected) in [
        TerminalStatus::Completed,
        TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.qwen.headless.malformed_stream",
            "Qwen Code emitted malformed stream output",
        )),
    ]
    .into_iter()
    .enumerate()
    {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("qwen-mismatch-{}", index + 1)).expect("valid turn"),
                    OperationContent::new(format!("prompt {}", index + 1)).expect("valid content"),
                )
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
                services.clone(),
            ),
        )
        .expect("bounded turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &expected);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    let error = match block_on(
        session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("qwen-mismatch-3").expect("valid turn"),
                OperationContent::new("must not start").expect("valid content"),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
            services,
        ),
    ) {
        Ok(_) => panic!("mismatched provider session must invalidate the handle"),
        Err(error) => error,
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.qwen.headless.session_unusable"
    );
    assert_eq!(states.len(), 2);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_catalogue_uses_qwen_control_protocol_and_joins_the_ephemeral_process() {
    let host_id = ExecutionHostId::new("fixture.qwen.catalogue").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let catalogue = prepare_qwen_catalogue(
        &prepared,
        QwenCatalogueProfileInput::new(RequestId::new("qwen-catalogue").expect("valid request"))
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
    )
    .expect("catalogue prepares");
    assert_eq!(
        catalogue.evidence().observable_activity().availability(),
        ObservableActivityAvailability::NotApplicable
    );
    let output = concat!(
        "{\"type\":\"control_response\",\"response\":{\"subtype\":\"success\",\"request_id\":\"swallowtail-initialize\",\"response\":{\"subtype\":\"initialize\",\"session_id\":\"fixture\",\"capabilities\":{\"can_get_available_models\":true}}}}\n",
        "{\"type\":\"control_response\",\"response\":{\"subtype\":\"success\",\"request_id\":\"swallowtail-models\",\"response\":{\"subtype\":\"get_available_models\",\"models\":[{\"id\":\"qwen-fixture\",\"label\":\"Qwen Fixture\",\"contextWindowSize\":131072}]}}}\n"
    );
    let (process, state) = FakeProcessService::completed(output);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let models = block_on(catalogue.list_models(services)).expect("catalogue succeeds");

    assert_eq!(models[0].id().as_str(), "qwen-fixture");
    assert!(state.stdin_closed());
    assert!(state.force_stopped());
    assert!(state.waited());
    let request = state.request();
    assert_eq!(
        request.arguments,
        [
            "--input-format",
            "stream-json",
            "--output-format",
            "stream-json",
            "--safe-mode",
            "--approval-mode",
            "default",
        ]
    );
    let stdin = String::from_utf8(state.stdin()).expect("stdin is UTF-8");
    assert!(stdin.contains("\"subtype\":\"initialize\""));
    assert!(stdin.contains("\"subtype\":\"get_available_models\""));
}

#[test]
fn later_stable_qwen_is_visible_and_executable_as_unverified_newer() {
    let host_id = ExecutionHostId::new("fixture.qwen.prepared.newer").expect("valid host");
    let (process, _) = FakeProcessService::completed("0.20.1\n");
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("newer Qwen remains executable");
    assert!(matches!(
        prepared.observation().compatibility(),
        InstalledExecutableCompatibility::UnverifiedNewer(_)
    ));
}

fn preparation_input(host: ExecutionHostId) -> QwenPreparationInput {
    QwenPreparationInput::new(
        ConfiguredInstanceId::new("qwen.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("qwen.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(QWEN_CODE_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("qwen.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("qwen.prepared.access").expect("valid access"),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("qwen-code/delegated-harness-auth")
                    .expect("valid namespace"),
            ),
            EntitlementMetering::Unknown,
            EndpointAudience::new("qwen-code").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("qwen.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> QwenPreparationProbe {
    QwenPreparationProbe::new(
        RequestId::new("qwen-prepared-probe").expect("valid request"),
        ScopeId::new("qwen-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
