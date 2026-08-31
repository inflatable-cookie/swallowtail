use swallowtail_core::SafeDiagnostic;

use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind};
use super::identity::ConsumerRouteProjectionSourceId;
use super::text::admit_text;
use super::{MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES, MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES};

/// Bounded provider-native descriptor identity qualified by route and version.
///
/// The extension carries no raw provider payload, command, path, or credential
/// material, and never widens support, availability, or lifecycle.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteNamespacedExtension {
    route: String,
    version_segment: String,
    semantic_id: String,
}

impl ConsumerRouteNamespacedExtension {
    /// Admits one bounded route, qualified version segment, and semantic id.
    pub fn new(
        route: impl Into<String>,
        version_segment: impl Into<String>,
        semantic_id: impl Into<String>,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let route = route.into();
        let version_segment = version_segment.into();
        let semantic_id = semantic_id.into();
        for value in [&route, &version_segment, &semantic_id] {
            admit_text(
                value,
                MAX_CONSUMER_ROUTE_EXTENSION_TEXT_BYTES,
                ConsumerRouteProjectionFailureKind::LimitExceeded,
                "swallowtail.consumer_route_projection.extension_text_limit_exceeded",
                "Namespaced extension text exceeds the fixed extension byte maximum",
            )?;
        }
        Ok(Self {
            route,
            version_segment,
            semantic_id,
        })
    }

    #[must_use]
    /// Returns the exact route this extension belongs to.
    pub fn route(&self) -> &str {
        &self.route
    }

    #[must_use]
    /// Returns the qualified provider-interface version segment.
    pub fn version_segment(&self) -> &str {
        &self.version_segment
    }

    #[must_use]
    /// Returns the route-local semantic id.
    pub fn semantic_id(&self) -> &str {
        &self.semantic_id
    }
}

/// Closed portable feature identity shared by every projection view.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteFeatureId {
    /// Enumerating models through the exact route.
    ModelCatalogue,
    /// A bounded structured run.
    StructuredRun,
    /// A multi-turn interactive session.
    InteractiveSession,
    /// A realtime media session.
    RealtimeMediaSession,
    /// Incremental operation events.
    StreamingEvents,
    /// Provider usage measurement evidence.
    UsageEvidence,
    /// Portable activity observation.
    ActivityObservation,
    /// Explicit reasoning selection.
    ReasoningSelection,
    /// Enforced structured output.
    StructuredOutput,
    /// Non-text request attachments.
    Attachments,
    /// Consumer-mediated tool exchange.
    ConsumerToolExchange,
    /// Consumer-mediated question exchange.
    QuestionExchange,
    /// Cancellation or interruption of active work.
    CancellationOrInterruption,
    /// Loading retained provider-session state.
    LoadSession,
    /// Resuming a loaded or retained provider session.
    ResumeSession,
    /// Listing bounded provider-session candidates.
    ProviderSessionCatalogue,
    /// Importing an exact provider-session candidate.
    ProviderSessionImport,
    /// Archiving a retained provider session.
    ProviderSessionArchive,
    /// Restoring an archived provider session.
    ProviderSessionRestore,
    /// Deleting retained provider-session data.
    ProviderSessionDelete,
    /// Reconciling retained provider-session truth.
    ProviderSessionReconciliation,
    /// Reading newest-first provider-session history.
    ProviderSessionHistory,
    /// Route persistence posture as lifecycle evidence only.
    PersistentSessionPosture,
    /// Binding the operation to a working resource.
    WorkingResource,
    /// Writing bounded workspace text through the host service.
    BoundedWorkspaceTextWrite,
    /// Requesting external search through the route.
    ExternalSearch,
    /// Applying an output-token ceiling.
    OutputTokenLimit,
    /// The exact prepared-operation facade itself.
    PreparedFacade,
    /// Exact active-session reasoning acknowledgement truth.
    ActiveSessionReasoningAcknowledgement,
    /// Bounded provider-native feature identity.
    Namespaced(ConsumerRouteNamespacedExtension),
}

/// Closed portable control identity shared by every projection view.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteControlId {
    /// Exact model-route selection.
    ModelSelection,
    /// Exact reasoning selection.
    ReasoningSelection,
    /// The exact session-option bundle.
    SessionOptions,
    /// Declared consumer tools.
    ToolDeclarations,
    /// Developer instruction content.
    DeveloperInstructions,
    /// Selected idiom values.
    Idioms,
    /// Per-turn consumer-mediated user-input exchange.
    UserInputExchange,
    /// Loading an exact retained provider session.
    LoadSession,
    /// Resuming an exact retained provider session.
    ResumeSession,
    /// Bounded provider-session catalogue query.
    SessionCatalogueBounds,
    /// Bounded provider-session history query.
    SessionHistoryBounds,
    /// Bounded provider-session reconciliation query.
    SessionReconciliation,
    /// Provider output-token maximum.
    MaximumOutputTokens,
    /// Fixed realtime media configuration.
    RealtimeMediaConfig,
    /// Planned connection-rollover policy.
    PlannedConnectionRollover,
    /// Bounded provider-native control identity.
    Namespaced(ConsumerRouteNamespacedExtension),
}

/// Shared semantic identity of one projected row.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteRowIdentity {
    /// A descriptive feature row.
    Feature(ConsumerRouteFeatureId),
    /// A control row with a value domain and omission semantics.
    Control(ConsumerRouteControlId),
}

impl ConsumerRouteRowIdentity {
    #[must_use]
    /// Returns the bounded namespaced extension when the row uses one.
    pub const fn namespaced_extension(&self) -> Option<&ConsumerRouteNamespacedExtension> {
        match self {
            Self::Feature(ConsumerRouteFeatureId::Namespaced(extension))
            | Self::Control(ConsumerRouteControlId::Namespaced(extension)) => Some(extension),
            _ => None,
        }
    }
}

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

/// Bounded safe reason copied from the source dimension that supplied it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteSafeReason {
    dimension: ConsumerRouteAvailabilityDimension,
    source: ConsumerRouteProjectionSourceId,
    diagnostic: SafeDiagnostic,
}

impl ConsumerRouteSafeReason {
    /// Admits one bounded safe reason supplied by its named source.
    pub fn new(
        dimension: ConsumerRouteAvailabilityDimension,
        source: ConsumerRouteProjectionSourceId,
        diagnostic: SafeDiagnostic,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        admit_text(
            diagnostic.message(),
            MAX_CONSUMER_ROUTE_SAFE_REASON_BYTES,
            ConsumerRouteProjectionFailureKind::SafeReasonLimitExceeded,
            "swallowtail.consumer_route_projection.safe_reason_limit_exceeded",
            "Projected safe reason exceeds the fixed safe-reason byte maximum",
        )?;
        Ok(Self {
            dimension,
            source,
            diagnostic,
        })
    }

    #[must_use]
    /// Returns the authoritative dimension the reason belongs to.
    pub const fn dimension(&self) -> ConsumerRouteAvailabilityDimension {
        self.dimension
    }

    #[must_use]
    /// Returns the source that supplied the reason.
    pub const fn source(&self) -> &ConsumerRouteProjectionSourceId {
        &self.source
    }

    #[must_use]
    /// Returns the bounded safe diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

/// Authoritative class of the record that proved one row.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteSourceClass {
    /// The immutable configured provider-instance record.
    ConfiguredInstanceRecord,
    /// The immutable prepared-operation record.
    PreparedOperationRecord,
    /// A portable capability profile or constraint.
    CapabilityProfile,
    /// A model-catalogue observation.
    ModelCatalogueObservation,
    /// A public runtime request or session-option type.
    RuntimeRequestType,
    /// An adapter prepared input and its route validation.
    AdapterPreparedInput,
    /// Route-driver or wire acknowledgement evidence.
    RouteAcknowledgementEvidence,
}

/// How strongly the named source proves the row.
///
/// Documentation and QA matrices have no variant here, so a matrix-only row
/// cannot enter a projection.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteEvidenceStrength {
    /// A public runtime or adapter type proves the shape only.
    RuntimeType,
    /// An exact adapter route validation admitted the value.
    RouteValidation,
    /// The exact immutable prepared-operation record proves it.
    PreparedOperation,
    /// An exact provider wire acknowledgement proves it.
    WireAcknowledgement,
}

/// Exact subset of request and provider state the source proves.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteStateSupport {
    requested: bool,
    prepared: bool,
    pending: bool,
    provider_effective: bool,
    rejected: bool,
    observed: bool,
}

impl ConsumerRouteStateSupport {
    #[must_use]
    /// Creates descriptor-only state support with no proven state.
    pub const fn descriptor_only() -> Self {
        Self {
            requested: false,
            prepared: false,
            pending: false,
            provider_effective: false,
            rejected: false,
            observed: false,
        }
    }

    #[must_use]
    /// Adds proven consumer-requested state.
    pub const fn with_requested(mut self) -> Self {
        self.requested = true;
        self
    }

    #[must_use]
    /// Adds proven prepared session-start intent.
    pub const fn with_prepared(mut self) -> Self {
        self.prepared = true;
        self
    }

    #[must_use]
    /// Adds proven pending acknowledgement state.
    pub const fn with_pending(mut self) -> Self {
        self.pending = true;
        self
    }

    #[must_use]
    /// Adds provider-confirmed effective state.
    pub const fn with_provider_effective(mut self) -> Self {
        self.provider_effective = true;
        self
    }

    #[must_use]
    /// Adds provider-confirmed rejected state.
    pub const fn with_rejected(mut self) -> Self {
        self.rejected = true;
        self
    }

    #[must_use]
    /// Adds post-open observation without a provider-effective claim.
    pub const fn with_observed(mut self) -> Self {
        self.observed = true;
        self
    }

    #[must_use]
    /// Reports whether the source proves nothing beyond the descriptor.
    pub const fn is_descriptor_only(&self) -> bool {
        !self.requested
            && !self.prepared
            && !self.pending
            && !self.provider_effective
            && !self.rejected
            && !self.observed
    }

    #[must_use]
    /// Reports proven consumer-requested state.
    pub const fn requested(&self) -> bool {
        self.requested
    }

    #[must_use]
    /// Reports proven prepared session-start intent.
    pub const fn prepared(&self) -> bool {
        self.prepared
    }

    #[must_use]
    /// Reports proven pending acknowledgement state.
    pub const fn pending(&self) -> bool {
        self.pending
    }

    #[must_use]
    /// Reports provider-confirmed effective state.
    pub const fn provider_effective(&self) -> bool {
        self.provider_effective
    }

    #[must_use]
    /// Reports provider-confirmed rejected state.
    pub const fn rejected(&self) -> bool {
        self.rejected
    }

    #[must_use]
    /// Reports post-open observation without a provider-effective claim.
    pub const fn observed(&self) -> bool {
        self.observed
    }
}

/// Exact route authority that permits a selectable or acknowledged claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsumerRouteMutationAuthority {
    /// No route mutation or acknowledgement source exists for this row.
    Absent,
    /// The exact prepared operation admits this value at session start.
    PreparedSessionStart(ConsumerRouteProjectionSourceId),
    /// The named source parsed an exact provider acknowledgement.
    Acknowledged(ConsumerRouteProjectionSourceId),
}

impl ConsumerRouteMutationAuthority {
    #[must_use]
    /// Returns the source that carries the authority, when one exists.
    pub const fn source(&self) -> Option<&ConsumerRouteProjectionSourceId> {
        match self {
            Self::Absent => None,
            Self::PreparedSessionStart(source) | Self::Acknowledged(source) => Some(source),
        }
    }

    #[must_use]
    /// Reports whether an exact provider acknowledgement proved the row.
    pub const fn is_acknowledged(&self) -> bool {
        matches!(self, Self::Acknowledged(_))
    }
}
