#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PiCatalogueProfileInput, PiModelSelection, PiPreparationInput,
    PiPreparationProbe, PiRunProfileInput, PiSessionProfileInput, prepare_pi_rpc,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, ConfiguredInstanceId,
    CredentialMechanism, CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization,
    EntitlementMetering, EntitlementState, ExecutionHostId, ExtensionNamespace,
    HarnessConfigurationPosture, HarnessIsolation, InstalledExecutableCompatibility,
    InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
    ResourceAccess, RuntimeReadiness, SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    ProviderRetentionPolicy, RequestId, ScopeId, SessionOptions, TerminalStatus,
    WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

#[test]
fn prepared_catalogues_ask_pi_for_configured_models_without_selecting_one() {
    for host_value in ["fixture.pi.catalogue.local", "fixture.pi.catalogue.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("0.80.10");
        let prepared = block_on(prepare_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("Pi prepares");
        let profile = prepared
            .prepare_catalogue(PiCatalogueProfileInput::new(
                RequestId::new("pi-prepared-catalogue").expect("valid request"),
            ))
            .expect("Pi catalogue profile prepares");

        assert!(profile.plan().provider_id().is_none());
        assert!(profile.plan().model_id().is_none());
        assert!(profile.plan().model_route_id().is_none());
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == Capability::ModelCatalog)
        );
        assert!(
            !profile
                .plan()
                .requirements()
                .host_services()
                .any(|service| service == swallowtail_core::HostServiceKind::WorkingResource)
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let operation = FixtureHost::new(Scenario::Complete);
        let models =
            block_on(profile.list_models(operation.services(host_id))).expect("Pi catalogue loads");
        assert_eq!(models.len(), 2);
        assert_eq!(models[0].id().as_str(), "fixture-model");
        assert_eq!(
            models[0].provider_id().map(ProviderId::as_str),
            Some("fixture-provider")
        );
        assert_eq!(
            operation.process_arguments(),
            [
                "--mode",
                "rpc",
                "--no-session",
                "--offline",
                "--no-tools",
                "--no-extensions",
                "--no-skills",
                "--no-prompt-templates",
                "--no-themes",
                "--no-context-files",
            ]
        );
        assert_eq!(
            operation.cleanup_events(),
            [
                support::CleanupEvent::ProcessWait,
                support::CleanupEvent::CredentialRelease,
            ]
        );
        assert!(!format!("{models:?}").contains("fixture-private.invalid"));
    }
}

#[test]
fn prepared_catalogues_bound_failures_deadlines_and_cleanup() {
    for (index, scenario, code) in [
        (
            1,
            Scenario::Malformed,
            "swallowtail.pi.rpc.catalogue_invalid",
        ),
        (
            2,
            Scenario::ProviderFailure,
            "swallowtail.pi.rpc.catalogue_rejected",
        ),
        (
            3,
            Scenario::ResponseMismatch,
            "swallowtail.pi.rpc.response_command_mismatch",
        ),
        (
            4,
            Scenario::Disconnect,
            "swallowtail.pi.rpc.connection_ended",
        ),
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.pi.catalogue.failure.{index}"))
            .expect("valid host");
        let profile = prepared_catalogue(host_id.clone(), None);
        let operation = FixtureHost::new(scenario);
        let error = block_on(profile.list_models(operation.services(host_id)))
            .expect_err("Pi catalogue failure remains typed");
        assert_eq!(error.diagnostic().code(), code);
        assert_eq!(
            operation.cleanup_events(),
            [
                support::CleanupEvent::ProcessWait,
                support::CleanupEvent::CredentialRelease,
            ]
        );
        assert!(!format!("{error:?}").contains("fixture private"));
    }

    let host_id = ExecutionHostId::new("fixture.pi.catalogue.timeout").expect("valid host");
    let profile = prepared_catalogue(
        host_id.clone(),
        Some(Deadline::at(MonotonicInstant::from_ticks(1_001))),
    );
    let operation = FixtureHost::new(Scenario::Hold).with_immediate_time();
    let error = block_on(profile.list_models(operation.services(host_id)))
        .expect_err("Pi catalogue deadline fires");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.catalogue_timed_out"
    );
    assert_eq!(
        operation.cleanup_events(),
        [
            support::CleanupEvent::ProcessWait,
            support::CleanupEvent::CredentialRelease,
        ]
    );
}

#[test]
fn catalogue_success_does_not_hide_process_cleanup_failure() {
    let host_id = ExecutionHostId::new("fixture.pi.catalogue.cleanup").expect("valid host");
    let profile = prepared_catalogue(host_id.clone(), None);
    let operation = FixtureHost::new(Scenario::Complete).with_process_wait_failure();
    let error = block_on(profile.list_models(operation.services(host_id)))
        .expect_err("Pi catalogue cleanup failure wins");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.rpc.catalogue_cleanup_failed"
    );
    assert_eq!(
        operation.cleanup_events(),
        [
            support::CleanupEvent::ProcessWait,
            support::CleanupEvent::CredentialRelease,
        ]
    );
}

#[test]
fn prepared_sessions_preserve_pi_rpc_policy_in_both_host_topologies() {
    for host_value in ["fixture.pi.prepared.local", "fixture.pi.prepared.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("0.80.10");
        let prepared = block_on(prepare_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("Pi prepares");
        assert_eq!(discovery.process_arguments(), ["--version"]);

        let profile = prepared
            .prepare_session(PiSessionProfileInput::new(
                RequestId::new("pi-prepared-open").expect("valid request"),
                PiModelSelection::new(
                    ModelRouteId::new("pi.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("fixture-provider").expect("valid provider"),
                    ModelId::new("fixture-model").expect("valid model"),
                ),
                WorkingResourceRef::new("pi.prepared.workspace").expect("valid resource"),
                SessionOptions::default(),
            ))
            .expect("Pi session profile prepares");

        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::ProviderSuppressed)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert_eq!(
            profile.plan().provider_id().map(ProviderId::as_str),
            Some("fixture-provider")
        );
        assert!(profile.plan().harness_rpc_policy().is_some());
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );

        let operation = FixtureHost::new(Scenario::Complete);
        let session = block_on(profile.open_session(operation.services(host_id)))
            .expect("prepared Pi session opens");
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_runs_preserve_the_one_prompt_rpc_projection_in_both_host_topologies() {
    for host_value in ["fixture.pi.run.local", "fixture.pi.run.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let discovery = FixtureHost::version_probe("0.80.10");
        let prepared = block_on(prepare_pi_rpc(
            preparation_input(host_id.clone()),
            probe(),
            discovery.services(host_id.clone()),
        ))
        .expect("Pi prepares");
        let run = prepared
            .prepare_run(PiRunProfileInput::new(
                RequestId::new("pi-prepared-run").expect("valid request"),
                PiModelSelection::new(
                    ModelRouteId::new("pi.prepared.run.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ProviderId::new("fixture-provider").expect("valid provider"),
                    ModelId::new("fixture-model").expect("valid model"),
                ),
                OperationContent::new("fixture private prompt").expect("valid content"),
                WorkingResourceRef::new("pi.prepared.workspace").expect("valid resource"),
                Deadline::at(MonotonicInstant::from_ticks(100_000)),
            ))
            .expect("Pi run profile prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::Prohibited
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());

        let operation = FixtureHost::new(Scenario::Complete);
        let mut handle =
            block_on(run.start_run(operation.services(host_id))).expect("prepared run starts");
        let mut events = handle.take_events().expect("events are available");
        let terminal = handle
            .take_terminal_outcome()
            .expect("terminal outcome is available");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("runtime event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn later_stable_pi_is_visible_and_executable_as_unverified_newer() {
    let host_id = ExecutionHostId::new("fixture.pi.prepared.newer").expect("valid host");
    let discovery = FixtureHost::version_probe("0.81.1");
    let prepared = block_on(prepare_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("newer Pi remains executable");
    assert!(matches!(
        prepared.observation().compatibility(),
        InstalledExecutableCompatibility::UnverifiedNewer(_)
    ));
}

fn preparation_input(host: ExecutionHostId) -> PiPreparationInput {
    PiPreparationInput::new(
        ConfiguredInstanceId::new("pi.prepared").expect("valid instance"),
        InstanceRevision::new("1").expect("valid revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("pi.prepared.executable").expect("valid executable"),
            InterfaceVersionAxis::new(PI_PACKAGE_AXIS).expect("valid axis"),
        ),
        EnvironmentRef::new("pi.prepared.environment").expect("valid environment"),
        AccessProfile::new(
            AccessProfileId::new("pi.prepared.access").expect("valid access"),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("pi/delegated-harness-auth").expect("valid namespace"),
            ),
            EntitlementMetering::Unknown,
            EndpointAudience::new("pi-harness").expect("valid audience"),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(
            CredentialRef::new("pi.prepared.credential").expect("valid credential"),
        ),
        PreparedAccessEvidence::caller_asserted(access_status()),
    )
}

fn access_status() -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new("pi.prepared.access").expect("valid access"),
        CredentialState::Ready,
        EntitlementState::Unknown,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    )
}

fn probe() -> PiPreparationProbe {
    PiPreparationProbe::new(
        RequestId::new("pi-prepared-probe").expect("valid request"),
        ScopeId::new("pi-prepared-probe").expect("valid scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn prepared_catalogue(
    host_id: ExecutionHostId,
    deadline: Option<Deadline>,
) -> swallowtail_adapter_pi::PiPreparedCatalogue {
    let discovery = FixtureHost::version_probe("0.80.10");
    let prepared = block_on(prepare_pi_rpc(
        preparation_input(host_id.clone()),
        probe(),
        discovery.services(host_id),
    ))
    .expect("Pi prepares");
    let input = PiCatalogueProfileInput::new(
        RequestId::new("pi-prepared-catalogue-failure").expect("valid request"),
    );
    let input = match deadline {
        Some(deadline) => input.with_deadline(deadline),
        None => input,
    };
    prepared
        .prepare_catalogue(input)
        .expect("Pi catalogue profile prepares")
}
