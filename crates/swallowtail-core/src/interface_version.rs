#![deny(missing_docs)]

use crate::diagnostic::{ValueRequired, required_text};

mod assessment;
mod claim;
mod error;
mod ordering;

pub use assessment::{
    InterfaceCompatibilityAssessment, InterfaceCompatibilityMatch, InterfaceUnverifiedNewer,
};
pub use claim::{InterfaceCompatibilityClaim, InterfaceVersionSegment};
pub use error::InvalidInterfaceCompatibilityClaim;

macro_rules! text_value {
    ($name:ident, $field:literal) => {
        #[doc = concat!("Validated, non-empty ", $field, ".")]
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Creates a ", $field, " after rejecting blank text.")]
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            /// Returns the validated text.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}

text_value!(InterfaceVersionAxis, "interface version axis");
text_value!(InterfaceVersion, "interface version");
text_value!(InterfaceBehaviorRevision, "interface behavior revision");
text_value!(
    InterfaceCompatibilityClaimId,
    "interface compatibility claim id"
);

/// One exact, safe version point observed or selected for an interface axis.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InterfaceVersionBinding {
    axis: InterfaceVersionAxis,
    version: InterfaceVersion,
}

impl InterfaceVersionBinding {
    /// Binds one version value to its exact interface axis.
    #[must_use]
    pub const fn new(axis: InterfaceVersionAxis, version: InterfaceVersion) -> Self {
        Self { axis, version }
    }

    /// Returns the interface axis.
    #[must_use]
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    /// Returns the observed or selected version.
    #[must_use]
    pub const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

/// Ordering scheme used to compare versions on one interface axis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterfaceVersionScheme {
    /// Semantic version ordering.
    Semantic,
    /// Monotonically increasing integer ordering.
    Integer,
    /// Calendar-date ordering.
    CalendarDate,
    /// Exact equality only; no ordering is inferred.
    Opaque,
}

/// Maintainer support status for one qualified interface segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterfaceSupportStatus {
    /// The claim's newest-revision segment: targeted for new integrations.
    Maintained,
    /// Retained for existing installed harnesses, not targeted for new
    /// integrations. Every segment whose behavior revision is not the
    /// claim's newest revision is deprecated by definition; removal is a
    /// called-out compatibility-window change, not an immediate schedule.
    Deprecated,
}

/// Policy for stable versions newer than the latest qualified segment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterfaceNewerVersionPosture {
    /// Reject every version outside qualified segments.
    QualifiedOnly,
    /// Admit stable newer versions with explicit unverified evidence.
    AllowUnverified,
}

#[cfg(test)]
#[path = "interface_version/tests.rs"]
mod tests;
