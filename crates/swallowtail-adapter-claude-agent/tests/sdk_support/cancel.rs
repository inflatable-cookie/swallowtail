//! Polling a public future exactly once, then dropping it.
//!
//! Caller cancellation is not a deadline: it drops the future wherever it
//! happens to be suspended. These proofs need that exact shape, so they poll
//! once with a waker that discards notifications and then drop.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// Polls `future` once and returns what that single poll produced.
///
/// The waker discards notifications: the point is the single poll and the drop
/// that follows it, not making progress afterwards.
pub fn poll_once<F: Future>(mut future: Pin<&mut F>) -> Poll<F::Output> {
    let mut context = Context::from_waker(Waker::noop());
    future.as_mut().poll(&mut context)
}

/// Drops `value` on another thread and fails if that drop does not return.
///
/// A synchronous join-on-drop then shows up as a failed assertion naming the
/// value, instead of as a hung test binary.
pub fn drop_within<T: Send + 'static>(what: &str, bound: std::time::Duration, value: T) {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("sdk-fixture-drop".to_owned())
        .spawn(move || {
            drop(value);
            let _ = sender.send(());
        })
        .expect("the fixture can start a dropping thread");
    assert!(
        receiver.recv_timeout(bound).is_ok(),
        "dropping {what} did not return inside {bound:?}: it joined live work instead of \
         handing it to the owning host"
    );
}
