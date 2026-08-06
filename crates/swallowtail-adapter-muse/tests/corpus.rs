mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use swallowtail_adapter_muse::{
    MUSE_CODE_PAYLOAD_BASENAME, MUSE_CODE_RELEASE_AXIS, MUSE_CODE_RELEASE_REVISION,
    MUSE_LOCAL_META_ACCOUNT_AUDIENCE, MUSE_META_PROVIDER_ID, MUSE_SPARK_MODEL_ID,
    MuseHeadlessModelSelection, MusePreparationInput, MusePreparationProbe, MuseRunProfileInput,
    muse_code_release_binding, muse_headless_claim, muse_local_meta_account_access_profile,
    prepare_muse_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CapabilityConstraint,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId,
    HarnessIsolation, InstanceRevision, InterfaceVersion, InterfaceVersionAxis, ModelId,
    ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, ProviderId, ReasoningMode,
    ResourceAccess, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, OperationPolicy,
    PreparedAccessEvidence, ProviderRetentionPolicy, RequestId, ScopeId, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::{
    ConformanceAssertion, SyntheticProfile, assert_prepared_operation_evidence_matches_plan,
    run_one_shot_structured_cli_profile,
};

const ARTIFACT: &str = include_str!("fixtures/muse-code-0.1.0-R708.1/artifact.json");
const PROTOCOL: &str = include_str!("fixtures/muse-code-0.1.0-R708.1/protocol.json");
const VERSION: &str = include_str!("fixtures/muse-code-0.1.0-R708.1/version.txt");
const SUCCESS: &str = include_str!("fixtures/muse-code-0.1.0-R708.1/meta-success.jsonl");

#[test]
fn exact_artifact_and_protocol_revisions_are_bound_together() {
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture parses");
    let protocol: Value = serde_json::from_str(PROTOCOL).expect("protocol fixture parses");
    assert_eq!(artifact["release"], MUSE_CODE_RELEASE_REVISION);
    assert_eq!(artifact["payload"]["basename"], MUSE_CODE_PAYLOAD_BASENAME);
    assert_eq!(protocol["artifact_revision"], MUSE_CODE_RELEASE_REVISION);
    assert_eq!(protocol["protocol_facade_revision"], "muse-code.events-v1");
    assert_eq!(protocol["meta_success"]["model_id"], MUSE_SPARK_MODEL_ID);
    assert_eq!(
        muse_code_release_binding(MUSE_CODE_RELEASE_REVISION)
            .expect("binding")
            .axis()
            .as_str(),
        MUSE_CODE_RELEASE_AXIS
    );
    assert!(
        muse_headless_claim()
            .supports(&InterfaceVersion::new(MUSE_CODE_RELEASE_REVISION).expect("version"))
    );
}

#[test]
fn launcher_is_not_the_selected_runtime_artifact() {
    let artifact: Value = serde_json::from_str(ARTIFACT).expect("artifact fixture parses");
    assert_eq!(artifact["launcher"]["selected_as_runtime_artifact"], false);
    assert_eq!(artifact["payload"]["selected_as_runtime_artifact"], true);
    assert_eq!(artifact["launcher"]["may_update_before_delegation"], true);
    assert_eq!(
        artifact["direct_payload_probe"]["launcher_update_path_executed"],
        false
    );
}

#[test]
fn prepared_facade_advertises_and_accepts_exactly_seven_efforts() {
    let host_id = host_id();
    let prepared = prepare(host_id);
    let advertised = prepared
        .instance()
        .capabilities()
        .iter()
        .find(|(capability, _)| *capability == Capability::ReasoningSelection)
        .expect("reasoning is advertised")
        .1
        .iter()
        .filter_map(|constraint| match constraint {
            CapabilityConstraint::ReasoningMode(mode) => Some(mode.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(
        advertised,
        ["high", "low", "medium", "minimal", "none", "ultra", "xhigh"]
    );

    for effort in ["none", "minimal", "low", "medium", "high", "xhigh", "ultra"] {
        let run = prepared
            .prepare_run(run_input(
                MuseHeadlessModelSelection::new(
                    ModelRouteId::new(format!("muse.fixture.{effort}")).expect("route"),
                    ModelRouteRevision::new("1").expect("route revision"),
                    ProviderId::new(MUSE_META_PROVIDER_ID).expect("provider"),
                    ModelId::new(MUSE_SPARK_MODEL_ID).expect("model"),
                ),
                effort,
            ))
            .expect("qualified effort prepares");
        assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
        assert_eq!(
            run.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );
        assert_eq!(
            run.request()
                .policy()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            Some(effort)
        );
    }
}

#[test]
fn prepared_run_uses_local_account_and_exact_read_only_cli_binding() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "low"))
        .expect("run prepares");
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        MUSE_LOCAL_META_ACCOUNT_AUDIENCE
    );
    assert_eq!(
        run.plan().requirements().harness_isolation(),
        Some(HarnessIsolation::ProviderEnforced)
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
    let host = support::FixtureHost::scripted([SUCCESS]);
    let mut handle = block_on(run.start_run(host.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let observed = host.observations();
    assert_eq!(observed.len(), 1);
    let process = &observed[0];
    assert_eq!(
        process.executable,
        format!("/fixture/bin/{MUSE_CODE_PAYLOAD_BASENAME}")
    );
    assert_eq!(process.environments, ["muse.fixture.environment"]);
    assert_eq!(
        process.working_resource.as_deref(),
        Some("muse.fixture.workspace")
    );
    for pair in [
        ["--provider", MUSE_META_PROVIDER_ID],
        ["--model", MUSE_SPARK_MODEL_ID],
        ["--reasoning-effort", "low"],
    ] {
        assert!(
            process
                .arguments
                .windows(2)
                .any(|arguments| arguments == pair),
            "missing {pair:?}"
        );
    }
    for exact in [
        "--disable-web-tools",
        "--no-foreign-personal-context",
        "--no-session-log",
        "--disable-write",
        "--disable-shell",
    ] {
        assert!(process.arguments.iter().any(|argument| argument == exact));
    }
    assert!(!process.arguments.iter().any(|argument| {
        argument.contains("token") || argument.contains("credential") || argument.contains("auth")
    }));
}

#[test]
fn facade_rejects_selection_access_target_and_binding_drift_before_model_execution() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    for (selection, effort, code) in [
        (
            MuseHeadlessModelSelection::new(
                ModelRouteId::new("muse.fixture.wrong-provider").unwrap(),
                ModelRouteRevision::new("1").unwrap(),
                ProviderId::new("other").unwrap(),
                ModelId::new(MUSE_SPARK_MODEL_ID).unwrap(),
            ),
            "low",
            "swallowtail.muse_code.preparation.model_selection_rejected",
        ),
        (
            MuseHeadlessModelSelection::new(
                ModelRouteId::new("muse.fixture.wrong-model").unwrap(),
                ModelRouteRevision::new("1").unwrap(),
                ProviderId::new(MUSE_META_PROVIDER_ID).unwrap(),
                ModelId::new("other-model").unwrap(),
            ),
            "low",
            "swallowtail.muse_code.preparation.model_selection_rejected",
        ),
        (
            model(),
            "extreme",
            "swallowtail.muse_code.preparation.effort_rejected",
        ),
    ] {
        let error = prepared
            .prepare_run(run_input(selection, effort))
            .expect_err("selection drift fails");
        assert_eq!(error.diagnostic().safe().code(), code);
    }

    let wrong_host = ExecutionHostId::new("muse.fixture.other-host").unwrap();
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );
    let wrong_target = InstalledExecutableTarget::new(
        ExecutableRef::new("/fixture/bin/muse").unwrap(),
        InterfaceVersionAxis::new(MUSE_CODE_RELEASE_AXIS).unwrap(),
    );
    assert!(
        prepared
            .validate_execution_binding(&host_id, &wrong_target)
            .is_err()
    );

    let access_id = AccessProfileId::new("muse.fixture.access").unwrap();
    let host = support::FixtureHost::scripted([VERSION]);
    let wrong_access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("wrong.audience").unwrap(),
        SupportAuthority::ProviderSupported,
    );
    let error = block_on(prepare_muse_headless(
        preparation_input(host_id.clone(), wrong_access, evidence(access_id)),
        probe(),
        host.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.muse_code.preparation.access_profile_rejected"
    );
    assert!(!host.started());

    let access_id = AccessProfileId::new("muse.fixture.access").unwrap();
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::Unknown,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let evidence_host = support::FixtureHost::scripted([VERSION]);
    let inaccessible = block_on(prepare_muse_headless(
        preparation_input(
            host_id.clone(),
            muse_local_meta_account_access_profile(access_id),
            PreparedAccessEvidence::caller_asserted(status),
        ),
        probe(),
        evidence_host.services(host_id.clone()),
    ))
    .expect("discovery does not use account state");
    assert!(inaccessible.prepare_run(run_input(model(), "low")).is_err());
    assert_eq!(evidence_host.observations().len(), 1);
    assert_eq!(evidence_host.observations()[0].arguments, ["--version"]);

    let access_id = AccessProfileId::new("muse.fixture.access").unwrap();
    let input = MusePreparationInput::new(
        ConfiguredInstanceId::new("muse.fixture.instance").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host_id.clone(),
        wrong_target,
        EnvironmentRef::new("muse.fixture.environment").unwrap(),
        muse_local_meta_account_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let target_host = support::FixtureHost::scripted([VERSION]);
    let error = block_on(prepare_muse_headless(
        input,
        probe(),
        target_host.services(host_id.clone()),
    ))
    .expect_err("mutable launcher fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.muse_code.preparation.target_rejected"
    );
    assert!(!target_host.started());

    let access_id = AccessProfileId::new("muse.fixture.access").unwrap();
    let release_host = support::FixtureHost::scripted(["Muse Code 0.1.0 (0.1.0-R708.2)\n"]);
    let error = block_on(prepare_muse_headless(
        preparation_input(
            host_id.clone(),
            muse_local_meta_account_access_profile(access_id.clone()),
            evidence(access_id),
        ),
        probe(),
        release_host.services(host_id),
    ))
    .expect_err("different release fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(release_host.observations().len(), 1);
    assert_eq!(release_host.observations()[0].arguments, ["--version"]);
}

#[test]
fn low_level_escape_hatch_rejects_missing_effort_and_resource_before_process() {
    let host_id = host_id();
    let prepared = prepare(host_id.clone());
    let run = prepared
        .prepare_run(run_input(model(), "low"))
        .expect("run prepares");
    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::Prohibited)
        .with_harness_isolation(HarnessIsolation::ProviderEnforced)
        .with_harness_configuration_posture(swallowtail_core::HarnessConfigurationPosture::Ambient);
    let request = StructuredRunRequest::new(
        RequestId::new("muse.fixture.drift").unwrap(),
        OperationContent::new("private prompt").unwrap(),
        policy,
    )
    .with_deadline(deadline());
    let host = support::FixtureHost::scripted([SUCCESS]);
    let result = block_on(run.low_level_driver().start_run(
        run.plan().clone(),
        request,
        host.services(host_id),
    ));
    let Err(error) = result else {
        panic!("missing effort and resource must fail");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.muse_code.headless.unsupported_request"
    );
    assert!(!host.started());
}

#[test]
fn descriptor_and_common_profile_keep_unsupported_surfaces_unavailable() {
    let descriptor = swallowtail_adapter_muse::muse_headless_descriptor();
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    for role in [
        DriverRole::ModelCatalog,
        DriverRole::InteractiveSession,
        DriverRole::ProviderSessionCatalogue,
    ] {
        assert!(!descriptor.supports_role(role));
    }
    let prepared = prepare(host_id());
    for capability in [
        Capability::ModelCatalog,
        Capability::InteractiveSession,
        Capability::ToolCalls,
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

fn prepare(host_id: ExecutionHostId) -> swallowtail_adapter_muse::MusePreparedIntegration {
    let access_id = AccessProfileId::new("muse.fixture.access").expect("access id");
    let host = support::FixtureHost::scripted([VERSION]);
    let prepared = block_on(prepare_muse_headless(
        preparation_input(
            host_id.clone(),
            muse_local_meta_account_access_profile(access_id.clone()),
            evidence(access_id),
        ),
        probe(),
        host.services(host_id),
    ))
    .expect("Muse Code prepares");
    assert_eq!(host.observations().len(), 1);
    assert_eq!(host.observations()[0].arguments, ["--version"]);
    prepared
}

fn preparation_input(
    host_id: ExecutionHostId,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
) -> MusePreparationInput {
    MusePreparationInput::new(
        ConfiguredInstanceId::new("muse.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("instance revision"),
        host_id,
        InstalledExecutableTarget::new(
            ExecutableRef::new(format!("/fixture/bin/{MUSE_CODE_PAYLOAD_BASENAME}"))
                .expect("executable"),
            InterfaceVersionAxis::new(MUSE_CODE_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("muse.fixture.environment").expect("environment"),
        access_profile,
        access_evidence,
    )
}

fn run_input(model: MuseHeadlessModelSelection, effort: &str) -> MuseRunProfileInput {
    MuseRunProfileInput::new(
        RequestId::new(format!("muse.fixture.run.{effort}")).expect("request"),
        model,
        OperationContent::new("private prompt").expect("prompt"),
        ReasoningMode::new(effort).expect("effort"),
        WorkingResourceRef::new("muse.fixture.workspace").expect("resource"),
        deadline(),
    )
}

fn model() -> MuseHeadlessModelSelection {
    MuseHeadlessModelSelection::new(
        ModelRouteId::new("muse.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("route revision"),
        ProviderId::new(MUSE_META_PROVIDER_ID).expect("provider"),
        ModelId::new(MUSE_SPARK_MODEL_ID).expect("model"),
    )
}

fn evidence(access_id: AccessProfileId) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access_id,
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

fn probe() -> MusePreparationProbe {
    MusePreparationProbe::new(
        RequestId::new("muse.fixture.probe").expect("request"),
        ScopeId::new("muse.fixture.probe").expect("scope"),
        deadline(),
        DiscoveryCancellation::new(),
    )
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("muse.fixture.host").expect("host")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}
