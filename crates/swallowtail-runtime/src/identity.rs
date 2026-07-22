use std::error::Error;
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeIdentityRequired {
    field: &'static str,
}

impl RuntimeIdentityRequired {
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl fmt::Display for RuntimeIdentityRequired {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} must not be empty", self.field)
    }
}

impl Error for RuntimeIdentityRequired {}

macro_rules! runtime_identity {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, RuntimeIdentityRequired> {
                let value = value.into();
                if value.trim().is_empty() {
                    Err(RuntimeIdentityRequired { field: $field })
                } else {
                    Ok(Self(value))
                }
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter
                    .debug_tuple(stringify!($name))
                    .field(&"<redacted>")
                    .finish()
            }
        }
    };
}

runtime_identity!(RequestId, "request id");
runtime_identity!(RuntimeRunId, "runtime run id");
runtime_identity!(RuntimeSessionId, "runtime session id");
runtime_identity!(RuntimeTurnId, "runtime turn id");
runtime_identity!(CallbackId, "callback id");
runtime_identity!(ServingInstanceId, "serving instance id");
runtime_identity!(ScopeId, "scope id");
runtime_identity!(MediaStreamId, "media stream id");
runtime_identity!(HarnessCommandId, "harness command id");

#[cfg(test)]
mod tests {
    use super::{CallbackId, RequestId};

    #[test]
    fn runtime_identity_rejects_blank_text() {
        let failure = RequestId::new(" ").expect_err("blank request id must fail");
        assert_eq!(failure.field(), "request id");
    }

    #[test]
    fn runtime_identities_are_redacted_by_default() {
        let callback = CallbackId::new("private-callback").expect("callback id is valid");

        assert!(!format!("{callback:?}").contains(callback.as_str()));
    }
}
