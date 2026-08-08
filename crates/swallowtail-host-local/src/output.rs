use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{ProcessOutputChunk, RuntimeFailure};

#[derive(Default)]
struct OutputInner {
    chunks: VecDeque<ProcessOutputChunk>,
    failure: Option<RuntimeFailure>,
    closed_readers: u8,
    abandoned: bool,
    terminal: bool,
    waiters: Vec<Waker>,
}

#[derive(Default)]
pub(crate) struct OutputState {
    inner: Mutex<OutputInner>,
}

impl OutputState {
    pub(crate) fn push(&self, chunk: ProcessOutputChunk) {
        let mut inner = self
            .inner
            .lock()
            .expect("local process output lock poisoned");
        inner.chunks.push_back(chunk);
        wake_all(&mut inner.waiters);
    }

    pub(crate) fn fail_limit(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("local process output lock poisoned");
        if inner.failure.is_none() {
            inner.failure = Some(failure(
                "swallowtail.local_process.output_limit_exceeded",
                "Local process output exceeded its host-approved limit",
            ));
        }
        wake_all(&mut inner.waiters);
    }

    pub(crate) fn fail_read(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("local process output lock poisoned");
        if inner.failure.is_none() {
            inner.failure = Some(failure(
                "swallowtail.local_process.output_read_failed",
                "Local process output could not be read",
            ));
        }
        wake_all(&mut inner.waiters);
    }

    pub(crate) fn close_reader(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("local process output lock poisoned");
        inner.closed_readers = inner.closed_readers.saturating_add(1);
        wake_all(&mut inner.waiters);
    }

    /// Marks output capture abandoned after the bounded reader-join window.
    ///
    /// Called by the supervisor when a reader thread is still blocked on a
    /// pipe held by a descendant. The stream then drains to terminal truth
    /// even though fewer than both readers have closed.
    pub(crate) fn close_abandoned(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("local process output lock poisoned");
        inner.abandoned = true;
        wake_all(&mut inner.waiters);
    }

    pub(crate) fn read(self: &Arc<Self>) -> OutputFuture {
        OutputFuture {
            state: Arc::clone(self),
        }
    }
}

pub(crate) struct OutputFuture {
    state: Arc<OutputState>,
}

impl Future for OutputFuture {
    type Output = Result<Option<ProcessOutputChunk>, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self
            .state
            .inner
            .lock()
            .expect("local process output lock poisoned");
        if inner.terminal {
            Poll::Ready(Ok(None))
        } else if let Some(chunk) = inner.chunks.pop_front() {
            Poll::Ready(Ok(Some(chunk)))
        } else if let Some(failure) = inner.failure.take() {
            Poll::Ready(Err(failure))
        } else if inner.closed_readers >= 2 || inner.abandoned {
            // Terminal is latched: after the stream reports end, a reader
            // abandoned on a descendant-held pipe can still push chunks or a
            // failure, and those must not resume the stream.
            inner.terminal = true;
            Poll::Ready(Ok(None))
        } else {
            if !inner
                .waiters
                .iter()
                .any(|waiter| waiter.will_wake(context.waker()))
            {
                inner.waiters.push(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

pub(crate) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

fn wake_all(waiters: &mut Vec<Waker>) {
    for waiter in waiters.drain(..) {
        waiter.wake();
    }
}
