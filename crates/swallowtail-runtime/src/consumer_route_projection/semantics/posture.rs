/// Lifecycle point at which one projected row is true.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteLifecycle {
    /// Selection-time feature or control truth.
    SelectionSummary,
    /// Fixed when the session starts; changing it needs a replacement session.
    SessionStartOnly,
    /// Supplied per turn rather than at session start.
    PerTurn,
    /// Negotiable between turns through an exact route mechanism.
    BetweenTurnNegotiable,
    /// Separately qualified mid-turn negotiation.
    QualifiedMidTurnNegotiable,
    /// Observed only after the session opens.
    PostOpenObservationOnly,
    /// Observed only after a provider operation completes without opening a session.
    PostOperationObservationOnly,
}

/// Who may act on one projected row.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteActorPosture {
    /// Descriptive only; nobody acts on it.
    Informational,
    /// The consumer may select the value.
    ConsumerSelectable,
    /// The embedding host controls the value.
    HostControlled,
    /// The operator controls the value.
    OperatorControlled,
    /// The provider selects the value.
    ProviderSelected,
    /// The row is observed and never selected.
    ObservationOnly,
}

/// Whether the exact route proves support, and how strongly.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteSupportPosture {
    /// The authoritative source proves route support.
    Supported,
    /// The authoritative source proves the route does not support it.
    Unsupported,
    /// The authoritative source does not settle support.
    Unknown,
}

/// Current availability, kept separate from descriptive support.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteAvailability {
    /// Currently usable under the exact evidence supplied.
    Available,
    /// Currently unusable under the exact evidence supplied.
    Unavailable,
    /// Usable only where a named further condition holds.
    Conditional,
    /// Available only through negotiated session state.
    NegotiatedOnly,
    /// The authoritative source does not settle availability.
    Unknown,
}

/// Authoritative source dimension a bounded safe reason belongs to.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteAvailabilityDimension {
    /// Credential state.
    Credential,
    /// Entitlement state.
    Entitlement,
    /// Endpoint authorization.
    EndpointAuthorization,
    /// Runtime readiness.
    RuntimeReadiness,
    /// Support authority.
    SupportAuthority,
    /// Model-catalogue result.
    CatalogueResult,
    /// Capability constraint.
    CapabilityConstraint,
    /// Preparation agreement.
    PreparationAgreement,
    /// Negotiated session state.
    NegotiatedState,
    /// Evidence freshness.
    EvidenceFreshness,
}
