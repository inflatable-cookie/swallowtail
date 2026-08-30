//! Bounded lossless watcher lifecycle delivery.

use super::WatcherSnapshot;
use crate::RuntimeFailure;
use futures_core::Stream;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Producer-independent cursor for one turn's retained watcher lifecycle.
///
/// Implementations retain accepted, running, and terminal snapshots until the
/// consumer projects them. Joined cleanup is not a second completed snapshot.
pub trait WatcherLifecycleFeed: Send {
    /// Polls the next retained lifecycle snapshot.
    ///
    /// `Ready(None)` means the feed is closed. `Ready(Some(Err(_)))` fails the
    /// owning operation.
    fn poll_snapshot(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<WatcherSnapshot, RuntimeFailure>>>;
}

/// Subscription to one turn-scoped watcher lifecycle feed.
///
/// Debug formatting is redacted. The subscription does not expose endpoint,
/// command, or process material.
pub struct WatcherLifecycleSubscription {
    inner: Box<dyn WatcherLifecycleFeed>,
}

impl WatcherLifecycleSubscription {
    /// Wraps one host-owned lifecycle feed.
    #[must_use]
    pub fn from_feed(inner: Box<dyn WatcherLifecycleFeed>) -> Self {
        Self { inner }
    }

    /// Polls the next retained lifecycle snapshot.
    pub fn poll_snapshot(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<WatcherSnapshot, RuntimeFailure>>> {
        self.inner.poll_snapshot(context)
    }
}

impl Stream for WatcherLifecycleSubscription {
    type Item = Result<WatcherSnapshot, RuntimeFailure>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_snapshot(context)
    }
}

impl fmt::Debug for WatcherLifecycleSubscription {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WatcherLifecycleSubscription")
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::{WatcherLifecycleFeed, WatcherLifecycleSubscription};
    use crate::RuntimeFailure;
    use std::task::{Context, Poll, Waker};

    struct ClosedFeed;

    impl WatcherLifecycleFeed for ClosedFeed {
        fn poll_snapshot(
            &mut self,
            _context: &mut Context<'_>,
        ) -> Poll<Option<Result<crate::WatcherSnapshot, RuntimeFailure>>> {
            Poll::Ready(None)
        }
    }

    #[test]
    fn subscription_debug_is_redacted_and_closed_feed_ends() {
        let mut subscription = WatcherLifecycleSubscription::from_feed(Box::new(ClosedFeed));
        assert!(!format!("{subscription:?}").contains("feed"));
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        assert!(matches!(
            subscription.poll_snapshot(&mut context),
            Poll::Ready(None)
        ));
    }
}
