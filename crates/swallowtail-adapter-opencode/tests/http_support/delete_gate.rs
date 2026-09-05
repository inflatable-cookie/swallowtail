use std::sync::{Arc, Condvar, Mutex};
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

impl DeleteResponseGate {
    pub(super) fn mark_dispatched_and_wait_for_release(&self) {
        let (state, changed) = &*self.state;
        let mut state = state.lock().expect("delete response gate lock poisoned");
        state.dispatched = true;
        changed.notify_all();
        let (_state, wait) = changed
            .wait_timeout_while(state, HANG_GUARD, |state| !state.released)
            .expect("delete response gate lock poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: DELETE response was never released within {HANG_GUARD:?}"
        );
    }

    pub(crate) fn wait_for_dispatch(&self) {
        let (state, changed) = &*self.state;
        let state = state.lock().expect("delete response gate lock poisoned");
        let (_state, wait) = changed
            .wait_timeout_while(state, HANG_GUARD, |state| !state.dispatched)
            .expect("delete response gate lock poisoned");
        assert!(
            !wait.timed_out(),
            "fixture hang guard: DELETE dispatch was never observed within {HANG_GUARD:?}"
        );
    }

    pub(crate) fn release(&self) {
        let (state, changed) = &*self.state;
        state
            .lock()
            .expect("delete response gate lock poisoned")
            .released = true;
        changed.notify_all();
    }
}
