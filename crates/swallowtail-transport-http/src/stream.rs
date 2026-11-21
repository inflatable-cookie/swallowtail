use crate::TransportError;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Condvar, Mutex};
use std::task::{Context, Poll, Waker};

/// One buffered streaming item produced by a shared HTTP exchange.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamItem<Item> {
    /// Filtered response headers, emitted once before the first body item.
    Headers(BTreeMap<String, String>),
    /// One decoded body item.
    Body(Item),
}

/// Terminal or per-item failure from a streaming exchange.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamError<Error> {
    /// The bounded HTTP exchange failed before or during delivery.
    Transport(TransportError),
    /// The peer answered with a non-success status.
    ///
    /// Adapters translate the status, headers and buffered error body into
    /// their own provider failure classification.
    Status {
        /// Final HTTP status code reported by the peer.
        status: u32,
        /// Filtered response headers, keyed by lower-cased name.
        headers: BTreeMap<String, String>,
        /// Bounded error body returned with the rejection.
        body: Vec<u8>,
    },
    /// The adapter's own body decoder rejected the stream.
    Decode(Error),
}

/// One incremental body decoder owned by a concrete adapter.
///
/// Implementations keep every provider-specific wire rule; the shared
/// transport owns only the channel, bounds, cancellation and framing.
pub trait BodyDecoder: Send + 'static {
    /// One decoded body item.
    type Item: Send + 'static;
    /// The adapter's decode failure.
    type Error: Send + 'static;

    /// Accepts one transport chunk and returns every complete item.
    ///
    /// # Errors
    ///
    /// Returns the adapter's decode failure when the chunk is inadmissible.
    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error>;

    /// Verifies that the stream ended outside a partial item.
    ///
    /// # Errors
    ///
    /// Returns the adapter's decode failure when the stream ended early.
    fn finish(self) -> Result<(), Self::Error>;
}

struct ChannelState<Item> {
    items: VecDeque<Item>,
    senders: usize,
    closed: bool,
    waker: Option<Waker>,
}

struct ChannelShared<Item> {
    capacity: usize,
    state: Mutex<ChannelState<Item>>,
    available: Condvar,
}

pub(crate) struct ChannelSender<Item> {
    shared: Arc<ChannelShared<Item>>,
}

pub(crate) struct ChannelReceiver<Item> {
    shared: Arc<ChannelShared<Item>>,
}

pub(crate) fn channel<Item>(capacity: usize) -> (ChannelSender<Item>, ChannelReceiver<Item>) {
    let shared = Arc::new(ChannelShared {
        capacity,
        state: Mutex::new(ChannelState {
            items: VecDeque::new(),
            senders: 1,
            closed: false,
            waker: None,
        }),
        available: Condvar::new(),
    });
    (
        ChannelSender {
            shared: Arc::clone(&shared),
        },
        ChannelReceiver { shared },
    )
}

impl<Item> ChannelSender<Item> {
    /// Sends one item, blocking this producer thread while the channel is full.
    ///
    /// Returns `false` when the consumer was dropped, which is the only
    /// condition that abandons a provider stream.
    pub(crate) fn send(&self, item: Item) -> bool {
        let mut state = self
            .shared
            .state
            .lock()
            .expect("shared HTTP channel lock is not poisoned");
        while state.items.len() >= self.shared.capacity && !state.closed {
            state = self
                .shared
                .available
                .wait(state)
                .expect("shared HTTP channel lock is not poisoned");
        }
        if state.closed {
            return false;
        }
        state.items.push_back(item);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
        true
    }
}

impl<Item> Drop for ChannelSender<Item> {
    fn drop(&mut self) {
        let mut state = self
            .shared
            .state
            .lock()
            .expect("shared HTTP channel lock is not poisoned");
        state.senders = state.senders.saturating_sub(1);
        if let Some(waker) = state.waker.take() {
            waker.wake();
        }
    }
}

pub(crate) fn result_channel<Item>() -> (ChannelSender<Item>, ChannelReceiver<Item>) {
    channel(1)
}

impl<Item> ChannelReceiver<Item> {
    /// Receives the one result sent by a finite blocking job.
    pub(crate) async fn recv(mut self) -> Option<Item> {
        std::future::poll_fn(|context| self.poll_next(context)).await
    }

    fn poll_next(&mut self, context: &mut Context<'_>) -> Poll<Option<Item>> {
        let mut state = self
            .shared
            .state
            .lock()
            .expect("shared HTTP channel lock is not poisoned");
        if let Some(item) = state.items.pop_front() {
            self.shared.available.notify_one();
            return Poll::Ready(Some(item));
        }
        if state.senders == 0 {
            return Poll::Ready(None);
        }
        state.waker = Some(context.waker().clone());
        Poll::Pending
    }

    fn try_take(&self) -> Option<Item> {
        let mut state = self
            .shared
            .state
            .lock()
            .expect("shared HTTP channel lock is not poisoned");
        state.items.pop_front()
    }
}

impl<Item> Drop for ChannelReceiver<Item> {
    fn drop(&mut self) {
        let mut state = self
            .shared
            .state
            .lock()
            .expect("shared HTTP channel lock is not poisoned");
        state.closed = true;
        self.shared.available.notify_all();
    }
}

type WorkerFuture = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>>;

/// Bounded, backpressured subscription to one streamed HTTP exchange.
///
/// The producer blocks when the bounded channel is full instead of aborting the
/// provider stream, and dropping the subscription cancels the worker.
pub struct Subscription<Item, Error> {
    receiver: Option<ChannelReceiver<StreamItem<Item>>>,
    terminal: ChannelReceiver<Result<(), StreamError<Error>>>,
    cancelled: Arc<std::sync::atomic::AtomicBool>,
    work: Option<WorkerFuture>,
}

impl<Item, Error> Subscription<Item, Error> {
    pub(crate) fn new(
        receiver: ChannelReceiver<StreamItem<Item>>,
        terminal: ChannelReceiver<Result<(), StreamError<Error>>>,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        work: WorkerFuture,
    ) -> Self {
        Self {
            receiver: Some(receiver),
            terminal,
            cancelled,
            work: Some(work),
        }
    }

    /// Returns the shared cancellation signal for this subscription.
    #[must_use]
    pub fn cancellation(&self) -> Arc<std::sync::atomic::AtomicBool> {
        Arc::clone(&self.cancelled)
    }

    /// Cancels the worker and waits for it to stop.
    ///
    /// # Errors
    ///
    /// Returns any terminal streaming failure other than the cancellation
    /// itself.
    pub async fn close(mut self) -> Result<(), StreamError<Error>> {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::SeqCst);
        drop(self.receiver.take());
        if let Some(work) = self.work.take() {
            work.await;
        }
        match self.terminal.try_take() {
            Some(Ok(())) => Ok(()),
            Some(Err(StreamError::Transport(error))) if error.is_cancelled() => Ok(()),
            Some(Err(error)) => Err(error),
            None => Ok(()),
        }
    }

    /// Polls the next buffered item or terminal failure.
    ///
    /// The subscription is fused: once a terminal outcome has been reported,
    /// every later poll returns [`Poll::Ready`]`(None)`.
    pub fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<StreamItem<Item>, StreamError<Error>>>> {
        let Some(receiver) = self.receiver.as_mut() else {
            return Poll::Ready(None);
        };
        match receiver.poll_next(context) {
            Poll::Ready(Some(item)) => Poll::Ready(Some(Ok(item))),
            Poll::Ready(None) => match self.terminal.poll_next(context) {
                Poll::Ready(Some(Ok(()))) => {
                    self.receiver = None;
                    Poll::Ready(None)
                }
                Poll::Ready(Some(Err(error))) => {
                    self.receiver = None;
                    Poll::Ready(Some(Err(error)))
                }
                Poll::Ready(None) => {
                    self.receiver = None;
                    Poll::Ready(Some(Err(StreamError::Transport(TransportError::Transport))))
                }
                Poll::Pending => Poll::Pending,
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Item, Error> Drop for Subscription<Item, Error> {
    fn drop(&mut self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }
}
