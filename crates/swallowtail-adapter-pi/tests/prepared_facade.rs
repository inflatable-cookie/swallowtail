#![allow(dead_code, unused_imports)]

mod support;

use futures_executor::block_on;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_pi::{
    PI_PACKAGE_AXIS, PiModelSelection, PiPreparationInput, PiPreparationProbe,
    PiSessionProfileInput, prepare_pi_rpc,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExtensionNamespace, HarnessConfigurationPosture,
    HarnessIsolation, InstalledExecutableCompatibility, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ResourceAccess, RuntimeReadiness,
    SessionAccessPolicy, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, RequestId, ScopeId,
    SessionOptions, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

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
