#![allow(dead_code)]

mod support;

#[path = "support/discovery.rs"]
mod discovery_support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_cline::{
    CLINE_EXECUTABLE_NAME, CLINE_LOCAL_ACCOUNT_AUDIENCE, CLINE_PACKAGE_AXIS, CLINE_PACKAGE_VERSION,
    ClinePreparationInput, ClinePreparationProbe, ClineSessionProfileInput,
    cline_local_account_access_profile, prepare_cline_acp,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CapabilityConstraint,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, HarnessMode,
    InstanceRevision, InterfaceVersionAxis, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, RuntimeTurnId, ScopeId, TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_session_names_cline_acp_and_package_then_drains_one_prompt() {
    let host_id = ExecutionHostId::new("fixture.prepared.local").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_cline_acp(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Cline ACP prepares");
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
        "swallowtail.cline.acp"
    );

    let session = prepared
        .prepare_session(session_input("prompt"))
        .expect("session prepares");
    assert_prepared_operation_evidence_matches_plan(session.evidence(), session.plan());
    assert_eq!(
        session.plan().driver_identity().id().as_str(),
        "swallowtail.cline.acp"
    );
    let versions: Vec<_> = session
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
    assert!(session.plan().credential_reference().is_none());
    assert!(session.request().deadline().is_none());

    let operation_services = operation.services(host_id);
    let mut handle = block_on(session.open_session(operation_services.clone())).expect("opens");
    let mut turn = block_on(handle.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("cline-prepared-turn").expect("turn"),
            OperationContent::new("private fixture prompt").expect("prompt"),
        ),
        operation_services.clone(),
    ))
    .expect("turn starts");
    let events = block_on(turn.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(
        terminal
            .output()
            .map(swallowtail_runtime::OperationContent::as_str),
        Some("fixture response.")
    );
    assert_eq!(operation.observed_process().arguments, ["--acp"]);
    assert!(
        !operation
            .observed_process()
            .arguments
            .iter()
            .any(|argument| argument == "--json" || argument == "--auto-approve")
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(handle.close(operation.cleanup_request(), operation_services)),
        CleanupOutcome::Clean
    );
    assert_eq!(operation.releases(), 1);
}

#[test]
fn preparation_rejects_access_axis_and_package_drift_before_acp_work() {
    let host_id = ExecutionHostId::new("fixture.prepared.reject").expect("host");
    let prepared = prepare(host_id.clone());
    let wrong_host = ExecutionHostId::new("fixture.prepared.other").expect("host");
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
    let error = block_on(prepare_cline_acp(
        ClinePreparationInput::new(
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
        "swallowtail.cline.acp.preparation.access_profile_rejected"
    );
    assert!(discovery.observed_process().is_none());

    let axis_host = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let error = block_on(prepare_cline_acp(
        ClinePreparationInput::new(
            ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id.clone(),
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}"))
                    .expect("executable"),
                InterfaceVersionAxis::new("cline.headless").expect("axis"),
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
    .expect_err("headless axis is not this route");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cline.acp.preparation.target_axis_mismatch"
    );
    assert!(axis_host.observed_process().is_none());

    let newer_host = ExecutionHostId::new("fixture.prepared.newer").expect("host");
    let newer = DiscoveryHost::new("3.0.56");
    let error = block_on(prepare_cline_acp(
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
fn session_prepare_fails_closed_without_working_resource_authority() {
    let host_id = ExecutionHostId::new("fixture.prepared.no-resource").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let prepared = block_on(prepare_cline_acp(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("discovery does not require a working resource");
    let error = prepared
        .prepare_session(session_input("missing-resource"))
        .expect_err("session preflight requires working-resource authority");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
}

fn prepare(host_id: ExecutionHostId) -> swallowtail_adapter_cline::ClinePreparedIntegration {
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_cline_acp(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("Cline ACP prepares")
}

fn preparation_input(host_id: ExecutionHostId) -> ClinePreparationInput {
    ClinePreparationInput::new(
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

#[test]
fn prepared_session_binds_optional_plan_mode_exactly() {
    let host_id = ExecutionHostId::new("fixture.prepared.plan").expect("host");
    let prepared = prepare(host_id);
    let omit = prepared
        .prepare_session(session_input("omit"))
        .expect("omission prepares");
    assert_eq!(omit.harness_mode(), None);
    assert_eq!(omit.request().options().harness_mode(), None);
    assert!(
        omit.plan()
            .requirements()
            .capabilities()
            .all(|required| required.capability() != Capability::HarnessModeSelection),
        "omission must not require HarnessModeSelection"
    );

    let input = session_input("plan").with_harness_mode(HarnessMode::Plan);
    assert_eq!(input.harness_mode(), Some(HarnessMode::Plan));
    let plan = prepared.prepare_session(input).expect("plan prepares");
    assert_eq!(plan.harness_mode(), Some(HarnessMode::Plan));
    assert_eq!(
        plan.request().options().harness_mode(),
        Some(HarnessMode::Plan)
    );
    assert!(
        plan.plan().requirements().capabilities().any(|required| {
            required.capability() == Capability::HarnessModeSelection
                && required.constraints().any(|constraint| {
                    *constraint == CapabilityConstraint::HarnessMode(HarnessMode::Plan)
                })
        }),
        "prepared Plan requires HarnessModeSelection(Plan)"
    );
    assert_prepared_operation_evidence_matches_plan(plan.evidence(), plan.plan());
}

fn session_input(id: &str) -> ClineSessionProfileInput {
    ClineSessionProfileInput::new(
        RequestId::new(format!("cline.fixture.session.{id}")).expect("request"),
        WorkingResourceRef::new("cline.fixture.workspace").expect("resource"),
    )
}

fn probe() -> ClinePreparationProbe {
    ClinePreparationProbe::new(
        RequestId::new("cline.fixture.probe").expect("request"),
        ScopeId::new("cline.fixture.probe").expect("scope"),
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
