//! Joining a host task inside a caller deadline, without ever blocking the
//! caller's executor thread.
//!
//! `JoinedTask::join` is allowed to be a blocking observation: the local host's
//! handle owns its worker thread, so its join future can park the thread it is
//! polled on, and dropping an unfinished handle joins as well. Racing that
//! future against a deadline is therefore not a bound at all — the first poll
//! can already overrun it.
//!
//! So the wait uses the trait's own non-blocking seam instead:
//! [`JoinedTask::is_finished`] with [`JoinedTask::register_waker`]. `join` is
//! called only once the task reports finished, where it cannot block on task
//! work. The handle itself is held in a slot the bounded wait only borrows, so
//! expiry never drops it.
//!
//! A host that implements neither observation reports `is_finished` as `false`
//! forever, so every join here reports unjoined. That is the fail-closed
//! reading: without the observation there is no evidence the task ended, and
//! this route never reports cleanup truth the host cannot support.
//!
//! # Unresolved: no seam for relinquishing unfinished scoped work
//!
//! A handle that is still unfinished when the caller's deadline expires has
//! nowhere correct to go. Dropping it is the blocking join the bound exists to
//! avoid; waiting on it breaks the bound. `ScopedTaskService` offers only
//! `spawn`, and `JoinedTask` offers no way to hand ownership back, so the host
//! cannot take the work back and reap it.
//!
//! [`park_unjoined`] is the placeholder for that missing seam and **is not**
//! host ownership: it is adapter-process state with no autonomous reaper, and
//! it only releases finished handles when some later expiry parks another one.
//! Contract 019 forbids exactly this shape. It is recorded as the blocking
//! prerequisite for card 055 rather than presented as a solution, and no
//! cleanup evidence depends on it — the guard's ordered cleanup reports itself
//! through its own completion signal, not through a join of this handle.

use crate::sdk::bounded::HostBound;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, OnceLock};
use std::task::{Context, Poll};
use swallowtail_runtime::JoinedTask;

type Slot = Arc<Mutex<Option<Box<dyn JoinedTask>>>>;

/// Resolves when the task in the slot reports finished, using only the trait's
/// non-blocking observation. Dropping this future leaves the handle in the
/// slot, so an expired wait never joins by dropping.
struct TaskFinished(Slot);

impl Future for TaskFinished {
    type Output = ();

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let slot = self.0.lock().expect("SDK task join slot poisoned");
        let Some(task) = slot.as_ref() else {
            return Poll::Ready(());
        };
        if task.is_finished() {
            return Poll::Ready(());
        }
        task.register_waker(context.waker());
        // Registering can race the task's own final notification; the second
        // observation closes that race without sleeping or blocking.
        if task.is_finished() {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

/// Joins `task` inside `bounded`. Reports whether the task was observed
/// finished and joined cleanly; a task still running at the deadline is
/// retained under host ownership and reported as unjoined.
pub(crate) async fn bounded_join(bounded: &HostBound, task: Box<dyn JoinedTask>) -> bool {
    let slot: Slot = Arc::new(Mutex::new(Some(task)));
    let waited = bounded.run(TaskFinished(Arc::clone(&slot))).await;
    let Some(task) = slot.lock().expect("SDK task join slot poisoned").take() else {
        return false;
    };
    if waited.is_none() {
        park_unjoined(task);
        return false;
    }
    task.join().await.is_ok()
}

/// Handles that outlived a caller deadline, held only because there is nowhere
/// correct to put them. See the module note: this is the recorded gap, not a
/// host-ownership mechanism.
fn parked() -> &'static Mutex<Vec<Box<dyn JoinedTask>>> {
    static PARKED: OnceLock<Mutex<Vec<Box<dyn JoinedTask>>>> = OnceLock::new();
    PARKED.get_or_init(|| Mutex::new(Vec::new()))
}

fn park_unjoined(task: Box<dyn JoinedTask>) {
    let finished = {
        let mut parked = parked()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut finished = Vec::new();
        let mut index = 0;
        while index < parked.len() {
            if parked[index].is_finished() {
                finished.push(parked.remove(index));
            } else {
                index += 1;
            }
        }
        parked.push(task);
        finished
    };
    // Released outside the lock: a handle drop is a join, and no join should
    // ever run while this lock is held.
    drop(finished);
}

#[cfg(test)]
mod joins_tests;
