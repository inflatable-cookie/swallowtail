use crate::realtime::worker::WorkerUpdate;
use futures_channel::mpsc;
use futures_core::Stream;
use std::future::poll_fn;
use std::pin::Pin;
use std::task::Poll;
use swallowtail_runtime::{BoxFuture, DeadlineObservation};

pub(super) enum Signal {
    Update(WorkerUpdate),
    Closed,
    Deadline,
}

pub(super) async fn next_signal(
    updates: &mut mpsc::Receiver<WorkerUpdate>,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Signal {
    poll_fn(|context| {
        if let Poll::Ready(update) = Pin::new(&mut *updates).poll_next(context) {
            return Poll::Ready(update.map_or(Signal::Closed, Signal::Update));
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Signal::Deadline);
        }
        Poll::Pending
    })
    .await
}
