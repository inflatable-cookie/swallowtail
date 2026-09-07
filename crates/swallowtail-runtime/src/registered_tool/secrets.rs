//! Driver-private endpoint and authority material for one registered lease.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use super::identity::admit_identity;
use std::fmt;
use zeroize::Zeroize;

macro_rules! private_material {
    ($name:ident, $doc:literal, $label:literal) => {
        #[doc = $doc]
        pub struct $name {
            secret: String,
        }

        impl $name {
            /// Creates nonempty driver-only material.
            pub fn new(secret: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
                let secret = secret.into();
                if secret.trim().is_empty() {
                    return Err(reject(RegisteredToolFailureKind::IdentityRejected));
                }
                Ok(Self { secret })
            }

            /// Compares presented material in constant time for equal lengths.
            #[must_use]
            pub fn matches(&self, presented: &str) -> bool {
                constant_time_eq(self.secret.as_bytes(), presented.as_bytes())
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                self.secret.zeroize();
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

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(concat!("<redacted ", $label, ">"))
            }
        }
    };
}

private_material!(
    RegisteredToolBearer,
    "Driver-only bearer capability bound to one open lease generation.",
    "registered tool bearer"
);
private_material!(
    RegisteredToolBridgeToken,
    "Unforgeable host binding for one live registered-tool lease.",
    "registered tool token"
);

impl RegisteredToolBridgeToken {
    /// Compares two host tokens without leaking length through early inequality.
    #[must_use]
    pub fn token_matches(&self, other: &Self) -> bool {
        constant_time_eq(self.secret.as_bytes(), other.secret.as_bytes())
    }
}

/// Driver-only transport endpoint bound to one open lease.
pub struct RegisteredToolEndpoint {
    value: String,
}

impl RegisteredToolEndpoint {
    /// Creates a nonempty driver-only endpoint value.
    pub fn new(value: impl Into<String>) -> Result<Self, RegisteredToolFailure> {
        admit_identity(value.into()).map(|value| Self { value })
    }

    /// Returns the endpoint for the authorized driver only.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.value
    }
}

impl Drop for RegisteredToolEndpoint {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

impl fmt::Debug for RegisteredToolEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("RegisteredToolEndpoint")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for RegisteredToolEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted registered tool endpoint>")
    }
}

impl RegisteredToolBearer {
    /// Returns the bearer for the authorized driver only.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.secret
    }
}

pub(super) fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |acc, (a, b)| acc | (a ^ b))
        == 0
}
