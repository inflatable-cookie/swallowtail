use crate::{CallbackId, OperationContent, RuntimeFailure};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::{ExtensionNamespace, ProviderRequestRef, SafeDiagnostic};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalOutcome {
    status: TerminalStatus,
    cleanup: CleanupOutcome,
    output: Option<OperationContent>,
}

impl TerminalOutcome {
    #[must_use]
    pub const fn new(status: TerminalStatus, cleanup: CleanupOutcome) -> Self {
        Self {
            status,
            cleanup,
            output: None,
        }
    }

    #[must_use]
    pub fn with_output(mut self, output: OperationContent) -> Self {
        self.output = Some(output);
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
mod tests {
    use super::{
        CleanupOutcome, ProviderRequestObservation, TerminalOutcome, TerminalStatus,
        terminal_outcome_channel,
    };
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn exactly_one_terminal_outcome_wins() {
        let (sender, _future) = terminal_outcome_channel();
        sender
            .complete(TerminalOutcome::new(
                TerminalStatus::Completed,
                CleanupOutcome::Clean,
            ))
            .expect("first terminal outcome wins");
        sender
            .complete(TerminalOutcome::new(
                TerminalStatus::Cancelled,
                CleanupOutcome::Clean,
            ))
            .expect_err("second terminal outcome must fail");
    }

    #[test]
    fn provider_success_does_not_hide_cleanup_failure() {
        let diagnostic =
            swallowtail_core::SafeDiagnostic::new("fixture.cleanup_failed", "Cleanup failed");
        let outcome = TerminalOutcome::new(
            TerminalStatus::Completed,
            CleanupOutcome::Failed(diagnostic.clone()),
        );

        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Failed(diagnostic));
    }

    #[test]
    fn terminal_future_resolves_to_the_single_winner() {
        let (sender, mut future) = terminal_outcome_channel();
        let expected = TerminalOutcome::new(TerminalStatus::TimedOut, CleanupOutcome::Clean);
        sender
            .complete(expected.clone())
            .expect("terminal outcome completes once");
        let mut context = Context::from_waker(Waker::noop());

        assert_eq!(
            Pin::new(&mut future).poll(&mut context),
            Poll::Ready(expected)
        );
    }

    #[test]
    fn terminal_failure_dimensions_remain_distinct() {
        let diagnostic = swallowtail_core::SafeDiagnostic::new("fixture.failure", "Failed");
        let statuses = [
            TerminalStatus::Completed,
            TerminalStatus::Cancelled,
            TerminalStatus::TimedOut,
            TerminalStatus::ProviderRequestObserved(ProviderRequestObservation::new(
                crate::CallbackId::new("fixture-callback").expect("callback id is valid"),
                swallowtail_core::ExtensionNamespace::new("fixture/provider-request")
                    .expect("namespace is valid"),
                swallowtail_core::ProviderRequestRef::new("provider-request-1")
                    .expect("provider request ref is valid"),
            )),
            TerminalStatus::ProviderFailed(diagnostic.clone()),
            TerminalStatus::HostFailed(diagnostic.clone()),
            TerminalStatus::RuntimeFailed(diagnostic),
        ];

        for (index, status) in statuses.iter().enumerate() {
            assert!(!statuses[index + 1..].contains(status));
        }
    }
}
