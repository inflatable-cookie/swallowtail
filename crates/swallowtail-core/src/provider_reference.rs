use crate::diagnostic::{SafeDiagnostic, ValueRequired, required_text};
use std::error::Error;
use std::fmt;

const MAX_PROVIDER_ACTIVITY_REF_BYTES: usize = 512;

macro_rules! opaque_provider_reference {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
                required_text($field, value).map(Self)
            }

            /// Passes the opaque value back to its owning provider adapter.
            #[must_use]
            pub fn as_provider_value(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<opaque>")
                    .finish()
            }
        }
    };
}

opaque_provider_reference!(SessionRef, "session reference");
opaque_provider_reference!(RunRef, "run reference");
opaque_provider_reference!(TurnRef, "turn reference");
opaque_provider_reference!(ProviderRequestRef, "provider request reference");

/// Opaque provider-native identity for one observable activity item.
#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderActivityRef(String);

impl ProviderActivityRef {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidProviderActivityRef> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(InvalidProviderActivityRef::new(
                "swallowtail.provider_activity_ref_required",
                "Provider activity reference must not be empty",
            ));
        }
        if value.len() > MAX_PROVIDER_ACTIVITY_REF_BYTES || value.chars().any(char::is_control) {
            return Err(InvalidProviderActivityRef::new(
                "swallowtail.provider_activity_ref_invalid",
                "Provider activity reference exceeded its safe bounded form",
            ));
        }
        Ok(Self(value))
    }

    /// Passes the opaque value back to its owning provider adapter.
    #[must_use]
    pub fn as_provider_value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for ProviderActivityRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("ProviderActivityRef")
            .field(&"<opaque>")
            .finish()
    }
}

impl fmt::Display for ProviderActivityRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<opaque provider activity reference>")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidProviderActivityRef {
    diagnostic: SafeDiagnostic,
}

impl InvalidProviderActivityRef {
    fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(code, message),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for InvalidProviderActivityRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for InvalidProviderActivityRef {}

#[cfg(test)]
mod tests {
    use super::{ProviderActivityRef, RunRef, SessionRef};

    #[test]
    fn provider_references_are_distinct_and_redacted_by_default() {
        let session =
            SessionRef::new("thread/provider/internal/42").expect("session reference is valid");
        let run = RunRef::new("run/provider/internal/42").expect("run reference is valid");

        assert_eq!(session.as_provider_value(), "thread/provider/internal/42");
        assert_eq!(run.as_provider_value(), "run/provider/internal/42");
        assert_eq!(format!("{session:?}"), "SessionRef(\"<opaque>\")");
        assert_eq!(format!("{run:?}"), "RunRef(\"<opaque>\")");
    }

    #[test]
    fn activity_references_are_bounded_and_redacted() {
        let reference =
            ProviderActivityRef::new("provider/private/item").expect("reference is valid");

        assert_eq!(reference.as_provider_value(), "provider/private/item");
        assert!(!format!("{reference:?}").contains(reference.as_provider_value()));
        assert!(
            !reference
                .to_string()
                .contains(reference.as_provider_value())
        );

        let oversized =
            ProviderActivityRef::new("x".repeat(513)).expect_err("oversized reference must fail");
        assert_eq!(
            oversized.diagnostic().code(),
            "swallowtail.provider_activity_ref_invalid"
        );
        assert!(!oversized.to_string().contains(&"x".repeat(513)));
    }
}
