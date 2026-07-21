use swallowtail_core::PreflightDimension;
use swallowtail_testkit::{
    ConformanceAssertion, PreflightFixtureCase, SyntheticProfile,
    assert_preflight_rejection_without_side_effects, run_all_synthetic_profiles,
};

const COMMON_ASSERTIONS: [ConformanceAssertion; 14] = [
    ConformanceAssertion::PreflightBeforeSideEffects,
    ConformanceAssertion::BoundSelection,
    ConformanceAssertion::StalePlanRejected,
    ConformanceAssertion::OrderedEvents,
    ConformanceAssertion::SingleTerminalOutcome,
    ConformanceAssertion::SemanticOverflowFails,
    ConformanceAssertion::CancellationAndTimeoutDistinct,
    ConformanceAssertion::CleanupRemainsVisible,
    ConformanceAssertion::ExternalOwnershipPreserved,
    ConformanceAssertion::Redaction,
    ConformanceAssertion::ScopedInputs,
    ConformanceAssertion::SchemaTransportOnly,
    ConformanceAssertion::ExtensionPolicyExplicit,
    ConformanceAssertion::NoImplicitFallback,
];

#[test]
fn all_ten_profiles_cover_every_common_contract_assertion() {
    let reports = run_all_synthetic_profiles();
    assert_eq!(reports.len(), 10);

    for report in &reports {
        for assertion in COMMON_ASSERTIONS {
            assert!(
                report.covers(assertion),
                "{:?} missed {assertion:?}",
                report.profile()
            );
        }
    }
}

#[test]
fn each_profile_proves_its_shape_specific_boundary() {
    let reports = run_all_synthetic_profiles();
    let expected = [
        (
            SyntheticProfile::OneShotStructuredCli,
            ConformanceAssertion::ProcessLifecycle,
        ),
        (
            SyntheticProfile::LongLivedRpcHarness,
            ConformanceAssertion::SessionLifecycle,
        ),
        (
            SyntheticProfile::LongLivedAcpHarness,
            ConformanceAssertion::WorkingResourceCallback,
        ),
        (
            SyntheticProfile::PersistentAcpHarness,
            ConformanceAssertion::PersistentSessionLifecycle,
        ),
        (
            SyntheticProfile::HostedDirectApi,
            ConformanceAssertion::HostedApiNeedsNoProcess,
        ),
        (
            SyntheticProfile::ProviderManagedRemoteHarness,
            ConformanceAssertion::ProviderManagedHarnessLifecycle,
        ),
        (
            SyntheticProfile::ConnectionScopedDirectSession,
            ConformanceAssertion::ConnectionScopedLeaseLifecycle,
        ),
        (
            SyntheticProfile::AttachedNetworkHarness,
            ConformanceAssertion::AttachedNetworkHarnessLifecycle,
        ),
        (
            SyntheticProfile::AttachedSelfHosted,
            ConformanceAssertion::AttachedServiceNeverStopped,
        ),
        (
            SyntheticProfile::OwnedSelfHosted,
            ConformanceAssertion::OwnedServiceStops,
        ),
    ];

    for (profile, assertion) in expected {
        let report = reports
            .iter()
            .find(|report| report.profile() == profile)
            .expect("profile report exists");
        assert!(report.covers(assertion));
    }
}

#[test]
fn long_lived_acp_profile_proves_process_callback_and_topology_boundaries() {
    let reports = run_all_synthetic_profiles();
    let acp = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::LongLivedAcpHarness)
        .expect("ACP profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::ProcessLifecycle,
        ConformanceAssertion::WorkingResourceCallback,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(acp.covers(assertion));
    }
    assert!(!acp.covers(ConformanceAssertion::CallbackExchange));
}

#[test]
fn long_lived_rpc_profile_proves_callback_exchange() {
    let reports = run_all_synthetic_profiles();
    let rpc = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::LongLivedRpcHarness)
        .expect("RPC profile report exists");

    assert!(rpc.covers(ConformanceAssertion::CallbackExchange));
}

#[test]
fn persistent_acp_profile_adds_lifecycle_write_auth_and_ambient_boundaries() {
    let reports = run_all_synthetic_profiles();
    let persistent = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::PersistentAcpHarness)
        .expect("persistent ACP profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::ProcessLifecycle,
        ConformanceAssertion::PersistentSessionLifecycle,
        ConformanceAssertion::ReplayPhase,
        ConformanceAssertion::WorkingResourceWriteCallback,
        ConformanceAssertion::AmbientHarnessAuthority,
        ConformanceAssertion::DelegatedAuthentication,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(persistent.covers(assertion));
    }
}

#[test]
fn owned_profile_proves_artifact_endpoint_cleanup_and_topology_boundaries() {
    let reports = run_all_synthetic_profiles();
    let owned = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::OwnedSelfHosted)
        .expect("owned profile report exists");

    for assertion in [
        ConformanceAssertion::OwnedArtifactLease,
        ConformanceAssertion::OwnedEndpointBinding,
        ConformanceAssertion::OwnedCleanupOrdered,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(owned.covers(assertion));
    }
}

#[test]
fn hosted_profile_proves_contract_014_foundations() {
    let reports = run_all_synthetic_profiles();
    let hosted = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::HostedDirectApi)
        .expect("hosted profile report exists");

    for assertion in [
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectRunNoResource,
        ConformanceAssertion::ProviderEvidenceSeparated,
    ] {
        assert!(hosted.covers(assertion));
    }
}

#[test]
fn connection_scoped_profile_proves_contract_016_boundaries() {
    let reports = run_all_synthetic_profiles();
    let direct = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::ConnectionScopedDirectSession)
        .expect("direct-session profile report exists");

    for assertion in [
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::HostTopologyPreserved,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectSessionNoResource,
        ConformanceAssertion::ConnectionScopedLeaseLifecycle,
        ConformanceAssertion::BilledCostTurnScoped,
        ConformanceAssertion::NoImplicitSessionRecovery,
    ] {
        assert!(direct.covers(assertion));
    }
}

#[test]
fn managed_harness_profile_proves_contract_022_boundaries() {
    let reports = run_all_synthetic_profiles();
    let managed = reports
        .iter()
        .find(|report| report.profile() == SyntheticProfile::ProviderManagedRemoteHarness)
        .expect("managed-harness profile report exists");

    for assertion in [
        ConformanceAssertion::ProviderManagedHarnessLifecycle,
        ConformanceAssertion::DurableRetentionExplicit,
        ConformanceAssertion::ManagedRecoveryExplicit,
        ConformanceAssertion::OwnedRemoteDeletionTruth,
        ConformanceAssertion::CallbackExchange,
        ConformanceAssertion::HostTopologyPreserved,
    ] {
        assert!(managed.covers(assertion));
    }
}

#[test]
fn deliberate_violations_name_the_exact_dimension_before_effects() {
    for (case, dimension) in [
        (
            PreflightFixtureCase::MissingModelRoute,
            PreflightDimension::ModelRoute,
        ),
        (
            PreflightFixtureCase::RejectedAccess,
            PreflightDimension::Access,
        ),
        (
            PreflightFixtureCase::RejectedOwnership,
            PreflightDimension::Ownership,
        ),
        (
            PreflightFixtureCase::WrongExecutionHost,
            PreflightDimension::Topology,
        ),
    ] {
        assert_preflight_rejection_without_side_effects(case, dimension);
    }
}
