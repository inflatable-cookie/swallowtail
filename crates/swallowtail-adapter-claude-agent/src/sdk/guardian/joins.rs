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
//! expiry never drops it — dropping is the blocking join this exists to avoid.
//! A task still running at the deadline is retained, which keeps host
//! ownership: nothing is detached, and a retained handle is released as soon as
//! a later retention observes it finished.
//!
//! A host that implements neither observation reports `is_finished` as `false`
//! forever, so every join here retains and reports unjoined. That is the
//! fail-closed reading: without the observation there is no evidence the task
//! ended, and this route never reports cleanup truth the host cannot support.

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
        retain(task);
        return false;
    }
    task.join().await.is_ok()
}

/// Joins `task` only if it has already finished, and otherwise retains it.
/// Used where there is no bound left to spend but a handle must still not be
/// dropped on the calling thread.
pub(crate) async fn join_if_finished(task: Box<dyn JoinedTask>) -> bool {
    if !task.is_finished() {
        retain(task);
        return false;
    }
    task.join().await.is_ok()
}

/// Handles that outlived a caller deadline. They are held, not detached: the
/// host still owns the task, and each retention releases whichever retained
/// handles have since finished, which cannot block.
fn retained() -> &'static Mutex<Vec<Box<dyn JoinedTask>>> {
    static RETAINED: OnceLock<Mutex<Vec<Box<dyn JoinedTask>>>> = OnceLock::new();
    RETAINED.get_or_init(|| Mutex::new(Vec::new()))
}

fn retain(task: Box<dyn JoinedTask>) {
    let finished = {
        let mut retained = retained()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut finished = Vec::new();
        let mut index = 0;
        while index < retained.len() {
            if retained[index].is_finished() {
                finished.push(retained.remove(index));
            } else {
                index += 1;
            }
        }
        retained.push(task);
        finished
    };
    // Released outside the lock: a handle drop is a join, and no join should
    // ever run while this lock is held.
    drop(finished);
}

#[cfg(test)]
mod joins_tests;
