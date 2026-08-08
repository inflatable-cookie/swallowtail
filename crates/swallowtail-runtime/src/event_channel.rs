#![deny(missing_docs)]

use crate::{OrderedEventBuffer, RuntimeEvent, RuntimeFailure};
use futures_core::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;

struct EventChannelState {
    buffer: OrderedEventBuffer,
    failure: Option<RuntimeFailure>,
    terminal: bool,
    closed: bool,
    sender_count: usize,
    waiters: Vec<Waker>,
}

/// Cloneable producer for one bounded ordered runtime-event stream.
///
/// The stream closes when the last sender clone is dropped, exactly like an
/// explicit [`RuntimeEventSender::close`], so a producer that dies without
/// closing cannot stall a consumer waiting for the stream to end.
pub struct RuntimeEventSender {
    state: Arc<Mutex<EventChannelState>>,
}

impl Clone for RuntimeEventSender {
    fn clone(&self) -> Self {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        state.sender_count = state.sender_count.saturating_add(1);
        drop(state);
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl Drop for RuntimeEventSender {
    fn drop(&mut self) {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        state.sender_count = state.sender_count.saturating_sub(1);
        if state.sender_count == 0 {
            state.closed = true;
            wake(&mut state);
        }
    }
}

impl RuntimeEventSender {
    /// Sends one event or closes the channel with the buffer's semantic failure.
    pub fn send(&self, event: RuntimeEvent) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        if state.closed {
            return Err(channel_closed());
        }
        if let Err(failure) = state.buffer.push(event) {
            let runtime_failure = RuntimeFailure::new(failure.diagnostic().clone());
            if !state.terminal {
                state.failure = Some(runtime_failure.clone());
                state.closed = true;
            }
            wake(&mut state);
            return Err(runtime_failure);
        }
        wake(&mut state);
        Ok(())
    }

    /// Marks terminal state after which the stream drains and ends.
    pub fn mark_terminal(&self) {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        state.buffer.mark_terminal();
        state.terminal = true;
        wake(&mut state);
    }

    /// Closes the stream without manufacturing terminal outcome truth.
    pub fn close(&self) {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        state.closed = true;
        wake(&mut state);
    }
}

/// Bounded ordered stream paired with a [`RuntimeEventSender`].
pub struct RuntimeEventStream {
    state: Arc<Mutex<EventChannelState>>,
}

impl Stream for RuntimeEventStream {
    type Item = Result<RuntimeEvent, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = self.state.lock().expect("event channel lock poisoned");
        if let Some(event) = state.buffer.pop_front() {
            return Poll::Ready(Some(Ok(event)));
        }
        if let Some(failure) = state.failure.take() {
            return Poll::Ready(Some(Err(failure)));
        }
        if state.closed || state.terminal {
            Poll::Ready(None)
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

/// Creates one bounded event sender and stream pair.
pub fn runtime_event_channel(
    capacity: usize,
) -> Result<(RuntimeEventSender, RuntimeEventStream), RuntimeFailure> {
    let buffer = OrderedEventBuffer::new(capacity)
        .map_err(|failure| RuntimeFailure::new(failure.diagnostic().clone()))?;
    let state = Arc::new(Mutex::new(EventChannelState {
        buffer,
        failure: None,
        terminal: false,
        closed: false,
        sender_count: 1,
        waiters: Vec::new(),
    }));
    Ok((
        RuntimeEventSender {
            state: Arc::clone(&state),
        },
        RuntimeEventStream { state },
    ))
}

fn wake(state: &mut EventChannelState) {
    for waiter in state.waiters.drain(..) {
        waiter.wake();
    }
}

fn channel_closed() -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(
        "swallowtail.event_channel_closed",
        "Operation event channel is closed",
    ))
}

#[cfg(test)]
mod tests {
    use super::runtime_event_channel;
    use crate::{RuntimeEvent, RuntimeEventKind};
    use futures_core::Stream;
    use std::pin::Pin;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn bounded_stream_preserves_order_and_closes_after_terminal() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        sender
            .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is accepted");
        sender
            .send(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
            .expect("output is accepted");
        sender.mark_terminal();
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 1
        ));
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 2
        ));
        assert_eq!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(None)
        );
    }

    #[test]
    fn semantic_overflow_becomes_a_stream_failure() {
        let (sender, mut stream) = runtime_event_channel(1).expect("capacity is valid");
        sender
            .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is accepted");
        sender
            .send(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
            .expect_err("semantic overflow is rejected");
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 1
        ));
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Err(_)))
        ));
    }

    #[test]
    fn dropping_the_last_sender_closes_the_stream() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        sender
            .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
            .expect("start is accepted");
        let second = sender.clone();
        drop(second);
        let mut context = Context::from_waker(Waker::noop());
        assert!(matches!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(Some(Ok(event))) if event.sequence() == 1
        ));
        assert_eq!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Pending,
            "a remaining sender keeps the stream open"
        );
        drop(sender);
        assert_eq!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(None),
            "the last sender closing resolves the pending stream"
        );
    }

    #[test]
    fn dropped_sender_wakes_a_pending_stream_consumer() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(Pin::new(&mut stream).poll_next(&mut context), Poll::Pending);
        drop(sender);
        assert_eq!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(None),
            "the drop must wake the pending consumer"
        );
    }
}
