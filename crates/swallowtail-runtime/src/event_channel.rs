#![deny(missing_docs)]

use crate::{OrderedEventBuffer, RuntimeEvent, RuntimeFailure};
use futures_core::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex, PoisonError};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;

struct EventChannelState {
    buffer: OrderedEventBuffer,
    failure: Option<RuntimeFailure>,
    terminal: bool,
    closed: bool,
    sender_count: usize,
    /// One stream consumer registers here; a wake is never held under the lock.
    waiter: Option<Waker>,
}

/// Cloneable producer for one bounded ordered runtime-event stream.
///
/// The stream closes when the last sender clone is dropped, exactly like an
/// explicit [`RuntimeEventSender::close`], so a producer that dies without
/// closing cannot stall a consumer waiting for the stream to end.
pub struct RuntimeEventSender {
    state: Arc<Mutex<EventChannelState>>,
}

fn locked(state: &Arc<Mutex<EventChannelState>>) -> std::sync::MutexGuard<'_, EventChannelState> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Clone for RuntimeEventSender {
    fn clone(&self) -> Self {
        let mut state = locked(&self.state);
        state.sender_count = state.sender_count.saturating_add(1);
        drop(state);
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl Drop for RuntimeEventSender {
    fn drop(&mut self) {
        let waiter = {
            let mut state = locked(&self.state);
            state.sender_count = state.sender_count.saturating_sub(1);
            if state.sender_count == 0 {
                state.closed = true;
                state.waiter.take()
            } else {
                None
            }
        };
        wake(waiter);
    }
}

impl RuntimeEventSender {
    /// Sends one event or closes the channel with the buffer's semantic failure.
    pub fn send(&self, event: RuntimeEvent) -> Result<(), RuntimeFailure> {
        // The waiter leaves the lock before it is woken: a waker may poll the
        // stream again immediately, and it must never observe a held lock.
        let (waiter, failure) = {
            let mut state = locked(&self.state);
            if state.closed {
                return Err(channel_closed());
            }
            match state.buffer.push(event) {
                Err(failure) => {
                    let runtime_failure = RuntimeFailure::new(failure.diagnostic().clone());
                    if !state.terminal {
                        state.failure = Some(runtime_failure.clone());
                        state.closed = true;
                    }
                    (state.waiter.take(), Some(runtime_failure))
                }
                Ok(()) => (state.waiter.take(), None),
            }
        };
        wake(waiter);
        match failure {
            Some(failure) => Err(failure),
            None => Ok(()),
        }
    }

    /// Marks terminal state after which the stream drains and ends.
    pub fn mark_terminal(&self) {
        let waiter = {
            let mut state = locked(&self.state);
            state.buffer.mark_terminal();
            state.terminal = true;
            state.waiter.take()
        };
        wake(waiter);
    }

    /// Closes the stream without manufacturing terminal outcome truth.
    pub fn close(&self) {
        let waiter = {
            let mut state = locked(&self.state);
            state.closed = true;
            state.waiter.take()
        };
        wake(waiter);
    }
}

/// Bounded ordered stream paired with a [`RuntimeEventSender`].
pub struct RuntimeEventStream {
    state: Arc<Mutex<EventChannelState>>,
}

impl Stream for RuntimeEventStream {
    type Item = Result<RuntimeEvent, RuntimeFailure>;

    fn poll_next(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut state = locked(&self.state);
        if let Some(event) = state.buffer.pop_front() {
            return Poll::Ready(Some(Ok(event)));
        }
        if let Some(failure) = state.failure.take() {
            return Poll::Ready(Some(Err(failure)));
        }
        if state.closed || state.terminal {
            Poll::Ready(None)
        } else {
            register(&mut state.waiter, context.waker());
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
        waiter: None,
    }));
    Ok((
        RuntimeEventSender {
            state: Arc::clone(&state),
        },
        RuntimeEventStream { state },
    ))
}

/// Registers the single stream consumer without replacing a live waker.
fn register(waiter: &mut Option<Waker>, waker: &Waker) {
    match waiter {
        Some(existing) if existing.will_wake(waker) => {}
        Some(existing) => *existing = waker.clone(),
        None => *waiter = Some(waker.clone()),
    }
}

/// Wakes the consumer after every lock is released.
fn wake(waiter: Option<Waker>) {
    if let Some(waiter) = waiter {
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

    /// Waker that records whether the channel lock was free when it ran.
    struct LockProbe {
        state: std::sync::Arc<std::sync::Mutex<super::EventChannelState>>,
        observed: std::sync::Mutex<Vec<bool>>,
    }

    impl std::task::Wake for LockProbe {
        fn wake(self: std::sync::Arc<Self>) {
            let free = self.state.try_lock().is_ok();
            self.observed.lock().expect("probe lock").push(free);
        }
    }

    #[test]
    fn a_woken_consumer_never_observes_the_channel_lock_held() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        let probe = std::sync::Arc::new(LockProbe {
            state: std::sync::Arc::clone(&stream.state),
            observed: std::sync::Mutex::new(Vec::new()),
        });
        let waker = Waker::from(std::sync::Arc::clone(&probe));
        let mut context = Context::from_waker(&waker);
        assert_eq!(Pin::new(&mut stream).poll_next(&mut context), Poll::Pending);

        sender.close();

        let observed = probe.observed.lock().expect("probe lock");
        assert_eq!(observed.len(), 1, "the pending consumer is woken once");
        assert!(
            observed[0],
            "the consumer must be woken after the channel lock is released"
        );
    }

    #[test]
    fn dropped_sender_wakes_a_pending_stream_consumer() {
        let (sender, mut stream) = runtime_event_channel(2).expect("capacity is valid");
        let woken = std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0));
        struct CountingWaker(std::sync::Arc<std::sync::atomic::AtomicUsize>);
        impl std::task::Wake for CountingWaker {
            fn wake(self: std::sync::Arc<Self>) {
                self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
        }
        let waker = Waker::from(std::sync::Arc::new(CountingWaker(std::sync::Arc::clone(
            &woken,
        ))));
        let mut context = Context::from_waker(&waker);
        assert_eq!(Pin::new(&mut stream).poll_next(&mut context), Poll::Pending);
        assert_eq!(woken.load(std::sync::atomic::Ordering::SeqCst), 0);
        drop(sender);
        assert_eq!(
            woken.load(std::sync::atomic::Ordering::SeqCst),
            1,
            "dropping the last sender must wake the registered consumer waker"
        );
        assert_eq!(
            Pin::new(&mut stream).poll_next(&mut context),
            Poll::Ready(None),
            "the drop must wake the pending consumer"
        );
    }
}
