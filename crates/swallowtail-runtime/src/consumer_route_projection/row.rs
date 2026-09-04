use super::acknowledgement::ConsumerRouteCompoundAcknowledgement;
use super::applicability::ConsumerRouteApplicability;
use super::identity::ConsumerRouteProjectionSourceIdentity;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteEvidenceStrength,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteRowIdentity,
    ConsumerRouteSafeReason, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture,
};
use super::value::ConsumerRouteControlValue;

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
    compound_acknowledgement: Option<ConsumerRouteCompoundAcknowledgement>,
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
            compound_acknowledgement: None,
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
    /// Adds independently state-associated reasoning and Plan acknowledgement truth.
    pub fn with_compound_acknowledgement(
        mut self,
        acknowledgement: ConsumerRouteCompoundAcknowledgement,
    ) -> Self {
        self.compound_acknowledgement = Some(acknowledgement);
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
    /// Returns the compound acknowledgement when this row carries one.
    pub const fn compound_acknowledgement(&self) -> Option<&ConsumerRouteCompoundAcknowledgement> {
        self.compound_acknowledgement.as_ref()
    }

    #[must_use]
    /// Returns the bounded safe reason when a source supplied one.
    pub const fn safe_reason(&self) -> Option<&ConsumerRouteSafeReason> {
        self.reason.as_ref()
    }
}
