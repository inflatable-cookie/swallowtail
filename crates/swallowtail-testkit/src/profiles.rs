use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyntheticProfile {
    OneShotStructuredCli,
    LongLivedRpcHarness,
    LongLivedAcpHarness,
    PersistentAcpHarness,
    AttachedNetworkHarness,
    HostedDirectApi,
    ProviderManagedRemoteHarness,
    ConnectionScopedDirectSession,
    LocallyContinuedDirectSession,
    RealtimeMediaDirectSession,
    AttachedSelfHosted,
    OwnedSelfHosted,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConformanceAssertion {
    PreflightBeforeSideEffects,
    BoundSelection,
    StalePlanRejected,
    OrderedEvents,
    SingleTerminalOutcome,
    SemanticOverflowFails,
    CancellationAndTimeoutDistinct,
    CleanupRemainsVisible,
    ExternalOwnershipPreserved,
    Redaction,
    ScopedInputs,
    SchemaTransportOnly,
    ExtensionPolicyExplicit,
    NoImplicitFallback,
    ProcessLifecycle,
    SessionLifecycle,
    CallbackExchange,
    WorkingResourceCallback,
    PersistentSessionLifecycle,
    ReplayPhase,
    WorkingResourceWriteCallback,
    AmbientHarnessAuthority,
    DelegatedAuthentication,
    HostTopologyPreserved,
    AttachedNetworkHarnessLifecycle,
    HostedApiNeedsNoProcess,
    HostedEndpointCredentialBinding,
    DirectRunNoResource,
    DirectRunOutputBound,
    DirectSessionNoResource,
    ConnectionScopedLeaseLifecycle,
    BilledCostTurnScoped,
    NoImplicitSessionRecovery,
    ProviderEvidenceSeparated,
    AttachedServiceNeverStopped,
    OwnedServiceStops,
    OwnedArtifactLease,
    OwnedEndpointBinding,
    OwnedCleanupOrdered,
    ProviderManagedHarnessLifecycle,
    DurableRetentionExplicit,
    ManagedRecoveryExplicit,
    OwnedRemoteDeletionTruth,
    NativeBudgetIndependent,
    NoTranscriptDeletionClaim,
    RealtimeMediaBoundary,
    RealtimeMediaOrdering,
    RealtimeMediaInterruptionEndsSession,
    PlannedConnectionRollover,
    RolloverNoReplay,
    RolloverCleanupOrdered,
    InterfaceVersionQualified,
    HarnessPolicyExact,
    HarnessScheduling,
    CommandAcknowledgement,
    HarnessUiRelay,
    ExplicitAttemptAuthorization,
    ConsumerToolExchange,
    PrivateContinuationBounded,
    ProviderCachePosture,
    RequestScopedLeaseLifecycle,
    AttachedRuntimeBinding,
    RuntimeManagedResidency,
    ClosedCompatibilityWindow,
    HarnessConfigurationExact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceReport {
    profile: SyntheticProfile,
    passed: BTreeSet<ConformanceAssertion>,
}

impl ConformanceReport {
    pub(crate) fn new(profile: SyntheticProfile) -> Self {
        Self {
            profile,
            passed: BTreeSet::new(),
        }
    }

    pub(crate) fn record(&mut self, assertion: ConformanceAssertion) {
        self.passed.insert(assertion);
    }

    #[must_use]
    pub const fn profile(&self) -> SyntheticProfile {
        self.profile
    }

    #[must_use]
    pub fn covers(&self, assertion: ConformanceAssertion) -> bool {
        self.passed.contains(&assertion)
    }

    pub fn passed(&self) -> impl ExactSizeIterator<Item = ConformanceAssertion> + '_ {
        self.passed.iter().copied()
    }
}

#[must_use]
pub fn run_one_shot_structured_cli_profile() -> ConformanceReport {
    crate::profile_one_shot::run()
}

#[must_use]
pub fn run_structured_harness_native_boundary_assertions() -> ConformanceReport {
    crate::profile_harness_native::run()
}

#[must_use]
pub fn run_long_lived_rpc_profile() -> ConformanceReport {
    crate::profile_rpc::run()
}

#[must_use]
pub fn run_harness_rpc_contract_assertions() -> ConformanceReport {
    crate::profile_harness_rpc_contract::run()
}

#[must_use]
pub fn run_harness_configuration_boundary_assertions() -> Vec<ConformanceReport> {
    crate::profile_harness_configuration::run()
}

#[must_use]
pub fn run_long_lived_acp_profile() -> ConformanceReport {
    crate::profile_acp::run()
}

#[must_use]
pub fn run_persistent_acp_profile() -> ConformanceReport {
    crate::profile_persistent_acp::run()
}

#[must_use]
pub fn run_hosted_direct_api_profile() -> ConformanceReport {
    crate::profile_hosted::run()
}

#[must_use]
pub fn run_connection_scoped_direct_session_profile() -> ConformanceReport {
    crate::profile_direct_session::run()
}

#[must_use]
pub fn run_locally_continued_direct_session_profile() -> ConformanceReport {
    crate::profile_local_continuation::run()
}

#[must_use]
pub fn run_realtime_media_direct_session_profile() -> ConformanceReport {
    crate::profile_realtime_media::run()
}

#[must_use]
pub fn run_realtime_rollover_boundary_assertions() -> ConformanceReport {
    crate::profile_realtime_rollover::run()
}

#[must_use]
pub fn run_provider_managed_harness_profile() -> ConformanceReport {
    crate::profile_managed_harness::run()
}

#[must_use]
pub fn run_attached_network_harness_profile() -> ConformanceReport {
    crate::profile_network_harness::run()
}

#[must_use]
pub fn run_attached_self_hosted_profile() -> ConformanceReport {
    crate::profile_attached::run()
}

#[must_use]
pub fn run_attached_runtime_boundary_assertions() -> ConformanceReport {
    crate::profile_attached_runtime::run()
}

#[must_use]
pub fn run_owned_self_hosted_profile() -> ConformanceReport {
    crate::profile_owned::run()
}

#[must_use]
pub fn run_all_synthetic_profiles() -> Vec<ConformanceReport> {
    vec![
        run_one_shot_structured_cli_profile(),
        run_long_lived_rpc_profile(),
        run_long_lived_acp_profile(),
        run_persistent_acp_profile(),
        run_attached_network_harness_profile(),
        run_hosted_direct_api_profile(),
        run_provider_managed_harness_profile(),
        run_connection_scoped_direct_session_profile(),
        run_locally_continued_direct_session_profile(),
        run_realtime_media_direct_session_profile(),
        run_attached_self_hosted_profile(),
        run_owned_self_hosted_profile(),
    ]
}
