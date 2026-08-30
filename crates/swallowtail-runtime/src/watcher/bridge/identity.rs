//! Redacted lease identities and admission labels.

use crate::InputValueRequired;
use std::fmt;
use std::num::NonZeroU64;
use zeroize::Zeroize;

/// Monotonic generation assigned to one opened bridge lease.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WatcherBridgeGeneration(NonZeroU64);

impl WatcherBridgeGeneration {
    /// Creates generation `1`.
    #[must_use]
    pub const fn initial() -> Self {
        Self(NonZeroU64::MIN)
    }

    /// Creates a generation from an exact positive counter.
    #[must_use]
    pub const fn new(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the raw generation counter.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }

    /// Returns the next generation after a later open.
    #[must_use]
    pub const fn next(self) -> Self {
        match self.0.checked_add(1) {
            Some(value) => Self(value),
            None => self,
        }
    }
}

impl fmt::Debug for WatcherBridgeGeneration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherBridgeGeneration")
            .field(&self.get())
            .finish()
    }
}

/// Driver-only loopback endpoint bound to one open lease.
pub struct WatcherBridgeEndpoint {
    value: String,
}

impl WatcherBridgeEndpoint {
    /// Creates a nonempty driver-only endpoint value.
    pub fn new(value: impl Into<String>) -> Result<Self, InputValueRequired> {
        crate::input::required_text("watcher bridge endpoint", value).map(|value| Self { value })
    }

    /// Returns the endpoint for the authorized driver only.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.value
    }
}

impl Drop for WatcherBridgeEndpoint {
    fn drop(&mut self) {
        self.value.zeroize();
    }
}

impl fmt::Debug for WatcherBridgeEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherBridgeEndpoint")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherBridgeEndpoint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher bridge endpoint>")
    }
}

/// Driver-only bearer capability bound to one open lease generation.
pub struct WatcherBridgeBearer {
    secret: String,
}

impl WatcherBridgeBearer {
    /// Creates nonempty driver-only bearer material.
    pub fn new(secret: impl Into<String>) -> Result<Self, InputValueRequired> {
        crate::input::required_text("watcher bridge bearer", secret).map(|secret| Self { secret })
    }

    /// Returns the bearer for the authorized driver only.
    #[must_use]
    pub fn expose(&self) -> &str {
        &self.secret
    }

    /// Compares presented bearer material without leaking length through early
    /// character inequality. Length mismatch still fails closed.
    #[must_use]
    pub fn matches(&self, presented: &str) -> bool {
        constant_time_eq(self.secret.as_bytes(), presented.as_bytes())
    }
}

impl Drop for WatcherBridgeBearer {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

impl fmt::Debug for WatcherBridgeBearer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherBridgeBearer")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherBridgeBearer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher bridge bearer>")
    }
}

/// Unforgeable host binding for one live lease. It is not driver-usable
/// authentication material and never appears in default formatting.
pub struct WatcherBridgeToken {
    secret: String,
}

impl WatcherBridgeToken {
    /// Creates nonempty host-binding material.
    pub fn new(secret: impl Into<String>) -> Result<Self, InputValueRequired> {
        crate::input::required_text("watcher bridge token", secret).map(|secret| Self { secret })
    }

    /// Compares two tokens without leaking length through early inequality.
    #[must_use]
    pub fn matches(&self, other: &Self) -> bool {
        constant_time_eq(self.secret.as_bytes(), other.secret.as_bytes())
    }
}

impl Drop for WatcherBridgeToken {
    fn drop(&mut self) {
        self.secret.zeroize();
    }
}

impl fmt::Debug for WatcherBridgeToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("WatcherBridgeToken")
            .field(&"<redacted>")
            .finish()
    }
}

impl fmt::Display for WatcherBridgeToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("<redacted watcher bridge token>")
    }
}

/// Public admission state for one bridge lease.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WatcherBridgeAdmission {
    /// The listener admits reserved protocol and watcher work.
    Open,
    /// New watcher-creating work is rejected; observation and stop remain.
    Frozen,
    /// The lease has released its listener and private material.
    Closed,
}

impl WatcherBridgeAdmission {
    /// Returns a stable public label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Frozen => "frozen",
            Self::Closed => "closed",
        }
    }

    /// Reports whether start and other creating work may still be admitted.
    #[must_use]
    pub const fn admits_new_work(self) -> bool {
        matches!(self, Self::Open)
    }
}

impl fmt::Display for WatcherBridgeAdmission {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
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
