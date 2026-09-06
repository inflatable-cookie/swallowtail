use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

#[derive(Clone, Default)]
pub(crate) struct DeleteResponseGate {
    state: Arc<(Mutex<DeleteResponseState>, Condvar)>,
}

#[derive(Default)]
struct DeleteResponseState {
    dispatched: bool,
    released: bool,
}

/// Large named hang guard for fixture waits that must resolve through
/// explicit test ordering. Expiry is a broken ordering contract, so it fails
/// loudly instead of hanging the run; no passing test relies on this bound.
const HANG_GUARD: Duration = Duration::from_secs(120);

/// Locks the gate without ever panicking. A hang-guard expiry panics while
/// holding the guard, which poisons the mutex; every later gate call still has
/// to work, because teardown must release the server thread rather than raise
/// a second panic on an already-unwinding thread.
fn lock(state: &Mutex<DeleteResponseState>) -> MutexGuard<'_, DeleteResponseState> {
    state.lock().unwrap_or_else(PoisonError::into_inner)
}

impl DeleteResponseGate {
    pub(super) fn mark_dispatched_and_wait_for_release(&self) {
        let (state, changed) = &*self.state;
        let mut state = lock(state);
        state.dispatched = true;
        changed.notify_all();
        let (_state, wait) = changed
            .wait_timeout_while(state, HANG_GUARD, |state| !state.released)
            .unwrap_or_else(PoisonError::into_inner);
        assert!(
            !wait.timed_out(),
            "fixture hang guard: DELETE response was never released within {HANG_GUARD:?}"
        );
    }

    pub(crate) fn wait_for_dispatch(&self) {
        let (state, changed) = &*self.state;
        let state = lock(state);
        let (_state, wait) = changed
            .wait_timeout_while(state, HANG_GUARD, |state| !state.dispatched)
            .unwrap_or_else(PoisonError::into_inner);
        assert!(
            !wait.timed_out(),
            "fixture hang guard: DELETE dispatch was never observed within {HANG_GUARD:?}"
        );
    }

    pub(crate) fn release(&self) {
        let (state, changed) = &*self.state;
        lock(state).released = true;
        changed.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteResponseGate;
    use std::panic::{AssertUnwindSafe, catch_unwind};

    /// A hang-guard expiry panics while holding the gate guard, which poisons
    /// the mutex. Every later gate call still has to work: `FixtureServer::drop`
    /// releases the gate while the test thread is already unwinding, and a
    /// second panic there is non-unwinding and aborts the whole binary.
    #[test]
    fn poisoned_gate_serves_every_call_without_panicking() {
        let gate = DeleteResponseGate::default();
        let (state, _changed) = &*gate.state;
        let poisoned = catch_unwind(AssertUnwindSafe(|| {
            let _guard = state.lock().expect("gate lock is unpoisoned here");
            panic!("poison the gate");
        }));
        assert!(poisoned.is_err(), "the poisoning panic is observed");
        assert!(state.is_poisoned(), "the gate mutex is poisoned");

        catch_unwind(AssertUnwindSafe(|| gate.release()))
            .expect("release must not panic on a poisoned gate");
        catch_unwind(AssertUnwindSafe(|| {
            gate.mark_dispatched_and_wait_for_release()
        }))
        .expect("dispatch mark must not panic on a poisoned gate");
        catch_unwind(AssertUnwindSafe(|| gate.wait_for_dispatch()))
            .expect("dispatch wait must not panic on a poisoned gate");
    }
}
