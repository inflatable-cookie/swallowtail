use crate::{BoxFuture, RuntimeFailure};
use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Waker;
use swallowtail_core::CancellationScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Immediate acknowledgement of a local cancellation request.
///
/// This is not evidence that provider work has stopped or reached a terminal
/// outcome.
pub enum CancellationAcknowledgement {
    /// This call recorded the first cancellation request.
    Requested,
    /// Cancellation had already been requested.
    AlreadyRequested,
}

/// Operation-scoped cancellation request surface.
pub trait CancellationControl: Send + Sync {
    /// Returns the operation shape to which cancellation applies.
    fn scope(&self) -> CancellationScope;
    /// Requests cancellation without claiming terminal provider truth.
    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>>;
}

/// In-memory idempotent cancellation signal with waiter notification.
pub struct ImmediateCancellation {
    scope: CancellationScope,
    requested: AtomicBool,
    waiter: Mutex<Option<Waker>>,
}

impl fmt::Debug for ImmediateCancellation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ImmediateCancellation")
            .field("scope", &self.scope)
            .field("requested", &self.is_requested())
            .finish()
    }
}

impl ImmediateCancellation {
    #[must_use]
    /// Creates an unrequested signal for an exact cancellation scope.
    pub const fn new(scope: CancellationScope) -> Self {
        Self {
            scope,
            requested: AtomicBool::new(false),
            waiter: Mutex::new(None),
        }
    }

    #[must_use]
    /// Returns whether cancellation has been requested.
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

    /// Resolves when cancellation is first requested.
    pub fn wait_requested(&self) -> BoxFuture<'_, ()> {
        Box::pin(std::future::poll_fn(|context| {
            if self.is_requested() {
                return std::task::Poll::Ready(());
            }
            let mut waiter = self
                .waiter
                .lock()
                .expect("cancellation waiter lock poisoned");
            if self.is_requested() {
                std::task::Poll::Ready(())
            } else {
                *waiter = Some(context.waker().clone());
                std::task::Poll::Pending
            }
        }))
    }
}

impl CancellationControl for ImmediateCancellation {
    fn scope(&self) -> CancellationScope {
        self.scope
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let acknowledgement = if self.requested.swap(true, Ordering::SeqCst) {
            CancellationAcknowledgement::AlreadyRequested
        } else {
            if let Some(waiter) = self
                .waiter
                .lock()
                .expect("cancellation waiter lock poisoned")
                .take()
            {
                waiter.wake();
            }
            CancellationAcknowledgement::Requested
        };
        Box::pin(async move { Ok(acknowledgement) })
    }
}
