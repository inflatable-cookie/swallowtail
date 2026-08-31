use std::collections::BTreeSet;

use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
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
