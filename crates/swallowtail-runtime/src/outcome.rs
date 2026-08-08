#![deny(missing_docs)]

use crate::{CallbackId, OperationContent, RuntimeFailure};
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    ExtensionNamespace, OwnedRemoteResourceKind, ProviderRequestRef, SafeDiagnostic,
};

/// Correlation evidence for a provider request that ended the operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRequestObservation {
    callback_id: CallbackId,
    namespace: ExtensionNamespace,
    provider_request_ref: ProviderRequestRef,
}

impl ProviderRequestObservation {
    /// Creates an observation from the callback and exact provider request identities.
    #[must_use]
    pub const fn new(
        callback_id: CallbackId,
        namespace: ExtensionNamespace,
        provider_request_ref: ProviderRequestRef,
    ) -> Self {
        Self {
            callback_id,
            namespace,
            provider_request_ref,
        }
    }

    #[must_use]
    /// Returns the portable callback identity.
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    /// Returns the provider extension namespace.
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    #[must_use]
    /// Returns the representation-aware provider request reference.
    pub const fn provider_request_ref(&self) -> &ProviderRequestRef {
        &self.provider_request_ref
    }
}

/// Exclusive terminal status of one runtime operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalStatus {
    /// Provider work completed normally.
    Completed,
    /// Observation detached while provider work may continue.
    Detached,
    /// The operation ended through cancellation.
    Cancelled,
    /// The operation exceeded its deadline.
    TimedOut,
    /// A provider request was observed but no response exchange was admitted.
    ProviderRequestObserved(ProviderRequestObservation),
    /// The provider reported a safe terminal failure.
    ProviderFailed(SafeDiagnostic),
    /// A host service reported a safe terminal failure.
    HostFailed(SafeDiagnostic),
    /// Runtime validation or coordination reported a safe terminal failure.
    RuntimeFailed(SafeDiagnostic),
}

#[non_exhaustive]
/// Boundary that originated a portable terminal failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TerminalFailureSource {
    /// Failure originated from the provider or harness.
    Provider,
    /// Failure originated from a host service.
    Host,
    /// Failure originated from portable runtime coordination.
    Runtime,
}

/// Borrowed route-neutral view of a terminal failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalFailure<'a> {
    source: TerminalFailureSource,
    diagnostic: &'a SafeDiagnostic,
}

impl<'a> TerminalFailure<'a> {
    /// Returns the boundary that originated the terminal failure.
    #[must_use]
    pub const fn source(self) -> TerminalFailureSource {
        self.source
    }

    #[must_use]
    /// Returns the redacted failure diagnostic.
    pub const fn diagnostic(self) -> &'a SafeDiagnostic {
        self.diagnostic
    }
}

impl TerminalStatus {
    /// Returns a common view for provider, host, and runtime terminal failures.
    #[must_use]
    pub const fn failure(&self) -> Option<TerminalFailure<'_>> {
        let (source, diagnostic) = match self {
            Self::ProviderFailed(diagnostic) => (TerminalFailureSource::Provider, diagnostic),
            Self::HostFailed(diagnostic) => (TerminalFailureSource::Host, diagnostic),
            Self::RuntimeFailed(diagnostic) => (TerminalFailureSource::Runtime, diagnostic),
            Self::Completed
            | Self::Detached
            | Self::Cancelled
            | Self::TimedOut
            | Self::ProviderRequestObserved(_) => return None,
        };
        Some(TerminalFailure { source, diagnostic })
    }
}

/// Result of joining and releasing operation-scoped resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CleanupOutcome {
    /// Every required cleanup action completed.
    Clean,
    /// Cleanup completed with a non-fatal degradation.
    Degraded(SafeDiagnostic),
    /// A required cleanup action failed.
    Failed(SafeDiagnostic),
    /// The operation owned no applicable cleanup action.
    NotApplicable,
}

impl CleanupOutcome {
    /// Returns the safe degradation or failure diagnostic when present.
    #[must_use]
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        match self {
            Self::Degraded(diagnostic) | Self::Failed(diagnostic) => Some(diagnostic),
            Self::Clean | Self::NotApplicable => None,
        }
    }
}

/// Best available truth about provider-side cancellation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderCancellationOutcome {
    /// The provider confirmed cancellation.
    Confirmed,
    /// Cancellation raced with provider completion.
    RacedWithCompletion,
    /// The provider-side result could not be confirmed.
    Unconfirmed,
}

/// Best available truth about deletion of one operation-owned remote resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteResourceDeletionOutcome {
    /// The provider confirmed deletion.
    Confirmed,
    /// Deletion could not be confirmed.
    Unconfirmed,
}

/// Terminal operation result with output, cleanup, and remote-effect truth.
///
/// Terminal status and cleanup outcome remain independent. Cancellation and
/// remote deletion records report only their exact observed strength.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalOutcome {
    status: TerminalStatus,
    cleanup: CleanupOutcome,
    output: Option<OperationContent>,
    provider_cancellation: Option<ProviderCancellationOutcome>,
    remote_resource_deletions: BTreeMap<OwnedRemoteResourceKind, RemoteResourceDeletionOutcome>,
}

impl TerminalOutcome {
    /// Creates a terminal outcome without output or remote-effect evidence.
    #[must_use]
    pub const fn new(status: TerminalStatus, cleanup: CleanupOutcome) -> Self {
        Self {
            status,
            cleanup,
            output: None,
            provider_cancellation: None,
            remote_resource_deletions: BTreeMap::new(),
        }
    }

    #[must_use]
    /// Returns the route-neutral failure view when the status is a failure.
    pub const fn failure(&self) -> Option<TerminalFailure<'_>> {
        self.status.failure()
    }

    #[must_use]
    /// Adds complete potentially sensitive operation output.
    pub fn with_output(mut self, output: OperationContent) -> Self {
        self.output = Some(output);
        self
    }

    #[must_use]
    /// Adds provider-side cancellation truth.
    pub const fn with_provider_cancellation(
        mut self,
        outcome: ProviderCancellationOutcome,
    ) -> Self {
        self.provider_cancellation = Some(outcome);
        self
    }

    #[must_use]
    /// Adds or replaces deletion truth for one remote resource kind.
    pub fn with_remote_resource_deletion(
        mut self,
        resource: OwnedRemoteResourceKind,
        outcome: RemoteResourceDeletionOutcome,
    ) -> Self {
        self.remote_resource_deletions.insert(resource, outcome);
        self
    }

    #[must_use]
    /// Returns the exclusive terminal status.
    pub const fn status(&self) -> &TerminalStatus {
        &self.status
    }

    #[must_use]
    /// Returns the independent cleanup outcome.
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }

    #[must_use]
    /// Returns complete operation output when present.
    pub const fn output(&self) -> Option<&OperationContent> {
        self.output.as_ref()
    }

    #[must_use]
    /// Returns provider-side cancellation truth when applicable.
    pub const fn provider_cancellation(&self) -> Option<ProviderCancellationOutcome> {
        self.provider_cancellation
    }

    #[must_use]
    /// Returns deletion truth for one remote resource kind.
    pub fn remote_resource_deletion(
        &self,
        resource: OwnedRemoteResourceKind,
    ) -> Option<RemoteResourceDeletionOutcome> {
        self.remote_resource_deletions.get(&resource).copied()
    }

    /// Iterates over remote-resource deletion outcomes by resource kind.
    pub fn remote_resource_deletions(
        &self,
    ) -> impl ExactSizeIterator<Item = (OwnedRemoteResourceKind, RemoteResourceDeletionOutcome)> + '_
    {
        self.remote_resource_deletions
            .iter()
            .map(|(resource, outcome)| (*resource, *outcome))
    }
}

struct TerminalState {
    outcome: Option<TerminalOutcome>,
    sender_count: usize,
    waiters: Vec<Waker>,
}

/// Exactly-once completion end of a terminal outcome channel.
///
/// When the last sender clone is dropped without publishing an outcome, the
/// pending future resolves to a `RuntimeFailed` outcome so a producer that
/// died cannot stall its consumer forever.
pub struct TerminalOutcomeSender {
    state: Arc<Mutex<TerminalState>>,
}

impl Clone for TerminalOutcomeSender {
    fn clone(&self) -> Self {
        let mut state = self.state.lock().expect("terminal state lock poisoned");
        state.sender_count = state.sender_count.saturating_add(1);
        drop(state);
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl Drop for TerminalOutcomeSender {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("terminal state lock poisoned");
        state.sender_count = state.sender_count.saturating_sub(1);
        if state.sender_count == 0 && state.outcome.is_none() {
            // Every producer is gone without publishing terminal truth. This
            // is an anomaly; resolve the pending future with a visible
            // failure instead of hanging it forever.
            state.outcome = Some(dropped_without_outcome());
            for waiter in state.waiters.drain(..) {
                waiter.wake();
            }
        }
    }
}

/// Synthesized terminal record for a sender dropped before publishing.
fn dropped_without_outcome() -> TerminalOutcome {
    let diagnostic = SafeDiagnostic::new(
        "swallowtail.terminal_sender_dropped",
        "Operation terminal outcome was not published",
    );
    TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(diagnostic.clone()),
        CleanupOutcome::Failed(diagnostic),
    )
}

impl TerminalOutcomeSender {
    /// Publishes the terminal outcome and wakes all waiters.
    ///
    /// A second completion attempt returns [`TerminalAlreadySet`].
    pub fn complete(&self, outcome: TerminalOutcome) -> Result<(), TerminalAlreadySet> {
        let mut state = self.state.lock().expect("terminal state lock poisoned");
        if state.outcome.is_some() {
            return Err(TerminalAlreadySet);
        }
        state.outcome = Some(outcome);
        for waiter in state.waiters.drain(..) {
            waiter.wake();
        }
        Ok(())
    }
}

/// Future that resolves after the matching sender publishes an outcome.
pub struct TerminalOutcomeFuture {
    state: Arc<Mutex<TerminalState>>,
}

impl Future for TerminalOutcomeFuture {
    type Output = TerminalOutcome;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut state = self.state.lock().expect("terminal state lock poisoned");
        if let Some(outcome) = &state.outcome {
            Poll::Ready(outcome.clone())
        } else {
            if !state
                .waiters
                .iter()
                .any(|waiter| waiter.will_wake(context.waker()))
            {
                state.waiters.push(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

/// Error returned when an operation already has a terminal outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalAlreadySet;

impl From<TerminalAlreadySet> for RuntimeFailure {
    fn from(_: TerminalAlreadySet) -> Self {
        Self::new(SafeDiagnostic::new(
            "swallowtail.terminal_already_set",
            "Operation already has a terminal outcome",
        ))
    }
}

#[must_use]
/// Creates the exactly-once sender and future for one terminal outcome.
pub fn terminal_outcome_channel() -> (TerminalOutcomeSender, TerminalOutcomeFuture) {
    let state = Arc::new(Mutex::new(TerminalState {
        outcome: None,
        sender_count: 1,
        waiters: Vec::new(),
    }));
    (
        TerminalOutcomeSender {
            state: Arc::clone(&state),
        },
        TerminalOutcomeFuture { state },
    )
}

#[cfg(test)]
#[path = "outcome/tests.rs"]
mod tests;
