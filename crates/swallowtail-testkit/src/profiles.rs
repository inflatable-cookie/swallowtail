use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SyntheticProfile {
    OneShotStructuredCli,
    LongLivedRpcHarness,
    HostedDirectApi,
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
    HostedApiNeedsNoProcess,
    AttachedServiceNeverStopped,
    OwnedServiceStops,
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
pub fn run_long_lived_rpc_profile() -> ConformanceReport {
    crate::profile_rpc::run()
}

#[must_use]
pub fn run_hosted_direct_api_profile() -> ConformanceReport {
    crate::profile_hosted::run()
}

#[must_use]
pub fn run_attached_self_hosted_profile() -> ConformanceReport {
    crate::profile_attached::run()
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
        run_hosted_direct_api_profile(),
        run_attached_self_hosted_profile(),
        run_owned_self_hosted_profile(),
    ]
}
