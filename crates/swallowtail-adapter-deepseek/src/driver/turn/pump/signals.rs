use super::super::super::lifecycle::TurnCancellation;
use crate::transport::{StreamItem, Subscription};
use futures_channel::oneshot;
use std::future::Future;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_runtime::{BoxFuture, DeadlineObservation, RuntimeFailure};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum StopSignal {
    Cancelled,
    TimedOut,
}

pub(super) async fn wait_work<T, F>(
    work: F,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> Result<T, WorkFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let mut work = Box::pin(work);
    enum Signal<T> {
        Work(Result<T, RuntimeFailure>),
        Cancel,
        Deadline,
    }
    let signal = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(Signal::Work(result));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(Signal::Cancel);
        }
        if deadline.as_mut().poll(context).is_ready() {
            return Poll::Ready(Signal::Deadline);
        }
        Poll::Pending
    })
    .await;
    match signal {
        Signal::Work(result) => result.map_err(WorkFailure::Runtime),
        Signal::Cancel => {
            let _ = work.await;
            Err(WorkFailure::Stopped(stop_reason(cancellation)))
        }
        Signal::Deadline => {
            cancellation.timeout();
            let _ = work.await;
            Err(WorkFailure::Stopped(StopSignal::TimedOut))
        }
    }
}

pub(super) enum WorkFailure {
    Runtime(RuntimeFailure),
    Stopped(StopSignal),
}

pub(super) enum StreamSignal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed,
    Stopped(StopSignal),
}

pub(super) async fn next_stream_signal(
    subscription: &mut Subscription,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> StreamSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(item) => StreamSignal::Item(item),
                None => StreamSignal::Closed,
            });
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(StreamSignal::Stopped(stop_reason(cancellation)));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(StreamSignal::Stopped(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

pub(super) async fn wait_results(
    receiver: &mut oneshot::Receiver<Vec<swallowtail_runtime::DirectToolResult>>,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> Result<Vec<swallowtail_runtime::DirectToolResult>, StopSignal> {
    poll_fn(|context| {
        if let Poll::Ready(result) = Pin::new(&mut *receiver).poll(context) {
            return Poll::Ready(result.map_err(|_| stop_reason(cancellation)));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(Err(stop_reason(cancellation)));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(Err(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

fn stop_reason(cancellation: &TurnCancellation) -> StopSignal {
    if cancellation.reason() == 2 {
        StopSignal::TimedOut
    } else {
        StopSignal::Cancelled
    }
}

use std::pin::Pin;
