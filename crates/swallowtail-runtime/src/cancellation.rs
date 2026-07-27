use crate::{BoxFuture, RuntimeFailure};
use std::fmt;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Waker;
use swallowtail_core::CancellationScope;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationAcknowledgement {
    Requested,
    AlreadyRequested,
}

pub trait CancellationControl: Send + Sync {
    fn scope(&self) -> CancellationScope;
    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>>;
}

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
    pub const fn new(scope: CancellationScope) -> Self {
        Self {
            scope,
            requested: AtomicBool::new(false),
            waiter: Mutex::new(None),
        }
    }

    #[must_use]
    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }

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
