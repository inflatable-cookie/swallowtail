use crate::diagnostic::{ValueRequired, required_text};
use std::fmt;

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

#[cfg(test)]
mod tests {
    use super::{RunRef, SessionRef};

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
}
