use std::collections::BTreeSet;

use super::applicability::ConsumerRouteApplicability;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::ConsumerRouteProjectionSourceIdentity;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteEvidenceStrength,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteRowIdentity,
    ConsumerRouteSafeReason, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture,
};
use super::text::admit_text;
use super::{MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES, MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES};

/// One bounded admitted or bound-describing control value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConsumerRouteEnumerableValue(String);

impl ConsumerRouteEnumerableValue {
    /// Admits one bounded, non-blank, control-free value.
    pub fn new(value: impl Into<String>) -> Result<Self, ConsumerRouteProjectionFailure> {
        let value = value.into();
        admit_text(
            &value,
            MAX_CONSUMER_ROUTE_ENUMERABLE_VALUE_BYTES,
            ConsumerRouteProjectionFailureKind::LimitExceeded,
            "swallowtail.consumer_route_projection.enumerable_value_limit_exceeded",
            "Projected control value exceeds the fixed enumerable value byte maximum",
        )?;
        Ok(Self(value))
    }

    #[must_use]
    /// Returns the exact admitted value text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Bounded set of exactly admitted control values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteEnumeratedValues {
    values: Vec<ConsumerRouteEnumerableValue>,
}

impl ConsumerRouteEnumeratedValues {
    /// Admits a bounded, duplicate-free set of exact control values.
    pub fn new(
        values: impl IntoIterator<Item = ConsumerRouteEnumerableValue>,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let values = values.into_iter().collect::<Vec<_>>();
        if values.len() > MAX_CONSUMER_ROUTE_ENUMERABLE_VALUES {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::LimitExceeded,
                "swallowtail.consumer_route_projection.enumerable_value_count_exceeded",
                "Projected control domain exceeds the fixed enumerable value maximum",
            ));
        }
        let mut seen = BTreeSet::new();
        if values.iter().any(|value| !seen.insert(value.clone())) {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ValueDomainInvalid,
                "swallowtail.consumer_route_projection.enumerable_value_duplicate",
                "Projected control domain repeats an admitted value",
            ));
        }
        Ok(Self { values })
    }

    /// Iterates admitted values in supplied order.
    pub fn values(&self) -> impl ExactSizeIterator<Item = &ConsumerRouteEnumerableValue> {
        self.values.iter()
    }
}

/// Admitted domain of one control value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConsumerRouteValueDomain {
    /// No consumer-selectable value; the row describes support only.
    Descriptor,
    /// Exactly these admitted values.
    Enumerated(ConsumerRouteEnumeratedValues),
    /// Explicitly unenumerated with the bound the source supplied.
    Unenumerated(ConsumerRouteEnumerableValue),
}

/// Portable kind of one control value.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteValueKind {
    /// Route capability state rather than a selectable value.
    CapabilityState,
    /// Observed output rather than a selectable value.
    Observation,
    /// One exact model route.
    ExactModelRoute,
    /// One value from a closed portable enumeration.
    BoundedEnumeration,
    /// One value from a closed enumeration the provider acknowledges.
    AcknowledgedEnumeration,
    /// Exact provider acknowledgement state.
    AcknowledgementState,
    /// A structured session-option bundle.
    StructuredOptions,
    /// Bounded structured declarations.
    StructuredDeclarations,
    /// Bounded structured content.
    StructuredContent,
    /// One bounded option value.
    BoundedOption,
    /// A bounded integer.
    BoundedInteger,
    /// A bounded policy value.
    BoundedPolicy,
    /// A bounded observation or management query.
    BoundedQuery,
    /// A fixed structured configuration.
    FixedStructuredConfig,
    /// A consumer-mediated exchange callback.
    ExchangeCallback,
    /// An exact lifecycle action binding.
    LifecycleAction,
}

/// What omitting the control means for the exact route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConsumerRouteOmissionSemantics {
    /// The row carries no selectable value.
    NotSelectable,
    /// The exact route constructor requires the value.
    Required,
    /// Omission supplies nothing and creates no Swallowtail default.
    SuppliesNothing,
    /// Omission preserves route behavior and creates no Swallowtail default.
    PreservesRouteBehavior,
}

/// Value kind, admitted domain, and omission truth of one control row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteControlValue {
    kind: ConsumerRouteValueKind,
    domain: ConsumerRouteValueDomain,
    omission: ConsumerRouteOmissionSemantics,
}

impl ConsumerRouteControlValue {
    #[must_use]
    /// Binds one value kind to its admitted domain and omission truth.
    pub const fn new(
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) -> Self {
        Self {
            kind,
            domain,
            omission,
        }
    }

    #[must_use]
    /// Returns the portable value kind.
    pub const fn kind(&self) -> ConsumerRouteValueKind {
        self.kind
    }

    #[must_use]
    /// Returns the admitted domain or explicit unenumerated bound.
    pub const fn domain(&self) -> &ConsumerRouteValueDomain {
        &self.domain
    }

    #[must_use]
    /// Returns what omitting the control means for the exact route.
    pub const fn omission(&self) -> ConsumerRouteOmissionSemantics {
        self.omission
    }
}

/// One immutable projected feature or control row.
///
/// A row is evidence. It authorizes no execution, mutation, or acknowledgement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerRouteProjectionRow {
    identity: ConsumerRouteRowIdentity,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    source_class: ConsumerRouteSourceClass,
    evidence_strength: ConsumerRouteEvidenceStrength,
    lifecycle: ConsumerRouteLifecycle,
    support: ConsumerRouteSupportPosture,
    availability: ConsumerRouteAvailability,
    actor: ConsumerRouteActorPosture,
    state_support: ConsumerRouteStateSupport,
    mutation_authority: ConsumerRouteMutationAuthority,
    value: Option<ConsumerRouteControlValue>,
    reason: Option<ConsumerRouteSafeReason>,
}

impl ConsumerRouteProjectionRow {
    #[must_use]
    /// Creates one row whose unset dimensions stay unknown, absent, or informational.
    pub const fn new(
        identity: ConsumerRouteRowIdentity,
        applicability: ConsumerRouteApplicability,
        source: ConsumerRouteProjectionSourceIdentity,
        source_class: ConsumerRouteSourceClass,
        evidence_strength: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> Self {
        Self {
            identity,
            applicability,
            source,
            source_class,
            evidence_strength,
            lifecycle,
            support: ConsumerRouteSupportPosture::Unknown,
            availability: ConsumerRouteAvailability::Unknown,
            actor: ConsumerRouteActorPosture::Informational,
            state_support: ConsumerRouteStateSupport::descriptor_only(),
            mutation_authority: ConsumerRouteMutationAuthority::Absent,
            value: None,
            reason: None,
        }
    }

    #[must_use]
    /// Sets descriptive route support without touching current availability.
    pub const fn with_support(mut self, support: ConsumerRouteSupportPosture) -> Self {
        self.support = support;
        self
    }

    #[must_use]
    /// Sets current availability without touching descriptive support.
    pub const fn with_availability(mut self, availability: ConsumerRouteAvailability) -> Self {
        self.availability = availability;
        self
    }

    #[must_use]
    /// Sets who may act on the row.
    pub const fn with_actor_posture(mut self, actor: ConsumerRouteActorPosture) -> Self {
        self.actor = actor;
        self
    }

    #[must_use]
    /// Sets the exact subset of request and provider state the source proves.
    pub const fn with_state_support(mut self, state_support: ConsumerRouteStateSupport) -> Self {
        self.state_support = state_support;
        self
    }

    #[must_use]
    /// Names the exact authority behind a selectable or acknowledged claim.
    pub fn with_mutation_authority(mut self, authority: ConsumerRouteMutationAuthority) -> Self {
        self.mutation_authority = authority;
        self
    }

    #[must_use]
    /// Adds the control value kind, admitted domain, and omission truth.
    pub fn with_control_value(mut self, value: ConsumerRouteControlValue) -> Self {
        self.value = Some(value);
        self
    }

    #[must_use]
    /// Adds a bounded safe reason the named source supplied.
    pub fn with_safe_reason(mut self, reason: ConsumerRouteSafeReason) -> Self {
        self.reason = Some(reason);
        self
    }

    #[must_use]
    /// Returns the shared semantic identity of the row.
    pub const fn identity(&self) -> &ConsumerRouteRowIdentity {
        &self.identity
    }

    #[must_use]
    /// Returns the exact applicability the row was admitted under.
    pub const fn applicability(&self) -> &ConsumerRouteApplicability {
        &self.applicability
    }

    #[must_use]
    /// Returns the exact source identity that proved the row.
    pub const fn source(&self) -> &ConsumerRouteProjectionSourceIdentity {
        &self.source
    }

    #[must_use]
    /// Returns the authoritative source class.
    pub const fn source_class(&self) -> ConsumerRouteSourceClass {
        self.source_class
    }

    #[must_use]
    /// Returns how strongly the named source proves the row.
    pub const fn evidence_strength(&self) -> ConsumerRouteEvidenceStrength {
        self.evidence_strength
    }

    #[must_use]
    /// Returns the lifecycle point at which the row is true.
    pub const fn lifecycle(&self) -> ConsumerRouteLifecycle {
        self.lifecycle
    }

    #[must_use]
    /// Returns descriptive route support.
    pub const fn support(&self) -> ConsumerRouteSupportPosture {
        self.support
    }

    #[must_use]
    /// Returns current availability, kept separate from support.
    pub const fn availability(&self) -> ConsumerRouteAvailability {
        self.availability
    }

    #[must_use]
    /// Returns who may act on the row.
    pub const fn actor_posture(&self) -> ConsumerRouteActorPosture {
        self.actor
    }

    #[must_use]
    /// Returns the exact proven state subset.
    pub const fn state_support(&self) -> ConsumerRouteStateSupport {
        self.state_support
    }

    #[must_use]
    /// Returns the exact authority behind a selectable or acknowledged claim.
    pub const fn mutation_authority(&self) -> &ConsumerRouteMutationAuthority {
        &self.mutation_authority
    }

    #[must_use]
    /// Returns the control value when the row is a control.
    pub const fn control_value(&self) -> Option<&ConsumerRouteControlValue> {
        self.value.as_ref()
    }

    #[must_use]
    /// Returns the bounded safe reason when a source supplied one.
    pub const fn safe_reason(&self) -> Option<&ConsumerRouteSafeReason> {
        self.reason.as_ref()
    }
}
