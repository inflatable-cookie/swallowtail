//! Cancellation- and deadline-aware waiting shared by Codex operations.

use std::future::Future;
use std::task::Poll;
use swallowtail_runtime::{BoxFuture, DeadlineObservation};

pub(crate) enum Controlled<T> {
    Completed(T),
    Cancelled,
    Deadline,
}

pub(crate) async fn wait_controlled<F, T>(
    operation: F,
    cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> Controlled<T>
where
    F: Future<Output = T>,
{
    wait_controlled_optional(operation, Some(cancellation), deadline).await
}

/// Like [`wait_controlled`], for operations whose caller supplies no
/// cancellation signal: only the deadline can end the wait early.
pub(crate) async fn wait_controlled_optional<F, T>(
    operation: F,
    cancellation: Option<&swallowtail_runtime::ImmediateCancellation>,
    deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> Controlled<T>
where
    F: Future<Output = T>,
{
    let mut operation = Box::pin(operation);
    let mut cancelled = cancellation.map(|cancellation| cancellation.wait_requested());
    let mut deadline = deadline;
    std::future::poll_fn(|context| {
        if let Poll::Ready(result) = operation.as_mut().poll(context) {
            return Poll::Ready(Controlled::Completed(result));
        }
        if cancelled
            .as_mut()
            .is_some_and(|cancelled| cancelled.as_mut().poll(context).is_ready())
        {
            return Poll::Ready(Controlled::Cancelled);
        }
        if deadline
            .as_mut()
            .is_some_and(|deadline| deadline.as_mut().poll(context).is_ready())
        {
            return Poll::Ready(Controlled::Deadline);
        }
        Poll::Pending
    })
    .await
}
