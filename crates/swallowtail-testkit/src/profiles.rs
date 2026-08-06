use std::collections::BTreeSet;

/// Synthetic provider topology exercised by one common conformance profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyntheticProfile {
    /// One bounded CLI invocation per operation.
    OneShotStructuredCli,
    /// Reusable request/response harness process.
    LongLivedRpcHarness,
    /// Reusable local ACP harness process.
    LongLivedAcpHarness,
    /// ACP harness with durable provider session state.
    PersistentAcpHarness,
    /// ACP harness reached through a remote network transport.
    RemoteAcpHarness,
    /// Externally owned harness reached through an attached network endpoint.
    AttachedNetworkHarness,
    /// Direct hosted model API without a harness process.
    HostedDirectApi,
    /// Provider-managed remote harness lifecycle.
    ProviderManagedRemoteHarness,
    /// Direct session whose continuation state belongs to one connection.
    ConnectionScopedDirectSession,
    /// Direct session continued from portable local state.
    LocallyContinuedDirectSession,
    /// Duplex realtime media session.
    RealtimeMediaDirectSession,
    /// Externally owned self-hosted model server.
    AttachedSelfHosted,
    /// Host-owned self-hosted model server.
    OwnedSelfHosted,
}

/// Portable invariant proven by a synthetic conformance profile.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConformanceAssertion {
    /// Preflight finishes before provider-side effects begin.
    PreflightBeforeSideEffects,
    /// Execution uses the exact prepared route and instance selection.
    BoundSelection,
    /// Stale preflight evidence is rejected.
    StalePlanRejected,
    /// Observable events retain their required ordering.
    OrderedEvents,
    /// An operation produces exactly one terminal outcome.
    SingleTerminalOutcome,
    /// Semantic event-buffer overflow fails instead of discarding evidence.
    SemanticOverflowFails,
    /// Cancellation and deadline expiry remain distinguishable.
    CancellationAndTimeoutDistinct,
    /// Cleanup failure or incompleteness remains observable.
    CleanupRemainsVisible,
    /// Swallowtail does not claim ownership of externally owned resources.
    ExternalOwnershipPreserved,
    /// Sensitive fixture values remain redacted from safe output.
    Redaction,
    /// Inputs remain bound to the admitted operation scope.
    ScopedInputs,
    /// Structured-output schemas travel through the host schema service only.
    SchemaTransportOnly,
    /// Provider extensions require an explicit policy.
    ExtensionPolicyExplicit,
    /// Unsupported selection does not silently fall back.
    NoImplicitFallback,
    /// Child-process start, stop, wait, and cleanup are observable.
    ProcessLifecycle,
    /// Interactive session open, turn, and close ordering is preserved.
    SessionLifecycle,
    /// Provider callbacks can be answered through the portable exchange.
    CallbackExchange,
    /// Working-resource callback authority remains explicit.
    WorkingResourceCallback,
    /// Durable provider session creation and resumption are explicit.
    PersistentSessionLifecycle,
    /// Loaded historical transcript is distinct from live events.
    ReplayPhase,
    /// Bounded working-resource writes use the callback exchange.
    WorkingResourceWriteCallback,
    /// Ambient harness policy is represented rather than inferred.
    AmbientHarnessAuthority,
    /// Delegated authentication remains distinct from secret credentials.
    DelegatedAuthentication,
    /// Configured and observed host topology remains exact.
    HostTopologyPreserved,
    /// Attached network harnesses are never treated as host-owned processes.
    AttachedNetworkHarnessLifecycle,
    /// Hosted APIs do not require process services.
    HostedApiNeedsNoProcess,
    /// Hosted endpoints and credentials remain audience-bound.
    HostedEndpointCredentialBinding,
    /// Direct structured runs may operate without a working resource.
    DirectRunNoResource,
    /// Direct run output remains bounded.
    DirectRunOutputBound,
    /// Direct interactive sessions may operate without a working resource.
    DirectSessionNoResource,
    /// Connection-scoped leases end with their owning session.
    ConnectionScopedLeaseLifecycle,
    /// Billed cost evidence is scoped to one turn.
    BilledCostTurnScoped,
    /// Private continuation state does not imply restart recovery.
    NoImplicitSessionRecovery,
    /// Provider-reported evidence remains distinct from host truth.
    ProviderEvidenceSeparated,
    /// Attached serving processes are never stopped by Swallowtail.
    AttachedServiceNeverStopped,
    /// Host-owned serving processes are stopped during cleanup.
    OwnedServiceStops,
    /// Host-owned serving instances hold an explicit artifact lease.
    OwnedArtifactLease,
    /// Published serving endpoints remain bound to the prepared instance.
    OwnedEndpointBinding,
    /// Owned serving cleanup follows the required release ordering.
    OwnedCleanupOrdered,
    /// Provider-managed harness ownership and lifecycle remain explicit.
    ProviderManagedHarnessLifecycle,
    /// Durable provider retention is selected rather than assumed.
    DurableRetentionExplicit,
    /// Provider-managed recovery requires an explicit operation.
    ManagedRecoveryExplicit,
    /// Remote resource deletion reports the provider's exact result.
    OwnedRemoteDeletionTruth,
    /// Native harness budget controls remain independent.
    NativeBudgetIndependent,
    /// Transcript deletion is not claimed without provider evidence.
    NoTranscriptDeletionClaim,
    /// Realtime media input, output, and control stay type-separated.
    RealtimeMediaBoundary,
    /// Realtime events preserve provider order.
    RealtimeMediaOrdering,
    /// Realtime interruption terminates the owning session.
    RealtimeMediaInterruptionEndsSession,
    /// Planned connection rollover is explicit and observable.
    PlannedConnectionRollover,
    /// Rollover does not claim unavailable replay.
    RolloverNoReplay,
    /// Rollover closes the prior connection before replacement use.
    RolloverCleanupOrdered,
    /// External interface versions are qualified before use.
    InterfaceVersionQualified,
    /// Harness policy matches the prepared selection exactly.
    HarnessPolicyExact,
    /// Harness scheduling uses host-scoped work.
    HarnessScheduling,
    /// Commands expose explicit acknowledgement state.
    CommandAcknowledgement,
    /// Harness UI requests can be relayed to a consumer.
    HarnessUiRelay,
    /// Each direct attempt has separate explicit authorization.
    ExplicitAttemptAuthorization,
    /// Consumer-owned tools use the portable request/result exchange.
    ConsumerToolExchange,
    /// Private continuation state is bounded and non-serializable.
    PrivateContinuationBounded,
    /// Provider cache behavior is represented explicitly.
    ProviderCachePosture,
    /// Request-scoped leases end with their owning request.
    RequestScopedLeaseLifecycle,
    /// Attached runtime identity matches the prepared binding.
    AttachedRuntimeBinding,
    /// Runtime-managed model residency remains explicit.
    RuntimeManagedResidency,
    /// Versions outside a closed compatibility window are rejected.
    ClosedCompatibilityWindow,
    /// Harness configuration matches the prepared evidence exactly.
    HarnessConfigurationExact,
    /// Remote ACP connection open, use, and close are observable.
    RemoteAcpConnectionLifecycle,
    /// Remote ACP connection affinity remains operation-scoped.
    RemoteAcpAffinityScoped,
    /// Remote ACP transport does not imply provider session recovery.
    RemoteAcpNoRecovery,
    /// ACP wire, RFD, transport SDK, and core SDK versions stay distinct.
    RemoteAcpVersionAxesSeparate,
}

/// Assertions proven by one completed synthetic profile.
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

    /// Returns the synthetic topology that produced the report.
    #[must_use]
    pub const fn profile(&self) -> SyntheticProfile {
        self.profile
    }

    /// Returns whether the profile proved one invariant.
    #[must_use]
    pub fn covers(&self, assertion: ConformanceAssertion) -> bool {
        self.passed.contains(&assertion)
    }

    /// Iterates over all invariants proved by the profile.
    pub fn passed(&self) -> impl ExactSizeIterator<Item = ConformanceAssertion> + '_ {
        self.passed.iter().copied()
    }
}

#[must_use]
/// Runs the one-shot structured CLI conformance profile.
pub fn run_one_shot_structured_cli_profile() -> ConformanceReport {
    crate::profile_one_shot::run()
}

#[must_use]
/// Runs assertions specific to native structured harness boundaries.
pub fn run_structured_harness_native_boundary_assertions() -> ConformanceReport {
    crate::profile_harness_native::run()
}

#[must_use]
/// Runs the common single-turn ACP activity projection assertions.
pub fn run_acp_single_turn_projection_assertions() -> ConformanceReport {
    crate::acp_projection_assertions::run()
}

#[must_use]
/// Runs the long-lived RPC harness conformance profile.
pub fn run_long_lived_rpc_profile() -> ConformanceReport {
    crate::profile_rpc::run()
}

#[must_use]
/// Runs portable harness RPC contract assertions.
pub fn run_harness_rpc_contract_assertions() -> ConformanceReport {
    crate::profile_harness_rpc_contract::run()
}

#[must_use]
/// Runs exact harness-configuration boundary profiles.
pub fn run_harness_configuration_boundary_assertions() -> Vec<ConformanceReport> {
    crate::profile_harness_configuration::run()
}

#[must_use]
/// Runs the long-lived local ACP harness profile.
pub fn run_long_lived_acp_profile() -> ConformanceReport {
    crate::profile_acp::run()
}

#[must_use]
/// Runs the durable ACP harness profile.
pub fn run_persistent_acp_profile() -> ConformanceReport {
    crate::profile_persistent_acp::run()
}

#[must_use]
/// Runs the remote ACP harness profile.
pub fn run_remote_acp_harness_profile() -> ConformanceReport {
    crate::profile_remote_acp::run()
}

#[must_use]
/// Runs the hosted direct-API profile.
pub fn run_hosted_direct_api_profile() -> ConformanceReport {
    crate::profile_hosted::run()
}

#[must_use]
/// Runs the connection-scoped direct-session profile.
pub fn run_connection_scoped_direct_session_profile() -> ConformanceReport {
    crate::profile_direct_session::run()
}

#[must_use]
/// Runs the locally continued direct-session profile.
pub fn run_locally_continued_direct_session_profile() -> ConformanceReport {
    crate::profile_local_continuation::run()
}

#[must_use]
/// Runs the realtime media direct-session profile.
pub fn run_realtime_media_direct_session_profile() -> ConformanceReport {
    crate::profile_realtime_media::run()
}

#[must_use]
/// Runs planned realtime connection-rollover assertions.
pub fn run_realtime_rollover_boundary_assertions() -> ConformanceReport {
    crate::profile_realtime_rollover::run()
}

#[must_use]
/// Runs the provider-managed remote harness profile.
pub fn run_provider_managed_harness_profile() -> ConformanceReport {
    crate::profile_managed_harness::run()
}

#[must_use]
/// Runs the attached network harness profile.
pub fn run_attached_network_harness_profile() -> ConformanceReport {
    crate::profile_network_harness::run()
}

#[must_use]
/// Runs the externally owned self-hosted serving profile.
pub fn run_attached_self_hosted_profile() -> ConformanceReport {
    crate::profile_attached::run()
}

#[must_use]
/// Runs assertions specific to attached runtime identity and residency.
pub fn run_attached_runtime_boundary_assertions() -> ConformanceReport {
    crate::profile_attached_runtime::run()
}

#[must_use]
/// Runs the host-owned self-hosted serving profile.
pub fn run_owned_self_hosted_profile() -> ConformanceReport {
    crate::profile_owned::run()
}

#[must_use]
/// Runs every primary synthetic conformance profile.
pub fn run_all_synthetic_profiles() -> Vec<ConformanceReport> {
    vec![
        run_one_shot_structured_cli_profile(),
        run_long_lived_rpc_profile(),
        run_long_lived_acp_profile(),
        run_persistent_acp_profile(),
        run_remote_acp_harness_profile(),
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
