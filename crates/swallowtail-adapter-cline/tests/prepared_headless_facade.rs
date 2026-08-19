#![allow(dead_code)]

#[path = "support/discovery.rs"]
mod discovery_support;
#[path = "support/headless.rs"]
mod headless_support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use headless_support::{FIXTURE_CWD, FixtureHost};
use swallowtail_adapter_cline::{
    CLINE_EXECUTABLE_NAME, CLINE_LOCAL_ACCOUNT_AUDIENCE, CLINE_PACKAGE_AXIS, CLINE_PACKAGE_VERSION,
    ClineHeadlessPreparationInput, ClineHeadlessPreparationProbe, ClineHeadlessRunProfileInput,
    cline_local_account_access_profile, prepare_cline_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, TerminalStatus, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const SUCCESS: &str = include_str!("fixtures/cline-headless-3.0.55/success.jsonl");

#[test]
fn prepared_run_names_cline_headless_and_package_then_drains_one_print() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.local").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_cline_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Cline headless prepares");
    assert_eq!(
        discovery
            .observed_process()
            .expect("version probe ran")
            .arguments,
        ["--version"]
    );
    assert_eq!(
        prepared.observation().version().axis().as_str(),
        CLINE_PACKAGE_AXIS
    );
    assert_eq!(
        prepared.observation().version().version().as_str(),
        CLINE_PACKAGE_VERSION
    );
    assert_eq!(
        prepared.access_profile().endpoint_audience().as_str(),
        CLINE_LOCAL_ACCOUNT_AUDIENCE
    );
    assert!(prepared.access_profile().credential_reference().is_none());
    assert_eq!(
        prepared.instance().driver_id().as_str(),
        "swallowtail.cline.headless"
    );
    assert!(
        prepared
            .instance()
            .capabilities()
            .iter()
            .all(|(capability, _)| capability != Capability::InteractiveSession)
    );

    let run = prepared
        .prepare_run(run_input("prompt"))
        .expect("run prepares");
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(
        run.plan().driver_identity().id().as_str(),
        "swallowtail.cline.headless"
    );
    let versions: Vec<_> = run
        .plan()
        .interface_versions()
        .map(|binding| {
            (
                binding.axis().as_str().to_owned(),
                binding.version().as_str().to_owned(),
            )
        })
        .collect();
    assert_eq!(
        versions,
        [(
            CLINE_PACKAGE_AXIS.to_owned(),
            CLINE_PACKAGE_VERSION.to_owned()
        )]
    );
    assert!(run.plan().credential_reference().is_none());
    assert!(run.plan().model_id().is_none());
    assert!(run.plan().model_route_id().is_none());
    assert!(run.request().deadline().is_some());

    let mut handle = block_on(run.start_run(operation.services(host_id))).expect("run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
        Some("Cline display text.")
    );
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    assert!(!format!("{terminal:?}").contains("Cline display text"));
    let observed = operation.observed();
    assert_eq!(
        observed.arguments,
        [
            "--json",
            "--auto-approve",
            "false",
            "-c",
            FIXTURE_CWD,
            "private fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo"] {
        assert!(
            !observed
                .arguments
                .iter()
                .any(|argument| argument == forbidden)
        );
    }
    assert!(
        !observed
            .arguments
            .windows(2)
            .any(|pair| pair == ["--auto-approve", "true"])
    );
    assert!(operation.stdin().is_empty());
    assert!(operation.stdin_closed());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(operation.joined());
}

#[test]
fn preparation_rejects_access_axis_and_package_drift_before_json_work() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.reject").expect("host");
    let prepared = prepare(host_id.clone());
    let wrong_host = ExecutionHostId::new("fixture.prepared.headless.other").expect("host");
    assert!(
        prepared
            .validate_execution_binding(&wrong_host, prepared.target())
            .is_err()
    );

    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let wrong_access = AccessProfile::new(
        AccessProfileId::new("cline.fixture.local-account").expect("access"),
        CredentialMechanism::LocalUnauthenticated,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("wrong.audience").expect("audience"),
        SupportAuthority::ProviderSupported,
    );
    let error = block_on(prepare_cline_headless(
        ClineHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            target(),
            EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
            wrong_access,
            evidence(),
        ),
        probe(),
        discovery.services(host_id.clone()),
    ))
    .expect_err("access drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cline.headless.preparation.access_profile_rejected"
    );
    assert!(discovery.observed_process().is_none());

    let axis_host = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let error = block_on(prepare_cline_headless(
        ClineHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}"))
                    .expect("executable"),
                InterfaceVersionAxis::new("cline.acp").expect("axis"),
            ),
            EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
            cline_local_account_access_profile(
                AccessProfileId::new("cline.fixture.local-account").expect("access"),
            ),
            evidence(),
        ),
        probe(),
        axis_host.services(host_id.clone()),
    ))
    .expect_err("ACP axis is not this route");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cline.headless.preparation.target_axis_mismatch"
    );
    assert!(axis_host.observed_process().is_none());

    let newer_host = ExecutionHostId::new("fixture.prepared.headless.newer").expect("host");
    let newer = DiscoveryHost::new("3.0.56");
    let error = block_on(prepare_cline_headless(
        preparation_input(newer_host.clone()),
        probe(),
        newer.services(newer_host),
    ))
    .expect_err("unqualified package fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::VersionParse
    );
    assert_eq!(
        newer.observed_process().expect("probe ran").arguments,
        ["--version"]
    );
}

#[test]
fn run_prepare_fails_closed_without_working_resource_authority() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.no-resource").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let prepared = block_on(prepare_cline_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("discovery does not require a working resource");
    let error = prepared
        .prepare_run(run_input("missing-resource"))
        .expect_err("run preflight requires working-resource authority");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
}

fn prepare(
    host_id: ExecutionHostId,
) -> swallowtail_adapter_cline::ClineHeadlessPreparedIntegration {
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_cline_headless(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("Cline headless prepares")
}

fn preparation_input(host_id: ExecutionHostId) -> ClineHeadlessPreparationInput {
    ClineHeadlessPreparationInput::new(
        ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host_id,
        target(),
        EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
        cline_local_account_access_profile(
            AccessProfileId::new("cline.fixture.local-account").expect("access"),
        ),
        evidence(),
    )
}

fn run_input(id: &str) -> ClineHeadlessRunProfileInput {
    ClineHeadlessRunProfileInput::new(
        RequestId::new(format!("cline.fixture.run.{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        WorkingResourceRef::new("cline.fixture.workspace").expect("resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn probe() -> ClineHeadlessPreparationProbe {
    ClineHeadlessPreparationProbe::new(
        RequestId::new("cline.fixture.headless.probe").expect("request"),
        ScopeId::new("cline.fixture.headless.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}")).expect("executable"),
        InterfaceVersionAxis::new(CLINE_PACKAGE_AXIS).expect("axis"),
    )
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("cline.fixture.local-account").expect("access"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}
