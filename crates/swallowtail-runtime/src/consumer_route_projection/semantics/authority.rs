use super::super::identity::ConsumerRouteProjectionSourceId;

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
///
/// A per-turn value the consumer may supply uses `ConsumerMediatedPerTurn`. It
/// never claims prepared session-start state and never implies that the
/// provider acknowledged or mutated anything.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsumerRouteMutationAuthority {
    /// No route mutation or acknowledgement source exists for this row.
    Absent,
    /// The exact prepared operation admits this value at session start.
    PreparedSessionStart(ConsumerRouteProjectionSourceId),
    /// The exact route admits this value per turn through the consumer.
    ConsumerMediatedPerTurn(ConsumerRouteProjectionSourceId),
    /// The named source parsed an exact provider acknowledgement.
    Acknowledged(ConsumerRouteProjectionSourceId),
}

impl ConsumerRouteMutationAuthority {
    #[must_use]
    /// Returns the source that carries the authority, when one exists.
    pub const fn source(&self) -> Option<&ConsumerRouteProjectionSourceId> {
        match self {
            Self::Absent => None,
            Self::PreparedSessionStart(source)
            | Self::ConsumerMediatedPerTurn(source)
            | Self::Acknowledged(source) => Some(source),
        }
    }

    #[must_use]
    /// Reports whether an exact provider acknowledgement proved the row.
    pub const fn is_acknowledged(&self) -> bool {
        matches!(self, Self::Acknowledged(_))
    }

    #[must_use]
    /// Reports whether the exact prepared operation admits session-start truth.
    pub const fn is_prepared_session_start(&self) -> bool {
        matches!(self, Self::PreparedSessionStart(_))
    }

    #[must_use]
    /// Reports whether the consumer may supply the value once per turn.
    pub const fn is_consumer_mediated_per_turn(&self) -> bool {
        matches!(self, Self::ConsumerMediatedPerTurn(_))
    }
}
