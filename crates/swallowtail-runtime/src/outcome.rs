use crate::{CallbackId, OperationContent, RuntimeFailure};
use std::collections::BTreeMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    ExtensionNamespace, OwnedRemoteResourceKind, ProviderRequestRef, SafeDiagnostic,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRequestObservation {
    callback_id: CallbackId,
    namespace: ExtensionNamespace,
    provider_request_ref: ProviderRequestRef,
}

impl ProviderRequestObservation {
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
    pub const fn callback_id(&self) -> &CallbackId {
        &self.callback_id
    }

    #[must_use]
    pub const fn namespace(&self) -> &ExtensionNamespace {
        &self.namespace
    }

    #[must_use]
    pub const fn provider_request_ref(&self) -> &ProviderRequestRef {
        &self.provider_request_ref
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TerminalStatus {
    Completed,
    Cancelled,
    TimedOut,
    ProviderRequestObserved(ProviderRequestObservation),
    ProviderFailed(SafeDiagnostic),
    HostFailed(SafeDiagnostic),
    RuntimeFailed(SafeDiagnostic),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CleanupOutcome {
    Clean,
    Degraded(SafeDiagnostic),
    Failed(SafeDiagnostic),
    NotApplicable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderCancellationOutcome {
    Confirmed,
    RacedWithCompletion,
    Unconfirmed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RemoteResourceDeletionOutcome {
    Confirmed,
    Unconfirmed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalOutcome {
    status: TerminalStatus,
    cleanup: CleanupOutcome,
    output: Option<OperationContent>,
    provider_cancellation: Option<ProviderCancellationOutcome>,
    remote_resource_deletions: BTreeMap<OwnedRemoteResourceKind, RemoteResourceDeletionOutcome>,
}

impl TerminalOutcome {
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
    pub fn with_output(mut self, output: OperationContent) -> Self {
        self.output = Some(output);
        self
    }

    #[must_use]
    pub const fn with_provider_cancellation(
        mut self,
        outcome: ProviderCancellationOutcome,
    ) -> Self {
        self.provider_cancellation = Some(outcome);
        self
    }

    #[must_use]
    pub fn with_remote_resource_deletion(
        mut self,
        resource: OwnedRemoteResourceKind,
        outcome: RemoteResourceDeletionOutcome,
    ) -> Self {
        self.remote_resource_deletions.insert(resource, outcome);
        self
    }

    #[must_use]
    pub const fn status(&self) -> &TerminalStatus {
        &self.status
    }

    #[must_use]
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }

    #[must_use]
    pub const fn output(&self) -> Option<&OperationContent> {
        self.output.as_ref()
    }

    #[must_use]
    pub const fn provider_cancellation(&self) -> Option<ProviderCancellationOutcome> {
        self.provider_cancellation
    }

    #[must_use]
    pub fn remote_resource_deletion(
        &self,
        resource: OwnedRemoteResourceKind,
    ) -> Option<RemoteResourceDeletionOutcome> {
        self.remote_resource_deletions.get(&resource).copied()
    }

    pub fn remote_resource_deletions(
        &self,
    ) -> impl ExactSizeIterator<Item = (OwnedRemoteResourceKind, RemoteResourceDeletionOutcome)> + '_
    {
        self.remote_resource_deletions
            .iter()
            .map(|(resource, outcome)| (*resource, *outcome))
    }
}

#[derive(Default)]
struct TerminalState {
    outcome: Option<TerminalOutcome>,
    waiters: Vec<Waker>,
}

#[derive(Clone)]
pub struct TerminalOutcomeSender {
    state: Arc<Mutex<TerminalState>>,
}

impl TerminalOutcomeSender {
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
pub fn terminal_outcome_channel() -> (TerminalOutcomeSender, TerminalOutcomeFuture) {
    let state = Arc::new(Mutex::new(TerminalState::default()));
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
