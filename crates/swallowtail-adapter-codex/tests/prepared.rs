#[path = "prepared_cases/mod.rs"]
mod prepared_cases;
use crate::support;

use futures_executor::block_on;
use std::sync::Arc;
use support::{FakeProcessService, host_services_for};
use swallowtail_adapter_codex::{
    CODEX_CLI_AXIS, CodexPreparationInput, CodexPreparationProbe, CodexPreparedDriver,
    prepare_codex,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, HarnessConfigurationPosture,
    InstalledExecutableCompatibility, InstanceOwnership, InstanceRevision, InterfaceSupportStatus,
    InterfaceVersionAxis, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    AccessEvidenceProvenance, AccessEvidenceSourceId, BoxFuture, CancellationControl, Deadline,
    DeadlineObservation, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, PreparationStage, PreparedAccessEvidence,
    ProcessHandle, ProcessRequest, ProcessService, RequestId, RuntimeFailure, ScopeId, TimeService,
};
use swallowtail_testkit::RecordingHostServices;

#[test]
fn process_spawn_output_exit_and_cleanup_failures_keep_their_stages() {
    let cases: Vec<(Arc<dyn ProcessService>, PreparationStage, &'static str)> = vec![
        (
            Arc::new(StartFailure),
            PreparationStage::ProcessSpawn,
            "swallowtail.codex.discovery_spawn_failed",
        ),
        {
            let (process, _) = FakeProcessService::failed_output(false);
            (
                process,
                PreparationStage::BoundedOutput,
                "swallowtail.codex.discovery_output_failed",
            )
        },
        {
            let (process, _) = FakeProcessService::failed_exit();
            (
                process,
                PreparationStage::ProcessExit,
                "swallowtail.codex.discovery_exit_failed",
            )
        },
        {
            let (process, _) = FakeProcessService::failed_output(true);
            (
                process,
                PreparationStage::Cleanup,
                "swallowtail.codex.discovery_cleanup_failed",
            )
        },
    ];

    for (process, expected_stage, expected_code) in cases {
        let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
        let failure = block_on(prepare_codex(
            fixture.input,
            fixture.probe,
            services(fixture.host, process),
        ))
        .expect_err("failed probe must not promote");

        assert_eq!(failure.stage(), expected_stage);
        assert_eq!(failure.diagnostic().safe().code(), expected_code);
    }
}

#[test]
fn nonzero_version_probe_exit_reports_status_and_sanitized_stderr() {
    let stderr = format!(
        "\u{1b}[31mcmux wrapper could not start Codex from /Users/private/bin \
         token=private {}after-capture-bound\u{1b}[0m",
        "detail ".repeat(200)
    );
    let (process, _) = FakeProcessService::failed_exit_with(126, stderr);
    let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
    let failure = block_on(prepare_codex(
        fixture.input,
        fixture.probe,
        services(fixture.host, process),
    ))
    .expect_err("non-zero probe exit must not promote");
    let diagnostic = failure.diagnostic().safe();

    assert_eq!(failure.stage(), PreparationStage::ProcessExit);
    assert_eq!(diagnostic.code(), "swallowtail.codex.discovery_exit_failed");
    assert!(diagnostic.message().contains("status 126"));
    assert!(
        diagnostic
            .message()
            .contains("cmux wrapper could not start Codex")
    );
    assert!(diagnostic.message().contains("<path>"));
    assert!(diagnostic.message().contains("<redacted>"));
    assert!(diagnostic.message().contains("[stderr truncated]"));
    for private in [
        "/Users/private",
        "token=private",
        "after-capture-bound",
        "\u{1b}",
    ] {
        assert!(!diagnostic.message().contains(private));
    }
}

#[test]
fn preparation_rejects_host_target_and_access_drift_before_starting_a_process() {
    let fixture = fixture(CodexPreparedDriver::StructuredExec, "host.local", "codex");
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let failure = block_on(prepare_codex(
        fixture.input.clone(),
        probe(),
        services(ExecutionHostId::new("host.other").unwrap(), process),
    ))
    .expect_err("host drift fails");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    assert!(!state.started());

    let mismatched_access = CodexPreparationInput::new(
        CodexPreparedDriver::StructuredExec,
        ConfiguredInstanceId::new("codex").unwrap(),
        InstanceRevision::new("1").unwrap(),
        fixture.host.clone(),
        fixture.target.clone(),
        EnvironmentRef::new("saved-login").unwrap(),
        access_profile(),
        PreparedAccessEvidence::caller_asserted(access_status("different-access")),
    );
    let (process, state) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let failure = block_on(prepare_codex(
        mismatched_access,
        fixture.probe,
        services(fixture.host, process),
    ))
    .expect_err("access drift fails");
    assert_eq!(failure.stage(), PreparationStage::AccessEvidence);
    assert!(!state.started());
}

#[test]
fn prepared_binding_rejects_later_host_or_target_substitution() {
    let fixture = fixture(
        CodexPreparedDriver::AppServer,
        "host.local",
        "/private/secret/codex-app-server",
    );
    let (process, _) = FakeProcessService::completed("codex-cli 0.145.0\n");
    let prepared = block_on(prepare_codex(
        fixture.input,
        fixture.probe,
        services(fixture.host.clone(), process),
    ))
    .expect("preparation succeeds");

    prepared
        .validate_execution_binding(&fixture.host, &fixture.target)
        .expect("exact binding remains valid");
    let failure = prepared
        .validate_execution_binding(
            &ExecutionHostId::new("host.other").unwrap(),
            &fixture.target,
        )
        .expect_err("host substitution fails");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    let failure = prepared
        .validate_execution_binding(&fixture.host, &target("other-codex"))
        .expect_err("target substitution fails");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
    let debug = format!("{prepared:?}");
    assert!(!debug.contains("saved-login"));
    assert!(!debug.contains("/private/secret"));
}

struct Fixture {
    host: ExecutionHostId,
    target: InstalledExecutableTarget,
    input: CodexPreparationInput,
    probe: CodexPreparationProbe,
}

fn fixture(driver: CodexPreparedDriver, host: &str, executable: &str) -> Fixture {
    fixture_with_cancellation(driver, host, executable, DiscoveryCancellation::new())
}

fn fixture_with_cancellation(
    driver: CodexPreparedDriver,
    host: &str,
    executable: &str,
    cancellation: DiscoveryCancellation,
) -> Fixture {
    let host = ExecutionHostId::new(host).unwrap();
    let target = target(executable);
    let input = CodexPreparationInput::new(
        driver,
        ConfiguredInstanceId::new("codex.prepared").unwrap(),
        InstanceRevision::new("1").unwrap(),
        host.clone(),
        target.clone(),
        EnvironmentRef::new("saved-login").unwrap(),
        access_profile(),
        PreparedAccessEvidence::observed(
            access_status("access.codex"),
            AccessEvidenceSourceId::new("host-access-observer").unwrap(),
        ),
    );
    let probe = probe_with_cancellation(cancellation);
    Fixture {
        host,
        target,
        input,
        probe,
    }
}

fn access_profile() -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new("access.codex").unwrap(),
        CredentialMechanism::InteractiveOauth,
        EntitlementMetering::SubscriptionAllowance,
        EndpointAudience::new("codex").unwrap(),
        SupportAuthority::ProviderSupported,
    )
}

fn access_status(profile: &str) -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new(profile).unwrap(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn target(executable: &str) -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(executable).unwrap(),
        InterfaceVersionAxis::new(CODEX_CLI_AXIS).unwrap(),
    )
}

fn probe() -> CodexPreparationProbe {
    probe_with_cancellation(DiscoveryCancellation::new())
}

fn probe_with_cancellation(cancellation: DiscoveryCancellation) -> CodexPreparationProbe {
    CodexPreparationProbe::new(
        RequestId::new("prepare-codex").unwrap(),
        ScopeId::new("prepare-codex").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        cancellation,
    )
}

fn services(
    host: ExecutionHostId,
    process: Arc<dyn swallowtail_runtime::ProcessService>,
) -> swallowtail_runtime::HostServices {
    host_services_for(host, process).with_time(Arc::new(PendingTime))
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

struct StartFailure;

impl ProcessService for StartFailure {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.codex.fixture_spawn_failed",
                "Fixture process did not start",
            )))
        })
    }
}
