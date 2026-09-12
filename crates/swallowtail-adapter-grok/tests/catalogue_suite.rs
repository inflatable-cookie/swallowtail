#![allow(dead_code)]

#[path = "installed_discovery/support.rs"]
mod discovery_support;

use discovery_support::{FakeProcessService, ImmediateTime};
use futures_executor::block_on;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, GROK_BUILD_CATALOGUE_VERSION,
    GrokCatalogueProfileInput, GrokPreparationInput, GrokPreparationProbe, grok_build_acp_claim,
    grok_build_catalogue_claim, grok_build_model_for_version,
    grok_build_subscription_access_profile, prepare_grok_build,
};
use swallowtail_core::{
    AccessStatus, ConfiguredInstanceId, CredentialRef, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceCompatibilityAssessment,
    InterfaceVersion, InterfaceVersionAxis, ReasoningMode,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    ConsumerRouteProjectionSourceId, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, HostServices, InstalledExecutableTarget, MonotonicInstant,
    PreparedAccessEvidence, ProcessExit, ProcessOutputChunk, ProcessOutputStream, RequestId,
    ScopeId,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const MODELS: &str = include_str!("fixtures/grok-1.0.25-model-catalogue/models.txt");
const UNKNOWN_IDS: &str = include_str!("fixtures/grok-1.0.25-model-catalogue/unknown-ids.txt");
const VERSION_OUTPUT: &str = "grok 1.0.25 (f7e67d6988e2) [stable]\n";

fn catalogue_input(request: RequestId) -> GrokCatalogueProfileInput {
    GrokCatalogueProfileInput::new(request)
}

fn temporary_root(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time follows epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!(
        "swallowtail-grok-{prefix}-{}-{nanos}",
        std::process::id()
    ));
    std::fs::create_dir_all(&root).expect("temporary root is created");
    root
}

#[test]
fn catalogue_claim_admits_only_exact_1_0_25() {
    assert_eq!(GROK_BUILD_CATALOGUE_VERSION, "1.0.25");
    let claim = grok_build_catalogue_claim();
    let qualified = claim.assess(&version("1.0.25"));
    assert!(matches!(
        qualified,
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert_eq!(
        qualified
            .behavior_revision()
            .map(|revision| revision.as_str()),
        Some("grok-build.catalogue.models-text-v1")
    );
    for candidate in ["0.2.117", "1.0.4", "1.0.5", "1.0.24", "1.0.26"] {
        assert_eq!(
            claim.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Incompatible,
            "{candidate} must fail closed"
        );
        assert!(!claim.permits(&version(candidate)));
    }
}

#[test]
fn acp_execution_claim_stays_unchanged() {
    assert_eq!(GROK_BUILD_ACP_LATEST_QUALIFIED_VERSION, "1.0.5");
    let claim = grok_build_acp_claim();
    assert!(matches!(
        claim.assess(&version("1.0.4")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert!(matches!(
        claim.assess(&version("1.0.5")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert!(!matches!(
        claim.assess(&version("1.0.25")),
        InterfaceCompatibilityAssessment::Qualified(_)
    ));
    assert_eq!(
        grok_build_model_for_version(&version("1.0.4")),
        Some("grok-4.6")
    );
    assert_ne!(
        grok_build_acp_claim().id(),
        grok_build_catalogue_claim().id()
    );
}

#[test]
fn catalogue_prepares_on_exact_1_0_25_and_lists_the_accepted_document() {
    let host_id = host_id("catalogue.accept");
    let prepared = prepared_integration("catalogue.accept", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.accept")))
        .expect("catalogue prepares on exact 1.0.25");
    assert_prepared_operation_evidence_matches_plan(catalogue.evidence(), catalogue.plan());
    assert_eq!(
        catalogue.plan().driver_identity().id().as_str(),
        "swallowtail.grok-build.catalogue"
    );
    assert_eq!(
        catalogue.plan().requirements().driver_role(),
        swallowtail_core::DriverRole::ModelCatalog
    );
    assert_eq!(
        catalogue.plan().instance_target_ref().as_host_value(),
        "catalogue.accept.executable"
    );
    // The catalogue is an authenticated metadata operation under the prepared
    // ambient posture; it is not a provider-suppressed operation.
    assert_eq!(
        catalogue.plan().harness_configuration_posture(),
        Some(swallowtail_core::HarnessConfigurationPosture::Ambient)
    );
    assert_eq!(
        catalogue
            .plan()
            .requirements()
            .harness_configuration_posture(),
        Some(swallowtail_core::HarnessConfigurationPosture::Ambient)
    );

    let (process, state) = FakeProcessService::completed(MODELS);
    let listing = block_on(catalogue.list_models_recorded(services(host_id.clone(), process)))
        .expect("catalogue executes");
    let models = listing.entries();
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id().as_str(), "grok-4.6");
    assert!(models[0].metadata().is_default());
    assert_eq!(models[0].metadata().display_name(), Some("Grok 4.6"));
    assert_eq!(
        models[0].metadata().description(),
        Some("SpaceXAI's latest frontier model")
    );
    assert_eq!(
        models[0]
            .metadata()
            .token_limits()
            .expect("context window maps")
            .maximum_input_tokens(),
        Some(500_000)
    );
    let reasoning = models[0].metadata().reasoning().expect("reasoning maps");
    assert_eq!(
        reasoning
            .supported_modes()
            .map(ReasoningMode::as_str)
            .collect::<Vec<_>>(),
        ["high", "low", "medium", "xhigh"]
    );
    assert_eq!(
        reasoning.default_mode().map(ReasoningMode::as_str),
        Some("high")
    );
    assert_eq!(models[1].id().as_str(), "grok-4.5");
    assert!(!models[1].metadata().is_default());
    assert_eq!(models[1].metadata().display_name(), Some("Grok 4.5"));
    assert_eq!(models[1].metadata().description(), None);
    assert!(models.iter().all(|model| model.provider_id().is_none()));
    assert_eq!(listing.evidence().stdout_bytes(), MODELS.len());
    assert_eq!(listing.evidence().stderr_bytes(), 0);
    assert_eq!(listing.evidence().stdout_sha256().len(), 64);
    assert_eq!(listing.evidence().stderr_sha256().len(), 64);

    let observed = state.request();
    assert_eq!(observed.executable, "catalogue.accept.executable");
    assert_eq!(observed.arguments, ["--no-auto-update", "models"]);
    assert_eq!(observed.environments, ["catalogue.accept.environment"]);
    assert!(state.stdin_closed());
    assert!(state.waited());
}

#[test]
fn catalogue_preparation_rejects_non_1_0_25_observation() {
    let prepared = prepared_integration("catalogue.reject", "grok 1.0.5 (5115b46bc909) [stable]\n");
    let error = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.reject")))
        .expect_err("1.0.5 cannot prepare a catalogue");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.grok.preparation.catalogue_version_incompatible"
    );
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::CompatibilityClassification
    );
}

#[test]
fn access_not_ready_blocks_preparation_before_catalogue() {
    let prefix = "catalogue.access";
    let host = ExecutionHostId::new(format!("{prefix}.host")).expect("host");
    let credential = CredentialRef::new(format!("{prefix}.credential")).expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Expired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        swallowtail_core::RuntimeReadiness::Ready,
        swallowtail_core::SupportAuthority::ProviderSupported,
    );
    let target = InstalledExecutableTarget::new(
        ExecutableRef::new(format!("{prefix}.executable")).expect("target"),
        InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
    );
    let (process, _) = FakeProcessService::completed(VERSION_OUTPUT);
    let error = block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new(format!("{prefix}.instance")).expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host.clone(),
            target,
            EnvironmentRef::new(format!("{prefix}.environment")).expect("environment"),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new(format!("{prefix}.probe")).expect("request"),
            ScopeId::new(format!("{prefix}.scope")).expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        ),
        services(host, process),
    ))
    .expect_err("expired credential blocks preparation");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.grok.preparation.access_evidence_mismatch"
    );
}

#[test]
fn unknown_ids_pass_through_with_empty_metadata() {
    let host_id = host_id("catalogue.unknown");
    let prepared = prepared_integration("catalogue.unknown", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.unknown")))
        .expect("catalogue prepares");
    let (process, _) = FakeProcessService::completed(UNKNOWN_IDS);
    let models =
        block_on(catalogue.list_models(services(host_id, process))).expect("catalogue executes");
    assert_eq!(models.len(), 3);
    let unknown = &models[2];
    assert_eq!(unknown.id().as_str(), "grok-4.7");
    assert!(!unknown.metadata().is_default());
    assert_eq!(unknown.metadata().display_name(), None);
    assert_eq!(unknown.metadata().description(), None);
    assert_eq!(unknown.metadata().token_limits(), None);
    assert_eq!(unknown.metadata().reasoning(), None);
}

#[test]
fn live_bullet_grammar_is_required_and_auth_preamble_is_tolerated() {
    let host_id = host_id("catalogue.grammar");
    let prepared = prepared_integration("catalogue.grammar", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.grammar")))
        .expect("catalogue prepares");
    // A bare two-space row is not the shipped exact-1.0.25 grammar.
    let (process, _) = FakeProcessService::completed(
        "Default model: grok-4.6\nAvailable models:\n  grok-4.6 (default)\n  grok-4.5\n",
    );
    let error = block_on(catalogue.list_models(services(host_id.clone(), process)))
        .expect_err("a bare row fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_invalid"
    );
    // The shipped auth-status preamble and bullet rows parse.
    let (process, _) = FakeProcessService::completed(MODELS);
    let models = block_on(catalogue.list_models(services(host_id, process)))
        .expect("the shipped grammar parses");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id().as_str(), "grok-4.6");
    assert!(models[0].metadata().is_default());
}

#[test]
fn malformed_output_fails_closed_after_join_with_sizes() {
    let host_id = host_id("catalogue.malformed");
    let prepared = prepared_integration("catalogue.malformed", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.malformed")))
        .expect("catalogue prepares");
    let document =
        "Default model: grok-4.5\nAvailable models:\n  * grok-4.6 (default)\n  - grok-4.5\n";
    let (process, state) = FakeProcessService::completed(document);
    let error = block_on(catalogue.list_models(services(host_id, process)))
        .expect_err("header and marker disagreement fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_default_ambiguous"
    );
    assert!(
        error
            .diagnostic()
            .message()
            .contains(&format!("stdout {} bytes", document.len()))
    );
    assert!(error.diagnostic().message().contains("stderr 0 bytes"));
    assert!(state.stdin_closed());
    assert!(state.waited());
}

#[test]
fn empty_stdout_reports_zero_captured_bytes() {
    let host_id = host_id("catalogue.empty");
    let prepared = prepared_integration("catalogue.empty", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.empty")))
        .expect("catalogue prepares");
    let (process, _) = FakeProcessService::completed("");
    let error = block_on(catalogue.list_models(services(host_id, process)))
        .expect_err("empty stdout fails closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_invalid"
    );
    assert!(
        error.diagnostic().message().contains("stdout 0 bytes")
            && error.diagnostic().message().contains("stderr 0 bytes")
    );
}

#[test]
fn stderr_is_captured_as_redacted_evidence() {
    let host_id = host_id("catalogue.stderr");
    let prepared = prepared_integration("catalogue.stderr", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.stderr")))
        .expect("catalogue prepares");
    let secret = "authentication failed for user@example.com token=private";
    let (process, _) = FakeProcessService::scripted(
        vec![
            ProcessOutputChunk::new(ProcessOutputStream::Stdout, MODELS.as_bytes().to_vec()),
            ProcessOutputChunk::new(ProcessOutputStream::Stderr, secret.as_bytes().to_vec()),
        ],
        ProcessExit::new(true, Some(0)),
    );
    let listing = block_on(catalogue.list_models_recorded(services(host_id, process)))
        .expect("a successful catalogue keeps its redacted evidence");
    assert_eq!(listing.entries().len(), 2);
    assert_eq!(listing.evidence().stderr_bytes(), secret.len());
    assert_eq!(listing.evidence().stderr_sha256().len(), 64);
    assert!(!format!("{:?}", listing.evidence()).contains("user@example.com"));
    assert!(!format!("{listing:?}").contains("token=private"));
}

#[test]
fn non_zero_exit_reports_status_without_stderr_secrets() {
    let host_id = host_id("catalogue.exit");
    let prepared = prepared_integration("catalogue.exit", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.exit")))
        .expect("catalogue prepares");
    let secret = "authentication failed for user@example.com token=private";
    let (process, state) = FakeProcessService::scripted(
        vec![
            ProcessOutputChunk::new(ProcessOutputStream::Stdout, MODELS.as_bytes().to_vec()),
            ProcessOutputChunk::new(ProcessOutputStream::Stderr, secret.as_bytes().to_vec()),
        ],
        ProcessExit::new(false, Some(1)),
    );
    let error = block_on(catalogue.list_models(services(host_id, process)))
        .expect_err("failed catalogue is diagnosed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_exit_failed"
    );
    assert!(error.diagnostic().message().contains("status 1"));
    assert!(!error.diagnostic().message().contains("user@example.com"));
    assert!(!error.diagnostic().message().contains("token=private"));
    assert!(state.waited());
}

#[test]
fn elapsed_deadline_rejects_before_spawn() {
    let host_id = host_id("catalogue.elapsed");
    let prepared = prepared_integration("catalogue.elapsed", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(
            catalogue_input(request_id("catalogue.elapsed"))
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(0))),
        )
        .expect("catalogue prepares");
    let (process, state) = FakeProcessService::completed(MODELS);
    let error = block_on(catalogue.list_models(timed_services(host_id, process)))
        .expect_err("elapsed deadline fails before spawn");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_deadline_elapsed"
    );
    assert!(!state.started());
}

#[test]
fn deadline_during_read_times_out_with_joined_cleanup() {
    let host_id = host_id("catalogue.timeout");
    let prepared = prepared_integration("catalogue.timeout", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(
            catalogue_input(request_id("catalogue.timeout"))
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
        )
        .expect("catalogue prepares");
    let (process, state) = FakeProcessService::held_open();
    let error = block_on(catalogue.list_models(timed_services(host_id, process)))
        .expect_err("stalled catalogue times out");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_timed_out"
    );
    assert!(state.force_stopped());
    assert!(state.waited());
}

#[test]
fn cleanup_failure_surfaces_when_join_fails() {
    let host_id = host_id("catalogue.cleanup");
    let prepared = prepared_integration("catalogue.cleanup", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.cleanup")))
        .expect("catalogue prepares");
    let (process, state) = FakeProcessService::wait_failed(MODELS);
    let error = block_on(catalogue.list_models(services(host_id, process)))
        .expect_err("failed join is diagnosed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.grok.catalogue_cleanup_failed"
    );
    assert!(state.waited());
}

#[test]
fn host_mismatch_fails_before_spawn() {
    let prepared = prepared_integration("catalogue.host", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.host")))
        .expect("catalogue prepares");
    let other = ExecutionHostId::new("catalogue.host.other").expect("host");
    let (process, state) = FakeProcessService::completed(MODELS);
    let error =
        block_on(catalogue.list_models(services(other, process))).expect_err("host mismatch fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!state.started());
}

#[test]
fn catalogue_projection_emits_only_catalogue_rows() {
    let prepared = prepared_integration("catalogue.projection", VERSION_OUTPUT);
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.projection")))
        .expect("catalogue prepares");
    let contribution = catalogue
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("catalogue.projection.source").expect("source"),
        )
        .expect("catalogue projection");
    let selection = contribution
        .selection_rows()
        .map(|row| format!("{:?}", row.identity()))
        .collect::<Vec<_>>();
    assert_eq!(
        selection,
        [
            "Feature(PreparedFacade)",
            "Feature(ModelCatalogue)",
            "Feature(UsageEvidence)",
        ]
    );
    assert_eq!(contribution.session_start_rows().count(), 0);
    assert_eq!(contribution.active_session_rows().count(), 0);
}

#[test]
fn real_local_host_runs_the_bounded_catalogue_command() {
    let root = temporary_root("catalogue.local-host");
    let script = root.join("fake-grok");
    let script_body = "#!/bin/sh\nfor probe_arg in \"$@\"; do\n  case \"$probe_arg\" in\n    --version) printf 'grok 1.0.25 (f7e67d6988e2) [stable]\\n'; exit 0;;\n    models) printf 'Default model: grok-4.6\\nAvailable models:\\n  * grok-4.6 (default)\\n  - grok-4.5\\n'; exit 0;;\n  esac\ndone\nexit 3\n";
    std::fs::write(&script, script_body).expect("script");
    let mut permissions = std::fs::metadata(&script).expect("metadata").permissions();
    std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o700);
    std::fs::set_permissions(&script, permissions).expect("executable");

    let execution_host_id = ExecutionHostId::new("catalogue.local-host.host").expect("host");
    let executable = ExecutableRef::new("catalogue.local-host.executable").expect("executable");
    let environment = EnvironmentRef::new("catalogue.local-host.environment").expect("environment");
    let (builder, target) = LocalProcessHost::builder(LocalProcessLimits::default())
        .approve_installed_executable(
            executable,
            InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
            &script,
        );
    let local = builder
        .approve_environment(
            environment.clone(),
            Vec::<(std::ffi::OsString, std::ffi::OsString)>::new(),
        )
        .build_services(execution_host_id.clone());

    let now = local
        .services()
        .time()
        .expect("local host has time service")
        .now();
    let deadline = Deadline::at(MonotonicInstant::from_ticks(
        now.ticks().saturating_add(60_000_000_000),
    ));
    let credential = CredentialRef::new("catalogue.local-host.credential").expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        swallowtail_core::RuntimeReadiness::Ready,
        swallowtail_core::SupportAuthority::ProviderSupported,
    );
    let prepared = block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new("catalogue.local-host.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            execution_host_id.clone(),
            target,
            environment,
            access,
            PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new("catalogue.local-host.probe").expect("request"),
            ScopeId::new("catalogue.local-host.scope").expect("scope"),
            deadline,
            DiscoveryCancellation::new(),
        ),
        local.services().clone(),
    ))
    .expect("installed discovery prepares through the real local host");
    let catalogue = prepared
        .prepare_catalogue(catalogue_input(request_id("catalogue.local-host.models")))
        .expect("catalogue prepares");
    let listing = block_on(catalogue.list_models_recorded(local.services().clone()))
        .expect("catalogue executes through the real local host");
    assert_eq!(listing.entries().len(), 2);
    assert_eq!(listing.entries()[0].id().as_str(), "grok-4.6");
    assert!(listing.entries()[0].metadata().is_default());
    assert_eq!(listing.entries()[1].id().as_str(), "grok-4.5");
    assert_eq!(listing.evidence().stderr_bytes(), 0);
    let _ = std::fs::remove_dir_all(&root);
}

fn prepared_integration(
    prefix: &str,
    version_output: &str,
) -> swallowtail_adapter_grok::GrokPreparedIntegration {
    let host = ExecutionHostId::new(format!("{prefix}.host")).expect("host");
    let credential = CredentialRef::new(format!("{prefix}.credential")).expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        swallowtail_core::RuntimeReadiness::Ready,
        swallowtail_core::SupportAuthority::ProviderSupported,
    );
    let target = InstalledExecutableTarget::new(
        ExecutableRef::new(format!("{prefix}.executable")).expect("target"),
        InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
    );
    let (process, _) = FakeProcessService::completed(version_output);
    block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new(format!("{prefix}.instance")).expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host.clone(),
            target,
            EnvironmentRef::new(format!("{prefix}.environment")).expect("environment"),
            access,
            PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new(format!("{prefix}.probe")).expect("request"),
            ScopeId::new(format!("{prefix}.scope")).expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            DiscoveryCancellation::new(),
        ),
        discovery_support::services(host, process),
    ))
    .expect("prepares")
}

fn host_id(prefix: &str) -> ExecutionHostId {
    ExecutionHostId::new(format!("{prefix}.host")).expect("host")
}

fn request_id(value: &str) -> RequestId {
    RequestId::new(value).expect("request id")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

fn services(host: ExecutionHostId, process: Arc<FakeProcessService>) -> HostServices {
    discovery_support::services(host, process)
}

fn timed_services(host: ExecutionHostId, process: Arc<FakeProcessService>) -> HostServices {
    discovery_support::services_with_time(host, process, Arc::new(ImmediateTime))
}
