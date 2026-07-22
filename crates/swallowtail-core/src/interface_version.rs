use crate::diagnostic::{ValueRequired, required_text};

mod claim;
mod error;
mod ordering;

pub use claim::{
    InterfaceCompatibilityClaim, InterfaceCompatibilityMatch, InterfaceVersionSegment,
};
pub use error::InvalidInterfaceCompatibilityClaim;

macro_rules! text_value {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

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
    #[must_use]
    pub const fn new(axis: InterfaceVersionAxis, version: InterfaceVersion) -> Self {
        Self { axis, version }
    }

    #[must_use]
    pub const fn axis(&self) -> &InterfaceVersionAxis {
        &self.axis
    }

    #[must_use]
    pub const fn version(&self) -> &InterfaceVersion {
        &self.version
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterfaceVersionScheme {
    Semantic,
    Integer,
    CalendarDate,
    Opaque,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterfaceSupportStatus {
    Maintained,
    Deprecated,
}

#[cfg(test)]
#[path = "interface_version/tests.rs"]
mod tests;
