#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use std::sync::Arc;
use support::{FakeProcessService, PendingTimeService, host_services_for};
use swallowtail_adapter_qwen::{
    QWEN_CODE_AXIS, QwenModelSelection, QwenPreparationInput, QwenPreparationProbe,
    QwenRunProfileInput, prepare_qwen_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
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
