use crate::{
    CopilotCliPreparationInput, CopilotCliPreparationProbe, CopilotCliSessionProfileInput,
    assessment::{NATIVE_EXECUTABLE_SHA256, sha256_reader},
    copilot_cli_acp_claim, copilot_cli_host_account_access_profile, copilot_cli_package_binding,
    prepare_copilot_cli_acp,
    prepared::{
        prepare_copilot_cli_acp_for_assessment, prepare_copilot_cli_acp_for_assessment_with_reader,
    },
    selection::{
        COPILOT_CLI_PACKAGE_AXIS, COPILOT_CLI_PACKAGE_VERSION, PRIVATE_ASSESSMENT_VERSION,
    },
};
use crate::{
    assessment_discovery_support::DiscoveryHost,
    assessment_test_support::{FixtureHost, Scenario},
};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::io::Cursor;
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    ActivityKind, ActivityLifecyclePhase, ActivityStatus, CleanupOutcome, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, ProviderObservation, RequestId,
    RuntimeEventKind, RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest, WorkingResourceRef,
};

const APPROVED_EXECUTABLE: &str = "/approved/copilot/1.0.95/copilot";
const APPROVED_ENVIRONMENT: &str = "host-approved.copilot-cli.account";
const SENTINEL_PATH: &str = "permission-sentinel.txt";
const SENTINEL_BEFORE: &[u8] = b"SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n";

#[test]
fn assessment_admission_is_private_and_hash_mismatch_stops_before_discovery() {
    assert_eq!(COPILOT_CLI_PACKAGE_VERSION, "1.0.80");
    assert!(copilot_cli_package_binding("1.0.80").is_some());
    assert!(copilot_cli_package_binding(PRIVATE_ASSESSMENT_VERSION).is_none());
    let claim = copilot_cli_acp_claim();
    assert_eq!(claim.milestones().len(), 1);
    assert_eq!(
        claim.milestones().next().unwrap().minimum().as_str(),
        "1.0.80"
    );
    assert_eq!(
        claim.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );

    let host = ExecutionHostId::new("assessment.fake.execution-host").expect("host");
    let fake_executable = b"changed approved executable bytes";
    let discovery = DiscoveryHost::new(PRIVATE_ASSESSMENT_VERSION);
    let error = block_on(prepare_copilot_cli_acp_for_assessment_with_reader(
        preparation_input(host.clone()),
        probe(),
        discovery.services(host.clone()),
        Cursor::new(fake_executable),
        NATIVE_EXECUTABLE_SHA256,
    ))
    .expect_err("artifact drift must fail before discovery");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.copilot-cli.acp.assessment.executable_digest_mismatch"
    );
    assert!(discovery.observed_process().is_none());

    let frozen_check = DiscoveryHost::new(PRIVATE_ASSESSMENT_VERSION);
    let repository_manifest = std::env::current_dir()
        .expect("test runner has a current directory")
        .join("Cargo.toml");
    let repository_manifest = repository_manifest
        .to_str()
        .expect("repository path is UTF-8");
    let error = block_on(prepare_copilot_cli_acp_for_assessment(
        preparation_input_with_executable(host.clone(), repository_manifest),
        probe(),
        frozen_check.services(host.clone()),
    ))
    .expect_err("actual assessment path hashes the approved reference before discovery");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.copilot-cli.acp.assessment.executable_digest_mismatch"
    );
    assert!(frozen_check.observed_process().is_none());

    let ordinary_discovery = DiscoveryHost::new(PRIVATE_ASSESSMENT_VERSION);
    let error = block_on(prepare_copilot_cli_acp(
        preparation_input(host.clone()),
        probe(),
        ordinary_discovery.services(host.clone()),
    ))
    .expect_err("ordinary callers must reject the assessment point");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(
        ordinary_discovery
            .observed_process()
            .expect("ordinary version probe ran")
            .arguments,
        ["--version"]
    );
}

#[test]
fn prepared_095_fake_assessment_correlates_execute_permission_cancels_and_joins() {
    let host = ExecutionHostId::new("assessment.fake.execution-host").expect("host");
    let discovery = DiscoveryHost::new(PRIVATE_ASSESSMENT_VERSION);
    let operation =
        FixtureHost::with_version(Scenario::AssessmentPermission, PRIVATE_ASSESSMENT_VERSION);
    let mut services = discovery.services(host.clone());
    services = services.with_working_resource(
        operation
            .services(host.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let fake_executable = b"fixture bytes for the approved assessment executable";
    let prepared = block_on(prepare_copilot_cli_acp_for_assessment_with_reader(
        preparation_input(host.clone()),
        probe(),
        services,
        Cursor::new(fake_executable),
        &sha256_reader(&fake_executable[..]).expect("fixture executable hashes"),
    ))
    .expect("the exact frozen point prepares through the real facade");
    let captured = prepared
        .assessment_binding
        .as_ref()
        .expect("safe host inputs were captured before discovery");
    assert_eq!(captured.execution_host_id, "assessment.fake.execution-host");
    assert_eq!(captured.executable_ref, APPROVED_EXECUTABLE);
    assert_eq!(captured.environment_ref, APPROVED_ENVIRONMENT);
    assert_eq!(
        captured.execution_host_id_sha256,
        sha256_reader(captured.execution_host_id.as_bytes()).expect("host id hashes")
    );
    assert_eq!(
        captured.executable_ref_sha256,
        sha256_reader(captured.executable_ref.as_bytes()).expect("executable ref hashes")
    );
    assert_eq!(
        captured.environment_ref_sha256,
        sha256_reader(captured.environment_ref.as_bytes()).expect("environment ref hashes")
    );
    assert_eq!(
        captured.executable_bytes_sha256,
        sha256_reader(&fake_executable[..]).expect("fixture executable hashes")
    );
    assert_eq!(
        captured.wrapper_archive_sha256,
        crate::assessment::WRAPPER_ARCHIVE_SHA256
    );
    assert_eq!(
        captured.native_archive_sha256,
        crate::assessment::NATIVE_ARCHIVE_SHA256
    );
    assert_eq!(
        prepared.observation().version().version().as_str(),
        PRIVATE_ASSESSMENT_VERSION
    );
    assert!(
        prepared
            .validate_execution_binding(&host, prepared.target())
            .is_ok()
    );
    let wrong_host = ExecutionHostId::new("assessment.wrong.execution-host").expect("host");
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );
    let wrong_target = InstalledExecutableTarget::new(
        ExecutableRef::new("/approved/copilot/other/copilot").expect("executable"),
        InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("axis"),
    );
    assert!(
        prepared
            .validate_execution_binding(&host, &wrong_target)
            .is_err()
    );

    let session = prepared
        .prepare_session(CopilotCliSessionProfileInput::new(
            RequestId::new("assessment.session").expect("request"),
            WorkingResourceRef::new("assessment.task-scratch").expect("resource"),
        ))
        .expect("assessment session preflights");
    let binding = session
        .plan()
        .interface_versions()
        .find(|version| version.axis().as_str() == COPILOT_CLI_PACKAGE_AXIS)
        .expect("one package binding");
    assert_eq!(binding.version().as_str(), PRIVATE_ASSESSMENT_VERSION);
    assert_eq!(
        session
            .plan()
            .assess_interface_version(binding)
            .behavior_revision()
            .unwrap()
            .as_str(),
        "copilot-cli.acp.stdio-v1"
    );
    assert_eq!(
        copilot_cli_acp_claim()
            .milestones()
            .next()
            .unwrap()
            .minimum()
            .as_str(),
        "1.0.80"
    );

    let mut handle = block_on(session.open_session(operation.services(host.clone())))
        .expect("prepared assessment driver opens");
    let mut turn = block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("assessment.turn").expect("turn"),
            OperationContent::new("Overwrite the existing file permission-sentinel.txt in this working directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline.").expect("prompt"),
        ),
        operation.services(host.clone()),
    ))
    .expect("the task-owned sentinel turn starts");
    let events = block_on(turn.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("driver events decode");
    let observations = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(observation) => Some(observation),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(observations.iter().any(|observation| {
        observation.kind() == &ActivityKind::ProviderOwnedTool
            && observation
                .provider_activity_ref()
                .map(|reference| reference.as_provider_value())
                == Some("sentinel-edit")
            && observation.phase() == ActivityLifecyclePhase::Started
            && observation.status() == ActivityStatus::Pending
    }));
    assert!(observations.iter().any(|observation| {
        observation.kind() == &ActivityKind::ProviderOwnedTool
            && observation
                .provider_activity_ref()
                .map(|reference| reference.as_provider_value())
                == Some("sentinel-edit")
            && observation.phase() == ActivityLifecyclePhase::Completed
            && observation.status() == ActivityStatus::Cancelled
    }));
    assert!(events.iter().any(|event| {
        matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(request))
                if request.as_provider_value() == "acp:900"
        )
    }));
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
    assert!(matches!(
        terminal.status(),
        TerminalStatus::ProviderRequestObserved(_)
    ));

    let process = operation.observed_process();
    assert_eq!(process.executable, APPROVED_EXECUTABLE);
    assert_eq!(process.environments, [APPROVED_ENVIRONMENT]);
    assert_eq!(process.arguments, ["--acp", "--stdio"]);
    let trace = operation.assessment_trace();
    assert!(assessment_trace_is_complete(&trace));
    assert_eq!(trace.sentinel_before, SENTINEL_BEFORE);
    assert_eq!(trace.sentinel_after, SENTINEL_BEFORE);
    assert_eq!(trace.effect_count, 0);
    let writes = operation.writes();
    let cancellation = writes
        .iter()
        .position(|message| message["method"] == "session/cancel")
        .expect("permission cancellation was sent");
    let permission_reply = writes
        .iter()
        .position(|message| message["id"] == 900)
        .expect("permission reply was sent");
    assert!(cancellation < permission_reply);
    assert_eq!(
        writes[permission_reply]["result"]["outcome"]["outcome"],
        "cancelled"
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(handle.close(operation.cleanup_request(), operation.services(host))),
        CleanupOutcome::Clean
    );
    assert_eq!(operation.releases(), 1);
}

#[test]
fn assessment_fake_gate_rejects_missing_or_mismatched_execute_metadata() {
    for scenario in [
        Scenario::AssessmentPermissionMissingAction,
        Scenario::AssessmentPermissionMismatchedAction,
    ] {
        let host = ExecutionHostId::new(format!("assessment.fake.{}", scenario_name(scenario)))
            .expect("host");
        let discovery = DiscoveryHost::new(PRIVATE_ASSESSMENT_VERSION);
        let operation = FixtureHost::with_version(scenario, PRIVATE_ASSESSMENT_VERSION);
        let mut services = discovery.services(host.clone());
        services = services.with_working_resource(
            operation
                .services(host.clone())
                .working_resource()
                .expect("resource service")
                .clone(),
        );
        let fake_executable = b"fixture bytes for the approved assessment executable";
        let prepared = block_on(prepare_copilot_cli_acp_for_assessment_with_reader(
            preparation_input(host.clone()),
            probe(),
            services,
            Cursor::new(fake_executable),
            &sha256_reader(&fake_executable[..]).expect("fixture executable hashes"),
        ))
        .expect("exact version assessment prepares");
        let session = prepared
            .prepare_session(CopilotCliSessionProfileInput::new(
                RequestId::new("assessment.negative.session").expect("request"),
                WorkingResourceRef::new("assessment.task-scratch").expect("resource"),
            ))
            .expect("session preflights");
        let mut handle = block_on(session.open_session(operation.services(host.clone())))
            .expect("prepared driver opens");
        let mut turn = block_on(handle.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new("assessment.negative.turn").expect("turn"),
                OperationContent::new("Overwrite the existing file permission-sentinel.txt in this working directory with exactly: SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1 followed by a newline.").expect("prompt"),
            ),
            operation.services(host.clone()),
        ))
        .expect("turn starts");
        let _events = block_on(turn.take_events().expect("events").collect::<Vec<_>>())
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .expect("events decode");
        let _terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert!(!assessment_trace_is_complete(&operation.assessment_trace()));
        assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
        assert_eq!(
            block_on(handle.close(operation.cleanup_request(), operation.services(host))),
            CleanupOutcome::Clean
        );
    }
}

fn assessment_trace_is_complete(trace: &crate::assessment_test_support::AssessmentTrace) -> bool {
    trace.initialized_version.as_deref() == Some(PRIVATE_ASSESSMENT_VERSION)
        && trace.session_new_seen
        && trace.announced_tool_call_id.as_deref() == Some("sentinel-edit")
        && trace.announced_kind.as_deref() == Some("execute")
        && trace.announced_path.as_deref() == Some(SENTINEL_PATH)
        && trace.announced_old_text.as_deref()
            == Some("SWALLOWTAIL_PERMISSION_SENTINEL_BEFORE_V1\n")
        && trace.announced_new_text.as_deref() == Some("SWALLOWTAIL_PERMISSION_SENTINEL_AFTER_V1\n")
        && trace.permission_request_id == Some(900)
        && trace.permission_tool_call_id == trace.announced_tool_call_id
        && trace.permission_reply.as_deref() == Some("cancelled")
        && trace.session_cancel_seen
        && trace.prompt_result.as_deref() == Some("cancelled")
        && trace.sentinel_before == SENTINEL_BEFORE
        && trace.sentinel_after == SENTINEL_BEFORE
        && trace.task_directory_before.len() == 1
        && trace
            .task_directory_before
            .get(SENTINEL_PATH)
            .map(Vec::as_slice)
            == Some(SENTINEL_BEFORE)
        && trace.task_directory_after == trace.task_directory_before
        && trace.effect_count == 0
}

const fn scenario_name(scenario: Scenario) -> &'static str {
    match scenario {
        Scenario::AssessmentPermissionMissingAction => "missing-action",
        Scenario::AssessmentPermissionMismatchedAction => "mismatched-action",
        _ => "unexpected",
    }
}

fn preparation_input(host: ExecutionHostId) -> CopilotCliPreparationInput {
    preparation_input_with_executable(host, APPROVED_EXECUTABLE)
}

fn preparation_input_with_executable(
    host: ExecutionHostId,
    executable: &str,
) -> CopilotCliPreparationInput {
    CopilotCliPreparationInput::new(
        ConfiguredInstanceId::new("copilot-cli.assessment.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new(executable).expect("executable"),
            InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new(APPROVED_ENVIRONMENT).expect("environment"),
        copilot_cli_host_account_access_profile(
            AccessProfileId::new("copilot-cli.assessment.host-account").expect("profile"),
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            AccessProfileId::new("copilot-cli.assessment.host-account").expect("profile"),
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        )),
    )
}

fn probe() -> CopilotCliPreparationProbe {
    CopilotCliPreparationProbe::new(
        RequestId::new("copilot-cli.assessment.probe").expect("request"),
        ScopeId::new("copilot-cli.assessment.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}
