//! Driver-only lease values and public lifecycle truth for one bridge open.

use crate::{InputValueRequired, RuntimeTurnId, ScopeId, WatcherSnapshot};
use std::fmt;
use std::num::NonZeroU64;
use swallowtail_core::ExecutionHostId;
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
        let expected = self.secret.as_bytes();
        let presented = presented.as_bytes();
        if expected.len() != presented.len() {
            return false;
        }
        expected
            .iter()
            .zip(presented)
            .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
            == 0
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

/// Request that opens one operation-scoped bridge lease.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherBridgeOpenRequest {
    scope: ScopeId,
    turn: RuntimeTurnId,
}

impl WatcherBridgeOpenRequest {
    /// Binds the lease to one operation scope and owning turn.
    #[must_use]
    pub const fn new(scope: ScopeId, turn: RuntimeTurnId) -> Self {
        Self { scope, turn }
    }

    /// Returns the operation scope that will own the lease.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the turn whose watcher service the lease must use.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }
}

/// Bounded completion-gate observation for the bound turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherBridgeCompletionState {
    admission: WatcherBridgeAdmission,
    active_or_unjoined: Vec<WatcherSnapshot>,
}

impl WatcherBridgeCompletionState {
    /// Creates a completion observation from admission and remaining work.
    #[must_use]
    pub const fn new(
        admission: WatcherBridgeAdmission,
        active_or_unjoined: Vec<WatcherSnapshot>,
    ) -> Self {
        Self {
            admission,
            active_or_unjoined,
        }
    }

    /// Returns the lease admission recorded with this observation.
    #[must_use]
    pub const fn admission(&self) -> WatcherBridgeAdmission {
        self.admission
    }

    /// Returns the bounded active or unjoined watcher snapshots.
    #[must_use]
    pub fn active_or_unjoined(&self) -> &[WatcherSnapshot] {
        &self.active_or_unjoined
    }

    /// Reports whether successful completion may be admitted.
    #[must_use]
    pub fn allows_successful_completion(&self) -> bool {
        self.admission == WatcherBridgeAdmission::Frozen && self.active_or_unjoined.is_empty()
    }
}

/// Handle for one opened watcher-bridge lease.
///
/// Endpoint and bearer stay driver-only. Public formatting exposes only
/// lifecycle identity that is already redacted by those identity types.
pub struct WatcherBridgeLease {
    execution_host_id: ExecutionHostId,
    scope: ScopeId,
    turn: RuntimeTurnId,
    generation: WatcherBridgeGeneration,
    endpoint: WatcherBridgeEndpoint,
    bearer: WatcherBridgeBearer,
    defensive_cleanup: Option<Box<dyn FnOnce() + Send + 'static>>,
}

impl WatcherBridgeLease {
    /// Creates a lease handle for one bound host, operation, turn, and generation.
    #[must_use]
    pub fn new(
        execution_host_id: ExecutionHostId,
        scope: ScopeId,
        turn: RuntimeTurnId,
        generation: WatcherBridgeGeneration,
        endpoint: WatcherBridgeEndpoint,
        bearer: WatcherBridgeBearer,
    ) -> Self {
        Self {
            execution_host_id,
            scope,
            turn,
            generation,
            endpoint,
            bearer,
            defensive_cleanup: None,
        }
    }

    /// Installs the host cleanup invoked if this handle is dropped.
    #[must_use]
    pub fn with_defensive_cleanup(mut self, cleanup: impl FnOnce() + Send + 'static) -> Self {
        self.defensive_cleanup = Some(Box::new(cleanup));
        self
    }

    /// Removes defensive Drop cleanup so an explicit close can report outcome.
    pub fn take_defensive_cleanup(&mut self) -> Option<Box<dyn FnOnce() + Send + 'static>> {
        self.defensive_cleanup.take()
    }

    /// Returns the execution host bound at open.
    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    /// Returns the operation scope bound at open.
    #[must_use]
    pub const fn scope(&self) -> &ScopeId {
        &self.scope
    }

    /// Returns the owning turn bound at open.
    #[must_use]
    pub const fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Returns the open generation bound at open.
    #[must_use]
    pub const fn generation(&self) -> WatcherBridgeGeneration {
        self.generation
    }

    /// Returns the driver-only endpoint bound at open.
    #[must_use]
    pub const fn endpoint(&self) -> &WatcherBridgeEndpoint {
        &self.endpoint
    }

    /// Returns the driver-only bearer bound at open.
    #[must_use]
    pub const fn bearer(&self) -> &WatcherBridgeBearer {
        &self.bearer
    }
}

impl Drop for WatcherBridgeLease {
    fn drop(&mut self) {
        if let Some(cleanup) = self.defensive_cleanup.take() {
            cleanup();
        }
    }
}

impl fmt::Debug for WatcherBridgeLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WatcherBridgeLease")
            .field("execution_host_id", &self.execution_host_id)
            .field("scope", &self.scope)
            .field("turn", &self.turn)
            .field("generation", &self.generation)
            .field("endpoint", &self.endpoint)
            .field("bearer", &self.bearer)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeEndpoint,
        WatcherBridgeGeneration, WatcherBridgeLease, WatcherBridgeOpenRequest,
    };
    use crate::{RuntimeTurnId, ScopeId};
    use swallowtail_core::ExecutionHostId;

    #[test]
    fn lease_values_redact_endpoint_and_bearer() {
        let endpoint =
            WatcherBridgeEndpoint::new("http://127.0.0.1:9/mcp").expect("endpoint is valid");
        let bearer = WatcherBridgeBearer::new("bridge-secret-token").expect("bearer is valid");
        let lease = WatcherBridgeLease::new(
            ExecutionHostId::new("host.local").expect("host id is valid"),
            ScopeId::new("scope-1").expect("scope is valid"),
            RuntimeTurnId::new("turn-1").expect("turn is valid"),
            WatcherBridgeGeneration::initial(),
            endpoint,
            bearer,
        );

        let debug = format!("{lease:?}");
        assert!(!debug.contains("127.0.0.1"));
        assert!(!debug.contains("bridge-secret-token"));
        assert!(!debug.contains("/mcp"));
        assert_eq!(
            format!("{}", lease.endpoint()),
            "<redacted watcher bridge endpoint>"
        );
        assert_eq!(
            format!("{}", lease.bearer()),
            "<redacted watcher bridge bearer>"
        );
        assert_eq!(lease.endpoint().expose(), "http://127.0.0.1:9/mcp");
        assert!(lease.bearer().matches("bridge-secret-token"));
        assert!(!lease.bearer().matches("other-secret-token"));
    }

    #[test]
    fn open_request_keeps_scope_and_turn() {
        let request = WatcherBridgeOpenRequest::new(
            ScopeId::new("scope-1").expect("scope is valid"),
            RuntimeTurnId::new("turn-1").expect("turn is valid"),
        );
        assert_eq!(request.scope().as_str(), "scope-1");
        assert_eq!(request.turn().as_str(), "turn-1");
    }

    #[test]
    fn frozen_empty_state_allows_successful_completion() {
        let blocked =
            super::WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Open, vec![]);
        let ready =
            super::WatcherBridgeCompletionState::new(WatcherBridgeAdmission::Frozen, Vec::new());
        assert!(!blocked.allows_successful_completion());
        assert!(ready.allows_successful_completion());
    }
}
