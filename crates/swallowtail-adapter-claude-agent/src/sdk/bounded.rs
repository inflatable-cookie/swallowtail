//! Host-time bounds for every await that touches the sidecar.
//!
//! Contract 019 requires cancellation and deadline to stop SDK work, close
//! owned transport state, and join it, and requires a bounded join to state
//! its bound and escalate on expiry rather than resolving. Nothing here
//! invents a duration: every bound is a `Deadline` the caller already supplied
//! on the request, and expiry is observed through the host's `TimeService`,
//! never a local timer.
//!
//! Close is deliberately absent here: `InteractiveSessionHandle::close` takes
//! no caller deadline, and monotonic tick units are host-defined, so there is
//! no honest way to derive a fresh close bound without a new timing seam.

use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::Poll;
use swallowtail_runtime::{Deadline, TimeService};

/// One host-observed deadline that bounded awaits race against.
#[derive(Clone)]
pub(crate) struct HostBound {
    time: Arc<dyn TimeService>,
    deadline: Deadline,
}

impl HostBound {
    /// Binds an absolute host deadline.
    pub(crate) fn new(time: Arc<dyn TimeService>, deadline: Deadline) -> Self {
        Self { time, deadline }
    }

    /// Reports whether the host clock already passed the bound.
    pub(crate) fn expired(&self) -> bool {
        self.time.now() >= self.deadline.instant()
    }

    /// Runs `future` against the bound. `None` means the bound expired first,
    /// which is never evidence about what the future would have produced.
    pub(crate) async fn run<F: Future>(&self, future: F) -> Option<F::Output> {
        let mut future = pin!(future);
        let mut expiry = self.time.wait_until(self.deadline);
        std::future::poll_fn(|context| {
            if let Poll::Ready(output) = future.as_mut().poll(context) {
                return Poll::Ready(Some(output));
            }
            if expiry.as_mut().poll(context).is_ready() {
                return Poll::Ready(None);
            }
            Poll::Pending
        })
        .await
    }
}
