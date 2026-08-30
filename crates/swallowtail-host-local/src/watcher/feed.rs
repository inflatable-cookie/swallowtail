use super::LocalWatcherState;
use crate::output::failure;
use std::collections::{BTreeMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::WatcherLifecyclePhase;
use swallowtail_runtime::{
    RuntimeFailure, RuntimeTurnId, WatcherLifecycleFeed, WatcherLifecycleSubscription,
    WatcherSnapshot,
};

pub(crate) struct LifecycleBuffer {
    capacity: usize,
    queue: VecDeque<WatcherSnapshot>,
    last_revision: BTreeMap<String, u64>,
    failure: Option<RuntimeFailure>,
    closed: bool,
    waker: Option<Waker>,
}

impl LifecycleBuffer {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            queue: VecDeque::new(),
            last_revision: BTreeMap::new(),
            failure: None,
            closed: false,
            waker: None,
        }
    }

    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    pub(super) fn push(&mut self, snapshot: WatcherSnapshot) -> Result<(), RuntimeFailure> {
        if let Some(failure) = self.failure.clone() {
            return Err(failure);
        }
        if self.closed {
            return Err(feed_closed());
        }
        if snapshot.phase() == WatcherLifecyclePhase::Joined {
            return Ok(());
        }
        let identity = snapshot.watcher_id().as_str().to_owned();
        let revision = snapshot.revision().get();
        if let Some(previous) = self.last_revision.get(&identity) {
            if revision == *previous {
                return Ok(());
            }
            if revision < *previous {
                let failure = feed_regression();
                self.failure = Some(failure.clone());
                self.closed = true;
                self.wake();
                return Err(failure);
            }
        }
        if self.queue.len() >= self.capacity {
            let failure = feed_overflow();
            self.failure = Some(failure.clone());
            self.closed = true;
            self.wake();
            return Err(failure);
        }
        self.last_revision.insert(identity, revision);
        self.queue.push_back(snapshot);
        self.wake();
        Ok(())
    }

    pub(super) fn close(&mut self) {
        self.closed = true;
        self.wake();
    }
}

pub(super) struct LocalLifecycleFeed {
    buffer: Arc<Mutex<LifecycleBuffer>>,
    state: Arc<Mutex<super::LocalWatcherState>>,
    turn: RuntimeTurnId,
}

impl LocalLifecycleFeed {
    pub(super) fn new(
        buffer: Arc<Mutex<LifecycleBuffer>>,
        state: Arc<Mutex<super::LocalWatcherState>>,
        turn: RuntimeTurnId,
    ) -> Self {
        Self {
            buffer,
            state,
            turn,
        }
    }
}

impl Drop for LocalLifecycleFeed {
    fn drop(&mut self) {
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        let owns_current = state
            .feeds
            .get(&self.turn)
            .is_some_and(|current| Arc::ptr_eq(current, &self.buffer));
        if owns_current {
            state.close_feed(&self.turn);
            state.feeds.remove(&self.turn);
        }
    }
}

impl WatcherLifecycleFeed for LocalLifecycleFeed {
    fn poll_snapshot(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<WatcherSnapshot, RuntimeFailure>>> {
        let mut buffer = self
            .buffer
            .lock()
            .expect("watcher lifecycle buffer lock poisoned");
        if let Some(failure) = buffer.failure.clone() {
            buffer.closed = true;
            return Poll::Ready(Some(Err(failure)));
        }
        if let Some(snapshot) = buffer.queue.pop_front() {
            return Poll::Ready(Some(Ok(snapshot)));
        }
        if buffer.closed {
            return Poll::Ready(None);
        }
        buffer.waker = Some(context.waker().clone());
        Poll::Pending
    }
}

impl LocalWatcherState {
    pub(super) fn publish(
        &self,
        turn: &RuntimeTurnId,
        snapshot: WatcherSnapshot,
    ) -> Result<(), RuntimeFailure> {
        let Some(buffer) = self.feeds.get(turn) else {
            return Ok(());
        };
        buffer
            .lock()
            .expect("watcher lifecycle buffer lock poisoned")
            .push(snapshot)
    }

    pub(super) fn close_feed(&self, turn: &RuntimeTurnId) {
        if let Some(buffer) = self.feeds.get(turn) {
            buffer
                .lock()
                .expect("watcher lifecycle buffer lock poisoned")
                .close();
        }
    }
}

impl super::LocalWatcherHostService {
    pub(super) fn open_lifecycle_feed_now(
        &self,
        turn: RuntimeTurnId,
    ) -> Result<WatcherLifecycleSubscription, RuntimeFailure> {
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        if state.is_retired(&turn) {
            return Err(crate::watcher::support::turn_retired_failure());
        }
        if state.feeds.contains_key(&turn) {
            return Err(crate::output::failure(
                "swallowtail.local_watcher.lifecycle_feed_duplicate",
                "Watcher lifecycle feed is already open for this turn",
            ));
        }
        let capacity = self.capacity.saturating_mul(4).max(8);
        let buffer = std::sync::Arc::new(Mutex::new(LifecycleBuffer::new(capacity)));
        state
            .feeds
            .insert(turn.clone(), std::sync::Arc::clone(&buffer));
        Ok(WatcherLifecycleSubscription::from_feed(Box::new(
            LocalLifecycleFeed::new(buffer, std::sync::Arc::clone(&self.state), turn),
        )))
    }

    pub(super) fn close_lifecycle_feed_now(
        &self,
        turn: RuntimeTurnId,
    ) -> Result<(), RuntimeFailure> {
        let mut state = self
            .state
            .lock()
            .expect("local watcher state lock poisoned");
        state.close_feed(&turn);
        state.feeds.remove(&turn);
        Ok(())
    }
}

fn feed_overflow() -> RuntimeFailure {
    failure(
        "swallowtail.local_watcher.lifecycle_feed_overflow",
        "Watcher lifecycle feed exceeded its positive bound",
    )
}

fn feed_closed() -> RuntimeFailure {
    failure(
        "swallowtail.local_watcher.lifecycle_feed_closed",
        "Watcher lifecycle feed is closed",
    )
}

fn feed_regression() -> RuntimeFailure {
    failure(
        "swallowtail.local_watcher.lifecycle_feed_regression",
        "Watcher lifecycle feed observed a regressing revision",
    )
}

#[cfg(test)]
#[path = "feed_buffer.rs"]
mod tests;
